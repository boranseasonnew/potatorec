use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rate {
    pub num: u32,
    pub den: u32,
}
impl Rate {
    pub fn new(num: u32, den: u32) -> Result<Self, ConfigError> {
        if num == 0 || den == 0 || u64::from(num) > 120 * u64::from(den) || num < den {
            return Err(ConfigError::Invalid(
                "FPS must be between 1 and 120; denominator must be positive".into(),
            ));
        }
        let mut a = num;
        let mut b = den;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Ok(Self {
            num: num / a,
            den: den / a,
        })
    }
    pub fn value(self) -> f64 {
        self.num as f64 / self.den as f64
    }
    pub fn timestamp_ns(self, frame: u64) -> u128 {
        frame as u128 * self.den as u128 * 1_000_000_000 / self.num as u128
    }
}
impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

macro_rules! choices {
    ($name:ident { $($v:ident => $s:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name { $($v),+ }
        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$v),+];
            pub fn label(self) -> &'static str { match self { $(Self::$v => $s),+ } }
        }
    }
}
choices!(Backend { Auto=>"Auto", Wgc=>"Windows Graphics Capture", Dxgi=>"DXGI Desktop Duplication" });
choices!(Encoder { Auto=>"Auto · Hardware H.264", AmdH264=>"AMD H.264", AmdHevc=>"AMD HEVC", NvidiaH264=>"NVIDIA H.264", NvidiaHevc=>"NVIDIA HEVC", IntelH264=>"Intel H.264", IntelHevc=>"Intel HEVC", Mf=>"Media Foundation H.264", Software=>"Software H.264" });
impl Encoder {
    pub fn codec(self) -> &'static str {
        match self {
            Self::Auto | Self::AmdH264 => "h264_amf",
            Self::AmdHevc => "hevc_amf",
            Self::NvidiaH264 => "h264_nvenc",
            Self::NvidiaHevc => "hevc_nvenc",
            Self::IntelH264 => "h264_qsv",
            Self::IntelHevc => "hevc_qsv",
            Self::Mf => "h264_mf",
            Self::Software => "libx264",
        }
    }
    pub fn hardware(self) -> bool {
        !matches!(self, Self::Software)
    }
}
choices!(Quality { VeryLow=>"Very Low", Low=>"Low", Medium=>"Medium", High=>"High", VeryHigh=>"Very High" });
impl Quality {
    pub fn qp(self) -> u32 {
        match self {
            Self::VeryLow => 35,
            Self::Low => 30,
            Self::Medium => 25,
            Self::High => 21,
            Self::VeryHigh => 17,
        }
    }
}
choices!(Speed { UltraFast=>"Ultra Fast", Fast=>"Fast", Balanced=>"Balanced", Quality=>"Quality", Maximum=>"Maximum Quality" });
choices!(Profile { Potato=>"Potato Mode", Fast=>"Fast", Balanced=>"Balanced", Quality=>"Quality", Maximum=>"Maximum Quality" });
choices!(Priority { Low=>"Low", BelowNormal=>"Below Normal", Normal=>"Normal", AboveNormal=>"Above Normal", High=>"High" });
choices!(Container { Mkv=>"MKV", Mp4=>"MP4", FragmentedMp4=>"Fragmented MP4" });
impl Container {
    pub fn ext(self) -> &'static str {
        if self == Self::Mkv { "mkv" } else { "mp4" }
    }
}
choices!(RateControl { Cqp=>"CQP / CQ", Cbr=>"CBR", Vbr=>"VBR" });
choices!(Scaling { Point=>"Point", Bilinear=>"Bilinear", Bicubic=>"Bicubic" });
impl Scaling {
    pub fn ffmpeg(self) -> &'static str {
        match self {
            Self::Point => "point",
            Self::Bilinear => "bilinear",
            Self::Bicubic => "bicubic",
        }
    }
}
choices!(Language {
    English=>"English", Turkish=>"Türkçe", Azerbaijani=>"Azərbaycanca", Russian=>"Русский",
    German=>"Deutsch", French=>"Français", Spanish=>"Español", Portuguese=>"Português (Brasil)",
    Italian=>"Italiano", Dutch=>"Nederlands", Polish=>"Polski", Ukrainian=>"Українська",
    Romanian=>"Română", Hungarian=>"Magyar", Czech=>"Čeština", Slovak=>"Slovenčina",
    Bulgarian=>"Български", Serbian=>"Српски", Croatian=>"Hrvatski", Greek=>"Ελληνικά",
    Swedish=>"Svenska", Danish=>"Dansk", Finnish=>"Suomi"
});
impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Turkish => "tr",
            Self::Azerbaijani => "az",
            Self::Russian => "ru",
            Self::German => "de",
            Self::French => "fr",
            Self::Spanish => "es",
            Self::Portuguese => "pt-BR",
            Self::Italian => "it",
            Self::Dutch => "nl",
            Self::Polish => "pl",
            Self::Ukrainian => "uk",
            Self::Romanian => "ro",
            Self::Hungarian => "hu",
            Self::Czech => "cs",
            Self::Slovak => "sk",
            Self::Bulgarian => "bg",
            Self::Serbian => "sr",
            Self::Croatian => "hr",
            Self::Greek => "el",
            Self::Swedish => "sv",
            Self::Danish => "da",
            Self::Finnish => "fi",
        }
    }
    pub fn from_code(code: &str) -> Self {
        let code = code.replace('_', "-").to_lowercase();
        Self::ALL
            .iter()
            .copied()
            .find(|l| {
                l.code().to_lowercase() == code
                    || l.code().split('-').next() == code.split('-').next()
            })
            .unwrap_or(Self::English)
    }
}
choices!(Theme { Dark=>"Dark", Light=>"Light" });
choices!(FilterKind { Suppression=>"RNNoise", Gate=>"Noise Gate", Expander=>"Expander", Compressor=>"Compressor", Limiter=>"Limiter", Gain=>"Gain" });
choices!(VoicePreset { Natural=>"Natural", Clean=>"Clean Voice", Strong=>"Strong Noise Reduction", LowCpu=>"Low CPU", Custom=>"Custom" });

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Source {
    Monitor { index: u32, name: String },
    Window { hwnd: u64, title: String, pid: u32 },
}
impl Default for Source {
    fn default() -> Self {
        Self::Monitor {
            index: 0,
            name: "Display 1".into(),
        }
    }
}
impl Source {
    pub fn label(&self) -> &str {
        match self {
            Self::Monitor { name, .. } => name,
            Self::Window { title, .. } => title,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Video {
    pub transfer: crate::video::GpuPath,
    pub backend: Backend,
    pub source: Source,
    pub cursor: bool,
    pub width: u32,
    pub height: u32,
    pub fps: Rate,
    pub fps_mode: u8,
    pub encoder: Encoder,
    pub quality: Quality,
    pub speed: Speed,
    pub rate_control: RateControl,
    pub bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
    pub keyframe_seconds: u32,
    pub profile: String,
    pub bframes: u8,
    pub scaling: Scaling,
}
impl Default for Video {
    fn default() -> Self {
        Self {
            transfer: crate::video::GpuPath::CpuTransfer,
            backend: Backend::Auto,
            source: Source::default(),
            cursor: true,
            width: 0,
            height: 0,
            fps: Rate { num: 60, den: 1 },
            fps_mode: 0,
            encoder: Encoder::Auto,
            quality: Quality::Medium,
            speed: Speed::Balanced,
            rate_control: RateControl::Cqp,
            bitrate_kbps: 10000,
            max_bitrate_kbps: 16000,
            keyframe_seconds: 2,
            profile: "high".into(),
            bframes: 0,
            scaling: Scaling::Bilinear,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Performance {
    pub workers: u8,
    pub buffers: u8,
    pub priority: Priority,
    pub profile: Profile,
    pub reduce_ui: bool,
    pub low_memory: bool,
    pub prefer_hardware: bool,
    pub detailed_logs: bool,
    pub minimize_on_start: bool,
}
impl Default for Performance {
    fn default() -> Self {
        Self {
            workers: 0,
            buffers: 0,
            priority: Priority::Normal,
            profile: Profile::Balanced,
            reduce_ui: true,
            low_memory: false,
            prefer_hardware: true,
            detailed_logs: false,
            minimize_on_start: false,
        }
    }
}
impl Performance {
    pub fn worker_count(&self) -> usize {
        if self.workers == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get().saturating_sub(2).clamp(1, 3))
                .unwrap_or(1)
        } else {
            self.workers as usize
        }
    }
    pub fn buffer_count(&self) -> usize {
        if self.low_memory {
            1
        } else if self.buffers == 0 {
            2
        } else {
            self.buffers as usize
        }
    }
    pub fn apply(&mut self, profile: Profile, video: &mut Video) {
        self.profile = profile;
        (
            video.speed,
            self.buffers,
            video.scaling,
            self.low_memory,
            self.reduce_ui,
        ) = match profile {
            Profile::Potato => (Speed::UltraFast, 1, Scaling::Point, true, true),
            Profile::Fast => (Speed::Fast, 2, Scaling::Bilinear, false, true),
            Profile::Balanced => (Speed::Balanced, 0, Scaling::Bilinear, false, true),
            Profile::Quality => (Speed::Quality, 3, Scaling::Bicubic, false, true),
            Profile::Maximum => (Speed::Maximum, 4, Scaling::Bicubic, false, false),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Filter {
    pub kind: FilterKind,
    pub enabled: bool,
    pub threshold_db: f32,
    pub close_db: f32,
    pub ratio: f32,
    pub attack_ms: f32,
    pub hold_ms: f32,
    pub release_ms: f32,
    pub gain_db: f32,
    pub strength: f32,
}
impl Default for Filter {
    fn default() -> Self {
        Self {
            kind: FilterKind::Gain,
            enabled: false,
            threshold_db: -24.,
            close_db: -45.,
            ratio: 3.,
            attack_ms: 5.,
            hold_ms: 100.,
            release_ms: 100.,
            gain_db: 0.,
            strength: 0.65,
        }
    }
}
pub fn filters() -> Vec<Filter> {
    [
        FilterKind::Suppression,
        FilterKind::Gate,
        FilterKind::Expander,
        FilterKind::Compressor,
        FilterKind::Limiter,
        FilterKind::Gain,
    ]
    .into_iter()
    .map(|kind| Filter {
        kind,
        threshold_db: match kind {
            FilterKind::Gate => -38.,
            FilterKind::Expander => -40.,
            FilterKind::Limiter => -1.,
            _ => -24.,
        },
        enabled: kind == FilterKind::Limiter,
        ..Filter::default()
    })
    .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSource {
    pub enabled: bool,
    pub device_id: String,
    pub gain_db: f32,
    pub mute: bool,
    pub mono: bool,
    pub mixed: bool,
    pub isolated: bool,
}
impl Default for AudioSource {
    fn default() -> Self {
        Self {
            enabled: false,
            device_id: String::new(),
            gain_db: 0.,
            mute: false,
            mono: false,
            mixed: true,
            isolated: true,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Audio {
    pub system: AudioSource,
    pub mic: AudioSource,
    pub additional: AudioSource,
    pub filters: Vec<Filter>,
    pub preset: VoicePreset,
    pub export_stems: bool,
}
impl Default for Audio {
    fn default() -> Self {
        Self {
            system: AudioSource {
                enabled: true,
                ..Default::default()
            },
            mic: AudioSource::default(),
            additional: AudioSource::default(),
            filters: filters(),
            preset: VoicePreset::Natural,
            export_stems: false,
        }
    }
}
impl Audio {
    pub fn apply(&mut self, p: VoicePreset) {
        self.preset = p;
        self.filters = filters();
        for f in &mut self.filters {
            f.enabled = match f.kind {
                FilterKind::Suppression => matches!(p, VoicePreset::Clean | VoicePreset::Strong),
                FilterKind::Gate => p == VoicePreset::Strong,
                FilterKind::Expander => matches!(p, VoicePreset::Clean | VoicePreset::LowCpu),
                FilterKind::Compressor => matches!(p, VoicePreset::Clean | VoicePreset::Strong),
                FilterKind::Limiter => true,
                _ => false,
            };
            if p == VoicePreset::Strong {
                f.strength = 1.;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Hotkeys {
    pub record: u32,
    pub pause: u32,
    pub mute: u32,
    pub visibility: u32,
    pub replay: u32,
}
impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            record: 0x78,
            pause: 0x79,
            mute: 0x7a,
            visibility: 0x76,
            replay: 0x77,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub version: u32,
    pub language: Language,
    pub theme: Theme,
    pub video: Video,
    pub performance: Performance,
    pub audio: Audio,
    pub output_dir: PathBuf,
    pub filename_format: String,
    pub container: Container,
    pub ffmpeg: PathBuf,
    pub hotkeys: Hotkeys,
    pub replay_seconds: u32,
    pub replay_memory_mb: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            language: if std::env::var("LANG").unwrap_or_default().starts_with("tr") {
                Language::Turkish
            } else {
                Language::English
            },
            theme: Theme::Dark,
            video: Video::default(),
            performance: Performance::default(),
            audio: Audio::default(),
            output_dir: std::env::var_os("USERPROFILE")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Videos")
                .join("Potato Recorder"),
            filename_format: "Potato_%Y-%m-%d_%H-%M-%S".into(),
            container: Container::Mkv,
            ffmpeg: PathBuf::from("ffmpeg.exe"),
            hotkeys: Hotkeys::default(),
            replay_seconds: 30,
            replay_memory_mb: 64,
        }
    }
}
impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let fail = |s: &str| Err(ConfigError::Invalid(s.into()));
        if self.version != 1 {
            return fail("Unsupported configuration version; original file preserved");
        }
        if ![15, 30, 60, 120].contains(&self.replay_seconds)
            || !(16..=1024).contains(&self.replay_memory_mb)
        {
            return fail("Replay must use 15/30/60/120 seconds and 16–1024 MiB");
        }
        Rate::new(self.video.fps.num, self.video.fps.den)?;
        let (w, h) = (self.video.width, self.video.height);
        if !((w == 0 && h == 0)
            || (w >= 64 && h >= 64 && w <= 7680 && h <= 4320 && w % 2 == 0 && h % 2 == 0))
        {
            return fail(
                "Resolution must use even dimensions between 64×64 and 7680×4320, or Source (0×0)",
            );
        }
        if self.performance.workers > 5 || self.performance.buffers > 5 {
            return fail("Workers and buffers must be Auto (0) or 1–5");
        }
        if self.video.bitrate_kbps < 100
            || self.video.max_bitrate_kbps < self.video.bitrate_kbps
            || self.video.max_bitrate_kbps > 500_000
        {
            return fail("Bitrate must be 100–500000 Kbps, maximum >= target");
        }
        if !(1..=30).contains(&self.video.keyframe_seconds)
            || self.video.bframes > 3
            || !["high", "main", "baseline"].contains(&self.video.profile.as_str())
        {
            return fail("Invalid keyframe interval, B-frames or H.264 profile");
        }
        if self.filename_format.is_empty()
            || self
                .filename_format
                .contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|'])
            || self.filename_format.contains("..")
        {
            return fail(
                "Filename format must be a filename, with no path components or reserved characters",
            );
        }
        if self.output_dir.as_os_str().is_empty() {
            return fail("Select a recording directory");
        }
        if self.audio.filters.len() > 16 {
            return fail("At most 16 microphone filters");
        }
        for s in [&self.audio.system, &self.audio.mic, &self.audio.additional] {
            if !s.gain_db.is_finite() || !(-60.0..=30.).contains(&s.gain_db) {
                return fail("Audio gain must be finite and between -60 and +30 dB");
            }
        }
        for f in &self.audio.filters {
            if ![
                f.threshold_db,
                f.close_db,
                f.ratio,
                f.attack_ms,
                f.hold_ms,
                f.release_ms,
                f.gain_db,
                f.strength,
            ]
            .iter()
            .all(|x| x.is_finite())
                || !(-90.0..=0.).contains(&f.threshold_db)
                || !(-90.0..=0.).contains(&f.close_db)
                || !(1.0..=20.).contains(&f.ratio)
                || !(0.1..=1000.).contains(&f.attack_ms)
                || !(0.0..=2000.).contains(&f.hold_ms)
                || !(1.0..=3000.).contains(&f.release_ms)
                || !(-60.0..=30.).contains(&f.gain_db)
                || !(0.0..=1.).contains(&f.strength)
            {
                return fail("Invalid audio filter parameters");
            }
        }
        let keys = [
            self.hotkeys.record,
            self.hotkeys.pause,
            self.hotkeys.mute,
            self.hotkeys.visibility,
            self.hotkeys.replay,
        ];
        for (i, k) in keys.iter().enumerate() {
            if *k == 0x7b {
                return fail("F12 is reserved by Windows; choose another function key");
            }
            if !(0x70..=0x87).contains(k) || keys[..i].contains(k) {
                return fail("Global shortcuts must be distinct function keys F1–F24");
            }
        }
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let mut c: Self = serde_json::from_slice(&fs::read(path)?)?;
        // Migrate the original default, which Windows reserves for debuggers.
        if c.hotkeys.visibility == 0x7b
            && ![
                c.hotkeys.record,
                c.hotkeys.pause,
                c.hotkeys.mute,
                c.hotkeys.replay,
            ]
            .contains(&0x76)
        {
            c.hotkeys.visibility = 0x76;
        }
        c.validate()?;
        Ok(c)
    }
    pub fn save(&self, path: &Path) -> Result<(), ConfigError> {
        self.validate()?;
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?;
        }
        let temp = path.with_extension("json.tmp");
        fs::write(&temp, serde_json::to_vec_pretty(self)?)?;
        fs::rename(temp, path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rational_timestamps_do_not_drift() {
        let r = Rate::new(60000, 1001).unwrap();
        assert_eq!(r.timestamp_ns(60_000), 1_001_000_000_000);
        assert_eq!(Rate::new(120000, 1001).unwrap().num, 120000);
        assert!(Rate::new(121, 1).is_err());
        assert!(Rate::new(1, 0).is_err());
    }
    #[test]
    fn rejects_unsafe_settings() {
        let mut c = Config {
            filename_format: "../other".into(),
            ..Default::default()
        };
        assert!(c.validate().is_err());
        c.filename_format = "good_%Y".into();
        c.video.width = 1279;
        c.video.height = 720;
        assert!(c.validate().is_err());
    }
    #[test]
    fn config_roundtrip_and_partial_migration() {
        let c = Config::default();
        let s = serde_json::to_string(&c).unwrap();
        let copy: Config = serde_json::from_str(&s).unwrap();
        copy.validate().unwrap();
        let partial: Config = serde_json::from_str("{\"version\":1}").unwrap();
        assert_eq!(partial.video.fps, Rate { num: 60, den: 1 });
    }
    #[test]
    fn settings_persist_after_reopening_and_unknown_version_is_preserved() {
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../test-output/config-fixtures");
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join(format!(
            "potato-config-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut c = Config {
            language: Language::Azerbaijani,
            ..Default::default()
        };
        c.video.fps = Rate::new(60000, 1001).unwrap();
        c.audio.mic.gain_db = 6.;
        c.save(&path).unwrap();
        let loaded = Config::load(&path).unwrap();
        assert_eq!(loaded.language, Language::Azerbaijani);
        assert_eq!(loaded.video.fps, c.video.fps);
        assert_eq!(loaded.audio.mic.gain_db, 6.);
        let bytes = b"{\"version\":999}";
        std::fs::write(&path, bytes).unwrap();
        assert!(Config::load(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
    #[test]
    fn reserved_f12_is_rejected_and_legacy_visibility_migrates() {
        let mut c = Config::default();
        assert_eq!(c.hotkeys.visibility, 0x76);
        c.hotkeys.visibility = 0x7b;
        assert!(c.validate().is_err());
        let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../test-output/config-fixtures");
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join(format!("legacy-hotkeys-{}.json", std::process::id()));
        std::fs::write(&path, serde_json::to_vec(&c).unwrap()).unwrap();
        let migrated = Config::load(&path).unwrap();
        assert_eq!(migrated.hotkeys.visibility, 0x76);
        assert_eq!(migrated.video.fps, c.video.fps);
    }
}
