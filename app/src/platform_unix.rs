//! Native Unix integration. Unsupported Windows-only controls are disabled in the UI.
use anyhow::{Context, Result};
use potato_core::config::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
    time::Instant,
};

pub fn command(exe: impl AsRef<std::ffi::OsStr>) -> Command {
    let exe = exe.as_ref();
    #[cfg(target_os = "macos")]
    if exe == "ffmpeg" || exe == "ffprobe" {
        for prefix in ["/opt/homebrew/bin", "/usr/local/bin"] {
            let path = Path::new(prefix).join(exe);
            if path.is_file() {
                return Command::new(path);
            }
        }
    }
    Command::new(exe)
}
pub fn qpc_100ns() -> u64 {
    static START: OnceLock<Instant> = OnceLock::new();
    START
        .get_or_init(Instant::now)
        .elapsed()
        .as_nanos()
        .div_ceil(100) as u64
}
pub fn os_language() -> Language {
    Language::from_code(
        &std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_else(|_| "en".into())
            .split('.')
            .next()
            .unwrap_or("en"),
    )
}
pub fn free_space(path: &Path) -> Result<u64> {
    let path = path.canonicalize()?;
    let o = command("df").arg("-Pk").arg(path).output()?;
    anyhow::ensure!(o.status.success(), "Cannot query free disk space");
    let text = String::from_utf8_lossy(&o.stdout);
    let row = text.lines().last().context("df returned no data")?;
    let blocks: u64 = row
        .split_whitespace()
        .nth(3)
        .context("df free-space field missing")?
        .parse()?;
    Ok(blocks.saturating_mul(1024))
}
pub fn priority(p: Priority) -> Result<()> {
    anyhow::ensure!(
        p == Priority::Normal,
        "Only Normal process priority is supported in the macOS/Linux preview"
    );
    Ok(())
}
pub fn child_priority(_: u32, p: Priority) -> Result<()> {
    priority(p)
}
pub fn priorities() -> &'static [Priority] {
    &[Priority::Normal]
}
pub fn backends() -> &'static [Backend] {
    if cfg!(target_os = "macos") {
        &[Backend::Auto, Backend::AvFoundation]
    } else {
        &[Backend::Auto, Backend::X11]
    }
}
pub fn has_tray() -> bool {
    false
}
pub fn system_audio_supported() -> bool {
    !cfg!(target_os = "macos")
}
pub fn note() -> &'static str {
    if cfg!(target_os = "macos") {
        "Experimental macOS: display capture via AVFoundation, VideoToolbox or software encoding, microphone/additional input. Grant screen and microphone permissions in macOS when requested. Native system loopback, individual-window capture, global shortcuts, tray and process priority controls are unavailable. Use a configured virtual loopback input as an additional device for desktop audio."
    } else {
        "Experimental Linux: X11 display/window capture and PulseAudio/PipeWire-Pulse audio. Wayland capture portals are not implemented; use an Xorg session. Global shortcuts, tray and process priority controls are unavailable. X11 capture/scale uses system memory; no zero-copy claim."
    }
}
pub fn open(path: &Path) -> Result<()> {
    let status = command(if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    })
    .arg(path)
    .status()?;
    anyhow::ensure!(status.success(), "Could not open {}", path.display());
    Ok(())
}
pub fn config_path() -> PathBuf {
    if std::env::args().any(|a| a == "--ui-test") {
        return std::env::current_dir()
            .unwrap_or_else(|_| ".".into())
            .join("test-output/ui-test-settings.json");
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| ".".into());
    let root = if cfg!(target_os = "macos") {
        home.join("Library/Application Support")
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"))
    };
    root.join("potatorec/config.json")
}
pub fn ensure_output(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    anyhow::ensure!(
        free_space(path)? > 512 * 1024 * 1024,
        "Less than 512 MiB free; choose another recording folder"
    );
    Ok(())
}

#[derive(Debug, Clone)]
pub enum NativeEvent {
    Record,
    Pause,
    Mute,
    Show,
    OpenFolder,
    Exit,
    Replay,
    Error(String),
}
pub struct NativeBridge {
    pub rx: crossbeam_channel::Receiver<NativeEvent>,
}
impl NativeBridge {
    pub fn start(_: Hotkeys, _: Language, _: eframe::egui::Context) -> Self {
        let (_, rx) = crossbeam_channel::unbounded();
        Self { rx }
    }
}
#[derive(Debug, Clone, Default)]
pub struct Resources {
    pub cpu_percent: f64,
    pub ram_mb: f64,
}
#[derive(Default)]
pub struct ProcessSampler;
impl ProcessSampler {
    pub fn new() -> Self {
        Self
    }
    pub fn sample(&mut self, pid: Option<u32>) -> Resources {
        let Ok(o) = command("ps")
            .args([
                "-p",
                &pid.unwrap_or_else(std::process::id).to_string(),
                "-o",
                "%cpu=,rss=",
            ])
            .output()
        else {
            return Resources::default();
        };
        let s = String::from_utf8_lossy(&o.stdout);
        let mut fields = s.split_whitespace();
        Resources {
            cpu_percent: fields
                .next()
                .and_then(|x| x.parse::<f64>().ok())
                .unwrap_or(0.)
                / std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1) as f64,
            ram_mb: fields
                .next()
                .and_then(|x| x.parse::<f64>().ok())
                .unwrap_or(0.)
                / 1024.,
        }
    }
}

/// Device enumeration opens no camera, microphone or screen capture stream.
pub fn avfoundation_devices(ffmpeg: &Path) -> Vec<(bool, u32, String)> {
    let Ok(o) = command(ffmpeg)
        .args([
            "-hide_banner",
            "-f",
            "avfoundation",
            "-list_devices",
            "true",
            "-i",
            "",
        ])
        .output()
    else {
        return Vec::new();
    };
    let mut audio = false;
    let mut out = Vec::new();
    for line in String::from_utf8_lossy(&o.stderr).lines() {
        if line.contains("AVFoundation audio devices") {
            audio = true;
            continue;
        }
        if line.contains("AVFoundation video devices") {
            audio = false;
            continue;
        }
        let Some(end) = line.find("] [") else {
            continue;
        };
        let tail = &line[end + 3..];
        let Some((index, name)) = tail.split_once("] ") else {
            continue;
        };
        if let Ok(index) = index.trim_start_matches('[').parse::<u32>() {
            out.push((audio, index, name.to_owned()));
        }
    }
    out
}
pub fn sources() -> Vec<Source> {
    sources_for(Path::new("ffmpeg"))
}
pub fn sources_for(ffmpeg: &Path) -> Vec<Source> {
    if cfg!(target_os = "macos") {
        return avfoundation_devices(ffmpeg)
            .into_iter()
            .filter(|(audio, _, name)| !*audio && name.contains("Capture screen"))
            .map(|(_, device, name)| Source::MacDisplay { device, name })
            .collect();
    }
    let Ok(display) = std::env::var("DISPLAY") else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if let Ok(o) = command("xrandr").arg("--listmonitors").output() {
        for line in String::from_utf8_lossy(&o.stdout).lines().skip(1) {
            let columns: Vec<_> = line.split_whitespace().collect();
            if columns.len() < 3 {
                continue;
            }
            let values: Vec<i32> = columns[2]
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse().ok())
                .collect();
            let dims = match values.as_slice() {
                [w, _, h, _, x, y] => Some((*w, *h, *x, *y)),
                [w, h, x, y] => Some((*w, *h, *x, *y)),
                _ => None,
            };
            if let Some((w, h, x, y)) = dims.filter(|(w, h, _, _)| *w > 0 && *h > 0) {
                out.push(Source::X11Display {
                    display: display.clone(),
                    x,
                    y,
                    width: w as u32,
                    height: h as u32,
                    name: format!("{} · {w} × {h}", columns[1]),
                });
            }
        }
    }
    if out.is_empty() {
        if let Ok(o) = command("xdpyinfo").output() {
            for line in String::from_utf8_lossy(&o.stdout).lines() {
                if let Some(res) = line
                    .trim()
                    .strip_prefix("dimensions:")
                    .and_then(|s| s.split_whitespace().next())
                {
                    if let Some((w, h)) = res
                        .split_once('x')
                        .and_then(|(w, h)| Some((w.parse::<u32>().ok()?, h.parse::<u32>().ok()?)))
                    {
                        out.push(Source::X11Display {
                            display: display.clone(),
                            x: 0,
                            y: 0,
                            width: w,
                            height: h,
                            name: format!("X11 · {w} × {h}"),
                        });
                        break;
                    }
                }
            }
        }
    }
    if let Ok(o) = command("wmctrl").args(["-lpG"]).output() {
        for line in String::from_utf8_lossy(&o.stdout).lines() {
            let c: Vec<_> = line.split_whitespace().collect();
            if c.len() < 9 {
                continue;
            }
            if let (Ok(id), Ok(pid), Ok(width), Ok(height)) = (
                u64::from_str_radix(c[0].trim_start_matches("0x"), 16),
                c[2].parse(),
                c[5].parse(),
                c[6].parse(),
            ) {
                if pid != std::process::id() {
                    out.push(Source::X11Window {
                        id,
                        pid,
                        width,
                        height,
                        title: c[8..].join(" "),
                    });
                }
            }
        }
    }
    out
}
pub fn prepare_config(c: &mut Config) -> Result<()> {
    if matches!(c.video.source, Source::Monitor { .. }) {
        c.video.source = sources_for(&c.ffmpeg).into_iter().next().context(note())?;
    }
    Ok(())
}
pub fn validate_source(source: &Source) -> Result<()> {
    match source {
        Source::X11Display { width, height, .. } => {
            anyhow::ensure!(*width > 0 && *height > 0, "Invalid X11 dimensions")
        }
        Source::X11Window { id, pid, .. } => anyhow::ensure!(
            sources()
                .iter()
                .any(|s| matches!(s,Source::X11Window{id:i,pid:p,..}if i==id&&p==pid)),
            "Selected X11 window no longer exists"
        ),
        Source::MacDisplay { .. } => {}
        _ => anyhow::bail!("Select a capture source for the current operating system"),
    };
    Ok(())
}
pub fn minimized(_: &Source) -> bool {
    false
}
