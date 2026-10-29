# Release Checklist

Quick reference for creating a new LightNVR release.

## Prerequisites

- [ ] All changes committed and pushed to main branch
- [ ] All tests passing
- [ ] Submodules match the pinned commits (`git submodule update --init --recursive`)
- [ ] Clean git working directory (`git status` shows no changes)
- [ ] On the main branch

## Release Steps

### Option 1: Automated Release (Recommended)

```bash
# One command does everything!
./scripts/release.sh 0.13.0
```

This will:
- ✅ Bump version in all files
- ✅ Commit changes
- ✅ Create git tag
- ✅ Push to GitHub
- ✅ Trigger CI/CD builds

### Option 2: Manual Steps

If you need more control:

```bash
# 1. Synchronize pinned submodules before changing or staging files
git submodule update --init --recursive

# 2. Bump version
./scripts/bump-version.sh 0.13.0

# 3. Review changes, including submodule commit summaries
git diff --submodule=log

# 4. Stage only the tracked version files and review before committing
git add -- CMakeLists.txt web/package.json
git diff --cached --submodule=log
# Stop if the staged diff includes unexpected files or submodule changes.
git commit -m "Bump version to 0.13.0"

# 5. Tag
git tag -a v0.13.0 -m "Release v0.13.0"

# 6. Push
git push && git push --tags
```

Commit intentional dependency updates separately before synchronizing submodules.
Avoid `git add -A` for a version bump: it can stage a stale submodule checkout and
revert a dependency update. Generated version files are ignored by git.

## Post-Release

- [ ] Verify GitHub Actions build succeeds: https://github.com/opensensor/lightNVR/actions
- [ ] Check Docker images published: https://github.com/opensensor/lightNVR/pkgs/container/lightnvr
- [ ] Test Docker image: `docker pull ghcr.io/opensensor/lightnvr:0.13.0`
- [ ] Update release notes on GitHub (optional)

## Common Issues

### "Git working directory is not clean"
```bash
git status  # See what's uncommitted
git diff --submodule=log
git diff --cached --submodule=log
```

Synchronize stale submodule checkouts before staging anything, and stage and
commit intentional changes explicitly.

### "Tag already exists"
```bash
# Delete local and remote tag
git tag -d v0.13.0
git push origin :refs/tags/v0.13.0
```

### Test release locally without pushing
```bash
./scripts/release.sh 0.13.0 --no-push
```

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 0.13.0)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes

## Files Updated Automatically

The release scripts update:
- `CMakeLists.txt` - Project version
- `web/package.json` - Web interface version
- `include/core/version.h` - C header (generated)
- `web/js/version.js` - JavaScript version (generated)

## What Happens in CI/CD

When you push a tag:

1. **GitHub Actions triggers** multi-arch Docker builds
2. **Web assets built** during Docker build (Node.js + Vite)
3. **Docker images published** to GitHub Container Registry
4. **Tags created**: `latest`, `0.13.0`, `0.13`, `0`

## Important Notes

- ⚠️ **Web assets (`web/dist/`) are NOT checked into git**
- ⚠️ They are built automatically during CI/CD
- ⚠️ For local development, run `npm run build` in the `web/` directory

## Need Help?

See detailed documentation: [docs/RELEASE_PROCESS.md](docs/RELEASE_PROCESS.md)

