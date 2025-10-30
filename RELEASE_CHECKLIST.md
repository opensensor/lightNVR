# Release Checklist

Quick reference for creating a new LightNVR release.

## Prerequisites

- [ ] All changes committed and pushed to main branch
- [ ] All tests passing
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
# 1. Bump version
./scripts/bump-version.sh 0.13.0

# 2. Review changes
git diff

# 3. Commit
git add -A
git commit -m "Bump version to 0.13.0"

# 4. Tag
git tag -a v0.13.0 -m "Release v0.13.0"

# 5. Push
git push && git push --tags
```

## Post-Release

- [ ] Verify GitHub Actions build succeeds: https://github.com/opensensor/lightNVR/actions
- [ ] Check Docker images published: https://github.com/opensensor/lightNVR/pkgs/container/lightnvr
- [ ] Test Docker image: `docker pull ghcr.io/opensensor/lightnvr:0.13.0`
- [ ] Update release notes on GitHub (optional)

## Common Issues

### "Git working directory is not clean"
```bash
git status  # See what's uncommitted
git add -A && git commit -m "Your message"
```

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

