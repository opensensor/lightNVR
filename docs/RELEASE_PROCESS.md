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
3. Push the preparation commit to `main`
4. Wait for Rust regeneration and successful CI on the final commit
5. Create and push only the release tag, starting the C and Rust publishers

## Detailed Process

### Submodule checkpoint

Before starting either release path, synchronize submodules to the commits recorded
in the repository:

```bash
git submodule update --init --recursive
git diff --submodule=log
git diff --cached --submodule=log
```

Commit any intentional dependency updates separately before this step. Do not use
`--remote`, which follows submodule branches instead of the pinned commits. A stale
checkout staged with `git add -A` can silently undo a security update, as happened
in [issue #680](https://github.com/opensensor/lightNVR/issues/680).

Version-bump commits should contain only the tracked version files,
`CMakeLists.txt` and `web/package.json`. Review the staged diff before committing;
it should not include submodule changes. The generated version files are ignored
by git.

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

### Documentation checkpoint

Before cutting a release, confirm the documentation still describes what the code does.
This is not a general "review the docs" ask — it is a specific check against the changes
going into this release.

**If a change in this release altered any of the following, the documentation change ships
with it, not later:**

| Changed | Check |
|---|---|
| A default value | `docs/CONFIGURATION.md`, and the sample in `config/lightnvr.ini` |
| A port | `docs/DOCKER.md`, `README.md`, `docs/HOME_ASSISTANT.md`, `docker-compose.yml` |
| A path (config, data, web root, models) | `docs/DOCKER.md`, `docs/INSTALLATION.md`, `docs/TROUBLESHOOTING*.md` |
| Credentials or auth behavior | `README.md` first-login section, `docs/CONFIGURATION.md`, `docs/DOCKER.md` |
| An environment variable | `docs/DOCKER.md` environment table — including *removing* one that no longer does anything |
| A CLI flag, or a `scripts/*.sh` option | Every code block that uses it |
| Where images are published | `README.md`, `docs/INSTALLATION.md`, `docs/DOCKER.md` |

The failure mode this exists to prevent is documentation that quietly becomes false: it
does not break a build, no test catches it, and the first person to notice is a user who
followed it and got a different result. Every item in that table is there because it
happened.

### 2. Creating a Release

The recommended way to create a release is using the automated script:

```bash
./scripts/release.sh 0.13.0
```

#### What the Release Script Does

The command requires Python 3, Git, an authenticated GitHub CLI (`gh`) with Actions
read access, and permission to push `main` and tags through `origin`. Run it from
a `main` checkout with no uncommitted tracked changes, containing the changes you
intend to release. Untracked local notes and files are left alone and do not block
the release; Git still refuses to overwrite them during a fast-forward. It refuses
to silently pull new product changes into the release.

1. Validates the version, checkout and absence of both `X.Y.Z` and `vX.Y.Z` tags.
2. Runs `bump-version.sh`, stages only `CMakeLists.txt` and `web/package.json`,
   and commits the version bump. If that version is already prepared, it resumes.
3. Pushes the preparation to `main`, without creating a tag.
4. Follows generated `rust/` commits while CI runs. Any intervening commit that
   changes files outside `rust/`, or a rewritten history, stops the release for review.
5. Requires successful main-push runs on the exact final commit for Rust,
   integration tests, sanitizers, static analysis, CodeQL and container scanning.
   Any other main-push workflow that ran on that commit must also succeed.
   Missing or unfinished runs wait; failed, cancelled or skipped runs stop publication.
6. Checks that Rust CI's **Verify checked-in Rust is current** step succeeded.
   A successful run that regenerated stale Rust does not qualify: its generated
   commit must get its own successful CI run. CMake, web and Cargo versions must agree.
7. Fast-forwards the local checkout to the verified commit, creates `vX.Y.Z`,
   and pushes only that tag alongside a leased main ref in an atomic push.
   Unrelated local tags are never pushed. Publishing the tag starts release builds;
   it does not mean those builds have finished.

The default CI wait is four hours, polling every 30 seconds. Timeouts or CI failures
leave the preparation available to resume, without creating a tag. If a push loses
its connection, the local tag is retained because the remote might have accepted it;
inspect the remote before retrying.

#### Release Script Options

```bash
# Normal release: prepare, wait for generated Rust and CI, then tag and push
./scripts/release.sh 0.13.0

# Split preparation and publication across sessions
./scripts/release.sh 0.13.0 --prepare-only
./scripts/release.sh 0.13.0 --publish-only

# Prepare a local commit only: does not push OR create a tag
./scripts/release.sh 0.13.0 --no-push
# Later, push that preparation and complete the release
./scripts/release.sh 0.13.0 --publish-only

# Optional: skip the prompt or change the CI wait limit (seconds)
./scripts/release.sh 0.13.0 --publish-only --yes --timeout 14400

./scripts/release.sh --help
```

`--no-push` is local preparation, not a dry run: it updates version files and
creates a commit. Preflight still reads the remote to check main and existing tags.

#### Tag publication gate

Docker, Debian, Rust and changelog workflows all call `release-preflight.yml`
before publishing a tag. It verifies versions and the same exact-commit CI and
Rust freshness proof, including for tags pushed manually. An existing tag never
follows `main` to a newer commit. Branch `workflow_dispatch` packaging builds remain
available without the release gate; dispatching on a tag still uses the gate.

Do not use the old bump-and-immediately-tag sequence. A version bump changes the
generated Cargo package and Rust sources too. Use `release.sh` to let CI finish
regeneration first. These gates prevent publication from stale or untested commits;
packaging workflows still run independently and a later build failure can require
rerunning that publisher.

### 3. GitHub Actions Build Process

When you push a tag, GitHub Actions automatically:

1. **Builds Docker images** for multiple architectures:
   - linux/amd64
   - linux/arm64 on a native GitHub-hosted ARM64 runner
   - linux/arm/v7, cross-compiled on x86_64 (QEMU is used only to assemble the
     target runtime layer)

   Each platform also imports and updates a platform-specific BuildKit cache in
   GHCR. This avoids rebuilding unchanged package and dependency layers between
   releases.

2. **Builds web assets** during the Docker build:
   - Installs Node.js 24.x LTS
   - Runs `npm ci --ignore-scripts` from the committed lockfile
   - Runs `npm run build` to build with Vite
   - Copies built assets to `/var/lib/lightnvr/www/` in the runtime image

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

### "Tracked files have uncommitted changes"

You have staged or unstaged changes to tracked files. Either commit them or stash
them. Untracked files do not need to be committed, stashed or deleted for a release:

```bash
git status
git diff --submodule=log
git diff --cached --submodule=log
# or
git stash
```

If only a submodule checkout is stale, run `git submodule update --init --recursive`
before staging anything. Stage and commit intentional changes explicitly.

### CI failed, was cancelled, or the wait timed out

Inspect the reported run, fix the failure or rerun it, then resume:

```bash
./scripts/release.sh 0.13.0 --publish-only
```

If product changes landed on main while waiting, the script stops instead of
including them silently. Review and pull those changes before resuming. Generated
Rust-only follow-up commits are picked up automatically.

### "Tag X.Y.Z already exists"

The script uses annotated `vX.Y.Z` tags and checks both prefixed and unprefixed
names to prevent duplicate versions. Historical unprefixed tags remain supported
by the publishers. For a failed packaging job on a valid tag, rerun that job.
Do not move an already published release tag to pick up a later regeneration commit.

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

This can happen if you have different Node.js versions. The Docker build uses Node.js 24.x LTS. To match locally:

```bash
# Install the version pinned in .nvmrc (Node.js 24.x LTS)
nvm install
nvm use

# Then rebuild from the committed dependency graph
cd web
npm ci
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
3. **CI before tagging:** Rust regeneration and tests finish on the tagged commit
4. **Clean repository:** No binary assets in git

This ensures consistency, reduces errors, and makes releases faster and more reliable.
