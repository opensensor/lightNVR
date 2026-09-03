#!/usr/bin/env bash
set -euo pipefail

repo_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
fixture_root="$repo_root/tests/fixtures/system_health/replays"
benchmark=${SYSTEM_HEALTH_BENCHMARK:-"$repo_root/build/swarm-t24/bin/test_system_health_benchmark"}

usage() {
    printf '%s\n' \
        "usage: $0 --all [--benchmark PATH]" \
        "       $0 --fixture FILE [--benchmark PATH]" \
        "       $0 --verify-fixtures" \
        "       $0 --verify-field-report FILE"
}

verify_fixtures() {
    python3 - "$fixture_root" <<'PY'
import csv
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
files = sorted(root.rglob("*.csv"))
if not files:
    raise SystemExit("no system-health replay fixtures")
header = ["elapsed_ms", "metric", "resource", "scope", "capability", "value",
          "unit", "service_degraded", "recording_expected"]
required_tokens = {
    "cpu", "oom", "swap", "filesystem", "inode", "read-only", "eio",
    "permission", "link", "reset", "clock", "smart", "recovery", "material",
    "sensor",
}
corpus = ""
for path in files:
    lines = path.read_text(encoding="utf-8").splitlines()
    corpus += "\n" + path.name.lower() + "\n" + "\n".join(lines).lower()
    expectations = [line for line in lines if line.startswith("# expect=")]
    if len(expectations) != 1:
        raise SystemExit(f"{path}: expected exactly one # expect= directive")
    data = [line for line in lines if line and not line.startswith("#")]
    rows = list(csv.reader(data))
    if not rows or rows[0] != header or len(rows) < 2:
        raise SystemExit(f"{path}: invalid replay schema or empty fixture")
    prior = -1
    for number, row in enumerate(rows[1:], 2):
        if len(row) != len(header):
            raise SystemExit(f"{path}:{number}: expected {len(header)} columns")
        elapsed = int(row[0])
        if elapsed < prior:
            raise SystemExit(f"{path}:{number}: elapsed_ms is not monotonic")
        prior = elapsed
        if row[3] not in {"process", "container", "host", "filesystem", "device"}:
            raise SystemExit(f"{path}:{number}: invalid scope")
        if row[4] not in {"available", "unsupported", "permission_denied", "stale", "error"}:
            raise SystemExit(f"{path}:{number}: invalid capability")
        float(row[5])
missing = sorted(token for token in required_tokens if token not in corpus)
if missing:
    raise SystemExit("fixtures do not describe: " + ", ".join(missing))
print(f"fixtures: {len(files)} deterministic replay files verified")
PY
}

verify_field_report() {
    local report=$1
    python3 - "$report" <<'PY'
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
folded = text.casefold()
required = [
    "256 mb", "arm", "x86", "docker", "cgroup", "rotating", "ssd", "nvme",
    "network mount", "kernel", "provider", "version", "observation duration",
    "false positive", "detection time", "recommendation", "capability",
]
missing = [item for item in required if item not in folded]
if missing:
    raise SystemExit("field report missing required coverage: " + ", ".join(missing))
privacy = {
    "IPv4 address": r"(?<![\d.])(?:\d{1,3}\.){3}\d{1,3}(?![\d.])",
    "credential URL": r"[a-z][a-z0-9+.-]*://[^\s/:]+:[^\s/@]+@",
    "device serial": r"\b(?:serial(?:_number)?|wwn)\s*[:=]\s*[^\s|]+",
    "raw device path": r"/(?:dev|home|root|mnt|media)/[^\s|]+",
    "raw SMART payload": r"smartctl\s+-[a-z]",
}
for label, pattern in privacy.items():
    if re.search(pattern, text, re.IGNORECASE):
        raise SystemExit(f"field report contains privacy-unsafe {label}")
print(f"field-report: schema and privacy verified: {path}")
PY
}

need_benchmark() {
    if [[ ! -x "$benchmark" ]]; then
        printf 'benchmark not executable: %s\n' "$benchmark" >&2
        printf 'set SYSTEM_HEALTH_BENCHMARK or pass --benchmark after the T24 build\n' >&2
        exit 2
    fi
}

run_docker_cgroup() {
    if [[ ${T24_SKIP_DOCKER:-0} == 1 ]]; then
        printf '%s\n' "docker-cgroup: skipped by T24_SKIP_DOCKER=1"
        return
    fi
    command -v docker >/dev/null || { printf '%s\n' "docker is required" >&2; return 1; }
    docker info >/dev/null
    docker run --rm --memory 128m --pids-limit 64 \
        -v "$benchmark:/t24-benchmark:ro" \
        -v /lib:/lib:ro -v /lib64:/lib64:ro -v /usr/lib:/usr/lib:ro \
        -v /usr/local/lib:/usr/local/lib:ro \
        --entrypoint /t24-benchmark \
        "${T24_DOCKER_IMAGE:-debian:trixie-slim}" --docker-cgroup
}

mode=
fixture=
field_report=
while (($#)); do
    case $1 in
        --all|--verify-fixtures) mode=$1; shift ;;
        --fixture) [[ $# -ge 2 ]] || { usage; exit 2; }; mode=$1; fixture=$2; shift 2 ;;
        --verify-field-report) [[ $# -ge 2 ]] || { usage; exit 2; }; mode=$1; field_report=$2; shift 2 ;;
        --benchmark) [[ $# -ge 2 ]] || { usage; exit 2; }; benchmark=$2; shift 2 ;;
        *) usage; exit 2 ;;
    esac
done

case $mode in
    --verify-fixtures)
        verify_fixtures
        ;;
    --verify-field-report)
        verify_field_report "$field_report"
        ;;
    --fixture)
        need_benchmark
        "$benchmark" --replay "$fixture"
        ;;
    --all)
        verify_fixtures
        need_benchmark
        while IFS= read -r -d '' fixture; do
            "$benchmark" --replay "$fixture"
        done < <(find "$fixture_root" -type f -name '*.csv' -print0 | sort -z)
        "$benchmark" --restart-classification
        "$benchmark" --stress
        "$benchmark" --sqlite-full
        run_docker_cgroup
        ;;
    *)
        usage
        exit 2
        ;;
esac
