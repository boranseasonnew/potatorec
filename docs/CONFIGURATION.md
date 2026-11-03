# Configuration

Default file: `%LOCALAPPDATA%\Potato Recorder\config.json`. Settings save after an 800 ms debounce and on safe exit. Invalid input is not persisted. Version 1 tolerates missing fields using current defaults, rejects unknown schema versions and preserves invalid configurations in a timestamped backup before creating defaults.

| Field | Default | Actual effect |
|---|---|---|
| `video.fps` | `{ "num": 60, "den": 1 }` | Exact rational WGC/DXGI maximum capture rate; 1–120 FPS. VFR output. |
| `video.width`, `height` | 0, 0 | Source dimensions; WGC rounds to even dimensions. Custom dimensions must both be even. |
| `video.encoder` | `Auto` | Tested hardware H.264 in AMF/NVENC/QSV/MF order, then software. |
| `video.transfer` | `CpuTransfer` | Explicit hardware-frame download; alternative GPU routes require driver support. |
| `video.quality` | `Medium` | QP/CQ/CRF 25 (presets: 35/30/25/21/17); MF quality equivalent. In bitrate modes, bitrate replaces quantization quality. |
| `video.speed` | `Balanced` | Real codec preset; AMF has three native choices, NVENC/QSV/x264 have finer mappings. MF has no independent speed mapping. |
| `video.rate_control` | `Cqp` | CQP/CQ/CRF; or explicit CBR/VBR bitrate configuration. |
| `performance.workers` | 0 | Auto DSP pool 1–3 workers, otherwise 1–5. Also software conversion/encoding thread options. |
| `performance.buffers` | 0 | AMF async depth auto 2, otherwise 1–5. Does not configure WGC's internal frame pool. |
| `performance.low_memory` | false | Forces AMF depth to 1; audio queues remain bounded. |
| `performance.priority` | `Normal` | Native process priority for app and video engine; never real-time. |
| `audio.system.enabled` | true | Selected render endpoint loopback. |
| `audio.mic.enabled` | false | Selected capture endpoint; explicit opt-in. |
| `audio.additional.enabled` | false | Extra capture endpoint. |
| Source `device_id` | empty | Resolve the Windows default endpoint when recording starts. |
| Source `mixed`, `isolated` | true, true | Actual track routing at final mux. |
| `audio.filters` | limiter only enabled | Ordered stateful mic chain; RNNoise off by default. |
| `container` | `Mkv` | Final container. Video capture itself always uses a recoverable MKV intermediate. |
| `filename_format` | `Potato_%Y-%m-%d_%H-%M-%S` | Chrono date tokens; path separators/reserved filename characters rejected; collisions get a unique suffix. |
| `replay_seconds` | 30 | 15/30/60/120, video-only compressed replay. |
| `replay_memory_mb` | 64 | 16–1024 MiB ring cap. Snapshot save can temporarily retain older blocks while the ring fills new blocks. |

Preset profiles change encoder speed, AMF depth, WGC scaling, memory mode and UI refresh. They never change requested FPS, resolution, game graphics or the game's FPS cap. The frame-transfer selection is a compatibility preference and is not silently switched to an unverified GPU path.

Development-only `POTATO_ALLOW_SELF_CAPTURE=1` skips the recorder window's capture exclusion so the UI can be visually inspected with WGC screenshots. It does not start recording. Normal launches request `WDA_EXCLUDEFROMCAPTURE` for the recorder's own window where Windows supports it.

## Localization

23 locale JSON files are embedded at compile time. Each contains the same keys as `en.json`; automated tests enforce non-empty translations, stable saved language enums, numbered template placeholders and a corresponding localized Markdown guide. `scripts/update_translations.py --machine-translate` is an explicit maintainer-only network operation for generic public strings; it is never run by the app, Cargo build or CI.

For visual QA, `potato-recorder.exe --ui-test ru Settings` selects a language/page, uses a separate settings file under `test-output`, and permits screenshots of the app window. It does not automatically begin recording.
