#!/bin/sh
set -eu

guard=$1
probe=$2
probe_dir=$(mktemp -d)
trap 'rm -rf "$probe_dir"' EXIT
ln -s "$probe" "$probe_dir/ffmpeg"
PATH="$probe_dir:$PATH"
export PATH

"$guard" -version
LIGHTNVR_FFMPEG_MAX_RSS_MB=128 "$guard" -c:a aac -vn \
    --expect-as=402653184
LIGHTNVR_FFMPEG_MAX_RSS_MB=invalid "$guard" -c:a aac -vn \
    --expect-as=1073741824 >"$probe_dir/invalid.log" 2>&1
grep -q 'invalid LIGHTNVR_FFMPEG_MAX_RSS_MB' "$probe_dir/invalid.log"

set +e
LIGHTNVR_FFMPEG_MAX_RSS_MB=128 "$guard" -c:v libx264 \
    --allocate-mib=192 >"$probe_dir/rss.log" 2>&1
status=$?
set -e
if [ "$status" -ne 137 ]; then
    cat "$probe_dir/rss.log"
    exit 1
fi
grep -q 'exceeds 128 MiB; killing transcoder' "$probe_dir/rss.log"
