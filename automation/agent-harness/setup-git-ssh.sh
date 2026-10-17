#!/usr/bin/env bash
set -euo pipefail

repo="${1:?Pass owner/repo}"
case "$repo" in
  opensensor/lightNVR) ;;
  *) echo 'Unsupported repository' >&2; exit 2 ;;
esac
: "${GIT_DEPLOY_KEY_B64:?Missing GIT_DEPLOY_KEY_B64}"

key_dir="${HOME}/.ssh"
mkdir -p "$key_dir"
chmod 700 "$key_dir"
printf '%s' "$GIT_DEPLOY_KEY_B64" | base64 -d > "$key_dir/harness_deploy_key"
chmod 600 "$key_dir/harness_deploy_key"
git config --global core.sshCommand "ssh -i $key_dir/harness_deploy_key -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new"
git remote set-url origin git@github.com:matteius/lightNVR.git
