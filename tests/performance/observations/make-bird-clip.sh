#!/usr/bin/env bash
# X2 detection-producing clip for the INT_01 performance workload.
#
#   tests/performance/observations/make-bird-clip.sh CARDINAL.jpg OUT.mp4
#
# Renders a real bird photo moving across a static, lightly textured 1280x720
# background as H.264 matching workload.json (15 fps, 15-frame GOP, no
# B-frames, yuv420p, 30 s), so SOD VOC tiny20.sod reports "bird" on every
# sampled frame and evidence is produced at the detector cadence.
#
# The photo is NOT committed: it is CC BY-SA 4.0, which does not fit this
# repository. Use the same file X1 pinned (tests/integration/yawamf_e2e/README.md):
#   Wikimedia Commons File:Northern_cardinal_male_(84894).jpg, 330 px thumbnail
#   https://upload.wikimedia.org/wikipedia/commons/thumb/9/97/Northern_cardinal_male_%2884894%29.jpg/330px-Northern_cardinal_male_%2884894%29.jpg
#   sha256 85bf66b1c51ad1aeb08d3d90f1a39a59f6cb6161d91f7e7b51d4d187847f3cdf
# The script refuses another file unless BIRD_CLIP_ANY_IMAGE=1.
#
# Output is deterministic for a given photo and ffmpeg build (fixed noise seed,
# single-threaded x264); run-workload.mjs records its sha256 either way.
set -euo pipefail

PHOTO_SHA256=85bf66b1c51ad1aeb08d3d90f1a39a59f6cb6161d91f7e7b51d4d187847f3cdf
if [ $# -ne 2 ]; then
  echo "usage: $0 CARDINAL.jpg OUT.mp4" >&2
  exit 2
fi
photo=$1
out=$2
actual=$(sha256sum "$photo" | cut -d' ' -f1)
if [ "$actual" != "$PHOTO_SHA256" ] && [ "${BIRD_CLIP_ANY_IMAGE:-0}" != 1 ]; then
  echo "$photo: sha256 $actual is not the pinned cardinal photo ($PHOTO_SHA256); set BIRD_CLIP_ANY_IMAGE=1 to override" >&2
  exit 1
fi
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# Static texture (a fixed-seed noise frame, slightly blurred): realistic JPEG
# sizes for the evidence byte budgets without a temporally noisy recording.
ffmpeg -nostdin -hide_banner -loglevel error -y \
  -f lavfi -i "color=c=0x6f7d5c:s=1280x720:d=1,noise=alls=28:allf=u:all_seed=8675309,gblur=sigma=1.2" \
  -frames:v 1 "$work/background.png"

# The bird (360 px high) bounces across the frame at 160 px/s and bobs
# vertically, always fully in frame.
ffmpeg -nostdin -hide_banner -loglevel error -y \
  -loop 1 -framerate 15 -i "$work/background.png" \
  -loop 1 -framerate 15 -i "$photo" \
  -filter_complex "[1:v]scale=-2:360,format=yuv420p[bird];[0:v]format=yuv420p[bg];[bg][bird]overlay=x='abs(mod(t*160\,2*(W-w))-(W-w))':y='(H-h)/2+120*sin(t*0.9)':shortest=0,scale=out_range=tv,format=yuv420p[v]" \
  -map "[v]" -t 30 -r 15 -an -c:v libx264 -threads 1 -preset ultrafast -pix_fmt yuv420p \
  -g 15 -keyint_min 15 -sc_threshold 0 -bf 0 -movflags +faststart "$out"

sha256sum "$out"
