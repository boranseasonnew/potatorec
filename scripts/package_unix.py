"""Build real Linux .deb/tar.gz or a universal macOS .app/.dmg on the native host."""
import argparse
import hashlib
import json
import os
import plistlib
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
parser = argparse.ArgumentParser()
parser.add_argument("platform", choices=["linux", "macos"])
args = parser.parse_args()
os.chdir(ROOT)
meta = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"]))
version = next(p["version"] for p in meta["packages"] if p["name"] == "potato-recorder")
dist = ROOT / "dist"
dist.mkdir(exist_ok=True)

def docs(destination):
    destination.mkdir(parents=True, exist_ok=True)
    for file in ("README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"):
        shutil.copy2(ROOT / file, destination)
    shutil.copytree(ROOT / "docs", destination / "docs", dirs_exist_ok=True)
    subprocess.run(["python3", "scripts/collect_licenses.py", str(destination / "third-party-licenses")], check=True)

with tempfile.TemporaryDirectory(prefix="potatorec-package-") as temporary:
    stage = Path(temporary)
    if args.platform == "linux":
        package = stage / "deb"
        binary_dir = package / "usr/lib/potatorec"
        binary_dir.mkdir(parents=True)
        for name in ("potato-recorder", "potato-recorder-cli"):
            shutil.copy2(ROOT / "target/release" / name, binary_dir / name)
            (binary_dir / name).chmod(0o755)
        bindir = package / "usr/bin"
        bindir.mkdir(parents=True)
        (bindir / "potatorec").symlink_to("../lib/potatorec/potato-recorder")
        (bindir / "potatorec-cli").symlink_to("../lib/potatorec/potato-recorder-cli")
        desktop = package / "usr/share/applications"
        desktop.mkdir(parents=True)
        (desktop / "potatorec.desktop").write_text("[Desktop Entry]\nType=Application\nName=Potato Recorder\nComment=Lightweight desktop recorder (X11 preview)\nExec=potatorec\nIcon=potatorec\nTerminal=false\nCategories=AudioVideo;Recorder;\n", encoding="utf-8")
        icons = package / "usr/share/icons/hicolor/scalable/apps"
        icons.mkdir(parents=True)
        shutil.copy2(ROOT / "assets/potatorec.svg", icons / "potatorec.svg")
        docs(package / "usr/share/doc/potatorec")
        control = package / "DEBIAN"
        control.mkdir()
        (control / "control").write_text(f"Package: potatorec\nVersion: {version}\nArchitecture: amd64\nMaintainer: Potato Recorder contributors\nSection: video\nPriority: optional\nDepends: libc6 (>= 2.39), libgcc-s1, ffmpeg (>= 5.1), libx11-6, libx11-xcb1, libxcb1, libxkbcommon0, libxcursor1, libxi6, libxrandr2, libxinerama1, libwayland-client0, libwayland-cursor0, libwayland-egl1, libgl1, x11-xserver-utils, x11-utils, wmctrl, pulseaudio-utils, xdg-utils\nDescription: Native Rust desktop recorder, experimental Linux X11 build\n Captures X11 displays/windows and PulseAudio/PipeWire-Pulse audio.\n Wayland capture, global shortcuts and tray are not implemented.\n", encoding="utf-8")
        subprocess.run(["dpkg-deb", "--root-owner-group", "--build", str(package), str(dist / f"potatorec-{version}-linux-amd64.deb")], check=True)
        portable = stage / f"potatorec-{version}-linux-x86_64"
        portable.mkdir()
        for name in ("potato-recorder", "potato-recorder-cli"):
            shutil.copy2(binary_dir / name, portable / name)
        docs(portable)
        with tarfile.open(dist / f"potatorec-{version}-linux-x86_64.tar.gz", "w:gz") as archive:
            archive.add(portable, arcname=portable.name)
    else:
        volume = stage / "volume"
        app = volume / "Potato Recorder.app"
        macos = app / "Contents/MacOS"
        resources = app / "Contents/Resources"
        macos.mkdir(parents=True)
        for name in ("potato-recorder", "potato-recorder-cli"):
            subprocess.run(["lipo", "-create", str(ROOT / "target/x86_64-apple-darwin/release" / name), str(ROOT / "target/aarch64-apple-darwin/release" / name), "-output", str(macos / name)], check=True)
            subprocess.run(["lipo", str(macos / name), "-verify_arch", "x86_64", "arm64"], check=True)
            (macos / name).chmod(0o755)
        info = {"CFBundleName": "Potato Recorder", "CFBundleDisplayName": "Potato Recorder", "CFBundleIdentifier": "com.boranseason.potatorec", "CFBundleExecutable": "potato-recorder", "CFBundlePackageType": "APPL", "CFBundleShortVersionString": version, "CFBundleVersion": version, "LSMinimumSystemVersion": "12.0", "NSHighResolutionCapable": True, "NSMicrophoneUsageDescription": "Record microphone commentary only when you enable recording or a microphone test.", "NSScreenCaptureUsageDescription": "Record the display you explicitly select."}
        with (app / "Contents/Info.plist").open("wb") as handle:
            plistlib.dump(info, handle)
        docs(resources)
        (volume / "Applications").symlink_to("/Applications")
        shutil.copy2(ROOT / "docs/CROSS_PLATFORM.md", volume / "Read Me - Experimental Builds.md")
        for name in ("potato-recorder", "potato-recorder-cli"):
            subprocess.run(["codesign", "--force", "--sign", "-", str(macos / name)], check=True)
        subprocess.run([str(macos / "potato-recorder-cli"), "--diagnose"], check=True)
        subprocess.run(["codesign", "--force", "--deep", "--sign", "-", str(app)], check=True)
        subprocess.run(["codesign", "--verify", "--deep", "--strict", str(app)], check=True)
        subprocess.run(["hdiutil", "create", "-volname", "Potato Recorder", "-srcfolder", str(volume), "-ov", "-format", "UDZO", str(dist / f"potatorec-{version}-macos-universal.dmg")], check=True)
        subprocess.run(["ditto", "-c", "-k", "--sequesterRsrc", "--keepParent", str(app), str(dist / f"potatorec-{version}-macos-universal.zip")], check=True)

for file in dist.iterdir():
    if file.is_file() and version in file.name and file.suffix in (".dmg", ".zip", ".gz", ".deb"):
        (dist / (file.name + ".sha256")).write_text(hashlib.sha256(file.read_bytes()).hexdigest() + "  " + file.name + "\n", encoding="ascii")

