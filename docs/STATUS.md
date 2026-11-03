# Implementation and verification status

This is a functional first release, not a claim that the complete original product specification has been accepted.

| Area | Implemented | Verification / remaining work |
|---|---|---|
| Rust workspace, egui, configuration | Native app, validation, defaults, recovery backup for invalid configuration, atomic file replacement | Windows GNU debug/release build; automated tests. MSVC CI provided; not locally verified. |
| UI | Custom title bar, vector navigation, source illustration, record control, audio toggles, settings cards, dark/light theme, Turkish/English primary controls | Actual window visually inspected. 23 complete-key catalogs and 23 localized usage guides; machine-assisted translations need native review. OS/codec diagnostics remain original. Accessibility tree support is limited. |
| Capture | WGC HWND/display; DXGI display; cursor, scaling | Real WGC desktop and DXGI desktop recordings decoded successfully. Named games, Alt+Tab, source resize and long-duration stress require manual target-machine tests. Protected/minimized/exclusive sources have OS limitations. |
| Encoding | Auto, AMF H.264/HEVC, NVENC H.264/HEVC, QSV H.264/HEVC, hardware-only MF H.264, x264 | Real AMF H.264 verified. Other codecs are shown only after runtime encode probes. On this host HEVC probe timed out, NVIDIA/Intel were unavailable and MF negotiation failed. |
| GPU interoperability | Direct, D3D11 NV12, AMF NV12 and CPU-transfer paths | Direct and GPU conversion paths failed on the development host; CPU transfer worked. No zero-copy or lower-than-OBS overhead claim. |
| Video controls | Exact rational rates 1–120 FPS, resolution, quality/speed mapping, CQP/CBR/VBR, bitrate, keyframes, H.264 profiles, B-frames, scaling | Exact rational clock and argument tests; actual 60000/1001 DXGI recording. Some driver option combinations can still fail. Advanced controls do not fully enumerate every per-device capability. MF has no independent speed API. |
| Performance | Actual DSP/software workers, AMF async depth, Windows process priority, profiles, reduced UI updates, memory option, detailed logs | Parameter tests and Windows execution. High priority is explicitly warned against. No real-time priority. No adaptive optimization. |
| Metrics | App/engine CPU, working set, encoded frames/FPS, encoder-reported drops, video write rate, WASAPI error/drop counts | Measured counters. WGC capture FPS/drops, GPU activity, sensor/encode latency and precise source pacing are unavailable. |
| Audio | WASAPI desktop loopback, mic, extra input, device selection, 48 kHz Windows resampling, gain/mute/mono, mixed/isolated routing | Actual desktop+microphone+RNNoise+pause session produced four streams and decoded. Synthetic 440/880 Hz test verifies isolation by spectral energy, not only stream count. |
| DSP | RNNoise, gate, expander, compressor, limiter, gain, filter ordering/presets, explicit 8-second mic test and playback | Dynamics/streaming tests. Subjective voice quality, keyboard reduction and all Bluetooth devices require user listening tests. |
| Containers | MKV, MP4, fMP4, separate AAC audio tracks, WAV stems, stream-copy remux | Actual MKV and fMP4 output and WAV export verified; media decoded via FFmpeg. Playback in editing software and players still needs manual tests. |
| Pause | Close segment, stop audio, resume new segment, concatenate on stop | Actual two-segment WGC+audio recording finalized and decoded. Boundary timestamps for long takes remain a manual check. |
| Recovery | Keep MKV/PCM, manifest, explicit recovery UI/CLI; stop timeout and device/encoder error reporting | Recovery artifacts preserved. Not every GPU/device failure automatically restarts. Incomplete first MKV clusters can be unrecoverable. Interrupted mux artifacts are retained under unique names before retrying. |
| Hotkeys and tray | Windows global F-key registration, conflict reports, no repeat, native tray menu, safe exit | Implemented; restricted sandbox prevented hotkey registration in the first UI inspection. Immediate shutdown during native control startup is covered by a regression test; interactive hotkey verification is listed in the test report. |
| Replay | Explicit video-only compressed RAM ring, 15/30/60/120 seconds, memory cap, save shortcut, stream-copy MKV | Real capture→RAM→save test succeeded. Replay audio is not implemented. Keyframe/memory boundaries can shorten the clip. During saving, shared snapshot blocks plus newly captured blocks can transiently approach twice the selected ring limit. |

## Explicitly unfinished

- Application-specific/process loopback audio and independent game/system sources.
- Live headphone monitoring, push-to-talk/push-to-mute.
- Replay audio, adaptive performance mode, HDR/custom color range/space controls.
- Preview renderer, crash-safe live multi-track muxing, GPU reset/reconnect automation.
- Exact first-frame A/V clock handoff from the external FFmpeg capture process. Startup timing is estimated; perceptual and long-session synchronization is not certified.
- A directly embedded Media Foundation/AMF engine with a measured GPU-resident queue and lower capture overhead.
- PresentMon/OBS/Game Bar comparisons on the target games and hardware, and signed installer/distribution.

These are engineering work remaining, not supported features disguised as buttons. No benchmark values are invented.
