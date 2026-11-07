#!/usr/bin/env bash
# Build lightnvr-rs_<version>_<suite>_<arch>.deb, mirroring the lightnvr .deb
# layout (see .github/workflows/debian-package.yml). Web UI, migrations, go2rtc
# and lightnvr-ffmpeg-guard come from a lightnvr runtime image; the backend is
# the C2Rust-generated lightnvr-rs binary.
#
#   package-deb.sh <runtime-image> <lightnvr-rs binary> <version> <suite> <arch> <out-dir>
set -euo pipefail
[[ $# -eq 6 ]] || { sed -n '2,9p' "$0"; exit 2; }
image="$1" binary="$2" version="$3" suite="$4" arch="$5" out="$6"
repo="$(cd "$(dirname "$0")/../.." && pwd)"
name="lightnvr-rs_${version}_${suite}_${arch}"
pkg="$(mktemp -d)/${name}"
trap 'rm -rf "$(dirname "$pkg")"; [[ -n "${cid:-}" ]] && docker rm -f "$cid" >/dev/null 2>&1 || true' EXIT

mkdir -p "$pkg"/{DEBIAN,usr/bin,etc/lightnvr/go2rtc,var/lib/lightnvr/www,var/lib/lightnvr/data} \
         "$pkg"/{var/log/lightnvr,var/run/lightnvr,usr/share/lightnvr/migrations,lib/systemd/system}

install -m 0755 "$binary" "$pkg/usr/bin/lightnvr-rs"
cid="$(docker create "$image")"
docker cp "$cid:/bin/go2rtc" "$pkg/usr/bin/go2rtc"
docker cp "$cid:/bin/lightnvr-ffmpeg-guard" "$pkg/usr/bin/lightnvr-ffmpeg-guard"
docker cp "$cid:/var/lib/lightnvr/www/." "$pkg/var/lib/lightnvr/www/"
docker cp "$cid:/usr/share/lightnvr/migrations/." "$pkg/usr/share/lightnvr/migrations/"
chmod 0755 "$pkg"/usr/bin/*
install -m 0644 "$repo/config/lightnvr.ini" "$pkg/etc/lightnvr/lightnvr.ini"

cat > "$pkg/etc/lightnvr/go2rtc/go2rtc.yaml" <<'EOF'
# go2rtc configuration file
# See https://github.com/AlexxIT/go2rtc for documentation

api:
  listen: :1984
  origin: "*"

webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]

log:
  level: info

streams:
  # Streams will be added dynamically by LightNVR
EOF

cat > "$pkg/lib/systemd/system/lightnvr-rs.service" <<'EOF'
[Unit]
Description=LightNVR (lightnvr-rs) - Lightweight Network Video Recorder
After=network.target network-online.target
Wants=network-online.target

[Service]
Type=forking
PIDFile=/var/run/lightnvr.pid
User=root
Group=root
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
Environment="HOME=/root"
ExecStart=/usr/bin/lightnvr-rs -c /etc/lightnvr/lightnvr.ini -d
ExecReload=/bin/kill -HUP $MAINPID
KillMode=mixed
KillSignal=SIGTERM
TimeoutStartSec=30
TimeoutStopSec=30
Restart=on-failure
RestartSec=5
ExecStartPre=/bin/mkdir -p /var/lib/lightnvr/data /var/log/lightnvr /var/run/lightnvr
StandardOutput=journal
StandardError=journal
SyslogIdentifier=lightnvr-rs

[Install]
WantedBy=multi-user.target
EOF

# Depend on exactly the sonames the binary links (Debian names them <lib><soname>).
needed="$(readelf -d "$binary" | sed -n 's/.*Shared library: \[\(.*\)\]$/\1/p')"
soname_pkg() { printf '%s\n' "$needed" | sed -n "s/^$1\.so\.\([0-9.]*\)\$/$2\1/p" | head -n1; }
deps=(ffmpeg)
for lib in libavcodec libavformat libavutil libswscale libswresample libavfilter; do
  p="$(soname_pkg "$lib" "$lib")"
  [[ -n "$p" ]] || { echo "ERROR: lightnvr-rs does not link $lib" >&2; exit 1; }
  [[ "$lib" == libavcodec ]] && p="$p | libavcodec-extra"
  deps+=("$p")
done
for pair in libmbedtls:libmbedtls libmbedx509:libmbedx509- libmbedcrypto:libmbedcrypto; do
  p="$(soname_pkg "${pair%%:*}" "${pair#*:}")"
  [[ -n "$p" ]] || { echo "ERROR: lightnvr-rs does not link ${pair%%:*}" >&2; exit 1; }
  deps+=("$p")
done
deps+=("libuv1t64" "libsqlite3-0" "libcurl4t64 | libcurl4" "libmosquitto1" "libcjson1" "libatomic1" "zlib1g" "procps" "tzdata")

cat > "$pkg/DEBIAN/control" <<EOF
Package: lightnvr-rs
Version: ${version}
Architecture: ${arch}
Maintainer: OpenSensor <support@opensensor.io>
Installed-Size: $(du -sk "$pkg" | cut -f1)
Depends: $(IFS=,; echo "${deps[*]}" | sed 's/,/, /g')
Conflicts: lightnvr
Replaces: lightnvr
Section: net
Priority: optional
Homepage: https://github.com/opensensor/lightNVR
Description: Lightweight Network Video Recorder (Rust build)
 lightnvr-rs is LightNVR with its backend mechanically translated from C to
 Rust by C2Rust. It uses the same configuration, database, web UI and go2rtc
 integration as lightnvr, and replaces it when installed.
 .
 Note: This package targets Debian ${suite}.
EOF
printf '/etc/lightnvr/lightnvr.ini\n/etc/lightnvr/go2rtc/go2rtc.yaml\n' > "$pkg/DEBIAN/conffiles"

cat > "$pkg/DEBIAN/postinst" <<'EOF'
#!/bin/sh
set -e
if command -v systemctl >/dev/null 2>&1 && systemctl is-system-running --quiet 2>/dev/null; then
  systemctl daemon-reload || true
fi
echo "LightNVR (lightnvr-rs) installed."
echo "  Start:       sudo systemctl start lightnvr-rs"
echo "  Enable boot: sudo systemctl enable lightnvr-rs"
echo "  Config:      /etc/lightnvr/lightnvr.ini"
echo "  Web UI:      http://localhost:8080"
EOF
cat > "$pkg/DEBIAN/prerm" <<'EOF'
#!/bin/sh
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl stop    lightnvr-rs 2>/dev/null || true
  systemctl disable lightnvr-rs 2>/dev/null || true
fi
EOF
chmod 0755 "$pkg/DEBIAN/postinst" "$pkg/DEBIAN/prerm"

mkdir -p "$out"
dpkg-deb --build --root-owner-group "$pkg" "$out/${name}.deb" >/dev/null
echo "$out/${name}.deb"
