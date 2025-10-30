# Build Process Improvements

## Summary

The LightNVR build process has been completely automated to eliminate manual steps and ensure consistency across releases.

## Problems Solved

### Before
1. ❌ Manual version bumping in CMakeLists.txt
2. ❌ Manual web asset building before releases
3. ❌ Web assets checked into git (100+ files, bloating repository)
4. ❌ Manual HTML updates to point to new assets
5. ❌ Easy to forget steps, leading to version mismatches
6. ❌ GitHub Actions could fail if assets weren't checked in
7. ❌ Wasted GitHub storage with binary assets in git history

### After
1. ✅ Automated version bumping with `./scripts/bump-version.sh`
2. ✅ Web assets built automatically during CI/CD
3. ✅ Web assets NOT checked into git (cleaner repository)
4. ✅ Single command release process: `./scripts/release.sh X.Y.Z`
5. ✅ Impossible to forget steps - everything is automated
6. ✅ GitHub Actions builds everything from source
7. ✅ Clean git history with only source code

## New Scripts

### 1. `scripts/bump-version.sh`
Bumps version across all project files:
- `CMakeLists.txt` - Main project version
- `web/package.json` - Web interface version
- `include/core/version.h` - Generated C header
- `web/js/version.js` - Generated JavaScript version

**Usage:**
```bash
./scripts/bump-version.sh 0.13.0
```

### 2. `scripts/release.sh`
Automates the entire release process:
1. Runs pre-flight checks (clean git, valid version, etc.)
2. Bumps version in all files
3. Commits changes
4. Creates git tag
5. Pushes to GitHub
6. Triggers CI/CD builds

**Usage:**
```bash
# Full release (recommended)
./scripts/release.sh 0.13.0

# Dry run (don't push)
./scripts/release.sh 0.13.0 --no-push
```

### 3. `scripts/migrate-remove-web-dist.sh`
One-time migration script to remove web/dist from git tracking.

**Usage:**
```bash
./scripts/migrate-remove-web-dist.sh
```

## Changes Made

### 1. Dockerfile
- Added Node.js 20.x LTS installation
- Added web asset build step (`npm ci && npm run build`)
- Web assets built before C/C++ compilation
- Assets copied to `/usr/share/lightnvr/web-template/`

### 2. .gitignore
- Added `web/dist/` to ignore built assets
- Removed `web/dist/node_modules/` (redundant)

### 3. Documentation
- Created `docs/RELEASE_PROCESS.md` - Comprehensive release guide
- Created `RELEASE_CHECKLIST.md` - Quick reference for releases
- Updated `README.md` - Added link to release documentation

### 4. GitHub Actions
- No changes needed! Existing workflow already triggers on tags
- Web assets now built during Docker build automatically

## Migration Steps

For existing checkouts with web/dist tracked in git:

```bash
# Run the migration script
./scripts/migrate-remove-web-dist.sh

# Push the changes
git push
```

This will:
1. Remove ~100+ files from git tracking
2. Keep files on disk (for local development)
3. Create a commit documenting the change

## New Release Workflow

### For Maintainers

**Old workflow (manual):**
1. Edit CMakeLists.txt version
2. Run `./scripts/build_web_vite.sh`
3. Run `./scripts/update_html_for_vite.sh`
4. Check in web assets
5. Commit changes
6. Create git tag
7. Push commits and tags
8. Hope you didn't forget anything!

**New workflow (automated):**
```bash
./scripts/release.sh 0.13.0
```

That's it! Everything else happens automatically.

### For Developers

**Local development:**
```bash
# Start dev server (hot reload)
cd web
npm run start

# Build for production (optional)
cd web
npm run build
```

**Note:** You don't need to build web assets for local C development. The web interface can be developed independently.

## CI/CD Process

When you push a tag (e.g., `v0.13.0`):

1. **GitHub Actions triggers** for multiple architectures:
   - linux/amd64
   - linux/arm64
   - linux/arm/v7

2. **Docker build process:**
   - Installs Node.js 20.x LTS
   - Runs `npm ci` to install dependencies
   - Runs `npm run build` to build with Vite
   - Builds C/C++ code
   - Creates minimal runtime image
   - Copies web assets to `/usr/share/lightnvr/web-template/`

3. **Docker images published** to GitHub Container Registry:
   - `ghcr.io/opensensor/lightnvr:latest`
   - `ghcr.io/opensensor/lightnvr:0.13.0`
   - `ghcr.io/opensensor/lightnvr:0.13`
   - `ghcr.io/opensensor/lightnvr:0`

4. **Multi-arch manifest** created combining all platforms

## Benefits

### For Maintainers
- ✅ **Faster releases** - One command instead of 8+ manual steps
- ✅ **No mistakes** - Automated process prevents forgetting steps
- ✅ **Consistent versioning** - All files updated together
- ✅ **Clean git history** - No binary assets cluttering commits

### For Developers
- ✅ **Smaller clones** - No web/dist in git history
- ✅ **Faster pulls** - No binary assets to download
- ✅ **Clearer diffs** - Only source code changes visible
- ✅ **Independent development** - Web and C code can be developed separately

### For Users
- ✅ **Reliable builds** - Always built from source
- ✅ **Consistent versions** - No mismatches between code and assets
- ✅ **Faster downloads** - Smaller repository size

## Backward Compatibility

### For thinginfo-firmware

✅ **The buildroot package has been updated!**

The `lightnvr-buildroot/` directory now includes the necessary changes to build web assets during the Buildroot build process.

**What changed in the buildroot package:**

1. **Added `host-nodejs` dependency** (line 12 in `lightnvr.mk`)
2. **Added pre-build hook** to build web assets (lines 31-41)
3. **Installation unchanged** - still copies from `web/dist`, but now it's built

**Updated files:**
- `lightnvr-buildroot/lightnvr.mk` - Main package definition
- `lightnvr-buildroot/README.md` - Comprehensive documentation
- `lightnvr-buildroot/CHANGELOG.md` - Change tracking

**To use the updated package:**

```bash
# Copy to your Buildroot tree
cp -r lightnvr-buildroot/ /path/to/buildroot/package/lightnvr/

# Rebuild
make lightnvr-dirclean
make lightnvr
```

**Alternative: Use pre-built releases**

If you prefer not to build web assets in Buildroot, you can extract them from Docker images:

```bash
docker pull ghcr.io/opensensor/lightnvr:0.13.0
docker create --name temp ghcr.io/opensensor/lightnvr:0.13.0
docker cp temp:/usr/share/lightnvr/web-template ./web-assets
docker rm temp
```

## Testing

### Test the new scripts

```bash
# Test version bumping (dry run)
./scripts/bump-version.sh 0.13.0
git diff  # Review changes
git reset --hard  # Undo changes

# Test release script (without pushing)
./scripts/release.sh 0.13.0 --no-push
git log -1  # Review commit
git tag -l  # Review tag
git reset --hard HEAD~1  # Undo commit
git tag -d v0.13.0  # Delete tag
```

### Test Docker build locally

```bash
# Build Docker image
docker build -t lightnvr:test .

# Verify web assets exist
docker run --rm lightnvr:test ls -la /usr/share/lightnvr/web-template/

# Run the image
docker run -p 8080:8080 lightnvr:test
```

## Troubleshooting

### "npm: command not found" in Docker build
- Node.js installation failed
- Check Dockerfile Node.js installation step
- Verify NodeSource repository is accessible

### "web/dist not found" in Docker build
- Web build failed
- Check `npm run build` output in Docker logs
- Verify package.json and vite.config.js are correct

### Version mismatch after release
- Run `./scripts/bump-version.sh` again
- Verify all files were updated correctly
- Check git commit includes all version files

## Files Modified

### New Files
- `scripts/bump-version.sh` - Version bumping automation
- `scripts/release.sh` - Release automation
- `scripts/migrate-remove-web-dist.sh` - Migration helper
- `docs/RELEASE_PROCESS.md` - Comprehensive documentation
- `RELEASE_CHECKLIST.md` - Quick reference guide
- `BUILD_PROCESS_IMPROVEMENTS.md` - This file

### Modified Files
- `Dockerfile` - Added Node.js and web build steps
- `.gitignore` - Added web/dist/ exclusion
- `README.md` - Added link to release documentation

### Unchanged Files
- `.github/workflows/docker-publish.yml` - Already works with new process
- `CMakeLists.txt` - Still source of truth for version
- `web/package.json` - Still contains version
- All other source files

## Next Steps

1. **Run migration script** to remove web/dist from git:
   ```bash
   ./scripts/migrate-remove-web-dist.sh
   ```

2. **Test the new release process** (dry run):
   ```bash
   ./scripts/release.sh 0.12.9 --no-push
   git reset --hard HEAD~1
   git tag -d v0.12.9
   ```

3. **Create actual release** when ready:
   ```bash
   ./scripts/release.sh 0.13.0
   ```

4. **Update thinginfo-firmware** buildroot package (later)

## Questions?

See the detailed documentation:
- [docs/RELEASE_PROCESS.md](docs/RELEASE_PROCESS.md) - Full release guide
- [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) - Quick reference

Or open an issue on GitHub.

