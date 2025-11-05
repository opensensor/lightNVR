# LightNVR Scripts

This directory contains various scripts for building, installing, and maintaining LightNVR.

## Installation Scripts

### `install.sh`
Main installation script for LightNVR.

**Usage:**
```bash
sudo bash scripts/install.sh
```

**What it does:**
- Installs the LightNVR binary
- Installs SOD library (if enabled)
- Creates necessary directories
- Installs configuration files
- Installs web assets (if available)
- Sets up systemd service

**Important:** Make sure to build web assets before running this script:
```bash
cd web
npm install
npm run build
cd ..
sudo bash scripts/install.sh
```

### `install_web_assets.sh`
Dedicated script for installing or reinstalling web interface assets.

**Usage:**
```bash
sudo bash scripts/install_web_assets.sh
```

**What it does:**
- Checks for prebuilt web assets in `web/dist/`
- Builds assets if needed (requires Node.js/npm)
- Installs assets to `/var/lib/lightnvr/www/`
- Sets proper permissions
- Verifies installation

**When to use:**
- After a fresh installation if web interface shows blank page
- After updating web interface code
- To fix web interface issues

### `install_go2rtc.sh`
Installs the go2rtc binary for WebRTC streaming.

**Usage:**
```bash
sudo bash scripts/install_go2rtc.sh
```

## Diagnostic Scripts

### `diagnose_web_issue.sh`
Diagnoses common web interface issues.

**Usage:**
```bash
sudo bash scripts/diagnose_web_issue.sh
```

**What it checks:**
- LightNVR service status
- Configuration file
- Web root directory existence and permissions
- Critical web files (index.html, etc.)
- Assets directory
- Recent logs for errors

**Output:**
- Detailed diagnostic report
- Recommendations for fixing issues

**When to use:**
- Web interface shows blank page
- Getting 404 errors
- Web interface not loading properly

## Build Scripts

### `build.sh`
Main build script for LightNVR.

**Usage:**
```bash
bash scripts/build.sh
```

**Options:**
- `--clean`: Clean build directory before building
- `--release`: Build in release mode (default is debug)
- `--install`: Install after building

### `build_web_vite.sh`
Builds web assets using Vite.

**Usage:**
```bash
bash scripts/build_web_vite.sh
```

**What it does:**
- Extracts version from CMakeLists.txt
- Installs npm dependencies if needed
- Builds web assets with Vite
- Outputs to `web/dist/`

**Requirements:**
- Node.js and npm installed

### `build_and_update_web_vite.sh`
Builds web assets and updates HTML files.

**Usage:**
```bash
bash scripts/build_and_update_web_vite.sh
```

**What it does:**
- Runs `build_web_vite.sh`
- Runs `update_html_for_vite.sh`
- Ensures all assets are properly linked

## Utility Scripts

### `optimize_web_assets.sh`
Optimizes web assets for production.

**Usage:**
```bash
bash scripts/optimize_web_assets.sh
```

### `update_html_for_vite.sh`
Updates HTML files for Vite build system.

**Usage:**
```bash
bash scripts/update_html_for_vite.sh
```

### `extract_version.sh` / `extract_version.js`
Extracts version information from CMakeLists.txt.

**Usage:**
```bash
bash scripts/extract_version.sh
# or
node scripts/extract_version.js
```

### `bump-version.sh`
Bumps version number in project files.

**Usage:**
```bash
bash scripts/bump-version.sh [major|minor|patch]
```

### `release.sh`
Creates a release package.

**Usage:**
```bash
bash scripts/release.sh
```

## Common Workflows

### Fresh Installation

```bash
# 1. Build the application
bash scripts/build.sh --release

# 2. Build web assets
cd web
npm install
npm run build
cd ..

# 3. Install everything
sudo bash scripts/install.sh

# 4. Install go2rtc
sudo bash scripts/install_go2rtc.sh

# 5. Start service
sudo systemctl start lightnvr
sudo systemctl enable lightnvr
```

### Fixing Blank Web Page

```bash
# 1. Diagnose the issue
sudo bash scripts/diagnose_web_issue.sh

# 2. Install/reinstall web assets
sudo bash scripts/install_web_assets.sh

# 3. Restart service
sudo systemctl restart lightnvr
```

### Updating Web Interface Only

```bash
# 1. Make your changes to web files

# 2. Build web assets
cd web
npm run build
cd ..

# 3. Install web assets
sudo bash scripts/install_web_assets.sh

# 4. Restart service
sudo systemctl restart lightnvr
```

### Updating Application Only

```bash
# 1. Make your changes to source code

# 2. Build
bash scripts/build.sh --release

# 3. Stop service
sudo systemctl stop lightnvr

# 4. Install binary
sudo cp build/bin/lightnvr /usr/local/bin/

# 5. Start service
sudo systemctl start lightnvr
```

## Troubleshooting

### Web interface shows blank page
Run the diagnostic script:
```bash
sudo bash scripts/diagnose_web_issue.sh
```

Then follow the recommendations, or see [TROUBLESHOOTING_WEB_INTERFACE.md](../docs/TROUBLESHOOTING_WEB_INTERFACE.md)

### Build fails
Check that you have all dependencies installed:
```bash
# Debian/Ubuntu
sudo apt-get install build-essential cmake libsqlite3-dev

# For web assets
sudo apt-get install nodejs npm
```

### Installation fails
Make sure you're running as root:
```bash
sudo bash scripts/install.sh
```

Check that the build completed successfully before installing.

## Script Dependencies

### System Requirements
- **For building C/C++ code:**
  - GCC or Clang
  - CMake
  - SQLite3 development libraries

- **For building web assets:**
  - Node.js (v14 or later)
  - npm (v6 or later)

- **For installation:**
  - Root/sudo access
  - systemd (for service management)

### Installing Dependencies

**Debian/Ubuntu:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential cmake libsqlite3-dev nodejs npm
```

**Fedora/RHEL:**
```bash
sudo dnf install -y gcc gcc-c++ cmake sqlite-devel nodejs npm
```

**Arch Linux:**
```bash
sudo pacman -S base-devel cmake sqlite nodejs npm
```

## Contributing

When adding new scripts:
1. Make them executable: `chmod +x scripts/your_script.sh`
2. Add proper error handling (`set -e`)
3. Add usage information in comments
4. Update this README
5. Test on a clean system if possible

