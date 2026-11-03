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

## TypeScript

Vite compiles `.ts` and `.tsx` with Oxc using the same Preact JSX settings as
`.jsx`, and only strips types. `npm run typecheck` is what checks them, and CI
runs it in `static-analysis.yml`.

- `tsconfig.base.json` holds the shared strict options. Do not relax them.
- `tsconfig.json` covers `js/**`; Vite's client types come from `js/env.d.ts`.
- `tsconfig.tests.json` covers TS tests under `tests/` with Jest and Node types.
- `allowJs` is on and `checkJs` is off, so TS can import legacy `.js` modules
  without those modules being checked. `// @ts-nocheck` opts a TS file out until
  its domain is typed.
- Jest compiles `.ts`, `.tsx`, and `.jsx` through the overrides in
  `babel.config.cjs`. Plain `.js` compiles as before.
- ESLint parses `.ts` and `.tsx` with the TypeScript parser and keeps the a11y and
  i18n rules on them. `no-explicit-any` and `ban-ts-comment` are errors;
  `@ts-expect-error` needs a reason on the same line.
- `i18n-literal-baseline.json` keeps its `.jsx` keys: the rule reads `X.tsx` as
  `X.jsx`, so counts carry across a rename.
- `tests/types/tsxFixture.spec.tsx` runs TSX through Preact in Jest. Its
  `@ts-expect-error` is a permanent check that prop types are enforced, so
  removing that line must make `npm run typecheck` fail.

## Directory Structure

Vite uses a slightly different output structure than Snowpack:

- JavaScript, CSS, and other assets are placed in the `dist/assets` directory
- HTML files are placed in the `dist` directory

## Notes

- The migration maintains the same functionality as the Snowpack build
- Vite provides better performance and more features
- The build process is faster with Vite
