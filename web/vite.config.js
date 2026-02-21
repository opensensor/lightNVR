// Vite Configuration File
// Migrated from Snowpack configuration

import { defineConfig } from 'vite';
import { resolve, join, basename, dirname } from 'path';
import { fileURLToPath } from 'url';
import legacy from '@vitejs/plugin-legacy';
import preact from '@preact/preset-vite';
import viteCompression from 'vite-plugin-compression';
import themeInjectPlugin from './vite-plugin-theme-inject.js';
import * as fsPromises from 'fs/promises';

const __dirname = dirname(fileURLToPath(import.meta.url));

const MISSING_APP_JS_IMPORTERS = ['streams.html'];
const MIN_COMPRESSION_SIZE_BYTES = 1024;

/**
 * Check whether the given filesystem path exists.
 * Returns true if it exists, false if it does not (ENOENT),
 * and rethrows any other error.
 *
 * This centralizes the access/ENOENT handling pattern used
 * across multiple plugins.
 *
 * @param {string} path
 * @returns {Promise<boolean>}
 */
async function pathExists(path) {
  try {
    await fsPromises.access(path);
    return true;
  } catch (err) {
    // ENOENT means the path does not exist; return false.
    if (err && err.code === 'ENOENT') {
      return false;
    }
    // For any other error, rethrow so callers can handle/log appropriately.
    throw err;
  }
}

// Custom plugin to remove "use client" directives
const removeUseClientDirective = () => {
  return {
    name: 'remove-use-client-directive',
    transform(code, id) {
      // Only target files from @preact-signals/query package
      if (id.includes('@preact-signals/query')) {
        // Regex components for matching a single "use client" directive line:
        // - leadingWhitespace: optional spaces/tabs at the start of the line
        // - quoteGroup: opening quote (single or double), captured for reuse
        // - directiveText: the directive text itself, allowing multiple spaces
        // - backreference \\1: matches the same quote used to open the string
        // - optionalSemicolon: optional trailing semicolon
        // - trailingComment: optional inline // comment or /* block comment */
        // - optionalTrailingWhitespace: optional whitespace up to the end of the line
        const leadingWhitespace = '^\\s*';
        const quoteGroup = "(['\"])";
        const directiveText = 'use\\s+client';
        const optionalSemicolon = ';?';
        const trailingComment = '(?:\\s*(?:\\/\\/[^\\n]*|\\/\\*[\\s\\S]*?\\*\\/))?';
        const optionalTrailingWhitespace = '\\s*$';
        const useClientRegex = new RegExp(
          leadingWhitespace +
            quoteGroup +
            directiveText +
            '\\1' +
            optionalSemicolon +
            trailingComment +
            optionalTrailingWhitespace,
          'gm'
        );
        // Remove the "use client" directive (if present) and return the modified code
        const transformedCode = code.replace(useClientRegex, '');
        if (transformedCode !== code) {
          return {
            code: transformedCode,
            // Do not emit a custom sourcemap; allow Vite/Rollup to handle sourcemaps if enabled
            map: null
          };
        }
      }
      return null; // Return null to indicate no transformation needed
    }
  };
};

export default defineConfig({
  // Configure esbuild to handle "use client" directives
  esbuild: {
    supported: {
      'top-level-await': true, // Enable top level await
    },
    legalComments: 'none', // Remove all legal comments
    // Ignore specific warnings
    logOverride: {
      'module-level-directive': 'silent', // Silence module level directive warnings
    },
  },
  // Base public path when served in production
  base: './',

  // Configure the build
  build: {
    // Output directory for the build (equivalent to Snowpack's out)
    outDir: 'dist',

    // Enable source maps only if BUILD_SOURCEMAPS env var is set to 'true'
    sourcemap: process.env.BUILD_SOURCEMAPS === 'true',

    // Ensure assets are correctly referenced
    assetsDir: 'assets',

    // Clean the output directory before building
    emptyOutDir: true,

    // Ensure CSS is properly extracted and included
    cssCodeSplit: true,

    // Configure esbuild to handle "use client" directives
    commonjsOptions: {
      transformMixedEsModules: true
    },

    // Configure esbuild to ignore specific warnings
    minify: 'esbuild',
    // Removed target: 'es2015' as it's handled by the legacy plugin

    // Rollup options
    rollupOptions: {
      input: {
        // Add all HTML files as entry points
        index: resolve(__dirname, 'index.html'),
        login: resolve(__dirname, 'login.html'),
        recordings: resolve(__dirname, 'recordings.html'),
        settings: resolve(__dirname, 'settings.html'),
        streams: resolve(__dirname, 'streams.html'),
        system: resolve(__dirname, 'system.html'),
        timeline: resolve(__dirname, 'timeline.html'),
        users: resolve(__dirname, 'users.html'),
        hls: resolve(__dirname, 'hls.html'),
      },
      output: {
        // Ensure CSS files are properly named and placed, with hashes for cache busting
        assetFileNames: (assetInfo) => {
          if (/\.css$/i.test(assetInfo.name)) {
            return `css/[name]-[hash][extname]`;
          }
          return `assets/[name]-[hash][extname]`;
        },
      },
    },
  },

  // Configure the dev server
  server: {
    // Set the port for the dev server (same as Snowpack)
    port: 8080,

    // Don't open the browser on start
    open: false,
  },

  // Configure plugins
  plugins: [
    // Preact plugin to handle JSX
    preact(),
    // Custom plugin to remove "use client" directives
    removeUseClientDirective(),
    // Theme injection plugin - reads COLOR_THEMES from theme-init.js and injects into HTML
    themeInjectPlugin(),
    // Add legacy browser support with explicit targets
    legacy({
      targets: ['defaults', 'not IE 11'],
      modernPolyfills: true
    }),
    // Custom plugin to handle non-module scripts
    {
      name: 'handle-non-module-scripts',
      transformIndexHtml(html) {
        // Replace dist/* references with ./* for Vite to process them in a single pass.
        // Alternations: src="dist/js/ | href="dist/css/ | src="dist/img/ | href="dist/img/ |
        //               src="dist/fonts/ | href="dist/fonts/ | href="css/ (no dist/ prefix)
        const pathMap = {
          'src="dist/js/': 'src="./js/',
          'href="dist/css/': 'href="./css/',
          'src="dist/img/': 'src="./img/',
          'href="dist/img/': 'href="./img/',
          'src="dist/fonts/': 'src="./fonts/',
          'href="dist/fonts/': 'href="./fonts/',
          // Also handle direct CSS references without dist/ prefix
          'href="css/': 'href="./css/',
        };

        const transformed = html.replace(
          /src="dist\/js\/|href="dist\/css\/|src="dist\/img\/|href="dist\/img\/|src="dist\/fonts\/|href="dist\/fonts\/|href="css\//g,
          (match) => pathMap[match] || match
        );

        return transformed;
      }
    },
    {
      name: 'handle-missing-app-js',
      // List of HTML files that import a missing ./js/app.js and should be treated as external
      resolveId(id, importer) {
        if (
          id === './js/app.js' &&
          importer &&
          MISSING_APP_JS_IMPORTERS.includes(basename(importer))
        ) {
          // Mark this import as external so Vite/Rollup does not try to resolve it during build
          return { id, external: true };
        }
      }
    },
    // Custom plugin to copy CSS files
    {
      name: 'copy-css-files',
      async writeBundle() {
        try {
          // Create dist/css directory if it doesn't exist
          await fsPromises.mkdir('dist/css', { recursive: true });

          // Ensure source css directory exists before reading
          const srcCssDir = 'css';
          const cssDirExists = await pathExists(srcCssDir);
          if (!cssDirExists) {
            console.warn(
              `Source CSS directory "${srcCssDir}" (resolved to "${resolve(__dirname, srcCssDir)}") does not exist; skipping CSS copy.`
            );
            return;
          }

          // Read all directory entries from css, including type information
          const cssEntries = await fsPromises.readdir(srcCssDir, { withFileTypes: true });

          // Copy each regular CSS file to dist/css
          for (const entry of cssEntries) {
            // Skip anything that isn't a regular file or doesn't end with .css
            if (!entry.isFile() || !entry.name.endsWith('.css')) {
              continue;
            }

            const srcPath = join('css', entry.name);
            const destPath = join('dist/css', entry.name);

            try {
              await fsPromises.copyFile(srcPath, destPath);
              console.log(`Copied ${entry.name} to dist/css/`);
            } catch (copyError) {
              // Log and continue copying other files
              console.error(`Error copying CSS file ${srcPath} to ${destPath}:`, copyError);
            }
          }
        } catch (error) {
          console.error('Error copying CSS files:', error);
        }
      }
    },
    // Custom plugin to copy img files
    {
      name: 'copy-img-files',
      async writeBundle() {
        try {
          // Create dist/img directory if it doesn't exist
          await fsPromises.mkdir('dist/img', { recursive: true });

          // Check if img directory exists using shared helper
          const imgDirExists = await pathExists('img');
          if (!imgDirExists) {
            console.log('No img directory found, skipping');
            return;
          }

          // Read all entries from img (files and possibly subdirectories)
          const imgEntries = await fsPromises.readdir('img', { withFileTypes: true });

          // Copy each image file to dist/img, skipping subdirectories and non-file entries
          for (const entry of imgEntries) {
            if (!entry.isFile()) {
              continue;
            }
            const file = entry.name;
            try {
              await fsPromises.copyFile(
                  join('img', file),
                  join('dist/img', file)
              );
              console.log(`Copied ${file} to dist/img/`);
            } catch (err) {
              console.error(`Failed to copy ${file} to dist/img/:`, err);
              // Continue copying other files instead of failing the entire build step
              // (execution naturally proceeds to the next iteration)
            }
          }
        } catch (error) {
          console.error('Error copying img files:', error);
        }
      }
    },
    // Gzip compression for static assets - generates .gz files alongside originals
    viteCompression({
      verbose: true,
      disable: false,
      // Only compress files larger than 1 KiB to avoid wasting gzip overhead on tiny assets
      threshold: MIN_COMPRESSION_SIZE_BYTES,
      algorithm: 'gzip',
      ext: '.gz',
      // Compress JS, CSS, HTML, JSON, and SVG files
      filter: /\.(js|css|html|json|svg)$/i,
    }),
  ],

  // Configure CSS
  css: {
    // PostCSS configuration is loaded from postcss.config.js
    postcss: true,
    // Ensure CSS files are properly processed - only enable in dev or if BUILD_SOURCEMAPS is set
    devSourcemap: process.env.BUILD_SOURCEMAPS === 'true',
  },

  // Preserve the directory structure
  publicDir: 'public',

  // Resolve configuration
  resolve: {
    alias: {
      // Add aliases for source asset paths (JS, CSS, images, fonts)
      '@js': resolve(__dirname, 'js'),
      '@css': resolve(__dirname, 'css'),
      '@img': resolve(__dirname, 'img'),
      '@fonts': resolve(__dirname, 'fonts'),

      // Add React to Preact aliases
      'react': '@preact/compat',
      'react-dom': '@preact/compat',
      'react-dom/test-utils': '@preact/compat/test-utils',
      'react/jsx-runtime': '@preact/compat/jsx-runtime'
    },
  },
});
