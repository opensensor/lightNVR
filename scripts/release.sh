#!/usr/bin/env bash
# Prepare, wait for Rust regeneration and CI, then publish one release tag.
set -euo pipefail
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$PROJECT_ROOT/scripts/release.py" "$@"
