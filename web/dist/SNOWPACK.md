# Using Snowpack with LightNVR Web Interface

This document explains how to use [Snowpack](https://www.snowpack.dev/) to build the LightNVR web interface.

## Overview

Snowpack is a modern, lightweight build tool that provides several advantages over traditional bundlers:

- Faster development builds
- No bundling required in development
- Modern ESM-based development
- Optimized production builds
- Built-in development server with hot module replacement

## Setup

The project has been configured to use Snowpack for building the web interface. The following files have been added or modified:

- `web/package.json` - Defines dependencies and scripts for npm
- `web/snowpack.config.js` - Configures Snowpack
- `scripts/build_web_snowpack.sh` - Script to build the web interface using Snowpack
- `scripts/update_html_for_snowpack.sh` - Script to update HTML files to use Snowpack-built assets

## Development Workflow

### Prerequisites

- Node.js (v14 or later)
- npm (v6 or later)

### Installing Dependencies

Navigate to the web directory and install dependencies:

```bash
cd web
npm install
```

### Development Server

To start the development server:

```bash
cd web
npm start
```

This will start a development server at http://localhost:8080 with hot module replacement.

### Building for Production

To build the web interface for production:

```bash
./scripts/build_web_snowpack.sh
```

This script will:
1. Install dependencies if needed
2. Build the web interface using Snowpack
3. Update the web_optimization.json configuration file

After building, you can update the HTML files to use the Snowpack-built assets:

```bash
./scripts/update_html_for_snowpack.sh
```

## Reverting to the Original Build Process

If you need to revert to the original build process, you can:

1. Restore the original HTML files from the backups created by the update script:
   ```bash
   find web -name "*.html.bak" -exec bash -c 'cp "{}" "$(dirname "{}")/$(basename "{}" .bak)"' \;
   ```

2. Restore the original web_optimization.json from the backup:
   ```bash
   cp config/web_optimization.json.bak config/web_optimization.json
   ```

3. Use the original optimization script:
   ```bash
   ./scripts/optimize_web_assets.sh
   ```

## Troubleshooting

### Common Issues

- **Module not found errors**: Make sure all imports are correctly updated to use npm packages or relative paths.
- **Build errors**: Check the Snowpack build output for specific errors.
- **Runtime errors**: Check the browser console for JavaScript errors.

### Debugging

- Use the development server for easier debugging
- Check the browser console for errors
- Verify that all imports are correctly resolved

## Additional Resources

- [Snowpack Documentation](https://www.snowpack.dev/reference/configuration)
- [Preact Documentation](https://preactjs.com/)
