use crate::config::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpuPath {
    Direct,
    D3d11Nv12,
    AmfNv12,
    CpuTransfer,
}
impl GpuPath {
    pub fn label(self) -> &'static str {
        match self {
            Self::Direct => "D3D11 surfaces → hardware encoder",
            Self::D3d11Nv12 => "D3D11 GPU conversion → encoder",
            Self::AmfNv12 => "AMF GPU conversion → encoder",
            Self::CpuTransfer => "GPU → RAM → encoder (compatibility path)",
        }
    }
}

pub fn capture_filter(c: &Config, backend: Backend) -> Result<String, String> {
    let v = &c.video;
    let fps = v.fps.to_string();
    if backend == Backend::Dxgi {
        let Source::Monitor { index, .. } = v.source else {
            return Err("DXGI captures displays only; select WGC for a game/window".into());
        };
        Ok(format!(
            "ddagrab=output_idx={index}:framerate={fps}:draw_mouse={}:dup_frames=0",
            u8::from(v.cursor)
        ))
    } else {
        let source = match v.source {
            Source::Monitor { index, .. } => format!("monitor_idx={index}"),
            Source::Window { hwnd, .. } => format!("hwnd={hwnd}"),
            _ => return Err("Select a Windows capture source for WGC".into()),
        };
        let (w, h) = if v.width == 0 {
            (-2, -2)
        } else {
            (v.width as i32, v.height as i32)
        };
        Ok(format!(
            "gfxcapture={source}:max_framerate={fps}:capture_cursor={}:width={w}:height={h}:resize_mode=scale_aspect:scale_mode={}",
            u8::from(v.cursor),
            v.scaling.ffmpeg()
        ))
    }
}

pub fn encoder_args(c: &Config, encoder: Encoder) -> Vec<String> {
    let v = &c.video;
    let mut a = vec!["-c:v".into(), encoder.codec().into()];
    let mut put = |k: &str, v: String| {
        a.push(k.into());
        a.push(v);
    };
    let qp = v.quality.qp();
    let speed = v.speed;
    match encoder {
        Encoder::AmdH264 | Encoder::AmdHevc => {
            put(
                "-quality",
                match speed {
                    Speed::UltraFast | Speed::Fast => "speed",
                    Speed::Balanced => "balanced",
                    _ => "quality",
                }
                .into(),
            );
            put("-async_depth", c.performance.buffer_count().to_string());
            put(
                "-rc",
                match v.rate_control {
                    RateControl::Cqp => "cqp",
                    RateControl::Cbr => "cbr",
                    RateControl::Vbr => "vbr_peak",
                }
                .into(),
            );
            if v.rate_control == RateControl::Cqp {
                put("-qp_i", qp.to_string());
                put("-qp_p", qp.to_string());
            }
        }
        Encoder::NvidiaH264 | Encoder::NvidiaHevc => {
            put(
                "-preset",
                match speed {
                    Speed::UltraFast => "p1",
                    Speed::Fast => "p2",
                    Speed::Balanced => "p4",
                    Speed::Quality => "p5",
                    Speed::Maximum => "p7",
                }
                .into(),
            );
            put(
                "-rc",
                if v.rate_control == RateControl::Cbr {
                    "cbr"
                } else {
                    "vbr"
                }
                .into(),
            );
            if v.rate_control == RateControl::Cqp {
                put("-cq", qp.to_string());
                put("-b:v", "0".into());
            }
            put("-rc-lookahead", "0".into());
        }
        Encoder::IntelH264 | Encoder::IntelHevc => {
            put(
                "-preset",
                match speed {
                    Speed::UltraFast => "veryfast",
                    Speed::Fast => "faster",
                    Speed::Balanced => "medium",
                    Speed::Quality => "slow",
                    Speed::Maximum => "veryslow",
                }
                .into(),
            );
            if v.rate_control == RateControl::Cqp {
                put("-global_quality", qp.to_string());
            }
            put("-look_ahead", "0".into());
        }
        Encoder::Software => {
            put(
                "-preset",
                match speed {
                    Speed::UltraFast => "ultrafast",
                    Speed::Fast => "superfast",
                    Speed::Balanced => "veryfast",
                    Speed::Quality => "fast",
                    Speed::Maximum => "medium",
                }
                .into(),
            );
            put("-threads:v", c.performance.worker_count().to_string());
            if v.rate_control == RateControl::Cqp {
                put("-crf", qp.to_string());
            }
        }
        Encoder::Mf => {
            put("-hw_encoding", "1".into());
            put(
                "-rate_control",
                match v.rate_control {
                    RateControl::Cqp => "quality",
                    RateControl::Cbr => "cbr",
                    RateControl::Vbr => "pc_vbr",
                }
                .into(),
            );
            if v.rate_control == RateControl::Cqp {
                put("-quality", (100 - qp * 2).to_string());
            }
            put("-b:v", (v.bitrate_kbps * 1000).to_string());
        }
        Encoder::AppleH264 | Encoder::AppleHevc => {
            put("-allow_sw", "0".into());
            put("-realtime", "1".into());
            put(
                "-prio_speed",
                if matches!(speed, Speed::UltraFast | Speed::Fast) {
                    "1"
                } else {
                    "0"
                }
                .into(),
            );
            if v.rate_control == RateControl::Cqp {
                put("-q:v", (100 - qp * 2).to_string());
            }
        }
        Encoder::Auto => {}
    }
    if v.rate_control != RateControl::Cqp {
        put("-b:v", (v.bitrate_kbps * 1000).to_string());
        put(
            "-maxrate",
            (if v.rate_control == RateControl::Cbr {
                v.bitrate_kbps
            } else {
                v.max_bitrate_kbps
            } * 1000)
                .to_string(),
        );
        put("-bufsize", (v.max_bitrate_kbps * 2000).to_string());
        if v.rate_control == RateControl::Cbr && encoder == Encoder::Software {
            put("-minrate", (v.bitrate_kbps * 1000).to_string());
        }
    }
    put(
        "-g",
        ((v.fps.num as u64 * v.keyframe_seconds as u64).div_ceil(v.fps.den as u64)).to_string(),
    );
    if matches!(
        encoder,
        Encoder::Software | Encoder::AmdH264 | Encoder::NvidiaH264 | Encoder::IntelH264
    ) {
        put("-profile:v", v.profile.clone());
    }
    if encoder != Encoder::Mf {
        put("-bf", v.bframes.to_string());
    }
    a
}

pub fn recording_args(
    c: &Config,
    backend: Backend,
    encoder: Encoder,
    path: GpuPath,
    out: &Path,
) -> Result<Vec<String>, String> {
    c.validate().map_err(|e| e.to_string())?;
    if matches!(backend, Backend::X11 | Backend::AvFoundation) {
        return unix_recording_args(c, backend, encoder, out);
    }
    let mut graph = capture_filter(c, backend)?;
    match path {
        GpuPath::Direct => {}
        GpuPath::D3d11Nv12 => graph.push_str(",scale_d3d11=format=nv12"),
        GpuPath::AmfNv12 => graph.push_str(",vpp_amf=format=nv12"),
        GpuPath::CpuTransfer => graph.push_str(",hwdownload,format=bgra,format=nv12"),
    }
    if backend == Backend::Dxgi && c.video.width > 0 {
        if path == GpuPath::CpuTransfer {
            graph.push_str(&format!(
                ",scale={}:{}:flags={}",
                c.video.width,
                c.video.height,
                if c.video.scaling == Scaling::Point {
                    "neighbor"
                } else {
                    c.video.scaling.ffmpeg()
                }
            ));
        } else {
            return Err("DXGI custom scaling requires the CPU compatibility path; WGC supports GPU canvas scaling".into());
        }
    }
    let mut a = vec![
        "-hide_banner".into(),
        "-loglevel".into(),
        if c.performance.detailed_logs {
            "verbose"
        } else {
            "info"
        }
        .into(),
        "-n".into(),
        "-filter_complex_threads".into(),
        c.performance.worker_count().to_string(),
        "-filter_complex".into(),
        format!("{graph}[video]"),
        "-map".into(),
        "[video]".into(),
        "-an".into(),
    ];
    a.extend(encoder_args(c, encoder));
    if matches!(encoder, Encoder::IntelH264 | Encoder::IntelHevc) {
        a.splice(
            0..0,
            [
                "-init_hw_device".into(),
                "qsv=potato:hw,child_device_type=d3d11va".into(),
            ],
        );
    }
    a.extend([
        "-fps_mode".into(),
        "vfr".into(),
        "-stats_period".into(),
        "0.5".into(),
        "-progress".into(),
        "pipe:1".into(),
        "-cluster_time_limit".into(),
        "1000".into(),
        "-f".into(),
        "matroska".into(),
        out.to_string_lossy().into_owned(),
    ]);
    Ok(a)
}

fn unix_recording_args(
    c: &Config,
    backend: Backend,
    encoder: Encoder,
    out: &Path,
) -> Result<Vec<String>, String> {
    let v = &c.video;
    let mut a = vec![
        "-hide_banner".into(),
        "-loglevel".into(),
        if c.performance.detailed_logs {
            "verbose"
        } else {
            "info"
        }
        .into(),
        "-n".into(),
    ];
    match (backend, &v.source) {
        (
            Backend::X11,
            Source::X11Display {
                display,
                x,
                y,
                width,
                height,
                ..
            },
        ) => {
            a.extend([
                "-f".into(),
                "x11grab".into(),
                "-framerate".into(),
                v.fps.to_string(),
                "-draw_mouse".into(),
                u8::from(v.cursor).to_string(),
                "-video_size".into(),
                format!("{width}x{height}"),
                "-i".into(),
                format!("{display}+{x},{y}"),
            ]);
        }
        (
            Backend::X11,
            Source::X11Window {
                id, width, height, ..
            },
        ) => {
            let display = std::env::var("DISPLAY").map_err(|_| "X11 DISPLAY is unavailable")?;
            a.extend([
                "-f".into(),
                "x11grab".into(),
                "-framerate".into(),
                v.fps.to_string(),
                "-draw_mouse".into(),
                u8::from(v.cursor).to_string(),
                "-window_id".into(),
                id.to_string(),
                "-video_size".into(),
                format!("{width}x{height}"),
                "-i".into(),
                display,
            ]);
        }
        (Backend::AvFoundation, Source::MacDisplay { name, .. }) => {
            if !name
                .strip_prefix("Capture screen ")
                .is_some_and(|s| s.parse::<u32>().is_ok())
            {
                return Err("AVFoundation recording accepts display sources only".into());
            }
            a.extend([
                "-f".into(),
                "avfoundation".into(),
                "-framerate".into(),
                v.fps.to_string(),
                "-capture_cursor".into(),
                u8::from(v.cursor).to_string(),
                "-i".into(),
                format!("{name}:none"),
            ]);
        }
        _ => return Err("Select a source supported by this operating system".into()),
    }
    let filter = if v.scaling == Scaling::Point {
        "neighbor"
    } else {
        v.scaling.ffmpeg()
    };
    let scale = if v.width == 0 {
        format!("scale=trunc(iw/2)*2:trunc(ih/2)*2:flags={filter}")
    } else {
        format!(
            "scale={}:{}:force_original_aspect_ratio=decrease:force_divisible_by=2:flags={filter},pad={}:{}:(ow-iw)/2:(oh-ih)/2",
            v.width, v.height, v.width, v.height
        )
    };
    a.extend([
        "-filter_complex_threads".into(),
        c.performance.worker_count().to_string(),
        "-filter_complex".into(),
        format!("[0:v]setpts=PTS-STARTPTS,{scale},format=nv12[video]"),
        "-map".into(),
        "[video]".into(),
        "-an".into(),
    ]);
    a.extend(encoder_args(c, encoder));
    a.extend([
        "-fps_mode".into(),
        "vfr".into(),
        "-stats_period".into(),
        "0.5".into(),
        "-progress".into(),
        "pipe:1".into(),
        "-cluster_time_limit".into(),
        "1000".into(),
        "-f".into(),
        "matroska".into(),
        out.to_string_lossy().into_owned(),
    ]);
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_rate_no_duplicate_filter() {
        let mut c = Config::default();
        c.video.fps = Rate::new(60000, 1001).unwrap();
        let a = recording_args(
            &c,
            Backend::Wgc,
            Encoder::AmdH264,
            GpuPath::CpuTransfer,
            Path::new("video.mkv"),
        )
        .unwrap();
        let graph = &a[a.iter().position(|x| x == "-filter_complex").unwrap() + 1];
        assert!(graph.contains("60000/1001"));
        assert!(!graph.contains(",fps="));
        assert!(a.windows(2).any(|x| x == ["-fps_mode", "vfr"]));
    }
    #[test]
    fn window_title_never_becomes_filter_syntax() {
        let mut c = Config::default();
        c.video.source = Source::Window {
            hwnd: 42,
            title: "a';movie=evil".into(),
            pid: 1,
        };
        let f = capture_filter(&c, Backend::Wgc).unwrap();
        assert!(f.contains("hwnd=42"));
        assert!(!f.contains("evil"));
        assert!(capture_filter(&c, Backend::Dxgi).is_err());
    }
    #[test]
    fn amf_maps_only_three_native_speeds() {
        let mut c = Config::default();
        c.video.speed = Speed::UltraFast;
        let a = encoder_args(&c, Encoder::AmdH264);
        assert!(a.windows(2).any(|x| x == ["-quality", "speed"]));
        c.video.quality = Quality::VeryHigh;
        assert_ne!(a, encoder_args(&c, Encoder::AmdH264));
    }
    #[test]
    fn native_capture_arguments_preserve_rate_and_reject_wrong_source() {
        let mut c = Config::default();
        c.video.fps = Rate::new(30000, 1001).unwrap();
        c.video.source = Source::X11Display {
            display: ":99".into(),
            x: 1280,
            y: 0,
            width: 1920,
            height: 1080,
            name: "display".into(),
        };
        let a = recording_args(
            &c,
            Backend::X11,
            Encoder::Software,
            GpuPath::CpuTransfer,
            Path::new("capture.mkv"),
        )
        .unwrap();
        assert!(a.windows(2).any(|x| x == ["-i", ":99+1280,0"]));
        assert!(a.windows(2).any(|x| x == ["-framerate", "30000/1001"]));
        assert!(
            recording_args(
                &c,
                Backend::AvFoundation,
                Encoder::AppleH264,
                GpuPath::CpuTransfer,
                Path::new("capture.mkv")
            )
            .is_err()
        );
        c.video.source = Source::MacDisplay {
            device: 2,
            name: "Capture screen 0".into(),
        };
        let a = recording_args(
            &c,
            Backend::AvFoundation,
            Encoder::AppleH264,
            GpuPath::CpuTransfer,
            Path::new("capture.mkv"),
        )
        .unwrap();
        assert!(a.windows(2).any(|x| x == ["-i", "Capture screen 0:none"]));
        assert!(a.windows(2).any(|x| x == ["-allow_sw", "0"]));
        assert!(!a.iter().any(|x| x.contains("d3d11")));
        c.video.source = Source::MacDisplay {
            device: 2,
            name: "Camera".into(),
        };
        assert!(
            recording_args(
                &c,
                Backend::AvFoundation,
                Encoder::AppleH264,
                GpuPath::CpuTransfer,
                Path::new("capture.mkv")
            )
            .is_err()
        );
    }
    #[test]
    fn priority_never_realtime() {
        assert_eq!(Priority::ALL.len(), 5);
    }
}
