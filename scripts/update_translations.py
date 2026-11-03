"""Explicit maintainer-only translation update. The application never contacts this service.

Only public, generic UI/documentation strings are sent, never recordings, device IDs,
configuration, logs or user content. Existing translations are cached per locale.
"""
import argparse
import concurrent.futures
import json
import re
import subprocess
import time
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LANGUAGES = {
    "en": "English", "tr": "Türkçe", "az": "Azərbaycanca", "ru": "Русский",
    "de": "Deutsch", "fr": "Français", "es": "Español", "pt-BR": "Português (Brasil)",
    "it": "Italiano", "nl": "Nederlands", "pl": "Polski", "uk": "Українська",
    "ro": "Română", "hu": "Magyar", "cs": "Čeština", "sk": "Slovenčina",
    "bg": "Български", "sr": "Српски", "hr": "Hrvatski", "el": "Ελληνικά",
    "sv": "Svenska", "da": "Dansk", "fi": "Suomi",
}
GUIDE = [
    "User guide", "A small native Windows studio for gameplay recording.",
    "Getting started", "Install FFmpeg 8.1 or newer and the matching ffprobe. Put them on PATH or beside the application. The app does not download tools or upload recordings.",
    "Open Settings to select your language. In Recording, select a display or game window. Windows Graphics Capture supports windows and displays; DXGI supports displays.",
    "Video and performance", "Start with 1280 × 720 at 60 FPS on integrated graphics, then measure game frame times. The default remains source resolution. This is a recommendation, not a performance guarantee.",
    "Auto tests real hardware encoders. Quality changes quantization; speed changes encoder effort. Exact fractional frame rates are supported up to 120 FPS. Actual source delivery may be lower.",
    "The default compatibility transfer downloads frames to RAM. It is not zero-copy. Extra workers can increase contention. Profiles never change game settings or silently lower resolution or FPS.",
    "Audio", "The microphone is off by default. Enable it explicitly in Audio and choose its device. Desktop sound captures the selected output endpoint, not only one game. Use headphones.",
    "Enable Mixed Audio and Independent Track routing as needed. The resulting file contains real separate tracks. RNNoise, gate, expander, compressor, limiter and gain can be enabled and reordered. Keyboard noise may remain.",
    "Use the eight-second microphone test and play the saved test to judge clarity. Live headphone monitoring and process-specific game audio are not implemented.",
    "Recording controls", "F9 starts or stops recording. F10 pauses or resumes. F11 mutes the microphone. F7 shows or hides the application. F8 saves the explicitly started video-only replay buffer. Conflicting shortcuts can be changed in Hotkeys.",
    "Drag the custom title bar to move the window. Double-click it to maximize. Closing during a recording stops and finalizes it safely. The source illustration is not a live preview.",
    "Files and recovery", "MKV is the default. MP4 and fragmented MP4 are finalized after stopping. Audio is captured into PCM sidecars and added after stop. Long sessions can take time to finalize.",
    "Each enabled stereo audio source uses about 1.38 GB per hour in temporary PCM, in addition to video and intermediate copies. Recovery folders keep originals. Check the completed recording before manually removing its session folder.",
    "Use Recordings to open files, remux MKV to MP4 without video re-encoding, or recover unfinished sessions. Original recordings are not automatically deleted.",
    "Limits and verification", "Replay currently saves video only. HDR, adaptive optimization, process audio isolation, live monitoring and automatic GPU-reset recovery are unfinished. Startup audio/video alignment is estimated; check lip sync before important recordings.",
    "Protected content, some exclusive fullscreen games and minimized windows may stop delivering frames. Alt+Tab keeps the session open, but cannot remove operating-system restrictions.",
    "No performance advantage over OBS or Xbox Game Bar has been established. See the benchmark protocol and implementation status for reproducible tests and remaining work.",
    "Translation note", "This language pack was machine-assisted and has not yet received a complete native-speaker review. Technical Windows and FFmpeg diagnostics retain their original wording.",
]

def translate(text, language):
    query = urllib.parse.urlencode({"client": "gtx", "sl": "en", "tl": language, "dt": "t", "q": text})
    url = "https://translate.googleapis.com/translate_a/single?" + query
    for attempt in range(5):
        try:
            # Use the host's HTTP client/proxy stack; no URL is interpolated into shell code.
            result = subprocess.run(["pwsh", "-NoProfile", "-Command",
                "$ErrorActionPreference='Stop'; [Console]::OutputEncoding=[Text.UTF8Encoding]::new($false); "
                "$uri=[Console]::In.ReadToEnd(); [Console]::Out.Write((Invoke-WebRequest $uri -TimeoutSec 30).Content)"],
                input=url, text=True, encoding="utf-8", capture_output=True, timeout=40,
                creationflags=0x08000000, check=True)
            payload = json.loads(result.stdout)
            return "".join(part[0] for part in payload[0] if part[0])
        except Exception:
            if attempt == 4:
                raise
            time.sleep(min(2 ** attempt, 8))

def batches(values, limit=1700):
    batch = []
    length = 0
    for value in values:
        if length + len(value) > limit and batch:
            yield batch
            batch, length = [], 0
        batch.append(value)
        length += len(value) + 1
    if batch:
        yield batch

def update_locale(language, name, source):
    file = ROOT / "locales" / (language + ".json")
    cache = json.loads(file.read_text(encoding="utf-8")) if file.exists() else {}
    if language == "en":
        cache = {key: key for key in source}
    else:
        for batch in batches([key for key in source if key not in cache]):
            result = translate("\n".join(batch), language).splitlines()
            if len(result) != len(batch):
                result = [translate(key, language) for key in batch]
            for key, value in zip(batch, result):
                if not value.strip():
                    raise ValueError(f"Empty translation: {language}: {key}")
                cache[key] = value.strip()
            file.write_text(json.dumps(cache, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if language == "tr":
        seed = ROOT / ".tools" / "turkish-seed.json"
        if seed.exists():
            cache.update(json.loads(seed.read_text(encoding="utf-8")))
    cache = {key: cache[key] for key in source}
    file.write_text(json.dumps(cache, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    guide_file = ROOT / "docs" / "languages" / (language + ".md")
    guide_cache_file = ROOT / ".tools" / ("guide-" + language + ".json")
    if language == "en":
        guide = GUIDE
    elif guide_cache_file.exists():
        guide = json.loads(guide_cache_file.read_text(encoding="utf-8"))
    else:
        guide = []
        for batch in batches(GUIDE):
            result = translate("\n".join(batch), language).splitlines()
            if len(result) != len(batch):
                result = [translate(key, language) for key in batch]
            guide.extend(result)
        guide_cache_file.write_text(json.dumps(guide, ensure_ascii=False), encoding="utf-8")
    guide = [text.replace("F12", "F7") for text in guide]
    lines = [f"# Potato Recorder — {name}", "", f"**{guide[0]}**", "", guide[1], ""]
    headings = {2, 5, 9, 13, 16, 20, 24}
    for index, text in enumerate(guide[2:], start=2):
        lines.extend([("## " if index in headings else "") + text, ""])
    lines.extend(["[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)", ""])
    guide_file.write_text("\n".join(lines), encoding="utf-8")
    return f"{language}: {len(cache)} strings and guide"

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--machine-translate", action="store_true", required=True,
                        help="Explicitly send generic public UI/documentation strings for translation")
    args = parser.parse_args()
    source = json.loads((ROOT / "locales" / "en.json").read_text(encoding="utf-8"))
    with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
        futures = [executor.submit(update_locale, code, name, source) for code, name in LANGUAGES.items()]
        for future in concurrent.futures.as_completed(futures):
            print(future.result(), flush=True)
