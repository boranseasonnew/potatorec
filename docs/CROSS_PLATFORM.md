# macOS and Linux preview builds

These are functional platform ports, not Windows binaries with renamed extensions. They do not yet have feature parity with the Windows build.

| Platform | Installer | Capture | Audio | Verification |
|---|---|---|---|---|
| Windows 11 x64 | Portable ZIP | WGC display/window, DXGI display | Native WASAPI system/microphone/additional | Real hardware recordings tested on development host |
| Linux x86_64 | Debian `.deb`, portable `.tar.gz` | X11 display/window via FFmpeg | PulseAudio or PipeWire-Pulse sources and monitor inputs | Native Linux CI, Xvfb desktop recording and output decode |
| macOS Intel + Apple Silicon | Universal `.dmg` and `.app` ZIP | AVFoundation displays only | AVFoundation microphone/additional inputs | Both architectures compiled; permission-dependent display/microphone recording still needs a real Mac acceptance test |

## Linux

The Linux assets target Ubuntu 24.04 or newer / compatible systems with glibc 2.39+. Install the Debian package using the system package manager, for example `sudo apt install ./potatorec-0.2.0-linux-amd64.deb`. It declares its FFmpeg and native runtime dependencies. Launch **Potato Recorder** from the application menu or run `potatorec`. The portable archive requires the same system libraries/tools. Terminal commands use `potatorec-cli` or the archive's `potato-recorder-cli`.

Use an Xorg/X11 session for recording. The UI can open on Wayland, but the PipeWire screen-capture portal is **not implemented** and XWayland is not presented as full desktop capture. Native X11 monitor geometry and window IDs are enumerated with `xrandr`, `xdpyinfo` and `wmctrl`. Some window managers may omit window metadata; display capture remains available. FFmpeg's X11 window capture can end when a window is unmapped or resized smaller; this differs from WGC.

Desktop audio selects a PulseAudio monitor source; PipeWire's PulseAudio compatibility service is supported. `pulseaudio-utils` supplies `pactl`. Physical microphone routing/quality must be tested on the actual user's devices.

## macOS

Open the DMG and drag **Potato Recorder.app** to Applications. The universal application contains both Intel and Apple Silicon code. Install an appropriate native FFmpeg and ffprobe separately, e.g. from Homebrew's `ffmpeg` formula. The application also searches `/opt/homebrew/bin` and `/usr/local/bin` when launched from Finder.

The preview is **ad-hoc signed, not Developer ID signed or notarized**. It is not presented as an Apple-trusted distribution. A Developer ID identity and notarization credentials are required for a polished public macOS distribution; none are included in the repository.

Screen and microphone permissions must be granted by the user in macOS. The app does not bypass privacy dialogs. Device discovery selects only AVFoundation entries explicitly named “Capture screen”; it does not substitute a camera for an unavailable screen. Saved display/audio indices are checked against current device names before capture. If hardware changes reorder the indices, refresh devices and select the intended source again; a stale screen index cannot silently open a camera. VideoToolbox probes disallow software fallback (`allow_sw=0`); when it fails the app offers the tested software encoder.

AVFoundation does not provide native system-output loopback here. Desktop Audio is disabled on this platform. A user-configured virtual loopback input can be selected as **Additional Input**. No audio driver or OS audio routing is installed/changed automatically.

## Shared limitations

Global shortcuts, tray integration, custom process priority, Windows GPU-transfer paths and Windows capture backends are unavailable in the macOS/Linux preview. Minimize uses the ordinary native window instead of hiding into a nonexistent tray. Unix capture/scaling uses system memory; no zero-copy or OBS performance advantage is claimed. Optional audio filters and track routing use the same real DSP/muxer implementation, but Unix startup A/V alignment uses first audio-buffer/video-frame arrival estimates and requires lip-sync testing.

FFmpeg is an external dependency and is not redistributed in these assets. Licenses/notices are included in each complete distribution. The first preview remains unsuitable for irreplaceable recordings without a short local capture/audio test.
