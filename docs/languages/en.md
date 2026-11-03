# Potato Recorder — English

**User guide**

A small native Windows studio for gameplay recording.

## Getting started

Install FFmpeg 8.1 or newer and the matching ffprobe. Put them on PATH or beside the application. The app does not download tools or upload recordings.

Open Settings to select your language. In Recording, select a display or game window. Windows Graphics Capture supports windows and displays; DXGI supports displays.

## Video and performance

Start with 1280 × 720 at 60 FPS on integrated graphics, then measure game frame times. The default remains source resolution. This is a recommendation, not a performance guarantee.

Auto tests real hardware encoders. Quality changes quantization; speed changes encoder effort. Exact fractional frame rates are supported up to 120 FPS. Actual source delivery may be lower.

The default compatibility transfer downloads frames to RAM. It is not zero-copy. Extra workers can increase contention. Profiles never change game settings or silently lower resolution or FPS.

## Audio

The microphone is off by default. Enable it explicitly in Audio and choose its device. Desktop sound captures the selected output endpoint, not only one game. Use headphones.

Enable Mixed Audio and Independent Track routing as needed. The resulting file contains real separate tracks. RNNoise, gate, expander, compressor, limiter and gain can be enabled and reordered. Keyboard noise may remain.

Use the eight-second microphone test and play the saved test to judge clarity. Live headphone monitoring and process-specific game audio are not implemented.

## Recording controls

F9 starts or stops recording. F10 pauses or resumes. F11 mutes the microphone. F7 shows or hides the application. F8 saves the explicitly started video-only replay buffer. Conflicting shortcuts can be changed in Hotkeys.

Drag the custom title bar to move the window. Double-click it to maximize. Closing during a recording stops and finalizes it safely. The source illustration is not a live preview.

## Files and recovery

MKV is the default. MP4 and fragmented MP4 are finalized after stopping. Audio is captured into PCM sidecars and added after stop. Long sessions can take time to finalize.

Each enabled stereo audio source uses about 1.38 GB per hour in temporary PCM, in addition to video and intermediate copies. Recovery folders keep originals. Check the completed recording before manually removing its session folder.

Use Recordings to open files, remux MKV to MP4 without video re-encoding, or recover unfinished sessions. Original recordings are not automatically deleted.

## Limits and verification

Replay currently saves video only. HDR, adaptive optimization, process audio isolation, live monitoring and automatic GPU-reset recovery are unfinished. Startup audio/video alignment is estimated; check lip sync before important recordings.

Protected content, some exclusive fullscreen games and minimized windows may stop delivering frames. Alt+Tab keeps the session open, but cannot remove operating-system restrictions.

No performance advantage over OBS or Xbox Game Bar has been established. See the benchmark protocol and implementation status for reproducible tests and remaining work.

## Translation note

This language pack was machine-assisted and has not yet received a complete native-speaker review. Technical Windows and FFmpeg diagnostics retain their original wording.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
