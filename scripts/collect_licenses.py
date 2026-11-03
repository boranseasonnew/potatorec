"""Collect upstream notices for a distribution; no FFmpeg binary is redistributed."""
import argparse
import json
import shutil
import subprocess
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("destination", type=Path)
args = parser.parse_args()
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--format-version", "1"]))
args.destination.mkdir(parents=True, exist_ok=True)
for package in metadata["packages"]:
    if package["name"] in ("potato-core", "potato-recorder"):
        continue
    source = Path(package["manifest_path"]).parent
    destination = args.destination / f"{package['name']}-{package['version']}"
    for file in source.iterdir():
        if file.is_file() and file.name.upper().startswith(("LICENSE", "LICENCE", "COPYING", "NOTICE", "AUTHORS", "COPYRIGHT")):
            destination.mkdir(parents=True, exist_ok=True)
            shutil.copy2(file, destination / file.name)
        elif file.is_dir() and file.name.lower() == "licenses":
            shutil.copytree(file, destination / "licenses", dirs_exist_ok=True)

