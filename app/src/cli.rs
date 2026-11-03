//! Console subsystem is intentional: PowerShell must wait and retain redirected output.
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!(
            "Potato Recorder diagnostics\n--diagnose\n--sources\n--self-test OUTPUT_DIR [--audio] [--mic] [--rnnoise] [--pause] [--dxgi] [--rate 60000/1001] [--resolution 1280x720] [--container mkv|mp4|fmp4] [--seconds N] [--hwnd HWND]\n--replay-test OUTPUT_DIR\n--recover session.json\n--remux recording.mkv"
        );
        return;
    }
    if let Err(e) = potato_recorder::engine::cli(&args) {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
