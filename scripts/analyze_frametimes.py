"""Analyze an explicitly selected game frame-time column, without external packages."""
import argparse
import csv
import json
import math
import statistics

parser = argparse.ArgumentParser()
parser.add_argument("csv")
parser.add_argument("--column", default="MsBetweenPresents")
args = parser.parse_args()
with open(args.csv, encoding="utf-8-sig", newline="") as handle:
    reader = csv.DictReader(handle)
    if args.column not in (reader.fieldnames or []):
        parser.error(f"Missing column {args.column!r}. Available: {reader.fieldnames}")
    values = []
    for row in reader:
        try:
            value = float(row[args.column])
            if math.isfinite(value) and value > 0:
                values.append(value)
        except (ValueError, TypeError):
            pass
if len(values) < 100:
    parser.error("Need at least 100 positive finite game frame times")
ordered = sorted(values)

def percentile(fraction):
    position = (len(ordered) - 1) * fraction
    low = math.floor(position)
    high = math.ceil(position)
    return ordered[low] + (ordered[high] - ordered[low]) * (position - low)

slowest = ordered[-max(1, math.ceil(len(ordered) * 0.01)):]
median = statistics.median(values)
print(json.dumps({
    "frames": len(values),
    "duration_seconds": sum(values) / 1000,
    "average_fps": 1000 / statistics.mean(values),
    "one_percent_low_fps_slowest_mean": 1000 / statistics.mean(slowest),
    "median_ms": median,
    "p95_ms": percentile(0.95),
    "p99_ms": percentile(0.99),
    "maximum_ms": max(values),
    "spikes_over_twice_median": sum(v > 2 * median for v in values),
}, indent=2))
