# Potato Recorder

A native Rust recording utility for Windows 11. Capture a display or supported game/application window, encode with a tested hardware encoder, and retain independently editable audio tracks. No Electron, browser runtime, advertisements or telemetry.

**Status: functional 0.1 engineering release, not the finished specification.** See [implementation status](docs/STATUS.md) before relying on it for irreplaceable footage. Performance superiority over OBS or Xbox Game Bar has **not** been established.

## Run

Open `dist/Potato Recorder/potato-recorder.exe` after building the portable package. Provide your own **FFmpeg 8.1 or newer** and matching `ffprobe.exe`, either on PATH or beside the executable. In Settings, you can set an absolute path to FFmpeg. The build must include `gfxcapture` and/or `ddagrab`, an H.264 encoder, AAC and the required muxers. The development machine already has FFmpeg installed.

Select a source in Recording, configure Video and Audio, then record. The microphone is **off by default** and is opened only by an explicit recording/test action. Desktop audio defaults to the default render endpoint. Use headphones to prevent acoustic feedback in microphone recordings.

Default shortcuts: F9 record/stop, F10 pause/resume, F11 mute microphone, F7 show/hide, F8 save the **explicitly started, video-only** replay buffer. Windows conflicts are reported; function keys F1–F24 can be assigned. Tray commands provide record/stop, pause, mute, show, open folder and safe exit.

The app has a custom native title bar, grouped navigation, dark/light themes, 23 embedded language packs, vector icons and a recording dashboard. Source art is a static illustration, **not a fake live preview**. Preview rendering remains disabled. Windows Segoe UI fonts are read from the local Windows installation and are not redistributed.

## Build

Recommended: current stable Rust with the MSVC target and Visual Studio C++ Build Tools/Windows SDK.

```powershell
cargo build --release --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

This workspace also supports a project-local GNU toolchain. The development environment's `.tools` folder is **not** part of the source/distribution. `scripts/cargo.ps1` discovers that local toolchain when present; otherwise it uses Cargo from PATH. A complete GNU toolchain needs `dlltool` and `as`, not only the Rust linker. `.cargo/config.toml` selects Rust's self-contained linker for the GNU target only; MSVC is unaffected.

```powershell
.\scripts\cargo.ps1 build --release --locked
.\scripts\package.ps1
```

FFmpeg is an external executable, invoked using argument arrays without a shell. It is deliberately not downloaded or bundled by the application. The package includes source/dependency license notices, not an arbitrary third-party FFmpeg binary.

## Recording pipeline

- FFmpeg WGC (`gfxcapture`) for a selected HWND or display; DXGI (`ddagrab`) for displays.
- Hardware codecs are probed by actually encoding generated color frames. Unsupported codecs are explained in Compatibility and excluded from the selector. QSV initialization explicitly requests hardware; Media Foundation uses `hw_encoding=1`.
- WGC can scale its output canvas on the GPU. Driver-compatible D3D11/AMF transfer paths are selectable; the default **GPU→RAM compatibility transfer** was the working path on the development machine. This is not a zero-copy claim.
- Native event-driven WASAPI loopback/input capture at 48 kHz float stereo, bounded packet queues, QPC timestamps, gap filling, independent DSP and disk writer threads.
- Optional local RNNoise (`nnnoiseless`), gate, expander, compressor, limiter and gain; reorderable microphone filters. A bounded shared Rayon pool handles per-channel DSP.
- Live video writes to MKV. Audio writes recoverable raw PCM sidecars. Stop joins audio into Mixed/Desktop/Microphone/Additional tracks as configured, then finalizes MKV/MP4/fMP4. Pausing closes a segment; resuming opens another; final video is concatenated without re-encoding.

**Disk and stop-time tradeoff:** finalization encodes audio after capture, and recovery sources are retained. Each enabled float stereo source uses about **1.38 GB/hour**, in addition to video and intermediate copies. Keep substantial free space. Stop may take time for long sessions. Finalization progress currently shows a state rather than a percentage. Do not remove `.potato-session-*` folders until you have checked the completed recording.

Audio packet clocks are real QPC measurements; the video/audio **startup alignment is estimated at first video-frame arrival**, preserving subsequent source timestamp spacing. End-to-end lip sync and long-session drift must still be validated on the target machine. WGC/DXGI capture callback counters, GPU utilization, and per-frame encode latency are unavailable rather than fabricated.

## Diagnostics

```powershell
potato-recorder-cli.exe --diagnose
potato-recorder-cli.exe --sources
potato-recorder-cli.exe --self-test C:\Recordings\Smoke --audio --mic --rnnoise --pause
potato-recorder-cli.exe --self-test C:\Recordings\DXGI --dxgi --rate 60000/1001 --resolution 1280x720 --container fmp4 --audio --stems
potato-recorder-cli.exe --self-test C:\Recordings\Window --hwnd 123456 --seconds 10
potato-recorder-cli.exe --replay-test C:\Recordings\Replay
potato-recorder-cli.exe --recover C:\Recordings\.potato-session-123\session.json
potato-recorder-cli.exe --remux C:\Recordings\clip.mkv
```

The self-test records the real desktop/window; `--audio`/`--mic` explicitly opt into actual device capture. It does not replace video with test patterns. Generated signals appear only in codec probes and clearly identified integration-test fixtures. `--seconds` accepts 1–7200. Use the separate `potato-recorder-cli.exe` console binary for terminal commands and redirected output. The GUI executable uses the Windows subsystem.

## Documentation

- [Türkçe kullanım kılavuzu](docs/KULLANIM.md)
- [Architecture and reliability](docs/ARCHITECTURE.md)
- [Configuration](docs/CONFIGURATION.md)
- [Features and limitations](docs/STATUS.md)
- [Reproducible performance protocol](docs/BENCHMARKS.md)
- [Third-party notices](THIRD_PARTY_NOTICES.md)

Project source is MIT licensed. Third-party components retain their own licenses. Your selected FFmpeg build may be GPL; its redistribution is a separate obligation.

## Languages

Language selection follows Windows on first launch and persists across restarts. All 23 catalogs are embedded; no translation service is contacted by the app. Each language has its own usage guide. Translations are machine-assisted with selected manual corrections and still need native-speaker review. Raw Windows/FFmpeg diagnostics remain technical source data.

[English](docs/languages/en.md) · [Türkçe](docs/languages/tr.md) · [Azərbaycanca](docs/languages/az.md) · [Русский](docs/languages/ru.md) · [Deutsch](docs/languages/de.md) · [Français](docs/languages/fr.md) · [Español](docs/languages/es.md) · [Português (Brasil)](docs/languages/pt-BR.md) · [Italiano](docs/languages/it.md) · [Nederlands](docs/languages/nl.md) · [Polski](docs/languages/pl.md) · [Українська](docs/languages/uk.md) · [Română](docs/languages/ro.md) · [Magyar](docs/languages/hu.md) · [Čeština](docs/languages/cs.md) · [Slovenčina](docs/languages/sk.md) · [Български](docs/languages/bg.md) · [Српски](docs/languages/sr.md) · [Hrvatski](docs/languages/hr.md) · [Ελληνικά](docs/languages/el.md) · [Svenska](docs/languages/sv.md) · [Dansk](docs/languages/da.md) · [Suomi](docs/languages/fi.md)
