use crate::{
    audio::{AudioCapture, Stem},
    platform,
};
use anyhow::{Context, Result};
use potato_core::{
    config::*,
    video::{self, GpuPath},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncoderSupport {
    pub encoder: Encoder,
    pub supported: bool,
    pub detail: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Capabilities {
    pub version: String,
    pub wgc: bool,
    pub dxgi: bool,
    pub x11: bool,
    pub avfoundation: bool,
    pub encoders: Vec<EncoderSupport>,
    pub ffmpeg_license: String,
    pub error: Option<String>,
}
impl Capabilities {
    pub fn detect(ffmpeg: &Path) -> Self {
        let run = || -> Result<Self> {
            let version = platform::command(ffmpeg)
                .args(["-hide_banner", "-version"])
                .output()
                .context("FFmpeg not found. Set its executable path in Settings.")?;
            anyhow::ensure!(version.status.success(), "Cannot run FFmpeg");
            let text = String::from_utf8_lossy(&version.stdout);
            let filters = platform::command(ffmpeg)
                .args(["-hide_banner", "-filters"])
                .output()?;
            let filters = String::from_utf8_lossy(&filters.stdout);
            let devices = platform::command(ffmpeg)
                .args(["-hide_banner", "-devices"])
                .output()?;
            let device_text = String::from_utf8_lossy(&devices.stdout);
            let mut out = Self {
                version: text.lines().next().unwrap_or("Unknown FFmpeg").into(),
                wgc: filters.contains("gfxcapture"),
                dxgi: filters.contains("ddagrab"),
                x11: cfg!(target_os = "linux")
                    && device_text.contains("x11grab")
                    && std::env::var_os("DISPLAY").is_some()
                    && std::env::var("XDG_SESSION_TYPE").unwrap_or_default() != "wayland",
                avfoundation: cfg!(target_os = "macos") && device_text.contains("avfoundation"),
                ffmpeg_license: if text.contains("--enable-gpl") {
                    "External FFmpeg build enables GPL; not bundled under the app's MIT license"
                } else {
                    "Check this FFmpeg build's license before redistribution"
                }
                .into(),
                ..Default::default()
            };
            for encoder in Encoder::ALL.iter().copied().filter(|e| {
                *e != Encoder::Auto
                    && if cfg!(windows) {
                        !matches!(e, Encoder::AppleH264 | Encoder::AppleHevc)
                    } else if cfg!(target_os = "macos") {
                        matches!(
                            e,
                            Encoder::AppleH264 | Encoder::AppleHevc | Encoder::Software
                        )
                    } else {
                        matches!(
                            e,
                            Encoder::NvidiaH264 | Encoder::NvidiaHevc | Encoder::Software
                        )
                    }
            }) {
                let mut c = Config::default();
                c.video.encoder = encoder;
                let mut cmd = platform::command(ffmpeg);
                if matches!(encoder, Encoder::IntelH264 | Encoder::IntelHevc) {
                    cmd.args(["-init_hw_device", "qsv=potato:hw,child_device_type=d3d11va"]);
                }
                cmd.args([
                    "-hide_banner",
                    "-loglevel",
                    "error",
                    "-f",
                    "lavfi",
                    "-i",
                    "color=size=128x128:rate=30",
                    "-frames:v",
                    "3",
                ]);
                cmd.args(video::encoder_args(&c, encoder))
                    .args(["-f", "null", "-"])
                    .stdin(Stdio::null());
                let result = run_timeout(cmd, Duration::from_secs(8));
                let (supported, detail) = match result {
                    Ok(o) if o.status.success() => (
                        true,
                        if encoder.hardware() {
                            "Hardware encoder initialized and encoded 3 test frames".into()
                        } else {
                            "Software encoder verified".into()
                        },
                    ),
                    Ok(o) => (
                        false,
                        String::from_utf8_lossy(&o.stderr)
                            .lines()
                            .next()
                            .unwrap_or("Encoder failed")
                            .into(),
                    ),
                    Err(e) => (false, e.to_string()),
                };
                out.encoders.push(EncoderSupport {
                    encoder,
                    supported,
                    detail,
                });
            }
            Ok(out)
        };
        run().unwrap_or_else(|e| Self {
            error: Some(format!("{e:#}")),
            ..Default::default()
        })
    }
    pub fn select(&self, c: &Config) -> Result<Encoder> {
        if let Some(e) = &self.error {
            anyhow::bail!("{e}");
        }
        let supported = |e| self.encoders.iter().any(|s| s.encoder == e && s.supported);
        if c.video.encoder != Encoder::Auto {
            anyhow::ensure!(
                supported(c.video.encoder),
                "Selected encoder failed the hardware test"
            );
            return Ok(c.video.encoder);
        }
        let order = if c.performance.prefer_hardware {
            vec![
                Encoder::AppleH264,
                Encoder::AmdH264,
                Encoder::NvidiaH264,
                Encoder::IntelH264,
                Encoder::Mf,
                Encoder::Software,
            ]
        } else {
            vec![Encoder::Software]
        };
        order
            .into_iter()
            .find(|e| supported(*e))
            .context("No tested H.264 encoder available")
    }
    pub fn backend(&self, c: &Config) -> Result<Backend> {
        let backend = match c.video.backend {
            Backend::Auto => {
                if self.avfoundation {
                    Backend::AvFoundation
                } else if self.x11 {
                    Backend::X11
                } else if self.wgc {
                    Backend::Wgc
                } else {
                    Backend::Dxgi
                }
            }
            b => b,
        };
        anyhow::ensure!(
            match backend {
                Backend::Wgc => self.wgc,
                Backend::Dxgi => self.dxgi,
                Backend::X11 => self.x11,
                Backend::AvFoundation => self.avfoundation,
                Backend::Auto => false,
            },
            "Capture backend is unavailable in this FFmpeg build"
        );
        if backend == Backend::Dxgi {
            anyhow::ensure!(
                matches!(c.video.source, Source::Monitor { .. }),
                "DXGI supports displays only; use WGC for games/windows"
            );
        }
        Ok(backend)
    }
}

fn run_timeout(mut cmd: std::process::Command, limit: Duration) -> Result<std::process::Output> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    let out = child.stdout.take().unwrap();
    let err = child.stderr.take().unwrap();
    let stdout = std::thread::spawn(move || {
        use std::io::Read;
        let mut b = Vec::new();
        let _ = out.take(4 * 1024 * 1024).read_to_end(&mut b);
        b
    });
    let stderr = std::thread::spawn(move || {
        use std::io::Read;
        let mut b = Vec::new();
        let _ = err.take(4 * 1024 * 1024).read_to_end(&mut b);
        b
    });
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break s;
        }
        if start.elapsed() > limit {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout.join();
            let _ = stderr.join();
            anyhow::bail!(
                "FFmpeg operation timed out after {} seconds",
                limit.as_secs()
            );
        }
        std::thread::sleep(Duration::from_millis(40));
    };
    Ok(std::process::Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Idle,
    Starting,
    Recording,
    Paused,
    Finalizing,
    Failed,
}
impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "Ready to record",
            Self::Starting => "Starting capture…",
            Self::Recording => "Recording",
            Self::Paused => "Paused",
            Self::Finalizing => "Finalizing audio tracks…",
            Self::Failed => "Recording needs attention",
        }
    }
}
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub phase: Phase,
    pub message: String,
    pub elapsed: f64,
    pub frames: u64,
    pub encoded_fps: f64,
    pub drops: u64,
    pub size: u64,
    pub write_mbps: f64,
    pub audio_errors: u64,
    pub mic_peak: f32,
    pub encoder: String,
    pub backend: String,
    pub output: Option<PathBuf>,
    pub ffmpeg_pid: Option<u32>,
    pub last_progress: Instant,
}
impl Default for Snapshot {
    fn default() -> Self {
        Self {
            phase: Phase::Idle,
            message: String::new(),
            elapsed: 0.,
            frames: 0,
            encoded_fps: 0.,
            drops: 0,
            size: 0,
            write_mbps: 0.,
            audio_errors: 0,
            mic_peak: 0.,
            encoder: String::new(),
            backend: String::new(),
            output: None,
            ffmpeg_pid: None,
            last_progress: Instant::now(),
        }
    }
}
#[derive(Clone, Copy)]
pub enum Control {
    Stop,
    PauseResume,
}
pub struct Controller {
    pub state: Arc<Mutex<Snapshot>>,
    tx: Option<crossbeam_channel::Sender<Control>>,
    join: Option<std::thread::JoinHandle<()>>,
    pub muted: Arc<AtomicBool>,
}
impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
impl Controller {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(Snapshot::default())),
            tx: None,
            join: None,
            muted: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn snapshot(&self) -> Snapshot {
        self.state.lock().unwrap().clone()
    }
    pub fn active(&self) -> bool {
        matches!(
            self.snapshot().phase,
            Phase::Starting | Phase::Recording | Phase::Paused | Phase::Finalizing
        )
    }
    pub fn start(&mut self, mut c: Config, cap: Capabilities) -> Result<()> {
        anyhow::ensure!(!self.active(), "A recording is already active");
        c.validate()?;
        platform::prepare_config(&mut c)?;
        platform::validate_source(&c.video.source)?;
        platform::ensure_output(&c.output_dir)?;
        cap.select(&c)?;
        cap.backend(&c)?;
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
        *self.state.lock().unwrap() = Snapshot {
            phase: Phase::Starting,
            ..Default::default()
        };
        self.muted.store(c.audio.mic.mute, Ordering::Relaxed);
        let state = self.state.clone();
        let muted = self.muted.clone();
        let (tx, rx) = crossbeam_channel::unbounded();
        self.tx = Some(tx);
        self.join = Some(std::thread::spawn(move || {
            let r = record_session(&c, &cap, &rx, &state, muted);
            let _ = platform::priority(Priority::Normal);
            let mut s = state.lock().unwrap();
            s.ffmpeg_pid = None;
            match r {
                Ok(output) => {
                    s.phase = Phase::Idle;
                    s.output = Some(output);
                    s.message = "Recording finalized. Independent audio tracks are ready.".into();
                }
                Err(e) => {
                    s.phase = Phase::Failed;
                    s.message = format!("{e:#}");
                }
            }
        }));
        Ok(())
    }
    pub fn send(&self, c: Control) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(c);
        }
    }
    pub fn recover(&mut self, path: PathBuf) -> Result<()> {
        anyhow::ensure!(!self.active(), "Stop recording before recovery");
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
        let state = self.state.clone();
        state.lock().unwrap().phase = Phase::Finalizing;
        self.join = Some(std::thread::spawn(move || {
            let result = (|| -> Result<PathBuf> {
                let mut m: Manifest = serde_json::from_slice(&fs::read(&path)?)?;
                finalize(&mut m, &path)?;
                Ok(m.output)
            })();
            let mut s = state.lock().unwrap();
            match result {
                Ok(p) => {
                    s.output = Some(p);
                    s.phase = Phase::Idle;
                    s.message = "Session recovered".into()
                }
                Err(e) => {
                    s.phase = Phase::Failed;
                    s.message = format!("Recovery failed; originals preserved: {e:#}")
                }
            }
        }));
        Ok(())
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        self.send(Control::Stop);
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Clip {
    video: PathBuf,
    stems: Vec<Stem>,
    ready: PathBuf,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Manifest {
    config: Config,
    output: PathBuf,
    clips: Vec<Clip>,
    complete: bool,
}
fn save_manifest(m: &Manifest, path: &Path) -> Result<()> {
    let temp = path.with_extension("json.tmp");
    fs::write(&temp, serde_json::to_vec_pretty(m)?)?;
    fs::rename(temp, path)?;
    Ok(())
}
fn session_folder(c: &Config) -> Result<(PathBuf, PathBuf)> {
    use chrono::format::{Item, StrftimeItems};
    anyhow::ensure!(
        !StrftimeItems::new(&c.filename_format).any(|i| matches!(i, Item::Error)),
        "Invalid filename date format"
    );
    let name = chrono::Local::now().format(&c.filename_format).to_string();
    anyhow::ensure!(
        !name.is_empty() && !name.ends_with(['.', ' ']),
        "Invalid generated filename"
    );
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let folder = c.output_dir.join(format!(".potato-session-{unique}"));
    fs::create_dir(&folder)?;
    let mut output = c.output_dir.join(format!("{name}.{}", c.container.ext()));
    if output.exists() {
        output = c
            .output_dir
            .join(format!("{name}-{unique}.{}", c.container.ext()));
    }
    Ok((folder, output))
}

fn record_session(
    c: &Config,
    cap: &Capabilities,
    rx: &crossbeam_channel::Receiver<Control>,
    state: &Arc<Mutex<Snapshot>>,
    muted: Arc<AtomicBool>,
) -> Result<PathBuf> {
    platform::priority(c.performance.priority)?;
    let encoder = cap.select(c)?;
    let backend = cap.backend(c)?;
    let (folder, output) = session_folder(c)?;
    let manifest_path = folder.join("session.json");
    let mut manifest = Manifest {
        config: c.clone(),
        output: output.clone(),
        clips: Vec::new(),
        complete: false,
    };
    save_manifest(&manifest, &manifest_path)?;
    let pool = Arc::new(
        rayon::ThreadPoolBuilder::new()
            .num_threads(c.performance.worker_count())
            .thread_name(|i| format!("audio-dsp-{i}"))
            .build()?,
    );
    {
        let mut s = state.lock().unwrap();
        s.encoder = encoder.label().into();
        s.backend = format!(
            "{} · {}",
            backend.label(),
            if matches!(backend, Backend::X11 | Backend::AvFoundation) {
                "CPU capture/scaling → encoder"
            } else {
                c.video.transfer.label()
            }
        );
        s.output = Some(output.clone());
    }
    let mut stop = false;
    let mut previous_elapsed = 0.;
    let mut previous_frames = 0;
    while !stop {
        let index = manifest.clips.len();
        let segment = folder.join(format!("segment-{index}"));
        fs::create_dir(&segment)?;
        let epoch = platform::qpc_100ns();
        let wall_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros();
        let mut captures = Vec::new();
        for (name, settings, loopback, filters) in [
            ("Desktop", &c.audio.system, true, Vec::new()),
            ("Microphone", &c.audio.mic, false, c.audio.filters.clone()),
            ("Additional", &c.audio.additional, false, Vec::new()),
        ] {
            if settings.enabled {
                captures.push(AudioCapture::start(
                    name,
                    settings.clone(),
                    loopback,
                    filters,
                    segment.clone(),
                    epoch,
                    pool.clone(),
                    if name == "Microphone" {
                        muted.clone()
                    } else {
                        Arc::new(AtomicBool::new(false))
                    },
                    c.ffmpeg.clone(),
                )?);
            }
        }
        let clip = Clip {
            video: segment.join("video.mkv"),
            stems: captures.iter().map(|a| a.stem.clone()).collect(),
            ready: segment.join("ready.mkv"),
        };
        manifest.clips.push(clip.clone());
        save_manifest(&manifest, &manifest_path)?;
        let path = if encoder == Encoder::Software
            || encoder == Encoder::Mf
            || matches!(encoder, Encoder::IntelH264 | Encoder::IntelHevc)
        {
            GpuPath::CpuTransfer
        } else {
            c.video.transfer
        };
        let mut args = video::recording_args(c, backend, encoder, path, &clip.video)
            .map_err(anyhow::Error::msg)?;
        // WGC makes its first PTS zero. Align that first frame's arrival to our audio epoch,
        // then retain source PTS spacing. Startup alignment is an estimate, not a measured sensor latency.
        let graph = args.iter().position(|a| a == "-filter_complex").unwrap() + 1;
        args[graph] = args[graph].replace(
            "[video]",
            &format!(
                ",setpts='PTS+(st(0,if(eq(N,0),RTCTIME,ld(0)))-{wall_epoch})/1000000/TB'[video]"
            ),
        );
        args.insert(0, "-copyts".into());
        let mut child = platform::command(&c.ffmpeg)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Cannot launch capture engine")?;
        let _ = platform::child_priority(child.id(), c.performance.priority);
        {
            let mut s = state.lock().unwrap();
            s.ffmpeg_pid = Some(child.id());
            s.phase = Phase::Recording;
            s.last_progress = Instant::now();
            s.message = String::new();
        }
        let progress_state = state.clone();
        let stdout = child.stdout.take().unwrap();
        let progress = std::thread::spawn(move || {
            read_progress(stdout, progress_state, previous_elapsed, previous_frames)
        });
        let stderr = child.stderr.take().unwrap();
        let log_path = segment.join("ffmpeg.log");
        let logger = std::thread::spawn(move || -> Result<String> {
            let mut file = File::create(log_path)?;
            let mut tail = std::collections::VecDeque::new();
            for line in BufReader::new(stderr).lines() {
                let line = line?;
                writeln!(file, "{line}")?;
                if tail.len() >= 30 {
                    tail.pop_front();
                }
                tail.push_back(line);
            }
            Ok(tail.into_iter().collect::<Vec<_>>().join("\n"))
        });
        let start = Instant::now();
        let mut pause = false;
        let mut last_space = Instant::now();
        let mut runtime_error = None;
        loop {
            if let Some(status) = child.try_wait()? {
                if !status.success() {
                    runtime_error = Some(format!("Capture engine exited with {status}"));
                } else {
                    state.lock().unwrap().message =
                        "Capture source closed; finalizing available media".into();
                }
                stop = true;
                break;
            }
            if captures.iter().any(|a| a.ended()) {
                runtime_error=Some("Audio device disconnected or processing failed; session preserved for recovery".into());
                stop = true;
                break;
            }
            let mut errors = 0;
            let mut peak = 0.;
            for a in &captures {
                errors += a.metrics.discontinuities.load(Ordering::Relaxed)
                    + a.metrics.dropped_packets.load(Ordering::Relaxed);
                if a.stem.name == "Microphone" {
                    peak = f32::from_bits(a.metrics.peak.load(Ordering::Relaxed));
                }
            }
            {
                let mut s = state.lock().unwrap();
                s.audio_errors = errors;
                s.mic_peak = peak;
                if platform::minimized(&c.video.source) {
                    s.message="Source minimized: Windows may stop delivering frames. Session remains active.".into();
                } else if s.last_progress.elapsed() > Duration::from_secs(5) {
                    s.message="No encoded progress for 5 seconds. Static/minimized sources may not produce frames.".into();
                } else {
                    s.message.clear();
                }
            }
            if last_space.elapsed() > Duration::from_secs(3) {
                if platform::free_space(&c.output_dir).unwrap_or(u64::MAX) < 256 * 1024 * 1024 {
                    runtime_error = Some(
                        "Low disk space; recording stopped. Free space before recovery.".into(),
                    );
                    stop = true;
                    break;
                }
                last_space = Instant::now();
            }
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(Control::Stop) => {
                    stop = true;
                    break;
                }
                Ok(Control::PauseResume) => {
                    pause = true;
                    break;
                }
                Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                    stop = true;
                    break;
                }
                Err(_) => {}
            }
        }
        {
            let mut s = state.lock().unwrap();
            s.phase = Phase::Finalizing;
            s.ffmpeg_pid = None;
        }
        let stop_result = stop_child(&mut child);
        let mut audio_error = None;
        for a in &mut captures {
            if let Err(e) = a.stop() {
                audio_error = Some(format!("{e:#}"));
            }
        }
        let _ = progress.join();
        let log = logger.join().ok().and_then(|r| r.ok()).unwrap_or_default();
        if let Some(e) = runtime_error.or(audio_error) {
            anyhow::bail!("{e}\n{log}\nRecovery manifest: {}", manifest_path.display());
        }
        stop_result?;
        let _ = start;
        previous_elapsed = state.lock().unwrap().elapsed;
        previous_frames = state.lock().unwrap().frames;
        if pause {
            state.lock().unwrap().phase = Phase::Paused;
            if !matches!(rx.recv(), Ok(Control::PauseResume)) {
                stop = true;
            }
        }
    }
    finalize(&mut manifest, &manifest_path).with_context(|| {
        format!(
            "Finalization failed; all source files preserved. Recover using {}",
            manifest_path.display()
        )
    })?;
    Ok(output)
}

pub(crate) fn stop_child(child: &mut Child) -> Result<()> {
    if child.try_wait()?.is_some() {
        return Ok(());
    }
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(b"q\n");
        let _ = stdin.flush();
    }
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            anyhow::ensure!(
                status.success(),
                "Encoder exited with {status}; recovery files preserved"
            );
            return Ok(());
        }
        if start.elapsed() > Duration::from_secs(8) {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!(
                "Encoder did not stop within 8 seconds; incomplete MKV preserved for recovery"
            );
        }
        std::thread::sleep(Duration::from_millis(40));
    }
}

fn read_progress(
    stdout: std::process::ChildStdout,
    state: Arc<Mutex<Snapshot>>,
    base_time: f64,
    base_frames: u64,
) {
    let mut frames = 0u64;
    let mut elapsed = 0.;
    let mut size = 0;
    let mut drops = 0;
    let mut last_size = 0;
    let mut last = Instant::now();
    let mut last_frames = 0;
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "frame" => frames = value.parse().unwrap_or(frames),
                "out_time_us" => elapsed = value.parse::<f64>().unwrap_or(0.) / 1_000_000.,
                "total_size" => size = value.parse().unwrap_or(size),
                "drop_frames" => drops = value.parse().unwrap_or(drops),
                "progress" => {
                    let dt = last.elapsed().as_secs_f64().max(0.001);
                    let mut s = state.lock().unwrap();
                    s.frames = base_frames + frames;
                    s.elapsed = base_time + elapsed;
                    s.size = size;
                    s.encoded_fps = frames.saturating_sub(last_frames) as f64 / dt;
                    s.write_mbps = size.saturating_sub(last_size) as f64 / dt / 1048576.;
                    s.drops = drops;
                    s.last_progress = Instant::now();
                    last = Instant::now();
                    last_size = size;
                    last_frames = frames;
                }
                _ => {}
            }
        }
    }
}

fn ffprobe(ffmpeg: &Path) -> PathBuf {
    if ffmpeg.parent().is_some_and(|p| !p.as_os_str().is_empty()) {
        ffmpeg.with_file_name(if cfg!(windows) {
            "ffprobe.exe"
        } else {
            "ffprobe"
        })
    } else {
        if cfg!(windows) {
            "ffprobe.exe"
        } else {
            "ffprobe"
        }
        .into()
    }
}
pub fn inspect(ffmpeg: &Path, path: &Path) -> Result<serde_json::Value> {
    let output = platform::command(ffprobe(ffmpeg))
        .args([
            "-v",
            "error",
            "-show_streams",
            "-show_format",
            "-of",
            "json",
        ])
        .arg(path)
        .output()?;
    anyhow::ensure!(
        output.status.success(),
        "ffprobe could not read {}: {}",
        path.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}
pub(crate) fn run_mux(ffmpeg: &Path, args: &[String]) -> Result<()> {
    let output = run_timeout(
        {
            let mut c = platform::command(ffmpeg);
            c.args(["-hide_banner", "-loglevel", "error", "-nostdin", "-n"])
                .args(args);
            c
        },
        Duration::from_secs(1800),
    )?;
    anyhow::ensure!(
        output.status.success(),
        "Muxing failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

fn mux_clip(c: &Config, clip: &Clip) -> Result<()> {
    if clip.ready.exists() {
        if inspect(&c.ffmpeg, &clip.ready).is_ok() {
            return Ok(());
        }
        let retained = clip
            .ready
            .with_extension(format!("interrupted-{}.mkv", platform::qpc_100ns()));
        fs::rename(&clip.ready, retained)?;
    }
    let info = inspect(&c.ffmpeg, &clip.video)?;
    let duration = info["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .context("Video duration unavailable")?;
    anyhow::ensure!(duration > 0., "No video frames were captured");
    let mut args = vec![
        "-copyts".into(),
        "-i".into(),
        clip.video.to_string_lossy().into_owned(),
    ];
    for stem in &clip.stems {
        if fs::metadata(&stem.file)?.len() == 0 {
            args.extend([
                "-f".into(),
                "lavfi".into(),
                "-i".into(),
                "anullsrc=r=48000:cl=stereo".into(),
            ]);
        } else {
            args.extend([
                "-f".into(),
                "f32le".into(),
                "-ar".into(),
                "48000".into(),
                "-ac".into(),
                "2".into(),
                "-i".into(),
                stem.file.to_string_lossy().into_owned(),
            ]);
        }
    }
    let mut graph = Vec::new();
    let mut mixes = Vec::new();
    let mut isolated = Vec::new();
    for (i, stem) in clip.stems.iter().enumerate() {
        let count = usize::from(stem.mixed) + usize::from(stem.isolated);
        if count == 0 {
            continue;
        }
        let prefix = format!(
            "[{}:a]atrim=start_sample={},asetpts=PTS-STARTPTS,apad,atrim=duration={duration:.6}",
            i + 1,
            stem.rnnoise_delay_samples
        );
        let mut outputs = Vec::new();
        if stem.mixed {
            let label = format!("m{i}");
            outputs.push(format!("[{label}]"));
            mixes.push(format!("[{label}]"));
        }
        if stem.isolated {
            let label = format!("s{i}");
            outputs.push(format!("[{label}]"));
            isolated.push((label, stem.name.clone()));
        }
        graph.push(format!(
            "{prefix}{}{}",
            if count > 1 {
                format!(",asplit={count}")
            } else {
                String::new()
            },
            outputs.join("")
        ));
    }
    if !mixes.is_empty() {
        graph.push(format!(
            "{}amix=inputs={}:normalize=0,alimiter=limit=0.98:level=0[mixed]",
            mixes.join(""),
            mixes.len()
        ));
    }
    if !graph.is_empty() {
        args.extend(["-filter_complex".into(), graph.join(";")]);
    }
    args.extend(["-map".into(), "0:v:0".into(), "-c:v".into(), "copy".into()]);
    let mut audio_index = 0;
    if !mixes.is_empty() {
        args.extend([
            "-map".into(),
            "[mixed]".into(),
            format!("-metadata:s:a:{audio_index}"),
            "title=Mixed Audio".into(),
        ]);
        audio_index += 1;
    }
    for (label, name) in isolated {
        args.extend([
            "-map".into(),
            format!("[{label}]"),
            format!("-metadata:s:a:{audio_index}"),
            format!("title={name}"),
        ]);
        audio_index += 1;
    }
    args.extend([
        "-c:a".into(),
        "aac".into(),
        "-b:a".into(),
        "192k".into(),
        "-ar".into(),
        "48000".into(),
        "-t".into(),
        format!("{duration:.6}"),
        "-f".into(),
        "matroska".into(),
        clip.ready.to_string_lossy().into_owned(),
    ]);
    run_mux(&c.ffmpeg, &args)?;
    let ready = inspect(&c.ffmpeg, &clip.ready)?;
    let streams = ready["streams"].as_array().context("No output streams")?;
    anyhow::ensure!(
        streams
            .iter()
            .filter(|s| s["codec_type"] == "audio")
            .count()
            == audio_index,
        "Audio track count does not match routing"
    );
    Ok(())
}

fn finalize(manifest: &mut Manifest, path: &Path) -> Result<()> {
    anyhow::ensure!(
        !manifest.complete,
        "Session already finalized; original recording preserved"
    );
    for clip in &manifest.clips {
        mux_clip(&manifest.config, clip)?;
    }
    anyhow::ensure!(!manifest.clips.is_empty(), "Session contains no clips");
    let c = &manifest.config;
    let mut args = Vec::new();
    if manifest.clips.len() == 1 {
        args.extend([
            "-i".into(),
            manifest.clips[0].ready.to_string_lossy().into_owned(),
        ]);
    } else {
        let list = path.parent().unwrap().join("concat.txt");
        let mut text = String::new();
        for (i, _) in manifest.clips.iter().enumerate() {
            text.push_str(&format!("file 'segment-{i}/ready.mkv'\n"));
        }
        fs::write(&list, text)?;
        args.extend([
            "-f".into(),
            "concat".into(),
            "-safe".into(),
            "1".into(),
            "-i".into(),
            list.to_string_lossy().into_owned(),
        ]);
    }
    args.extend(["-map".into(), "0".into(), "-c".into(), "copy".into()]);
    if c.container != Container::Mkv {
        args.extend([
            "-movflags".into(),
            if c.container == Container::FragmentedMp4 {
                "+frag_keyframe+empty_moov+default_base_moof"
            } else {
                "+faststart"
            }
            .into(),
        ]);
    }
    let partial = manifest
        .output
        .with_extension(format!("partial.{}", c.container.ext()));
    if partial.exists() {
        let retained = partial.with_extension(format!(
            "interrupted-{}.{}",
            platform::qpc_100ns(),
            c.container.ext()
        ));
        fs::rename(&partial, retained)?;
    }
    args.push(partial.to_string_lossy().into_owned());
    run_mux(&c.ffmpeg, &args)?;
    inspect(&c.ffmpeg, &partial)?;
    anyhow::ensure!(
        !manifest.output.exists(),
        "Destination already exists; refusing to replace it"
    );
    fs::rename(&partial, &manifest.output)?;
    manifest.complete = true;
    save_manifest(manifest, path)?;
    if c.audio.export_stems {
        let info = inspect(&c.ffmpeg, &manifest.output)?;
        let n = info["streams"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["codec_type"] == "audio")
            .count();
        for i in 0..n {
            let output = manifest.output.with_file_name(format!(
                "{}-track-{}.wav",
                manifest.output.file_stem().unwrap().to_string_lossy(),
                i + 1
            ));
            run_mux(
                &c.ffmpeg,
                &[
                    "-i".into(),
                    manifest.output.to_string_lossy().into_owned(),
                    "-map".into(),
                    format!("0:a:{i}"),
                    "-c:a".into(),
                    "pcm_s24le".into(),
                    "-rf64".into(),
                    "auto".into(),
                    output.to_string_lossy().into_owned(),
                ],
            )?;
        }
    }
    // Preserve capture sources until the user explicitly removes the recovery folder.
    Ok(())
}

pub fn remux(ffmpeg: &Path, input: &Path) -> Result<PathBuf> {
    let out = input.with_file_name(format!(
        "{}-remux.mp4",
        input
            .file_stem()
            .context("Invalid recording filename")?
            .to_string_lossy()
    ));
    run_mux(
        ffmpeg,
        &[
            "-i".into(),
            input.to_string_lossy().into_owned(),
            "-map".into(),
            "0".into(),
            "-c".into(),
            "copy".into(),
            "-movflags".into(),
            "+faststart".into(),
            out.to_string_lossy().into_owned(),
        ],
    )?;
    inspect(ffmpeg, &out)?;
    Ok(out)
}
pub fn recovery_sessions(folder: &Path) -> Vec<PathBuf> {
    fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path().join("session.json"))
        .filter(|p| p.exists())
        .filter(|p| {
            fs::read(p)
                .ok()
                .and_then(|b| serde_json::from_slice::<Manifest>(&b).ok())
                .is_some_and(|m| !m.complete)
        })
        .collect()
}

pub fn cli(args: &[String]) -> Result<()> {
    match args[0].as_str() {
        "--diagnose" => {
            let c = Config::default();
            let cap = Capabilities::detect(&c.ffmpeg);
            println!("{}", serde_json::to_string_pretty(&cap)?);
            anyhow::ensure!(cap.error.is_none(), "FFmpeg is unavailable");
        }
        "--self-test" => {
            let folder = args
                .get(1)
                .context("Usage: --self-test OUTPUT_DIR [--audio] [--mic] [--dxgi] [--pause]")?;
            let mut c = Config {
                output_dir: PathBuf::from(folder),
                ..Default::default()
            };
            c.video.width = 640;
            c.video.height = 360;
            c.video.fps = Rate::new(30, 1)?;
            c.audio.system.enabled = args.iter().any(|a| a == "--audio");
            c.audio.mic.enabled = args.iter().any(|a| a == "--mic");
            if args.iter().any(|a| a == "--rnnoise") {
                c.audio.apply(VoicePreset::Clean);
            }
            if args.iter().any(|a| a == "--dxgi") {
                c.video.backend = Backend::Dxgi;
            }
            let value = |key: &str| {
                args.iter()
                    .position(|a| a == key)
                    .and_then(|i| args.get(i + 1))
            };
            if let Some(rate) = value("--rate") {
                let (n, d) = rate.split_once('/').unwrap_or((rate, "1"));
                c.video.fps = Rate::new(n.parse()?, d.parse()?)?;
            }
            if let Some(res) = value("--resolution") {
                let (w, h) = res.split_once('x').context("Use WIDTHxHEIGHT")?;
                c.video.width = w.parse()?;
                c.video.height = h.parse()?;
            }
            if let Some(hwnd) = value("--hwnd") {
                let hwnd: u64 = hwnd.parse()?;
                c.video.source = platform::sources()
                    .into_iter()
                    .find(|s| matches!(s,Source::Window{hwnd:h,..} if *h==hwnd))
                    .context("Window handle is not available")?;
            }
            if let Some(container) = value("--container") {
                c.container = match container.as_str() {
                    "mkv" => Container::Mkv,
                    "mp4" => Container::Mp4,
                    "fmp4" => Container::FragmentedMp4,
                    _ => anyhow::bail!("Use mkv, mp4 or fmp4"),
                };
            }
            c.audio.export_stems = args.iter().any(|a| a == "--stems");
            let seconds: u64 = value("--seconds")
                .map(|s| s.parse())
                .transpose()?
                .unwrap_or(4);
            anyhow::ensure!((1..=7200).contains(&seconds), "Use 1–7200 test seconds");
            let cap = Capabilities::detect(&c.ffmpeg);
            let mut ctrl = Controller::new();
            ctrl.start(c.clone(), cap)?;
            std::thread::sleep(Duration::from_secs(seconds));
            if args.iter().any(|a| a == "--pause") {
                ctrl.send(Control::PauseResume);
                let t = Instant::now();
                while ctrl.snapshot().phase != Phase::Paused
                    && ctrl.active()
                    && t.elapsed() < Duration::from_secs(15)
                {
                    std::thread::sleep(Duration::from_millis(100));
                }
                ctrl.send(Control::PauseResume);
                std::thread::sleep(Duration::from_secs(3));
            }
            ctrl.send(Control::Stop);
            while ctrl.active() {
                std::thread::sleep(Duration::from_millis(100));
            }
            let s = ctrl.snapshot();
            anyhow::ensure!(s.phase != Phase::Failed, "{}", s.message);
            let out = s.output.context("No output file")?;
            println!(
                "{}",
                serde_json::to_string_pretty(&inspect(&c.ffmpeg, &out)?)?
            );
            println!("RECORDING={}", out.display());
        }
        "--replay-test" => {
            let folder = PathBuf::from(args.get(1).context("Usage: --replay-test OUTPUT_DIR")?);
            let mut c = Config {
                output_dir: folder,
                ..Default::default()
            };
            c.video.width = 640;
            c.video.height = 360;
            c.video.fps = Rate::new(30, 1)?;
            let cap = Capabilities::detect(&c.ffmpeg);
            let mut replay = crate::replay::Replay::new();
            replay.start(c.clone(), &cap)?;
            std::thread::sleep(Duration::from_secs(6));
            let job = replay.save(&c)?;
            let message = job()?;
            replay.stop();
            println!("{message}");
        }
        "--sources" => {
            for source in platform::sources() {
                println!("{}", serde_json::to_string(&source)?);
            }
        }
        "--recover" => {
            let path = PathBuf::from(args.get(1).context("Usage: --recover session.json")?);
            let mut m: Manifest = serde_json::from_slice(&fs::read(&path)?)?;
            finalize(&mut m, &path)?;
            println!("{}", m.output.display());
        }
        "--remux" => {
            let input = PathBuf::from(args.get(1).context("Usage: --remux recording.mkv")?);
            println!(
                "{}",
                remux(
                    Path::new(if cfg!(windows) {
                        "ffmpeg.exe"
                    } else {
                        "ffmpeg"
                    }),
                    &input
                )?
                .display()
            );
        }
        _ => anyhow::bail!(
            "Usage: potato-recorder [--diagnose | --self-test OUTPUT_DIR [--audio] [--mic] [--rnnoise] [--dxgi] [--pause] | --recover session.json | --remux recording.mkv]"
        ),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_audio_tracks_retain_distinct_signals() -> Result<()> {
        // Explicit synthetic test fixture. Production capture never substitutes generated data.
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("test-output")
            .join(format!("potato-routing-test-{}", platform::qpc_100ns()));
        fs::create_dir_all(&folder)?;
        let c = Config::default();
        let video = folder.join("video.mkv");
        run_mux(
            &c.ffmpeg,
            &[
                "-f".into(),
                "lavfi".into(),
                "-i".into(),
                "color=size=128x128:rate=30:duration=2".into(),
                "-c:v".into(),
                "libx264".into(),
                "-threads".into(),
                "1".into(),
                video.to_string_lossy().into_owned(),
            ],
        )?;
        let mut stems = Vec::new();
        for (name, hz) in [("Desktop", 440.), ("Microphone", 880.)] {
            let file = folder.join(format!("{name}.f32le"));
            let mut writer = std::io::BufWriter::new(File::create(&file)?);
            for frame in 0..96000 {
                let x = (std::f32::consts::TAU * hz * frame as f32 / 48000.).sin() * 0.15;
                for _ in 0..2 {
                    writer.write_all(&x.to_le_bytes())?;
                }
            }
            writer.flush()?;
            stems.push(Stem {
                name: name.into(),
                file,
                mixed: true,
                isolated: true,
                channels: 2,
                rnnoise_delay_samples: 0,
            });
        }
        let clip = Clip {
            video,
            stems,
            ready: folder.join("ready.mkv"),
        };
        fs::write(&clip.ready, b"interrupted mux fixture")?;
        mux_clip(&c, &clip)?;
        assert!(
            fs::read_dir(&folder)?
                .flatten()
                .any(|e| e.file_name().to_string_lossy().contains("interrupted-")),
            "Interrupted output must be retained before recovery"
        );
        let info = inspect(&c.ffmpeg, &clip.ready)?;
        let audio: Vec<_> = info["streams"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["codec_type"] == "audio")
            .collect();
        assert_eq!(audio.len(), 3);
        assert_eq!(audio[1]["tags"]["title"], "Desktop");
        assert_eq!(audio[2]["tags"]["title"], "Microphone");
        for (track, expected, other) in [(1, 440f64, 880f64), (2, 880., 440.)] {
            let o = platform::command(&c.ffmpeg)
                .args(["-v", "error", "-i"])
                .arg(&clip.ready)
                .args([
                    "-map",
                    &format!("0:a:{track}"),
                    "-ac",
                    "1",
                    "-f",
                    "f32le",
                    "pipe:1",
                ])
                .output()?;
            anyhow::ensure!(o.status.success(), "Track decode failed");
            let samples: Vec<_> = o
                .stdout
                .as_chunks::<4>()
                .0
                .iter()
                .map(|b| f32::from_le_bytes(*b) as f64)
                .collect();
            let power = |hz: f64| {
                let mut re = 0.;
                let mut im = 0.;
                for (i, x) in samples.iter().enumerate().skip(2048).take(48000) {
                    let a = std::f64::consts::TAU * hz * i as f64 / 48000.;
                    re += x * a.cos();
                    im += x * a.sin();
                }
                re * re + im * im
            };
            assert!(
                power(expected) > 20. * power(other),
                "Isolated track {track} lost source independence"
            );
        }
        // Fixture retained in TEMP for inspection after failure or success.
        Ok(())
    }
}
