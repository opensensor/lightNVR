#!/usr/bin/env bash
# Fail when a non-bot commit in BASE..HEAD touches rust/.
# rust/ is generated from C; change the C and let CI regenerate it.
set -euo pipefail
base="${1:?usage: guard.sh BASE_REF [HEAD_REF]}"
head="${2:-HEAD}"
bot="${LIGHTNVR_RS_BOT_EMAIL:-41898282+github-actions[bot]@users.noreply.github.com}"

bad=$(git log --no-merges --format='%H%x09%ae%x09%s' "$base..$head" -- rust/ \
      | awk -F'\t' -v bot="$bot" '$2 != bot')
if [[ -n "$bad" ]]; then
  echo "::error::rust/ is generated from C and cannot be edited directly."
  echo "These commits modify rust/:"
  echo "$bad" | sed 's/^/  /'
  echo
  echo "Drop those rust/ changes (e.g. git checkout origin/main -- rust/ && git commit),"
  echo "make the change in C instead, and CI will regenerate rust/ for you."
  echo "Compiler fixes for the translation belong in tools/rust-transpile/repairs.py."
  exit 1
fi
echo "rust/ is only modified by $bot"
