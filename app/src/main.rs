#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub use potato_recorder::{audio, engine, localization, platform, replay};
mod ui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "--ui-test" {
        if let Err(e) = engine::cli(&args[1..]) {
            eprintln!("{e:#}");
            std::process::exit(1);
        }
        return;
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Potato Recorder")
            .with_decorations(false)
            .with_inner_size([1280., 860.])
            .with_min_inner_size([980., 720.]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "Potato Recorder",
        options,
        Box::new(|cc| Ok(Box::new(ui::PotatoApp::new(cc)))),
    ) {
        eprintln!("{e}");
    }
}
