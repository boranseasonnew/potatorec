//! FFmpeg device audio on Unix, with the same local real-time DSP used on Windows.
use crate::{audio_dsp::Node, platform};
use anyhow::{Context, Result};
use potato_core::config::{AudioSource, Filter, FilterKind};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};
#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub render: bool,
}
pub fn devices() -> Result<Vec<Device>> {
    devices_for(Path::new("ffmpeg"))
}
pub fn devices_for(ffmpeg: &Path) -> Result<Vec<Device>> {
    if cfg!(target_os = "macos") {
        return Ok(platform::avfoundation_devices(ffmpeg)
            .into_iter()
            .filter(|(audio, _, _)| *audio)
            .map(|(_, id, name)| Device {
                id: id.to_string(),
                name,
                render: false,
            })
            .collect());
    }
    let o = platform::command("pactl")
        .args(["list", "short", "sources"])
        .output()
        .context("Install pulseaudio-utils; a PulseAudio/PipeWire-Pulse server is required")?;
    anyhow::ensure!(
        o.status.success(),
        "PulseAudio/PipeWire-Pulse is unavailable"
    );
    Ok(String::from_utf8_lossy(&o.stdout)
        .lines()
        .filter_map(|line| {
            let id = line.split_whitespace().nth(1)?.to_owned();
            Some(Device {
                render: id.ends_with(".monitor"),
                name: id.clone(),
                id,
            })
        })
        .collect())
}
#[derive(Default)]
pub struct AudioMetrics {
    pub discontinuities: AtomicU64,
    pub dropped_packets: AtomicU64,
    pub peak: AtomicU32,
    pub frames: AtomicU64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stem {
    pub name: String,
    pub file: PathBuf,
    pub mixed: bool,
    pub isolated: bool,
    pub channels: u16,
    pub rnnoise_delay_samples: u32,
}
struct Packet {
    frame: u64,
    samples: Vec<f32>,
}
pub struct AudioCapture {
    child: Child,
    capture: Option<JoinHandle<Result<()>>>,
    writer: Option<JoinHandle<Result<()>>>,
    pub metrics: Arc<AudioMetrics>,
    pub stem: Stem,
}
impl AudioCapture {
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        name: &str,
        settings: AudioSource,
        loopback: bool,
        filters: Vec<Filter>,
        folder: PathBuf,
        epoch: u64,
        pool: Arc<rayon::ThreadPool>,
        muted: Arc<AtomicBool>,
        ffmpeg: PathBuf,
    ) -> Result<Self> {
        let file = folder.join(format!("{name}.f32le"));
        let output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file)?;
        let stem = Stem {
            name: name.into(),
            file,
            mixed: settings.mixed,
            isolated: settings.isolated,
            channels: 2,
            rnnoise_delay_samples: if filters
                .iter()
                .any(|f| f.enabled && f.kind == FilterKind::Suppression)
            {
                960
            } else {
                0
            },
        };
        let mut args = vec!["-hide_banner".into(), "-loglevel".into(), "error".into()];
        if cfg!(target_os = "macos") {
            anyhow::ensure!(
                !loopback,
                "macOS AVFoundation has no system loopback; disable Desktop Audio and select a configured virtual device as Additional Input"
            );
            let device = if settings.device_id.is_empty() {
                "0"
            } else {
                &settings.device_id
            };
            let index: u32 = device
                .parse()
                .context("Select a valid AVFoundation microphone/input index")?;
            args.extend([
                "-f".into(),
                "avfoundation".into(),
                "-i".into(),
                format!("none:{index}"),
            ]);
        } else {
            let device = if !settings.device_id.is_empty() {
                settings.device_id.clone()
            } else if loopback {
                let o = platform::command("pactl")
                    .arg("get-default-sink")
                    .output()?;
                anyhow::ensure!(o.status.success(), "Cannot find the default audio output");
                format!("{}.monitor", String::from_utf8_lossy(&o.stdout).trim())
            } else {
                "default".into()
            };
            anyhow::ensure!(
                !loopback || device.ends_with(".monitor"),
                "Desktop audio requires a monitor source, not a microphone"
            );
            args.extend(["-f".into(), "pulse".into(), "-i".into(), device]);
        }
        args.extend([
            "-vn".into(),
            "-ar".into(),
            "48000".into(),
            "-ac".into(),
            "2".into(),
            "-f".into(),
            "f32le".into(),
            "pipe:1".into(),
        ]);
        let log = std::fs::File::create(folder.join(format!("{name}-capture.log")))?;
        let mut child = platform::command(ffmpeg)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(log)
            .spawn()?;
        let mut stdout = child.stdout.take().unwrap();
        let metrics = Arc::new(AudioMetrics::default());
        let (tx, rx) = crossbeam_channel::bounded::<Packet>(32);
        let (ready_tx, ready_rx) = crossbeam_channel::bounded(1);
        let m = metrics.clone();
        let capture = std::thread::spawn(move || -> Result<()> {
            let mut frame = 0u64;
            let mut first = true;
            loop {
                let mut bytes = vec![0u8; 480 * 8];
                match stdout.read_exact(&mut bytes) {
                    Ok(()) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(e.into()),
                };
                if first {
                    frame = platform::qpc_100ns()
                        .saturating_sub(epoch)
                        .saturating_mul(48000)
                        / 10_000_000;
                    let _ = ready_tx.send(());
                    first = false;
                }
                let samples = bytes
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|b| f32::from_le_bytes(*b))
                    .collect();
                if tx.try_send(Packet { frame, samples }).is_err() {
                    m.dropped_packets.fetch_add(1, Ordering::Relaxed);
                }
                frame += 480;
            }
            Ok(())
        });
        let m = metrics.clone();
        let writer = std::thread::spawn(move || -> Result<()> {
            let mut output = BufWriter::with_capacity(128 * 1024, output);
            let mut cursor = 0u64;
            let mut chains: [Vec<Node>; 2] = std::array::from_fn(|_| {
                filters
                    .iter()
                    .filter(|f| f.enabled)
                    .map(|f| Node::new(f.clone()))
                    .collect()
            });
            let mut channels = [Vec::new(), Vec::new()];
            let mut encoded = Vec::new();
            let zeros = [0u8; 8192];
            while let Ok(packet) = rx.recv() {
                let gap = packet.frame.saturating_sub(cursor);
                anyhow::ensure!(
                    gap <= 48000 * 30,
                    "Audio startup or clock gap exceeded 30 seconds"
                );
                let mut pad = gap as usize * 8;
                while pad > 0 {
                    let n = pad.min(zeros.len());
                    output.write_all(&zeros[..n])?;
                    pad -= n;
                }
                cursor += gap;
                for channel in &mut channels {
                    channel.clear();
                }
                let gain = 10f32.powf(settings.gain_db / 20.);
                for pair in packet.samples.as_chunks::<2>().0 {
                    let pair = if settings.mono {
                        [(pair[0] + pair[1]) * 0.5; 2]
                    } else {
                        *pair
                    };
                    for ch in 0..2 {
                        channels[ch].push(if settings.mute || muted.load(Ordering::Relaxed) {
                            0.
                        } else {
                            (pair[ch] * gain).clamp(-1., 1.)
                        });
                    }
                }
                if !filters.is_empty() {
                    pool.install(|| {
                        chains.par_iter_mut().zip(channels.par_iter_mut()).for_each(
                            |(chain, data)| {
                                for node in chain {
                                    node.process(data);
                                }
                            },
                        )
                    });
                }
                let mut peak = 0f32;
                encoded.clear();
                for i in 0..channels[0].len() {
                    for channel in &channels {
                        let x = if channel[i].is_finite() {
                            channel[i]
                        } else {
                            0.
                        };
                        peak = peak.max(x.abs());
                        encoded.extend_from_slice(&x.to_le_bytes());
                    }
                }
                output.write_all(&encoded)?;
                cursor += 480;
                m.peak.store(peak.to_bits(), Ordering::Relaxed);
                m.frames.store(cursor, Ordering::Relaxed);
            }
            output.flush()?;
            output.get_ref().sync_data()?;
            Ok(())
        });
        let mut result = Self {
            child,
            capture: Some(capture),
            writer: Some(writer),
            metrics,
            stem,
        };
        if ready_rx.recv_timeout(Duration::from_secs(15)).is_err() {
            let _ = result.stop();
            anyhow::bail!(
                "Audio did not start. Check device selection/OS microphone permission and the capture log."
            );
        }
        Ok(result)
    }
    pub fn stop(&mut self) -> Result<()> {
        let status = if self.child.try_wait()?.is_some() {
            Ok(())
        } else {
            crate::engine::stop_child(&mut self.child)
        };
        if let Some(h) = self.capture.take() {
            h.join()
                .map_err(|_| anyhow::anyhow!("Audio capture thread panicked"))??;
        }
        if let Some(h) = self.writer.take() {
            h.join()
                .map_err(|_| anyhow::anyhow!("Audio DSP/writer thread panicked"))??;
        }
        status
    }
    pub fn ended(&self) -> bool {
        self.capture.as_ref().is_some_and(|h| h.is_finished())
            || self.writer.as_ref().is_some_and(|h| h.is_finished())
    }
}
impl Drop for AudioCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
