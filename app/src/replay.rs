//! Optional video-only compressed replay ring. No background capture until explicitly started.
use crate::{
    engine::{self, Capabilities},
    platform,
};
use anyhow::{Context, Result};
use potato_core::{
    config::*,
    video::{self, GpuPath},
};
use std::{
    collections::VecDeque,
    io::{Read, Write},
    path::Path,
    process::Stdio,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

struct Chunk {
    time: Instant,
    data: Arc<Vec<u8>>,
}
#[derive(Default)]
struct Ring {
    chunks: VecDeque<Chunk>,
    bytes: usize,
    error: String,
}
pub struct Replay {
    ring: Arc<Mutex<Ring>>,
    stop: Arc<AtomicBool>,
    join: Option<std::thread::JoinHandle<()>>,
    pub active: Arc<AtomicBool>,
}
impl Default for Replay {
    fn default() -> Self {
        Self::new()
    }
}
impl Replay {
    pub fn new() -> Self {
        Self {
            ring: Arc::new(Mutex::new(Ring::default())),
            stop: Arc::new(AtomicBool::new(false)),
            join: None,
            active: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn start(&mut self, mut c: Config, cap: &Capabilities) -> Result<()> {
        anyhow::ensure!(
            !self.active.load(Ordering::Relaxed),
            "Replay already active"
        );
        c.validate()?;
        platform::prepare_config(&mut c)?;
        platform::ensure_output(&c.output_dir)?;
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
        let encoder = cap.select(&c)?;
        let backend = cap.backend(&c)?;
        let path = if matches!(
            encoder,
            Encoder::Software | Encoder::Mf | Encoder::IntelH264 | Encoder::IntelHevc
        ) {
            GpuPath::CpuTransfer
        } else {
            c.video.transfer
        };
        let mut args = video::recording_args(&c, backend, encoder, path, Path::new("unused"))
            .map_err(anyhow::Error::msg)?;
        let progress = args.iter().position(|a| a == "-progress").unwrap();
        args.drain(progress..progress + 2);
        let output = args
            .iter()
            .position(|a| a == "-cluster_time_limit")
            .unwrap();
        args.truncate(output);
        args.extend([
            "-f".into(),
            "mpegts".into(),
            "-mpegts_flags".into(),
            "+pat_pmt_at_frames".into(),
            "pipe:1".into(),
        ]);
        let mut child = platform::command(&c.ffmpeg)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let _ = platform::child_priority(child.id(), c.performance.priority);
        let mut stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        *self.ring.lock().unwrap() = Ring::default();
        self.stop.store(false, Ordering::Relaxed);
        self.active.store(true, Ordering::Relaxed);
        let ring = self.ring.clone();
        let stop = self.stop.clone();
        let active = self.active.clone();
        self.join = Some(std::thread::spawn(move || {
            let logs = std::thread::spawn(move || {
                use std::io::BufRead;
                let mut tail = VecDeque::new();
                for line in std::io::BufReader::new(stderr)
                    .lines()
                    .map_while(Result::ok)
                {
                    if tail.len() > 15 {
                        tail.pop_front();
                    }
                    tail.push_back(line);
                }
                tail.into_iter().collect::<Vec<_>>().join("\n")
            });
            let ring_reader = ring.clone();
            let reader = std::thread::spawn(move || {
                loop {
                    let mut data = vec![0; 188 * 64];
                    if stdout.read_exact(&mut data).is_err() {
                        break;
                    }
                    let now = Instant::now();
                    let mut r = ring_reader.lock().unwrap();
                    r.bytes += data.len();
                    r.chunks.push_back(Chunk {
                        time: now,
                        data: Arc::new(data),
                    });
                    while r.bytes > c.replay_memory_mb as usize * 1048576
                        || r.chunks.front().is_some_and(|s| {
                            now.duration_since(s.time)
                                > Duration::from_secs(c.replay_seconds as u64 + 2)
                        })
                    {
                        if let Some(old) = r.chunks.pop_front() {
                            r.bytes -= old.data.len();
                        } else {
                            break;
                        }
                    }
                }
            });
            while !stop.load(Ordering::Relaxed) {
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => std::thread::sleep(Duration::from_millis(100)),
                    Err(_) => break,
                }
            }
            let result = engine::stop_child(&mut child);
            let _ = reader.join();
            let log = logs.join().unwrap_or_default();
            let mut r = ring.lock().unwrap();
            if !stop.load(Ordering::Relaxed) {
                r.error = format!("Replay engine stopped. {log}");
            } else if let Err(e) = result {
                r.error = e.to_string();
            }
            active.store(false, Ordering::Relaxed);
        }));
        Ok(())
    }
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
    pub fn stats(&self) -> (f64, usize, String) {
        let r = self.ring.lock().unwrap();
        let duration = if let (Some(first), Some(last)) = (r.chunks.front(), r.chunks.back()) {
            last.time.duration_since(first.time).as_secs_f64()
        } else {
            0.
        };
        (duration, r.bytes, r.error.clone())
    }
    pub fn save(&self, c: &Config) -> Result<Box<dyn FnOnce() -> Result<String> + Send>> {
        let r = self.ring.lock().unwrap();
        anyhow::ensure!(!r.chunks.is_empty(), "Replay has no encoded data yet");
        let now = Instant::now();
        let chunks: Vec<_> = r
            .chunks
            .iter()
            .filter(|chunk| {
                now.duration_since(chunk.time) <= Duration::from_secs(c.replay_seconds as u64 + 2)
            })
            .map(|s| s.data.clone())
            .collect();
        drop(r);
        let ffmpeg = c.ffmpeg.clone();
        let output = c.output_dir.join(format!(
            "Replay-{}.mkv",
            chrono::Local::now().format("%Y%m%d-%H%M%S-%3f")
        ));
        let seconds = c.replay_seconds;
        Ok(Box::new(move || {
            let ts = output.with_extension("snapshot.ts");
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&ts)?;
            for chunk in chunks {
                file.write_all(&chunk)?;
            }
            file.flush()?;
            drop(file);
            // FFmpeg discards packets before the first decodable keyframe; no video re-encoding.
            engine::run_mux(
                &ffmpeg,
                &[
                    "-fflags".into(),
                    "+discardcorrupt".into(),
                    "-i".into(),
                    ts.to_string_lossy().into_owned(),
                    "-map".into(),
                    "0:v:0".into(),
                    "-c:v".into(),
                    "copy".into(),
                    "-t".into(),
                    seconds.to_string(),
                    output.to_string_lossy().into_owned(),
                ],
            )?;
            let info = engine::inspect(&ffmpeg, &output)?;
            anyhow::ensure!(
                info["streams"]
                    .as_array()
                    .context("Replay contains no video")?
                    .iter()
                    .any(|s| s["codec_type"] == "video"),
                "Replay contains no video"
            );
            Ok(format!(
                "Video replay saved: {}. Source snapshot retained at {}",
                output.display(),
                ts.display()
            ))
        }))
    }
}
impl Drop for Replay {
    fn drop(&mut self) {
        self.stop();
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_starts_empty_and_inactive() {
        let r = Replay::new();
        assert!(!r.active.load(Ordering::Relaxed));
        assert_eq!(r.stats().1, 0);
        assert!(r.save(&Config::default()).is_err());
    }
}
