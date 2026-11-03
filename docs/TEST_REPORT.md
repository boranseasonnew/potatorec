# Verification report — 2026-11-03

The tests below were executed on the available Windows host. They do not establish game-frame-time performance or complete acceptance of the original specification.

## Automated checks

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed. The restricted filesystem emitted Cargo incremental hard-link/cache warnings, separate from code lints.
- `cargo test --workspace`: **21 tests passed**, including exact rational timestamps, safe settings, saved settings after reopening, schema preservation, F12 migration, dynamics, RNNoise streaming packet sizes, window-handle validation, native-control startup/shutdown, all 23 catalogs/guides and translation placeholders.
- A synthetic integration fixture encodes two independently generated 440/880 Hz audio sources, muxes mixed and isolated tracks and verifies each isolated track's spectral energy. It also checks that an interrupted mux artifact is retained before recovery.
- The Windows GNU optimized executable built successfully. Its import table contains Windows system DLLs; no separately distributed GNU runtime DLL was required on the host.

## Actual recording tests

| Path | Test | Result |
|---|---|---|
| WGC desktop → AMF H.264 → MKV | 640×360, requested 60000/1001 FPS, native desktop+microphone WASAPI, Clean Voice/RNNoise, pause/resume | Passed; final 8.472-second file contained H.264 video plus three independent 48 kHz AAC streams: Mixed Audio, Desktop, Microphone. Entire file decoded without reported FFmpeg errors. |
| DXGI desktop → AMF H.264 → fragmented MP4 | 1280×720, requested 60000/1001 FPS, desktop audio, WAV export | Passed; final MP4 and two WAV tracks were created and video/audio decoded successfully. |
| WGC → compressed RAM replay → MKV | Explicit six-second video-only replay test | Passed; buffered encoded data saved as MKV and decoded successfully. Duration is bounded by available data/keyframes. |
| Selected WGC HWND → AMF H.264 → MP4 | Real native test-application window, three-second capture requested via the console executable | Passed; PowerShell source enumeration and redirected CLI completion worked, MP4 was created and the complete file decoded without reported errors. |
| GPU-resident transfer alternatives | Direct D3D11 surface, D3D11 NV12 conversion, AMF conversion | Failed on this driver/host. The explicit GPU→RAM compatibility route worked. These failures are disclosed; no zero-copy claim. |

Requested rate is a maximum capture target, not proof that a source actually delivered that FPS. No artificial FPS filter was used to inflate the result.

## Interface and defects corrected

- Actual native dashboard, Video and Azerbaijani Settings pages visually inspected after redesign; the native-name language selector scrolls through the embedded catalog list.
- Cyrillic Russian text rendered with the installed Windows font. An ambiguous machine translation of “Common FPS” was corrected, along with selected Azerbaijani/Russian labels.
- Long translated profile names now truncate inside the sidebar rather than widening its layout.
- First-frame layout expansion that hid dashboard content was corrected.
- A startup/drop race in the tray/hotkey bridge was corrected and covered by a regression test.
- Closed/reused HWND sources are rejected before recording.
- Interrupted mux output is preserved under a unique name before retrying.
- Audio recovery file creation is checked before starting device capture.
- Added a separate console executable for reliable PowerShell command completion and redirected diagnostics; the GUI executable retains the Windows subsystem.
- The frame-time analyzer passed a 1000-frame synthetic mathematical fixture, which is not a game benchmark.
- The F12 default was invalid: [Microsoft reserves F12 for debuggers](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey). Show/hide now defaults to **F7**; the old default migrates without resetting other settings. F12 is excluded from shortcut choices.
- Exactly **23 embedded locale JSON catalogs**, each with **325 keys**, and exactly **23 Markdown usage guides** exist under `docs/languages`. Key parity, nonempty values, persistence and numbered placeholders are tested.

## Remaining manual acceptance

Named games, exclusive fullscreen, Alt+Tab/resize stress, editing-suite playback, long sessions, perceptual microphone quality and lip-sync/drift have not been certified. WGC callback FPS/drops, sensor/encode latency and GPU activity are not exposed by this implementation. Full Game Bar/OBS/PresentMon comparisons remain to be run. Translation packs are machine-assisted and still need native-speaker review. See `STATUS.md` for unfinished product features.

Test recordings, raw audio, diagnostic logs, local toolchains and user settings are excluded from the public Git repository.
