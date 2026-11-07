#!/usr/bin/env bash
# Run a command inside the pinned lightnvr-rs transpile image (same bytes as CI).
#
#   tools/rust-transpile/docker.sh                       # regenerate rust/
#   tools/rust-transpile/docker.sh generate --check      # verify rust/ is current
#   tools/rust-transpile/docker.sh build                 # regenerate, build, smoke test
#   tools/rust-transpile/docker.sh bash                  # shell in the image
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
pin() { python3 -c "import json,sys;print(json.load(open('$here/toolchain.json'))[sys.argv[1]])" "$1"; }
image="${LIGHTNVR_RS_IMAGE:-lightnvr-rs-transpile:local}"

if [[ -z "${LIGHTNVR_RS_IMAGE:-}" ]]; then
  docker build -q -t "$image" -f "$here/Dockerfile" \
    --build-arg "C2RUST_REVISION=$(pin c2rust_revision)" \
    --build-arg "RUST_TOOLCHAIN=$(pin rust_toolchain)" \
    --build-arg "CLANG_MAJOR=$(pin clang_major)" \
    "$here" >/dev/null
fi

cmd=("${@:-generate}")
case "${cmd[0]}" in
  generate) cmd=(python3 tools/rust-transpile/generate.py --native "${cmd[@]:1}") ;;
  build) cmd=(bash -euo pipefail -c '
      python3 tools/rust-transpile/generate.py --native
      (cd rust && cargo build --release)
      python3 tools/rust-transpile/abi-audit.py
      node tools/rust-transpile/smoke.mjs rust/target/release/lightnvr-rs') ;;
esac

# Separate build dirs keep host and container CMake caches apart.
# /tmp on tmpfs: overlay storage trips LightNVR's storage placement checks.
exec docker run --rm ${LIGHTNVR_RS_TTY:+-it} --user "$(id -u):$(id -g)" --tmpfs /tmp:exec,size=4g,mode=1777 \
  -e HOME=/tmp -e CARGO_HOME=/tmp/cargo-home -e npm_config_cache=/tmp/npm-cache \
  -v "$repo:/work" -w /work "$image" "${cmd[@]}"
