#!/bin/sh
# Keep parent-delivered fixes reproducible until incorporated into the submodule pin.
set -eu
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
source_dir=${1:-"$script_dir/../go2rtc"}
patch_file="$script_dir/../patches/go2rtc/0001-close-rejected-http-responses.patch"
test -f "$source_dir/internal/http/http.go" || {
    echo "Missing go2rtc source: $source_dir" >&2
    exit 1
}
# Check both states before writing. Incompatible or partly applied source must
# be reviewed, never repaired by resetting a contributor's working tree.
if patch --directory="$source_dir" --strip=1 --fuzz=0 --force --reverse \
    --dry-run < "$patch_file" >/dev/null 2>&1; then
    echo "go2rtc HTTP response fix already applied"
    exit 0
fi
if ! patch --directory="$source_dir" --strip=1 --fuzz=0 --batch --forward \
    --dry-run < "$patch_file"; then
    echo "go2rtc source does not match the pending patch; review the source and patch together" >&2
    exit 1
fi
patch --directory="$source_dir" --strip=1 --fuzz=0 --batch --forward \
    --no-backup-if-mismatch < "$patch_file"
