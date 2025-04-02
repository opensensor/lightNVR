# Migration from Snowpack to Vite

This document outlines the migration from Snowpack to Vite for the LightNVR web interface.

## Why Migrate?

Snowpack is no longer maintained, while Vite is actively developed and offers better performance, features, and community support.

## Changes Made

1. **Configuration Files**:
   - Created `vite.config.js` to replace `snowpack.config.js`
   - Updated `postcss.config.js` to use ES modules syntax
   - Updated `tailwind.config.js` to use ES modules syntax

2. **Package.json**:
   - Updated scripts to use Vite instead of Snowpack:
     - `start` now runs `vite` instead of `snowpack dev`
     - `build` now runs `vite build` instead of `snowpack build`
     - Added `preview` script to run `vite preview`
   - Added Vite and related dependencies

3. **Build Scripts**:
   - Created `build_web_vite.sh` to replace `build_web_snowpack.sh`
   - Created `build_and_update_web_vite.sh` to replace `build_and_update_web_snowpack.sh`

## How to Use

### Development

To start the development server:

```bash
cd web
npm run start
```

This will start a development server at http://localhost:8080.

### Production Build

To build for production:

```bash
cd web
npm run build
```

Or use the build script:

```bash
./scripts/build_web_vite.sh
```

This will create a production build in the `web/dist` directory.

### Preview Production Build

To preview the production build:

```bash
cd web
npm run preview
```

## Directory Structure

Vite uses a slightly different output structure than Snowpack:

- JavaScript, CSS, and other assets are placed in the `dist/assets` directory
- HTML files are placed in the `dist` directory

## Notes

- The migration maintains the same functionality as the Snowpack build
- Vite provides better performance and more features
- The build process is faster with Vite
