use crate::localization::{current, fill, tr};
mod design;
use crate::{
    audio,
    engine::{self, Capabilities, Control, Controller, Phase},
    platform::{self, NativeBridge, NativeEvent, ProcessSampler, Resources},
    replay::Replay,
};
use eframe::egui::{self, Color32, RichText, Vec2};
use potato_core::{config::*, video::GpuPath};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

const ACCENT: Color32 = design::PURPLE;
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Dashboard,
    Recording,
    Video,
    Audio,
    Performance,
    Hotkeys,
    Recordings,
    Settings,
}
impl Page {
    const ALL: [Self; 8] = [
        Self::Dashboard,
        Self::Recording,
        Self::Video,
        Self::Audio,
        Self::Performance,
        Self::Hotkeys,
        Self::Recordings,
        Self::Settings,
    ];
    fn name(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Recording => "Recording",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::Performance => "Performance",
            Self::Hotkeys => "Hotkeys",
            Self::Recordings => "Recordings",
            Self::Settings => "Settings",
        }
    }
}
enum JobEvent {
    Done(Result<String, String>),
    Meter(Arc<audio::AudioMetrics>),
}
type Discovery = (Capabilities, Vec<audio::Device>, Vec<Source>);
pub struct PotatoApp {
    config: Config,
    config_path: PathBuf,
    page: Page,
    controller: Controller,
    replay: Replay,
    cap: Option<Capabilities>,
    detect: Option<crossbeam_channel::Receiver<Discovery>>,
    devices: Vec<audio::Device>,
    sources: Vec<Source>,
    bridge: Option<NativeBridge>,
    native_language: Language,
    notice: String,
    advanced: bool,
    dirty: Option<Instant>,
    last_serialized: String,
    hidden: bool,
    exit_when_done: bool,
    recordings: Vec<PathBuf>,
    recovery: Vec<PathBuf>,
    resources: Resources,
    child_resources: Resources,
    sampler: ProcessSampler,
    child_sampler: ProcessSampler,
    last_metrics: Instant,
    job: Option<crossbeam_channel::Receiver<JobEvent>>,
    meter: Option<Arc<audio::AudioMetrics>>,
    last_test: Option<PathBuf>,
}

impl PotatoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut config_path = platform::config_path();
        let (mut config, notice) = if config_path.exists() {
            match Config::load(&config_path) {
                Ok(c) => (c, String::new()),
                Err(e) => {
                    let backup = config_path
                        .with_extension(format!("invalid-{}.json", platform::qpc_100ns()));
                    let message = if std::fs::copy(&config_path, &backup).is_ok() {
                        format!(
                            "Configuration error: {e}. Original backed up at {}",
                            backup.display()
                        )
                    } else {
                        config_path = config_path.with_extension("recovered.json");
                        format!(
                            "Configuration error: {e}. Original preserved; new settings use {}",
                            config_path.display()
                        )
                    };
                    (Config::default(), message)
                }
            }
        } else {
            let c = Config {
                language: platform::os_language(),
                ..Default::default()
            };
            (c, String::new())
        };
        let qa_args: Vec<_> = std::env::args().collect();
        if qa_args.get(1).is_some_and(|a| a == "--ui-test") {
            config.language =
                Language::from_code(qa_args.get(2).map(String::as_str).unwrap_or("en"));
        }
        let mut fonts = egui::FontDefinitions::default();
        if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\segoeui.ttf") {
            fonts
                .font_data
                .insert("Segoe UI".into(), egui::FontData::from_owned(data).into());
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "Segoe UI".into());
        }
        if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\seguisb.ttf") {
            fonts.font_data.insert(
                "Segoe Display".into(),
                egui::FontData::from_owned(data).into(),
            );
            fonts.families.insert(
                egui::FontFamily::Name("Display".into()),
                vec!["Segoe Display".into(), "Segoe UI".into()],
            );
        } else {
            fonts.families.insert(
                egui::FontFamily::Name("Display".into()),
                fonts.families[&egui::FontFamily::Proportional].clone(),
            );
        }
        cc.egui_ctx.set_fonts(fonts);
        theme(&cc.egui_ctx, config.theme);
        let bridge = Some(NativeBridge::start(
            config.hotkeys.clone(),
            config.language,
            cc.egui_ctx.clone(),
        ));
        let mut app = Self {
            last_serialized: serde_json::to_string(&config).unwrap_or_default(),
            native_language: config.language,
            config,
            config_path,
            page: Page::Dashboard,
            controller: Controller::new(),
            replay: Replay::new(),
            cap: None,
            detect: None,
            devices: Vec::new(),
            sources: Vec::new(),
            bridge,
            notice,
            advanced: false,
            dirty: None,
            hidden: false,
            exit_when_done: false,
            recordings: Vec::new(),
            recovery: Vec::new(),
            resources: Resources::default(),
            child_resources: Resources::default(),
            sampler: ProcessSampler::new(),
            child_sampler: ProcessSampler::new(),
            last_metrics: Instant::now(),
            job: None,
            meter: None,
            last_test: None,
        };
        app.detect(cc.egui_ctx.clone());
        if let Some(page) = qa_args.get(3)
            && let Some(selected) = Page::ALL
                .into_iter()
                .find(|p| p.name().eq_ignore_ascii_case(page))
        {
            app.page = selected;
        }
        app.refresh_recordings();
        app
    }
    fn detect(&mut self, ctx: egui::Context) {
        if self.detect.is_some() {
            return;
        }
        let exe = self.config.ffmpeg.clone();
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.detect = Some(rx);
        std::thread::spawn(move || {
            let cap = Capabilities::detect(&exe);
            let devices = audio::devices_for(&exe).unwrap_or_default();
            let sources = platform::sources_for(&exe);
            let _ = tx.send((cap, devices, sources));
            ctx.request_repaint();
        });
    }
    fn refresh_recordings(&mut self) {
        self.recordings = std::fs::read_dir(&self.config.output_dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.is_file()
                    && p.extension().is_some_and(|e| e == "mkv" || e == "mp4")
                    && !p.to_string_lossy().contains(".partial.")
            })
            .collect();
        self.recordings.sort_by_cached_key(|p| {
            std::cmp::Reverse(std::fs::metadata(p).and_then(|m| m.modified()).ok())
        });
        self.recovery = engine::recovery_sessions(&self.config.output_dir);
    }
    fn save(&mut self) {
        match self.config.save(&self.config_path) {
            Ok(()) => {
                self.dirty = None;
                self.notice = String::new();
            }
            Err(e) => {
                self.dirty = None;
                self.notice = format!("Settings not saved: {e}");
            }
        }
    }
    fn toggle(&mut self, ctx: &egui::Context) {
        if self.controller.active() {
            if matches!(
                self.controller.snapshot().phase,
                Phase::Recording | Phase::Paused
            ) {
                self.controller.send(Control::Stop);
            }
        } else if self.replay.active.load(Ordering::Relaxed) {
            self.notice = "Stop the replay buffer before starting a recording".into();
        } else if self.job.is_some() {
            self.notice = "Wait for the audio test or conversion to finish".into();
        } else if let Some(cap) = &self.cap {
            match self.controller.start(self.config.clone(), cap.clone()) {
                Ok(()) => {
                    self.notice = String::new();
                    if self.config.performance.minimize_on_start {
                        self.hide(ctx);
                    }
                }
                Err(e) => self.notice = format!("{e:#}"),
            }
        }
    }
    fn hide(&mut self, ctx: &egui::Context) {
        if !platform::has_tray() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            return;
        }
        self.hidden = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
    }
    fn save_replay(&mut self, ctx: &egui::Context) {
        if self.job.is_some() {
            return;
        }
        match self.replay.save(&self.config) {
            Ok(save) => self.job(ctx.clone(), save),
            Err(e) => self.notice = e.to_string(),
        }
    }
    fn replay_controls(&mut self, ui: &mut egui::Ui) {
        let active = self.replay.active.load(Ordering::Relaxed);
        card(ui, |ui| {
            section(ui, tr(current(ui), "Replay buffer · video only"));
            ui.add_enabled_ui(!active, |ui| {
                egui::ComboBox::from_label(tr(current(ui), "Keep last"))
                    .selected_text(format!("{} seconds", self.config.replay_seconds))
                    .show_ui(ui, |ui| {
                        for seconds in [15, 30, 60, 120] {
                            ui.selectable_value(
                                &mut self.config.replay_seconds,
                                seconds,
                                format!("{seconds} seconds"),
                            );
                        }
                    });
                ui.add(
                    egui::Slider::new(&mut self.config.replay_memory_mb, 16..=1024)
                        .text(tr(current(ui), "Compressed memory limit · MiB")),
                );
            });
            let (seconds, bytes, error) = self.replay.stats();
            ui.label(format!(
                "Buffered {:.1} seconds · {} · {}",
                seconds,
                size(bytes as u64),
                if active { "Capturing video" } else { "Off" }
            ));
            if !error.is_empty() {
                ui.colored_label(Color32::LIGHT_RED, error);
            }
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        !self.controller.active() && self.cap.is_some() && self.job.is_none(),
                        egui::Button::new(if active {
                            "Stop replay buffer"
                        } else {
                            "Start replay buffer"
                        }),
                    )
                    .clicked()
                {
                    if active {
                        self.replay.stop();
                    } else if let Some(cap) = &self.cap
                        && let Err(e) = self.replay.start(self.config.clone(), cap)
                    {
                        self.notice = format!("{e:#}");
                    }
                }
                if ui
                    .add_enabled(
                        active && self.job.is_none(),
                        egui::Button::new(tr(current(ui), "Save replay · F8")),
                    )
                    .clicked()
                {
                    self.save_replay(ui.ctx());
                }
            });
            ui.label(tr(current(ui), "Explicit opt-in; never starts with the app. Compressed video stays in a bounded RAM ring. Saving starts at a decodable keyframe, so duration may be shorter. Replay audio is not supported. Recording and replay run separately to avoid competing for the encoder."));
        });
    }
    fn show(&mut self, ctx: &egui::Context) {
        self.hidden = false;
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
    }
    fn job(
        &mut self,
        ctx: egui::Context,
        f: impl FnOnce() -> anyhow::Result<String> + Send + 'static,
    ) {
        let (tx, rx) = crossbeam_channel::unbounded();
        self.job = Some(rx);
        std::thread::spawn(move || {
            let _ = tx.send(JobEvent::Done(f().map_err(|e| format!("{e:#}"))));
            ctx.request_repaint();
        });
    }
    fn mic_test(&mut self, ctx: egui::Context) {
        let c = self.config.clone();
        let file = c.output_dir.join(format!(
            "Microphone-test-{}.wav",
            chrono::Local::now().format("%Y%m%d-%H%M%S-%3f")
        ));
        self.last_test = Some(file.clone());
        let (tx, rx) = crossbeam_channel::unbounded();
        self.job = Some(rx);
        std::thread::spawn(move || {
            let run = || -> anyhow::Result<String> {
                platform::ensure_output(&c.output_dir)?;
                let folder = c
                    .output_dir
                    .join(format!(".potato-mic-test-{}", platform::qpc_100ns()));
                std::fs::create_dir(&folder)?;
                let pool = Arc::new(
                    rayon::ThreadPoolBuilder::new()
                        .num_threads(c.performance.worker_count())
                        .build()?,
                );
                let mut capture = audio::AudioCapture::start(
                    "Microphone",
                    c.audio.mic.clone(),
                    false,
                    c.audio.filters.clone(),
                    folder.clone(),
                    platform::qpc_100ns(),
                    pool,
                    Arc::new(AtomicBool::new(false)),
                    c.ffmpeg.clone(),
                )?;
                let _ = tx.send(JobEvent::Meter(capture.metrics.clone()));
                ctx.request_repaint();
                std::thread::sleep(Duration::from_secs(8));
                capture.stop()?;
                let output = platform::command(&c.ffmpeg)
                    .args([
                        "-hide_banner",
                        "-loglevel",
                        "error",
                        "-nostdin",
                        "-n",
                        "-f",
                        "f32le",
                        "-ar",
                        "48000",
                        "-ac",
                        "2",
                        "-i",
                    ])
                    .arg(&capture.stem.file)
                    .args(["-c:a", "pcm_s24le"])
                    .arg(&file)
                    .output()?;
                anyhow::ensure!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Ok(format!("Microphone test saved: {}", file.display()))
            };
            let result = run();
            let _ = tx.send(JobEvent::Done(result.map_err(|e| format!("{e:#}"))));
            ctx.request_repaint();
        });
    }
    fn header(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                design::eyebrow(ui, tr(l, "YOUR GAME. YOUR STORY."));
                ui.add_space(3.);
                ui.label(design::display(
                    if self.page == Page::Dashboard {
                        tr(l, "Make every moment count.")
                    } else {
                        tr(l, self.page.name())
                    },
                    29.,
                ));
                ui.label(
                    RichText::new(match self.page {
                        Page::Dashboard => tr(l, "Stay in the game. Potato handles the recording."),
                        Page::Recording => {
                            tr(l, "Choose your source. Give your next recording a home.")
                        }
                        Page::Video => tr(
                            l,
                            "Find your balance between picture quality and performance.",
                        ),
                        Page::Audio => tr(l, "Game sound and your voice. Each with its own space."),
                        Page::Performance => tr(
                            l,
                            "Make room for the game. Tune the work behind every frame.",
                        ),
                        Page::Hotkeys => tr(
                            l,
                            "Keep your hands in the game and the controls within reach.",
                        ),
                        Page::Recordings => {
                            tr(l, "The moments you kept, ready for what comes next.")
                        }
                        Page::Settings => tr(l, "A small studio, made to feel like yours."),
                    })
                    .size(13.)
                    .color(ui.visuals().weak_text_color()),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(if platform::has_tray() {
                                tr(l, "Minimize to tray  ↘")
                            } else {
                                "Minimize  ↘"
                            })
                            .size(12.),
                        )
                        .fill(ui.visuals().window_fill())
                        .corner_radius(8.),
                    )
                    .clicked()
                {
                    self.hide(ui.ctx());
                }
            });
        });
        ui.add_space(23.);
    }
    fn record_button(&mut self, ui: &mut egui::Ui) {
        let s = self.controller.snapshot();
        let l = self.config.language;
        ui.horizontal(|ui| {
            let busy = matches!(s.phase, Phase::Starting | Phase::Finalizing);
            let active = matches!(s.phase, Phase::Recording | Phase::Paused);
            let text = if active {
                tr(l, "Stop recording")
            } else {
                tr(l, "Start recording")
            };
            let b = egui::Button::new(
                RichText::new(format!("{}  {text}", if active { "■" } else { "●" }))
                    .size(16.)
                    .color(if active {
                        Color32::WHITE
                    } else {
                        Color32::from_rgb(30, 20, 54)
                    }),
            )
            .fill(if active {
                Color32::from_rgb(190, 60, 81)
            } else {
                Color32::from_rgb(166, 143, 249)
            })
            .corner_radius(10.)
            .min_size(Vec2::new(ui.available_width() - 38., 47.));
            if ui
                .add_enabled(!busy && self.cap.is_some() && self.job.is_none(), b)
                .clicked()
            {
                self.toggle(ui.ctx());
            }
            if active
                && ui
                    .button(if s.phase == Phase::Paused {
                        tr(l, "Resume")
                    } else {
                        tr(l, "Pause")
                    })
                    .clicked()
            {
                self.controller.send(Control::PauseResume);
            }
            ui.label(
                RichText::new(if platform::has_tray() {
                    tr(current(ui), "F9")
                } else {
                    ""
                })
                .small()
                .color(ui.visuals().weak_text_color()),
            );
        });
    }
    fn dashboard(&mut self, ui: &mut egui::Ui) {
        let s = self.controller.snapshot();
        let l = self.config.language;
        let colors = design::palette(self.config.theme);
        let active = self.controller.active();
        let encoder = self
            .cap
            .as_ref()
            .and_then(|cap| cap.select(&self.config).ok())
            .map(|e| e.label())
            .unwrap_or(tr(l, "Checking hardware…"));
        let total = ui.available_width();
        let left = (total - 18.) * 0.56;
        ui.allocate_ui_with_layout(
            Vec2::new(total, 280.),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                ui.vertical(|ui| {
                    ui.set_width(left);
                    egui::Frame::new()
                        .fill(colors.surface)
                        .stroke(egui::Stroke::new(1_f32, colors.border))
                        .corner_radius(16.)
                        .inner_margin(20.)
                        .show(ui, |ui| {
                            ui.set_width(left - 40.);
                            ui.horizontal(|ui| {
                                design::eyebrow(ui, tr(l, "CAPTURE SOURCE"));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        design::badge(
                                            ui,
                                            if matches!(
                                                self.config.video.source,
                                                Source::Monitor { .. }
                                            ) {
                                                "DESKTOP"
                                            } else {
                                                "WINDOW"
                                            },
                                            ACCENT,
                                        );
                                    },
                                );
                            });
                            design::source_art(
                                ui,
                                matches!(
                                    self.config.video.source,
                                    Source::Window { .. } | Source::X11Window { .. }
                                ),
                                colors,
                            );
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(design::display(
                                        self.config.video.source.label(),
                                        16.,
                                    ));
                                    ui.label(
                                        RichText::new(tr(
                                            l,
                                            "Preview off · more room for the game",
                                        ))
                                        .size(11.)
                                        .color(colors.muted),
                                    );
                                });
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .add_enabled(
                                                !active,
                                                egui::Button::new(tr(l, "Change"))
                                                    .corner_radius(7.),
                                            )
                                            .clicked()
                                        {
                                            self.page = Page::Recording;
                                        }
                                    },
                                );
                            });
                        });
                });
                ui.add_space(6.);
                ui.vertical(|ui| {
                    ui.set_width(total - left - 24.);
                    card(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 4.;
                        ui.horizontal(|ui| {
                            design::eyebrow(ui, tr(l, "RECORDING CONTROL"));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    design::badge(
                                        ui,
                                        if active {
                                            tr(l, s.phase.label())
                                        } else {
                                            tr(l, "●  READY")
                                        },
                                        if s.phase == Phase::Recording {
                                            Color32::from_rgb(255, 119, 145)
                                        } else {
                                            design::MINT
                                        },
                                    );
                                },
                            );
                        });
                        ui.add_space(6.);
                        ui.label(design::display(elapsed(s.elapsed), 40.).extra_letter_spacing(1.));
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{}  /  {} FPS  /  {}",
                                    resolution(&self.config.video, l),
                                    self.config.video.fps.value(),
                                    self.config.container.label()
                                ))
                                .size(11.)
                                .color(colors.muted),
                            );
                        });
                        ui.add_space(4.);
                        self.record_button(ui);
                        ui.add_space(6.);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(tr(current(ui), "●")).size(8.).color(ACCENT));
                            ui.label(
                                RichText::new(tr(l, "Your recordings stay on this computer."))
                                    .size(11.)
                                    .color(colors.muted),
                            );
                        });
                        ui.add_space(4.);
                    });
                });
            },
        );
        ui.add_space(16.);
        ui.columns(3, |cols| {
            summary_tile(
                &mut cols[0],
                Page::Video,
                tr(l, "ENCODER"),
                encoder,
                tr(l, "Verified on your hardware"),
                colors,
            );
            summary_tile(
                &mut cols[1],
                Page::Performance,
                tr(l, "RECORDING PERFORMANCE"),
                &if active {
                    format!("{:.1} FPS", s.encoded_fps)
                } else {
                    format!("{} FPS", self.config.video.fps.value())
                },
                &if active {
                    format!("{} {}", s.drops, tr(l, "encoder drops"))
                } else {
                    tr(l, "Target frame rate").into()
                },
                colors,
            );
            summary_tile(
                &mut cols[2],
                Page::Recordings,
                tr(l, "FILE SIZE"),
                &size(s.size),
                if active {
                    tr(l, "Measured from video output")
                } else {
                    tr(l, "Ready for your next recording")
                },
                colors,
            );
        });
        ui.add_space(16.);
        card(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(design::display(tr(l, "Audio desk"), 16.));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new(tr(l, "Audio settings  →"))
                                    .size(12.)
                                    .color(ACCENT),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        self.page = Page::Audio;
                    }
                });
            });
            ui.add_space(8.);
            ui.columns(2, |cols| {
                for (index, name, source) in [
                    (0, tr(l, "Desktop sound"), &mut self.config.audio.system),
                    (1, tr(l, "Microphone"), &mut self.config.audio.mic),
                ] {
                    cols[index].horizontal(|ui| {
                        ui.add_enabled_ui(!active, |ui| {
                            design::toggle(ui, &mut source.enabled);
                        });
                        ui.label(RichText::new(name).size(13.));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(if source.enabled {
                                    tr(l, "On")
                                } else {
                                    tr(l, "Off")
                                })
                                .size(11.)
                                .color(if source.enabled {
                                    design::MINT
                                } else {
                                    colors.muted
                                }),
                            );
                        });
                    });
                }
            });
        });
        ui.add_space(19.);
        ui.horizontal(|ui| {
            ui.label(design::display(tr(l, "Recent recordings"), 17.));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(tr(l, "View all  →")).size(12.).color(ACCENT),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    self.page = Page::Recordings;
                }
            });
        });
        ui.add_space(7.);
        let files: Vec<_> = self.recordings.iter().take(2).cloned().collect();
        let mut convert = None;
        if files.is_empty() {
            egui::Frame::new()
                .fill(colors.surface)
                .stroke(egui::Stroke::new(1_f32, colors.border))
                .corner_radius(13.)
                .inner_margin(20.)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let (r, _) = ui.allocate_exact_size(Vec2::splat(42.), egui::Sense::hover());
                        ui.painter().rect_filled(r, 10., colors.tint);
                        design::icon(ui.painter(), r.shrink(9.), Page::Recordings, ACCENT);
                        ui.add_space(8.);
                        ui.vertical(|ui| {
                            ui.label(design::display(
                                tr(l, "Your first great moment starts here."),
                                14.,
                            ));
                            ui.label(
                                RichText::new(tr(
                                    l,
                                    "Hit record. Your moments will be waiting right here.",
                                ))
                                .size(12.)
                                .color(colors.muted),
                            );
                        });
                    });
                });
        }
        for path in files {
            recording_row(ui, &path, l, |p| convert = Some(p));
        }
        if let Some(input) = convert
            && !active
            && self.job.is_none()
        {
            let ffmpeg = self.config.ffmpeg.clone();
            self.job(ui.ctx().clone(), move || {
                Ok(format!(
                    "Converted: {}",
                    engine::remux(&ffmpeg, &input)?.display()
                ))
            });
        }
    }
    fn recording(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        let active = self.controller.active();
        card(ui, |ui| {
            section(ui, tr(l, "Capture source"));
            ui.add_enabled_ui(!active,|ui|{
        egui::ComboBox::from_id_salt("source").width(500.).selected_text(self.config.video.source.label()).show_ui(ui,|ui|{for source in &self.sources{ui.selectable_value(&mut self.config.video.source,source.clone(),format!("{}  {}",if matches!(source,Source::Window{..}|Source::X11Window{..}){"Window / Game"}else{"Monitor"},source.label()));}});
        if ui.button(tr(l,"Refresh sources")).clicked(){self.sources=platform::sources_for(&self.config.ffmpeg);}
        choice(ui,tr(l,"Capture backend"),&mut self.config.video.backend,platform::backends(),Backend::label);
        ui.checkbox(&mut self.config.video.cursor,tr(l,"Capture cursor"));
        ui.label(tr(current(ui), "Game recording uses the selected game window. Alt+Tab keeps the session open. Minimized windows, DRM and some exclusive fullscreen games can stop producing frames."));
        if matches!(self.config.video.source,Source::Window{..})&&self.config.video.backend==Backend::Dxgi{ui.colored_label(Color32::LIGHT_RED,"DXGI cannot capture a selected window. Choose WGC.");}
    });
        });
        ui.add_space(16.);
        card(ui, |ui| {
            section(ui, tr(l, "Output folder"));
            ui.add_enabled_ui(!active,|ui|{let mut folder=self.config.output_dir.to_string_lossy().into_owned();if ui.add(egui::TextEdit::singleline(&mut folder).desired_width(f32::INFINITY)).changed(){self.config.output_dir=folder.into();}ui.horizontal(|ui|{if ui.button(tr(l,"Open folder")).clicked(){let r=std::fs::create_dir_all(&self.config.output_dir).map_err(anyhow::Error::from).and_then(|_|platform::open(&self.config.output_dir));if let Err(e)=r{self.notice=e.to_string();}}
if let Ok(free)=platform::free_space(&self.config.output_dir){ui.label(fill(l,"{0} available",&[size(free)]));}});choice(ui,tr(l,"Container"),&mut self.config.container,Container::ALL,Container::label);ui.label(tr(l,"Filename format"));ui.text_edit_singleline(&mut self.config.filename_format);ui.label(tr(current(ui), "Date tokens: %Y-%m-%d_%H-%M-%S. MKV protects live video. Audio tracks are joined after Stop; recovery sources stay on disk."));ui.checkbox(&mut self.config.audio.export_stems,tr(l,"Export WAV stems"));let kbps=if self.config.video.rate_control==RateControl::Cqp {self.config.video.bitrate_kbps}else{self.config.video.max_bitrate_kbps};ui.label(fill(l,"Bitrate reference: about {0} GB / hour at {1} Kbps; CQP size varies with content.",&[format!("{:.1}",kbps as f64*3600./8./1e6),kbps.to_string()]));});
        });
        ui.add_space(20.);
        self.record_button(ui);
        ui.add_space(20.);
        self.replay_controls(ui);
    }
    fn video(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        ui.add_enabled_ui(!self.controller.active(),|ui|{card(ui,|ui|{section(ui,tr(l,"Resolution"));let current_resolution=resolution(&self.config.video,l);egui::ComboBox::from_id_salt("resolution").selected_text(current_resolution).width(250.).show_ui(ui,|ui|{for (w,h) in [(0,0),(640,360),(854,480),(1280,720),(1600,900),(1920,1080),(2560,1440),(3840,2160)]{if ui.selectable_label(self.config.video.width==w&&self.config.video.height==h,if w==0{tr(l,"Source resolution").into()}else{format!("{w} × {h}")}).clicked(){self.config.video.width=w;self.config.video.height=h;}}});ui.horizontal(|ui|{ui.label(tr(current(ui), "Custom / source (0 × 0)"));ui.add(egui::DragValue::new(&mut self.config.video.width).range(0..=7680).speed(2));ui.label(tr(current(ui), "×"));ui.add(egui::DragValue::new(&mut self.config.video.height).range(0..=4320).speed(2));});ui.add_space(16.);section(ui,tr(l,"Frame rate"));ui.horizontal(|ui|{for (i,label) in ["Common FPS","Integer FPS","Fractional FPS"].iter().enumerate(){ui.selectable_value(&mut self.config.video.fps_mode,i as u8,tr(l,label));}});match self.config.video.fps_mode{0=>{egui::ComboBox::from_id_salt("commonfps").selected_text(self.config.video.fps.to_string()).show_ui(ui,|ui|{for n in [24,25,30,48,50,60,75,90,100,120]{ui.selectable_value(&mut self.config.video.fps,Rate{num:n,den:1},format!("{n} FPS"));}});},1=>{let mut fps=(self.config.video.fps.value().round() as u32).clamp(1,120);if ui.add(egui::DragValue::new(&mut fps).range(1..=120).suffix(" FPS")).changed(){self.config.video.fps=Rate{num:fps,den:1};}},_=>{egui::ComboBox::from_id_salt("fractionfps").selected_text(self.config.video.fps.to_string()).show_ui(ui,|ui|{for n in [24000,30000,60000,120000]{ui.selectable_value(&mut self.config.video.fps,Rate{num:n,den:1001},format!("{n}/1001 · {:.3}",n as f64/1001.));}});ui.horizontal(|ui|{ui.add(egui::DragValue::new(&mut self.config.video.fps.num).range(1..=1_000_000));ui.label(tr(current(ui), "/"));ui.add(egui::DragValue::new(&mut self.config.video.fps.den).range(1..=1_000_000));});}}ui.label(fill(l,"{0} FPS requested. Actual delivery depends on the source and encoder. VFR preserves timestamps without an artificial FPS duplication filter.",&[format!("{:.3}",self.config.video.fps.value())]));});
        ui.add_space(16.);card(ui,|ui|{section(ui,tr(l,"Encoder"));egui::ComboBox::from_id_salt("encoder").selected_text(tr(l,self.config.video.encoder.label())).width(300.).show_ui(ui,|ui|{ui.selectable_value(&mut self.config.video.encoder,Encoder::Auto,tr(l,Encoder::Auto.label()));if let Some(cap)=&self.cap{for support in &cap.encoders{if support.supported{ui.selectable_value(&mut self.config.video.encoder,support.encoder,tr(l,support.encoder.label()));}}}});choice(ui,tr(l,"Quality"),&mut self.config.video.quality,Quality::ALL,Quality::label);choice(ui,tr(l,"Encoding speed"),&mut self.config.video.speed,Speed::ALL,Speed::label);ui.label(tr(current(ui), "AMD maps these five choices onto its three native modes: speed / balanced / quality. Quality changes actual quantization; speed changes encoder effort."));});
        ui.add_space(12.);ui.checkbox(&mut self.advanced,tr(l,"Advanced controls"));if self.advanced{card(ui,|ui|{choice(ui,"Rate control",&mut self.config.video.rate_control,RateControl::ALL,RateControl::label);ui.add_enabled_ui(self.config.video.rate_control!=RateControl::Cqp,|ui|{ui.horizontal(|ui|{ui.label(tr(current(ui), "Target Kbps"));ui.add(egui::DragValue::new(&mut self.config.video.bitrate_kbps).range(100..=500000));ui.label(tr(current(ui), "Maximum Kbps"));ui.add(egui::DragValue::new(&mut self.config.video.max_bitrate_kbps).range(100..=500000));});});ui.horizontal(|ui|{ui.label(tr(current(ui), "Keyframe interval (seconds)"));ui.add(egui::DragValue::new(&mut self.config.video.keyframe_seconds).range(1..=30));});egui::ComboBox::from_label(tr(current(ui), "H.264 profile")).selected_text(&self.config.video.profile).show_ui(ui,|ui|{for p in ["high","main","baseline"]{ui.selectable_value(&mut self.config.video.profile,p.into(),p);}});ui.horizontal(|ui|{ui.label(tr(current(ui), "B-frames (0 recommended for compatibility)"));ui.add(egui::DragValue::new(&mut self.config.video.bframes).range(0..=3));});choice(ui,if cfg!(windows){"WGC scaling filter"}else{"Scaling filter"},&mut self.config.video.scaling,Scaling::ALL,Scaling::label);ui.add_enabled_ui(cfg!(windows),|ui|{egui::ComboBox::from_label(tr(current(ui), "Frame transfer path")).selected_text(self.config.video.transfer.label()).width(430.).show_ui(ui,|ui|{for p in [GpuPath::CpuTransfer,GpuPath::Direct,GpuPath::D3d11Nv12,GpuPath::AmfNv12]{ui.selectable_value(&mut self.config.video.transfer,p,p.label());}});});ui.label(tr(current(ui), "GPU paths depend on driver interoperability and may fail. The RAM compatibility path is verified on the development host. Intel QSV, Media Foundation and software use the RAM path. No zero-copy performance claim is made."));ui.add_enabled(false,egui::Checkbox::new(&mut false,"Encoder lookahead (disabled for low overhead)"));ui.label(tr(current(ui), "SDR only. Advanced HDR, custom color-space/range conversion and per-device B-frame capability enumeration are not implemented."));});}
if ui.button(tr(l,"Reset to defaults")).clicked(){self.config.video=Video::default();}});
    }
    fn audio(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        let active = self.controller.active() || self.job.is_some();
        ui.add_enabled_ui(!active, |ui| {
            for (name, source, render) in [
                (
                    "Desktop / system audio",
                    &mut self.config.audio.system,
                    true,
                ),
                ("Microphone", &mut self.config.audio.mic, false),
                ("Additional input", &mut self.config.audio.additional, false),
            ] {
                card(ui, |ui| {
                    audio_source(ui, tr(l, name), source, render, &self.devices, l);
                });
                ui.add_space(12.);
            }
        });
        let peak = self
            .meter
            .as_ref()
            .map(|m| f32::from_bits(m.peak.load(Ordering::Relaxed)))
            .unwrap_or(self.controller.snapshot().mic_peak);
        ui.horizontal(|ui| {
            ui.label(tr(current(ui), "Mic peak"));
            ui.add(
                egui::ProgressBar::new(peak.clamp(0., 1.))
                    .desired_width(280.)
                    .text(if peak > 0. {
                        format!("{:.1} dBFS", 20. * peak.log10())
                    } else {
                        "— dBFS".into()
                    }),
            );
            if self.controller.active() && ui.button(tr(l, "Mute microphone")).clicked() {
                self.controller.muted.fetch_xor(true, Ordering::Relaxed);
            }
        });
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!active, egui::Button::new(tr(l, "Test microphone (8 s)")))
                .clicked()
            {
                self.mic_test(ui.ctx().clone());
            }
            if let Some(path) = &self.last_test
                && ui
                    .add_enabled(path.exists(), egui::Button::new(tr(l, "Play last test")))
                    .clicked()
                && let Err(e) = platform::open(path)
            {
                self.notice = e.to_string();
            }
        });
        ui.add_space(16.);
        ui.add_enabled_ui(!active,|ui|{card(ui,|ui|{section(ui,tr(l,"Microphone filters"));let old=self.config.audio.preset;choice(ui,tr(l,"Voice preset"),&mut self.config.audio.preset,VoicePreset::ALL,VoicePreset::label);if old!=self.config.audio.preset{self.config.audio.apply(self.config.audio.preset);}ui.label(tr(current(ui), "RNNoise runs locally at 48 kHz only when enabled. It can reduce fan noise; keyboard transients can remain. Conservative settings preserve voice clarity."));let mut swap=None;let count=self.config.audio.filters.len();for (i,f) in self.config.audio.filters.iter_mut().enumerate(){ui.add_space(8.);ui.separator();ui.horizontal(|ui|{ui.checkbox(&mut f.enabled,tr(l,f.kind.label()));if ui.add_enabled(i>0,egui::Button::new(tr(current(ui), "↑"))).clicked(){swap=Some((i,i-1));}
if ui.add_enabled(i+1<count,egui::Button::new(tr(current(ui), "↓"))).clicked(){swap=Some((i,i+1));}});if f.enabled{match f.kind{FilterKind::Suppression=>{ui.add(egui::Slider::new(&mut f.strength,0.1..=1.).text(tr(current(ui), "Suppression mix · low / medium / high")));},FilterKind::Gain=>{ui.add(egui::Slider::new(&mut f.gain_db,-30.0..=30.).text(tr(current(ui), "Gain dB")));},_=>{ui.add(egui::Slider::new(&mut f.threshold_db,-80.0..=0.).text(tr(current(ui), "Threshold dB")));if f.kind==FilterKind::Gate{ui.add(egui::Slider::new(&mut f.close_db,-90.0..=0.).text(tr(current(ui), "Close threshold dB")));ui.add(egui::Slider::new(&mut f.hold_ms,0.0..=1000.).text(tr(current(ui), "Hold ms")));}
if matches!(f.kind,FilterKind::Compressor|FilterKind::Expander){ui.add(egui::Slider::new(&mut f.ratio,1.0..=20.).text(tr(current(ui), "Ratio")));}
if f.kind!=FilterKind::Limiter{ui.add(egui::Slider::new(&mut f.attack_ms,0.1..=100.).text(tr(current(ui), "Attack ms")));}ui.add(egui::Slider::new(&mut f.release_ms,1.0..=1000.).text(tr(current(ui), "Release ms")));if f.kind==FilterKind::Compressor{ui.add(egui::Slider::new(&mut f.gain_db,0.0..=20.).text(tr(current(ui), "Output gain dB")));}}}}}
if let Some((a,b))=swap{self.config.audio.filters.swap(a,b);self.config.audio.preset=VoicePreset::Custom;}
if ui.button(tr(l,"Reset to defaults")).clicked(){self.config.audio.apply(VoicePreset::Natural);}});});
        ui.add_space(12.);
        ui.label(tr(current(ui), "Application-only process loopback, live headphone monitoring and push-to-talk are not available in this release. Desktop audio captures the selected output endpoint; it does not isolate a game. Bluetooth microphone activation can change the headset's audio profile."));
    }
    fn performance(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        ui.add_enabled_ui(!self.controller.active(),|ui|{card(ui,|ui|{let p=self.config.performance.profile;choice(ui,tr(l,"Performance Profile"),&mut self.config.performance.profile,Profile::ALL,Profile::label);if p!=self.config.performance.profile{self.config.performance.apply(self.config.performance.profile,&mut self.config.video);}number_choice(ui,tr(l,"Processing Workers"),&mut self.config.performance.workers,l);ui.label(tr(current(ui), "Controls the application's audio DSP pool and FFmpeg CPU conversion/software encoding. Hardware encoder internal threads are controlled by its driver. Auto caps the DSP pool at 3."));ui.add_enabled_ui(cfg!(windows),|ui|{number_choice(ui,tr(l,"Frame Buffer Count"),&mut self.config.performance.buffers,l);});ui.label(tr(current(ui), "Controls AMF async_depth (in-flight encoder frames). Auto = 2. Other hardware backends retain their own capture queues. More buffering raises latency and memory use; low memory forces 1."));choice(ui,tr(l,"Process Priority"),&mut self.config.performance.priority,platform::priorities(),Priority::label);if self.config.performance.priority==Priority::High{ui.colored_label(Color32::YELLOW,"High priority can starve your game and audio services. Normal is recommended.");}for (label,value) in [("Reduce UI updates while recording",&mut self.config.performance.reduce_ui),("Low memory mode",&mut self.config.performance.low_memory),("Prefer hardware encoding",&mut self.config.performance.prefer_hardware),("Detailed engine logs",&mut self.config.performance.detailed_logs),("Minimize to tray on record",&mut self.config.performance.minimize_on_start)]{if !cfg!(windows)&&label=="Low memory mode"{continue;}ui.checkbox(value,if !cfg!(windows)&&label=="Minimize to tray on record"{"Minimize on record"}else{tr(l,label)});}ui.label(tr(current(ui), "Preview rendering stays off. Profiles never change game settings, recording resolution or target FPS."));ui.add_enabled(false,egui::Checkbox::new(&mut false,"Adaptive performance (not implemented)"));});});
        ui.add_space(16.);
        card(ui, |ui| {
            section(ui, tr(current(ui), "Measured statistics"));
            let s = self.controller.snapshot();
            egui::Grid::new("perfstats")
                .num_columns(2)
                .spacing([40., 10.])
                .show(ui, |ui| {
                    for (label, value) in [
                        (
                            "Application CPU (whole system)",
                            format!("{:.2}%", self.resources.cpu_percent),
                        ),
                        (
                            "Application working set",
                            format!("{:.1} MiB", self.resources.ram_mb),
                        ),
                        (
                            "FFmpeg CPU / working set",
                            format!(
                                "{:.2}% / {:.1} MiB",
                                self.child_resources.cpu_percent, self.child_resources.ram_mb
                            ),
                        ),
                        (
                            "Encoded output FPS (wall clock)",
                            format!("{:.2}", s.encoded_fps),
                        ),
                        ("Encoded frames", s.frames.to_string()),
                        ("Encoder-reported dropped frames", s.drops.to_string()),
                        (
                            "Video write throughput",
                            format!("{:.2} MiB/s", s.write_mbps),
                        ),
                        (
                            "Audio discontinuities / queue drops",
                            s.audio_errors.to_string(),
                        ),
                        ("Requested FPS", self.config.video.fps.to_string()),
                        (
                            "Capture FPS / capture drops",
                            "Unavailable: FFmpeg does not expose the WGC callback counters".into(),
                        ),
                        (
                            "GPU activity / capture latency / encode latency",
                            "Unavailable: no fabricated estimates".into(),
                        ),
                        (
                            "Startup A/V alignment",
                            "Estimated at first frame arrival; validate with clap/flash test"
                                .into(),
                        ),
                    ] {
                        ui.label(tr(current(ui), label));
                        ui.label(tr(current(ui), &value));
                        ui.end_row();
                    }
                });
        });
    }
    fn hotkeys(&mut self, ui: &mut egui::Ui) {
        if !platform::has_tray() {
            card(ui, |ui| {
                ui.label(platform::note());
            });
            return;
        }
        let l = self.config.language;
        ui.add_enabled_ui(!self.controller.active(),|ui|{card(ui,|ui|{section(ui,tr(current(ui), "Global shortcuts"));for (label,key) in [("Start recording",&mut self.config.hotkeys.record),("Pause / resume",&mut self.config.hotkeys.pause),("Mute microphone",&mut self.config.hotkeys.mute),("Show / hide app",&mut self.config.hotkeys.visibility),("Save replay",&mut self.config.hotkeys.replay)]{egui::ComboBox::from_label(tr(l,label)).selected_text(format!("F{}",*key-0x6f)).show_ui(ui,|ui|{for i in (1..=24).filter(|i|*i!=12){ui.selectable_value(key,0x6f+i,format!("F{i}"));}});}ui.label(tr(current(ui), "Windows RegisterHotKey detects conflicts with other apps. Duplicate assignments are rejected. Key repeat cannot stop a recording accidentally. Changes take effect when applied."));if ui.button(tr(l,"Apply shortcuts")).clicked(){match self.config.validate(){Ok(())=>{self.bridge.take();self.bridge=Some(NativeBridge::start(self.config.hotkeys.clone(),self.config.language,ui.ctx().clone()));self.save();},Err(e)=>self.notice=e.to_string()}}ui.label(tr(current(ui), "Save Replay saves the active video-only RAM buffer. Start it explicitly on the Recording page."));});});
    }
    fn recordings(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        ui.horizontal(|ui| {
            if ui.button("↻ Refresh").clicked() {
                self.refresh_recordings();
            }
            if ui.button(tr(l, "Open folder")).clicked()
                && let Err(e) = platform::open(&self.config.output_dir)
            {
                self.notice = e.to_string();
            }
        });
        ui.add_space(16.);
        let files = self.recordings.clone();
        let mut convert = None;
        for p in files {
            recording_row(ui, &p, l, |p| convert = Some(p));
            ui.add_space(8.);
        }
        if let Some(input) = convert
            && !self.controller.active()
            && self.job.is_none()
        {
            let ffmpeg = self.config.ffmpeg.clone();
            self.job(ui.ctx().clone(), move || {
                Ok(format!(
                    "Converted: {}",
                    engine::remux(&ffmpeg, &input)?.display()
                ))
            });
        }
        if !self.recovery.is_empty() {
            ui.add_space(20.);
            section(ui, tr(l, "Recovery sessions"));
            for p in self.recovery.clone() {
                ui.horizontal(|ui| {
                    ui.label(
                        p.parent()
                            .and_then(|p| p.file_name())
                            .unwrap_or_default()
                            .to_string_lossy(),
                    );
                    if ui
                        .add_enabled(
                            !self.controller.active(),
                            egui::Button::new(tr(l, "Recover")),
                        )
                        .clicked()
                        && let Err(e) = self.controller.recover(p.clone())
                    {
                        self.notice = e.to_string();
                    }
                });
            }
        }
        ui.label(tr(current(ui), "Recovery folders retain original MKV and PCM sources. A completed session can consume substantial extra space. Remove its .potato-session-* folder manually only after checking the final recording."));
    }
    fn settings(&mut self, ui: &mut egui::Ui) {
        let l = self.config.language;
        if !cfg!(windows) {
            ui.label(platform::note());
            ui.add_space(12.);
        }
        card(ui, |ui| {
            egui::ComboBox::from_label(tr(l, "Language"))
                .width(320.)
                .selected_text(self.config.language.label())
                .show_ui(ui, |ui| {
                    for language in Language::ALL {
                        ui.selectable_value(&mut self.config.language, *language, language.label());
                    }
                });
            let old = self.config.theme;
            choice(
                ui,
                tr(l, "Theme"),
                &mut self.config.theme,
                Theme::ALL,
                Theme::label,
            );
            if old != self.config.theme {
                theme(ui.ctx(), self.config.theme);
            }
            ui.label(tr(
                current(ui),
                "FFmpeg executable (8.1+; ffprobe.exe must be alongside it or on PATH)",
            ));
            ui.add_enabled_ui(!self.controller.active(), |ui| {
                let mut ffmpeg = self.config.ffmpeg.to_string_lossy().into_owned();
                if ui.text_edit_singleline(&mut ffmpeg).changed() {
                    self.config.ffmpeg = ffmpeg.into();
                }
            });
            ui.horizontal(|ui| {
                if ui.button(tr(l, "Save settings")).clicked() {
                    self.save();
                }
                if ui
                    .add_enabled(
                        self.detect.is_none() && !self.controller.active(),
                        egui::Button::new(tr(l, "Detect capabilities")),
                    )
                    .clicked()
                {
                    self.detect(ui.ctx().clone());
                }
            });
            ui.label(format!(
                "{}: {}",
                tr(l, "Configuration"),
                self.config_path.display()
            ));
        });
        ui.add_space(16.);
        card(ui, |ui| {
            section(ui, tr(l, "Compatibility"));
            if let Some(cap) = &self.cap {
                ui.label(&cap.version);
                if let Some(error) = &cap.error {
                    ui.colored_label(Color32::LIGHT_RED, error);
                }
                ui.label(format!("WGC: {} · DXGI: {}", cap.wgc, cap.dxgi));
                for e in &cap.encoders {
                    ui.horizontal_wrapped(|ui| {
                        ui.colored_label(
                            if e.supported {
                                Color32::LIGHT_GREEN
                            } else {
                                ui.visuals().weak_text_color()
                            },
                            format!(
                                "{} {}",
                                if e.supported { "✓" } else { "—" },
                                e.encoder.label()
                            ),
                        );
                        ui.label(&e.detail);
                    });
                }
                ui.add_space(10.);
                ui.label(&cap.ffmpeg_license);
            } else {
                ui.spinner();
                ui.label(tr(current(ui), "Testing real encoder initialization…"));
            }
            ui.separator();
            ui.label(tr(current(ui), "Implemented: WGC window/desktop, DXGI desktop, tested hardware/software encoders, rational FPS, recording/pause, WASAPI desktop + microphone + extra input, RNNoise and dynamics, independent tracks, MKV/MP4/fMP4, remux, recovery, bounded video replay, global keys and tray."));
            ui.label(tr(current(ui), "Not implemented: replay audio, process-specific game audio, live headphone monitoring, preview rendering, automatic GPU reset recovery, adaptive optimization and HDR. GPU-resident interoperability depends on the driver; compatibility transfer is the default. 23 language packs are embedded; technical operating-system and encoder diagnostics retain their original wording."));
            ui.label(tr(current(ui), "No telemetry, advertisements or runtime network requests. Encoder discovery only encodes generated color frames; recording and microphone tests require a user action."));
        });
    }
}

impl eframe::App for PotatoApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.native_language != self.config.language {
            self.bridge.take();
            self.bridge = Some(NativeBridge::start(
                self.config.hotkeys.clone(),
                self.config.language,
                ctx.clone(),
            ));
            self.native_language = self.config.language;
        }

        ctx.data_mut(|d| d.insert_temp(egui::Id::new("potato-language"), self.config.language));
        if let Some(rx) = &self.detect
            && let Ok((cap, devices, sources)) = rx.try_recv()
        {
            self.cap = Some(cap);
            self.devices = devices;
            self.sources = sources;
            #[cfg(not(windows))]
            if matches!(self.config.video.source, Source::Monitor { .. })
                && let Some(source) = self.sources.first()
            {
                self.config.video.source = source.clone();
            }
            self.detect = None;
        }
        let events: Vec<_> = self
            .bridge
            .as_ref()
            .map(|b| b.rx.try_iter().collect())
            .unwrap_or_default();
        for event in events {
            match event {
                NativeEvent::Record => self.toggle(ctx),
                NativeEvent::Pause => self.controller.send(Control::PauseResume),
                NativeEvent::Mute => {
                    self.controller.muted.fetch_xor(true, Ordering::Relaxed);
                }
                NativeEvent::Show => {
                    if self.hidden {
                        self.show(ctx)
                    } else {
                        self.hide(ctx)
                    }
                }
                NativeEvent::OpenFolder => {
                    if let Err(e) = platform::open(&self.config.output_dir) {
                        self.notice = e.to_string();
                    }
                }
                NativeEvent::Exit => {
                    self.exit_when_done = true;
                    self.controller.send(Control::Stop);
                    self.replay.stop();
                    self.show(ctx);
                }
                NativeEvent::Replay => {
                    self.save_replay(ctx);
                }
                NativeEvent::Error(e) => {
                    self.notice = e;
                    self.show(ctx);
                }
            }
        }
        let jobs: Vec<_> = self
            .job
            .as_ref()
            .map(|rx| rx.try_iter().collect())
            .unwrap_or_default();
        for event in jobs {
            match event {
                JobEvent::Meter(m) => self.meter = Some(m),
                JobEvent::Done(result) => {
                    self.notice = match result {
                        Ok(s) => s,
                        Err(e) => e,
                    };
                    self.job = None;
                    self.meter = None;
                    self.refresh_recordings();
                }
            }
        }
        let snapshot = self.controller.snapshot();
        if self.last_metrics.elapsed() > Duration::from_secs(1) {
            self.resources = self.sampler.sample(None);
            self.child_resources = if snapshot.ffmpeg_pid.is_some() {
                self.child_sampler.sample(snapshot.ffmpeg_pid)
            } else {
                Resources::default()
            };
            self.last_metrics = Instant::now();
            if !self.controller.active() {
                self.refresh_recordings();
            }
        }
        if ctx.input(|i| i.viewport().close_requested())
            && (self.controller.active()
                || self.replay.active.load(Ordering::Relaxed)
                || self.job.is_some())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.exit_when_done = true;
            self.controller.send(Control::Stop);
            self.replay.stop();
        }
        if self.exit_when_done
            && !self.controller.active()
            && !self.replay.active.load(Ordering::Relaxed)
            && self.job.is_none()
        {
            self.save();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        let l = self.config.language;
        let colors = design::palette(self.config.theme);
        design::titlebar(ctx, colors);
        egui::SidePanel::left("navigation")
            .resizable(false)
            .exact_width(218.)
            .frame(
                egui::Frame::new()
                    .fill(colors.sidebar)
                    .inner_margin(egui::Margin::symmetric(17, 24)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(40.), egui::Sense::hover());
                    design::potato(ui.painter(), r);
                    ui.add_space(3.);
                    ui.vertical(|ui| {
                        ui.label(design::display("potato", 25.));
                        ui.label(
                            RichText::new(tr(current(ui), "RECORDER"))
                                .size(9.)
                                .extra_letter_spacing(2.6)
                                .color(colors.muted),
                        );
                    });
                });
                ui.add_space(28.);
                for (index, page) in Page::ALL.iter().copied().enumerate() {
                    if [0, 2, 6].contains(&index) {
                        if index > 0 {
                            ui.add_space(17.);
                        }
                        design::eyebrow(
                            ui,
                            match index {
                                0 => tr(l, "WORKSPACE"),
                                2 => tr(l, "RECORDING SETUP"),
                                _ => tr(l, "LIBRARY"),
                            },
                        );
                        ui.add_space(7.);
                    }
                    if design::nav(ui, page, tr(l, page.name()), self.page == page, colors)
                        .clicked()
                    {
                        self.page = page;
                        if page == Page::Recordings {
                            self.refresh_recordings();
                        }
                    }
                    ui.add_space(2.);
                }
                ui.add_space((ui.available_height() - 144.).max(8.));
                egui::Frame::new()
                    .fill(colors.surface)
                    .stroke(egui::Stroke::new(1_f32, colors.border))
                    .corner_radius(11.)
                    .inner_margin(13.)
                    .show(ui, |ui| {
                        ui.set_width(158.);
                        design::eyebrow(ui, tr(l, "PERFORMANCE PROFILE"));
                        ui.horizontal(|ui| {
                            ui.colored_label(design::MINT, "●");
                            if ui
                                .add(
                                    egui::Label::new(design::display(
                                        tr(l, self.config.performance.profile.label()),
                                        13.,
                                    ))
                                    .truncate()
                                    .sense(egui::Sense::click()),
                                )
                                .on_hover_cursor(egui::CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.page = Page::Performance;
                            }
                        });
                        ui.label(
                            RichText::new(tr(l, "The game always comes first."))
                                .size(10.)
                                .color(colors.muted),
                        );
                    });
                ui.add_space(11.);
                ui.label(
                    RichText::new(format!("v{} · Potato Recorder", env!("CARGO_PKG_VERSION")))
                        .size(10.)
                        .color(colors.muted),
                );
            });
        egui::TopBottomPanel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(colors.background)
                    .inner_margin(egui::Margin::symmetric(28, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        if snapshot.phase == Phase::Recording {
                            Color32::LIGHT_RED
                        } else {
                            ACCENT
                        },
                        "●",
                    );
                    ui.label(tr(l, snapshot.phase.label()));
                    if self.detect.is_some() {
                        ui.spinner();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!(
                            "{:.1} MiB · {:.2}% CPU",
                            self.resources.ram_mb, self.resources.cpu_percent
                        ));
                        if self.dirty.is_some() {
                            ui.label(tr(current(ui), "Unsaved"));
                        }
                    });
                });
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(colors.background)
                    .inner_margin(egui::Margin::symmetric(28, 24)),
            )
            .show(ctx, |ui| {
                self.header(ui);
                if !self.notice.is_empty() {
                    let msg = self.notice.clone();
                    egui::Frame::new()
                        .fill(ui.visuals().extreme_bg_color)
                        .corner_radius(8.)
                        .inner_margin(12.)
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(crate::localization::message(self.config.language, &msg));
                                if ui.small_button("×").clicked() {
                                    self.notice.clear();
                                }
                            });
                        });
                    ui.add_space(12.);
                }
                if !snapshot.message.is_empty() {
                    ui.colored_label(
                        if snapshot.phase == Phase::Failed {
                            Color32::LIGHT_RED
                        } else {
                            ACCENT
                        },
                        crate::localization::message(self.config.language, &snapshot.message),
                    );
                    ui.add_space(10.);
                }
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        match self.page {
                            Page::Dashboard => self.dashboard(ui),
                            Page::Recording => self.recording(ui),
                            Page::Video => self.video(ui),
                            Page::Audio => self.audio(ui),
                            Page::Performance => self.performance(ui),
                            Page::Hotkeys => self.hotkeys(ui),
                            Page::Recordings => self.recordings(ui),
                            Page::Settings => self.settings(ui),
                        }
                        ui.add_space(30.);
                    });
            });
        let serialized = serde_json::to_string(&self.config).unwrap_or_default();
        if serialized != self.last_serialized {
            self.last_serialized = serialized;
            self.dirty = Some(Instant::now());
        }
        if self
            .dirty
            .is_some_and(|t| t.elapsed() > Duration::from_millis(800))
        {
            self.save();
        }
        ctx.request_repaint_after(
            if self.controller.active() && self.config.performance.reduce_ui {
                Duration::from_millis(500)
            } else if self.meter.is_some() {
                Duration::from_millis(80)
            } else {
                Duration::from_secs(1)
            },
        );
    }
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.save();
    }
}

fn theme(ctx: &egui::Context, t: Theme) {
    let colors = design::palette(t);
    let mut s = (*ctx.style()).clone();
    s.visuals = if t == Theme::Dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    s.visuals.panel_fill = colors.background;
    s.visuals.window_fill = colors.surface;
    s.visuals.override_text_color = Some(colors.text);
    s.visuals.weak_text_color = Some(colors.muted);
    s.visuals.selection.bg_fill = colors.tint;
    s.visuals.selection.stroke = egui::Stroke::new(1_f32, ACCENT);
    s.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1_f32, colors.border);
    s.visuals.widgets.inactive.bg_fill = colors.raised;
    s.visuals.widgets.inactive.weak_bg_fill = colors.raised;
    s.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1_f32, colors.border);
    s.visuals.widgets.hovered.bg_fill = colors.tint;
    s.visuals.widgets.hovered.weak_bg_fill = colors.tint;
    s.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1_f32, ACCENT.gamma_multiply(0.5));
    s.visuals.widgets.active.bg_fill = colors.tint;
    s.visuals.widgets.active.bg_stroke = egui::Stroke::new(1_f32, ACCENT);
    s.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(7);
    s.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(7);
    s.visuals.widgets.active.corner_radius = egui::CornerRadius::same(7);
    s.visuals.extreme_bg_color = colors.background;
    s.visuals.slider_trailing_fill = true;
    s.spacing.item_spacing = Vec2::new(12., 9.);
    s.spacing.button_padding = Vec2::new(13., 8.);
    s.spacing.interact_size = Vec2::new(40., 30.);
    s.animation_time = 0.06;
    s.text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(13.));
    s.text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(13.));
    s.text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(11.));
    ctx.set_style(s);
}
fn summary_tile(
    ui: &mut egui::Ui,
    page: Page,
    label: &str,
    value: &str,
    detail: &str,
    colors: design::Palette,
) {
    card(ui, |ui| {
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(29.), egui::Sense::hover());
            ui.painter().rect_filled(r, 8., colors.tint);
            design::icon(ui.painter(), r.shrink(5.), page, ACCENT);
            design::eyebrow(ui, label);
        });
        ui.add_space(7.);
        ui.label(design::display(value, 18.));
        ui.label(RichText::new(detail).size(11.).color(colors.muted));
    });
}
fn card(ui: &mut egui::Ui, contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(ui.visuals().window_fill())
        .stroke(egui::Stroke::new(
            1_f32,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(14.)
        .inner_margin(18.)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            contents(ui);
        });
}
fn section(ui: &mut egui::Ui, title: &str) {
    ui.label(RichText::new(title).size(17.).strong());
    ui.add_space(6.);
}

fn choice<T: Copy + PartialEq>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    all: &[T],
    name: impl Fn(T) -> &'static str,
) {
    ui.horizontal(|ui| {
        ui.label(tr(current(ui), label));
        egui::ComboBox::from_id_salt(label)
            .width(220.)
            .selected_text(tr(current(ui), name(*value)))
            .show_ui(ui, |ui| {
                for item in all {
                    ui.selectable_value(value, *item, tr(current(ui), name(*item)));
                }
            });
    });
}
fn number_choice(ui: &mut egui::Ui, label: &str, value: &mut u8, l: Language) {
    ui.horizontal(|ui| {
        ui.label(tr(current(ui), label));
        egui::ComboBox::from_id_salt(label)
            .selected_text(if *value == 0 {
                tr(l, "Auto").into()
            } else {
                value.to_string()
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(value, 0, tr(l, "Auto"));
                for i in 1..=5 {
                    ui.selectable_value(value, i, i.to_string());
                }
            });
    });
}
fn audio_source(
    ui: &mut egui::Ui,
    name: &str,
    s: &mut AudioSource,
    render: bool,
    devices: &[audio::Device],
    l: Language,
) {
    ui.push_id(name, |ui| {
        if render && !platform::system_audio_supported(){s.enabled=false;ui.label(name);ui.label("System loopback is unavailable on macOS. Use a configured virtual device as Additional Input.");return;}
        ui.horizontal(|ui| {
            ui.label(RichText::new(name).size(17.).strong());
            ui.checkbox(&mut s.enabled, tr(l, "Enabled"));
        });
        ui.add_enabled_ui(s.enabled, |ui| {
            egui::ComboBox::from_label(tr(l, "Device"))
                .width(360.)
                .selected_text(
                    devices
                        .iter()
                        .find(|d| d.id == s.device_id)
                        .map(|d| d.name.as_str())
                        .unwrap_or(tr(l, "Default device")),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut s.device_id, String::new(), tr(l, "Default device"));
                    for d in devices.iter().filter(|d| d.render == render) {
                        ui.selectable_value(&mut s.device_id, d.id.clone(), &d.name);
                    }
                });
            ui.add(
                egui::Slider::new(&mut s.gain_db, -30.0..=30.)
                    .text(tr(current(ui), "Input gain dB")),
            );
            ui.horizontal(|ui| {
                ui.checkbox(&mut s.mute, tr(l, "Mute"));
                ui.checkbox(&mut s.mono, tr(l, "Mono"));
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut s.mixed, tr(l, "Include in mixed track"));
                ui.checkbox(&mut s.isolated, tr(l, "Independent track"));
            });
        });
    });
}
fn recording_row(
    ui: &mut egui::Ui,
    path: &std::path::Path,
    l: Language,
    mut remux: impl FnMut(PathBuf),
) {
    card(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(path.file_name().unwrap_or_default().to_string_lossy()).strong(),
                );
                ui.label(
                    RichText::new(size(std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)))
                        .small()
                        .color(ui.visuals().weak_text_color()),
                );
            });
            if ui.button(tr(l, "Open")).clicked() {
                let _ = platform::open(path);
            }
            if path.extension().is_some_and(|x| x == "mkv")
                && ui.button(tr(l, "Remux to MP4")).clicked()
            {
                remux(path.to_owned());
            }
        });
    });
}
fn resolution(v: &Video, l: Language) -> String {
    if v.width == 0 {
        tr(l, "Source").into()
    } else {
        format!("{} × {}", v.width, v.height)
    }
}
fn size(n: u64) -> String {
    if n > 1024 * 1024 * 1024 {
        format!("{:.2} GiB", n as f64 / 1073741824.)
    } else {
        format!("{:.1} MiB", n as f64 / 1048576.)
    }
}
fn elapsed(t: f64) -> String {
    let t = t.max(0.) as u64;
    format!("{:02}:{:02}:{:02}", t / 3600, t / 60 % 60, t % 60)
}
