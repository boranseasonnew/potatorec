# Reproducible performance protocol

No comparative benchmark numbers are currently available. A successful encoder probe or desktop clip is not a game-performance benchmark.

## Control the experiment

Use the target Ryzen 7 5700U / Vega 8 / 16 GB single-channel machine, on the same power source, power plan, driver, game build, display refresh rate and background workload. Log them with each run. Warm up the game and shaders before measurement. Use a repeatable replay, saved scene or scripted camera route.

For every scene run: no recorder baseline, Xbox Game Bar, OBS, Potato. Rotate recorder order across at least three repetitions. Use the same resolution/rate and comparable **observed output quality**, rather than equal bitrate alone. Disable preview in OBS and Potato for the controlled comparison; record other OBS sources/filters so their costs are visible.

Scenarios: Minecraft without shaders, Minecraft with shaders, Brawlhalla, borderless window capture, Alt+Tab, desktop+microphone, 720p60, 1080p60, 1080p120 only if supported, and 30–120 minute stability tests. Roblox/Genshin can be added with the same controls.

## Capture measurements

Use an independently installed, official [PresentMon](https://github.com/GameTechDev/PresentMon) release to export **only the game's PID** frame-time CSV. Follow that release's CLI help; do not confuse recorder UI presents with game frames. Record at least 120 seconds after warm-up. Export GPU/CPU utilization and memory with Windows tools when available. The app shows separate CPU/working-set values for the GUI and FFmpeg; both contribute to total overhead.

```powershell
python scripts\analyze_frametimes.py game.csv --column MsBetweenPresents
```

The analyzer uses the arithmetic frame-time mean for average FPS, the mean of the slowest 1% frame times for a clearly defined 1% low, P95/P99 frame times, and spikes over 2× median. Choose the column explicitly when the PresentMon schema differs.

Report recorder drop counts separately from frame-time percentiles. Potato's current frame-drop counter is the encoder report; its WGC callback drop count is unavailable. Do not report zero capture drops just because that counter is absent.

## Synchronization and output

1. Record a visible flash/click or clap at the start, after Alt+Tab, immediately after pause/resume and near the end.
2. In an editor measure the video-frame-to-waveform offset on each **isolated** track. Save measurements with sign convention (positive = audio late), rate, device, driver, RNNoise state and recording length.
3. Check offset drift, not only initial alignment. Startup alignment in this implementation is estimated, so mark pass/fail against an explicitly chosen tolerance such as ±50 ms.
4. Decode every output with `ffmpeg -v error -i recording.mkv -map 0 -f null -`; inspect stream count with `ffprobe -show_streams`.
5. Open in at least VLC/Windows playback and the actual editing application. Confirm selected isolated tracks contain different sources and that pause boundaries do not introduce gaps, overlaps or audible clicks.
6. For subjective quality compare moving foliage, Minecraft shader effects, fine HUD text and dark gradients, not only a static desktop.

## Failure tests

- Minimize/restore and resize the captured window; document OS limitations.
- Close the source mid-recording; inspect final file and logs.
- Disconnect the microphone/output endpoint; verify explicit failure and recoverable originals.
- Test a deliberately unavailable encoder and invalid output path.
- Test stop/exit during recording, pause and finalization.
- Simulate an abnormal process termination only with disposable recordings. Recover from the session manifest; never experiment on irreplaceable media.
- Watch RAM/GPU memory and output growth during long recordings. Include PCM and intermediate files in disk consumption.

Retain original CSVs, configuration, FFmpeg version/build flags, capture logs, test lengths, game settings and output files. A “better than OBS” conclusion needs these controlled results.
