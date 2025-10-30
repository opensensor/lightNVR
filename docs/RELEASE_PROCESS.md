# LightNVR Release Process

This document describes the automated release process for LightNVR.

## Overview

The release process has been fully automated to eliminate manual steps and ensure consistency. The key improvements include:

1. **Automated version bumping** across all project files
2. **Web assets built during CI/CD** (not checked into git)
3. **Single command release** process
4. **Consistent versioning** across CMakeLists.txt, package.json, and generated files

## Quick Start

To create a new release, simply run:

```bash
./scripts/release.sh 0.13.0
```

This single command will:
1. Bump the version in all necessary files
2. Commit the changes
3. Create a git tag
4. Push to GitHub
5. Trigger GitHub Actions to build and publish Docker images

## Detailed Process

### 1. Version Bumping

The version is managed in the following files:
- `CMakeLists.txt` - Main project version (line 2)
- `web/package.json` - Web interface version
- `include/core/version.h` - Generated from template
- `web/js/version.js` - Generated for web interface

#### Manual Version Bump

If you need to bump the version without creating a release:

```bash
./scripts/bump-version.sh 0.13.0
```

This will:
- Update `CMakeLists.txt` with the new version
- Update `web/package.json` with the new version
- Regenerate `include/core/version.h` from the template
- Regenerate `web/js/version.js` with the new version

**Note:** This does NOT commit the changes. You'll need to commit them manually.

### 2. Creating a Release

The recommended way to create a release is using the automated script:

```bash
./scripts/release.sh 0.13.0
```

#### What the Release Script Does

1. **Pre-flight checks:**
   - Verifies git working directory is clean
   - Checks you're on the main branch (warns if not)
   - Validates version format (MAJOR.MINOR.PATCH)
   - Ensures the tag doesn't already exist

2. **Version bumping:**
   - Runs `bump-version.sh` to update all version files

3. **Git operations:**
   - Commits all changes with message: "Bump version to X.Y.Z"
   - Creates an annotated git tag: "vX.Y.Z"
   - Pushes commits and tags to GitHub

4. **CI/CD trigger:**
   - GitHub Actions automatically starts building Docker images
   - Web assets are built during the Docker build process

#### Release Script Options

```bash
# Normal release (bumps version, commits, tags, and pushes)
./scripts/release.sh 0.13.0

# Dry run (prepares release locally but doesn't push)
./scripts/release.sh 0.13.0 --no-push

# Show help
./scripts/release.sh --help
```

### 3. GitHub Actions Build Process

When you push a tag, GitHub Actions automatically:

1. **Builds Docker images** for multiple architectures:
   - linux/amd64
   - linux/arm64
   - linux/arm/v7

2. **Builds web assets** during the Docker build:
   - Installs Node.js 20.x LTS
   - Runs `npm ci --omit=dev` to install production dependencies only
   - Runs `npm run build` to build with Vite
   - Copies built assets to `/usr/share/lightnvr/web-template/`

3. **Publishes images** to GitHub Container Registry:
   - `ghcr.io/opensensor/lightnvr:latest` (for main branch)
   - `ghcr.io/opensensor/lightnvr:0.13.0` (for version tags)
   - `ghcr.io/opensensor/lightnvr:0.13` (major.minor)
   - `ghcr.io/opensensor/lightnvr:0` (major only)

4. **Creates multi-arch manifest** combining all platform images

### 4. Web Assets

**Important:** Web assets (`web/dist/`) are **NOT** checked into git anymore.

#### Why?

- **Reduces repository size** - No binary assets in git history
- **Cleaner git diffs** - Only source code changes are tracked
- **Prevents inconsistencies** - Assets are always built from source
- **Simplifies workflow** - No need to remember to rebuild assets

#### Where are they built?

- **During Docker builds** - GitHub Actions builds them automatically
- **For local development** - Run `npm run build` in the `web/` directory
- **For local testing** - Run `./scripts/build_web_vite.sh`

#### Local Development

For local development, you can:

```bash
# Start development server (with hot reload)
cd web
npm run start

# Build for production (creates web/dist/)
cd web
npm run build

# Or use the build script
./scripts/build_web_vite.sh
```

**Note:** The `web/dist/` directory is in `.gitignore` and will not be committed.

## Version Numbering

LightNVR follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backwards compatible)
- **PATCH** version: Bug fixes (backwards compatible)

Examples:
- `0.12.8` → `0.13.0` (new features)
- `0.13.0` → `0.13.1` (bug fixes)
- `0.13.1` → `1.0.0` (major release with breaking changes)

## Troubleshooting

### "Git working directory is not clean"

You have uncommitted changes. Either commit them or stash them:

```bash
git status
git add -A && git commit -m "Your changes"
# or
git stash
```

### "Tag vX.Y.Z already exists"

The version tag already exists. Either:
- Use a different version number
- Delete the existing tag (if it was a mistake):
  ```bash
  git tag -d v0.13.0
  git push origin :refs/tags/v0.13.0
  ```

### "Failed to extract version from CMakeLists.txt"

The version format in `CMakeLists.txt` is incorrect. It should be:

```cmake
project(LightNVR VERSION 0.13.0 LANGUAGES C CXX)
```

### Web assets not found in Docker image

If the Docker build fails to find web assets:

1. Check that Node.js was installed correctly in the Dockerfile
2. Verify `npm run build` completed successfully
3. Check the build logs for errors

### Local build doesn't match Docker build

This can happen if you have different Node.js versions. The Docker build uses Node.js 20.x LTS. To match locally:

```bash
# Install Node.js 20.x
# Then rebuild
cd web
rm -rf node_modules package-lock.json
npm install
npm run build
```

## Migration Notes

### For Existing Checkouts

If you have an existing checkout with `web/dist/` tracked in git:

```bash
# Remove web/dist from git tracking
git rm -r --cached web/dist/

# Commit the removal
git commit -m "Remove web/dist from git tracking"

# The .gitignore already excludes web/dist/
```

### For thinginfo-firmware Integration

The buildroot package will need to be updated to:

1. Install Node.js as a build dependency
2. Run `npm ci && npm run build` in the web directory
3. Copy the built assets from `web/dist/` to the target

Alternatively, you can download pre-built releases from GitHub and extract the web assets from the Docker image.

## Summary

The new release process is:

1. **One command:** `./scripts/release.sh X.Y.Z`
2. **Automatic builds:** GitHub Actions builds everything
3. **No manual steps:** Version bumping, tagging, and pushing are automated
4. **Clean repository:** No binary assets in git

This ensures consistency, reduces errors, and makes releases faster and more reliable.

