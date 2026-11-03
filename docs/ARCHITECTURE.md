# Architecture

## Workspace

`crates/potato-core` contains serializable configuration, rational rates, encoder argument construction and sample-rate-aware dynamics. It has no Windows/UI dependency. `app` contains the native interface, Windows integration, WASAPI capture, session coordinator, replay ring and FFmpeg process boundary.

The eframe GUI uses OpenGL (`glow`) with no scene preview, Chromium or live background shaders. Icons and the static source illustration are vector primitives. The main window refreshes at most twice/second while recording with reduced updates; the idle meter refreshes once/second. OS input can independently trigger repaint.

## Video

`Capture → WGC canvas scaling → optional GPU conversion or download → codec → MKV`

The external FFmpeg process owns capture, GPU textures, codec queues and video muxing. Rust never copies a video pixel buffer. This does **not** imply the complete pipeline is zero-copy: the default graph explicitly downloads BGRA frames and converts to NV12 in system RAM for driver compatibility. AMF's `async_depth` limits encoder in-flight work. WGC's internal frame pool and FFmpeg's scheduler are not advertised as application-owned frame buffer controls. DXGI disables duplicate-frame filling; no FPS filter duplicates frames to inflate delivered FPS.

WGC maintains a fixed output canvas during source resize, preserving aspect ratio. Minimized-window/source stalls are reported without immediately stopping the session. Window closure can produce EOF and finalize captured media. DXGI monitor indices follow DXGI's adapter/output ordering, which may differ from Windows monitor enumeration on multi-adapter systems; verify the chosen display before a long take.

## Audio

Each enabled endpoint has an event-driven WASAPI thread. The client requests float stereo 48 kHz shared mode with Windows audio engine conversion. Loopback selects a render endpoint. Input selects a capture endpoint. `AUDCLNT_BUFFERFLAGS_SILENT` may supply a null pointer; it is treated as silence without dereferencing it.

Packets carry QPC 100 ns timestamps and go through a bounded 32-packet channel. The capture thread never writes a file or runs RNNoise. A source writer aligns packet positions to the session's QPC epoch, inserts silence for gaps and trims overlaps. If the queue is full, the packet is dropped and the count increases; subsequent QPC positions preserve the gap rather than speeding up the audio timeline. Clock jumps over ten seconds abort with recoverable artifacts.

Mic packets pass through ordered, stateful channel filters in an application-owned Rayon pool. Extra workers parallelize independent channels/sources, not hardware encoder internals. RNNoise is allocated only if suppression is enabled. It processes 480-sample frames at 48 kHz with the documented 16-bit float amplitude scale, using a streaming adapter. Final mux trims the adapter/algorithm's nominal 960-sample delay; the exact perceptual result still needs validation. Dynamics state is separate per filter/channel.

## Session and synchronization

Each session has a unique `.potato-session-*` directory, an atomic JSON manifest, per-segment video MKV, FFmpeg log and raw PCM sources. Frame PTS retains source spacing. WGC normalizes its first QPC timestamp internally, so the external process uses a one-time wall-clock first-frame-arrival alignment to the audio session epoch. This startup offset is an **estimate** and cannot establish sensor-to-file latency or guaranteed lip sync. Native source PTS would need an embedded capture API to remove that ambiguity.

Stopping sends `q` to FFmpeg, drains audio, joins threads, encodes audio to AAC and muxes true independent tracks. A clip's PCM tracks are padded/trimmed to video duration. An optional mixed track sums only routed sources and limits the result; isolated tracks remain separate. Pause closes a segment and stops devices; resume opens a new one with the same settings. Completed segments concatenate without video re-encoding.

MKV is written live; raw PCM is recoverable without a final header. Final MP4/fMP4 is generated only after stop. Disk overhead and finalization latency are conscious limitations of this first implementation. All source artifacts remain until the user removes them. Recovery never replaces an existing final recording.

## Dependency selection

[windows-capture](https://github.com/NiiightmareXD/windows-capture) was evaluated from its published README/API and MIT license. Its 2.0.1 API exposes WGC callbacks, DXGI and a hardware-assisted Media Foundation `VideoEncoder`; the example sends frames from the capture callback. It is a credible candidate for a future embedded engine. No conclusion about actual benchmark superiority is inferred from the repository's description. It was not added here because this implementation reuses FFmpeg's existing WGC/DXGI, codec and muxer coverage, while implementing native multi-source WASAPI/DSP independently. That choice trades some control over GPU queues and first-frame timing for a smaller initial integration surface.

Relevant primary references: [FFmpeg filters](https://ffmpeg.org/ffmpeg-filters.html), [FFmpeg codecs](https://ffmpeg.org/ffmpeg-codecs.html), [WASAPI loopback](https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording), [IAudioCaptureClient::GetBuffer](https://learn.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudiocaptureclient-getbuffer), [nnnoiseless](https://github.com/jneem/nnnoiseless), [egui](https://github.com/emilk/egui).
