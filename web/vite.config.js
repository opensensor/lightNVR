// Vite Configuration File
// Migrated from Snowpack configuration

import { defineConfig } from 'vite';
import { resolve } from 'path';
import legacy from '@vitejs/plugin-legacy';
import preact from '@preact/preset-vite';
import viteCompression from 'vite-plugin-compression';

// Custom plugin to remove "use client" directives
const removeUseClientDirective = () => {
  return {
    name: 'remove-use-client-directive',
    transform(code, id) {
      // Only target files from @preact-signals/query package
      if (id.includes('@preact-signals/query')) {
        // Check for "use client" directive with various possible formats
        const useClientRegex = /^(['"]use client['"])/;
        if (useClientRegex.test(code)) {
          // Remove the "use client" directive and return the modified code
          return {
            code: code.replace(useClientRegex, ''),
            map: null // We're not generating a sourcemap for this transformation
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
        // Ensure CSS files are properly named and placed
        assetFileNames: (assetInfo) => {
          const info = assetInfo.name.split('.');
          const ext = info[info.length - 1];
          if (/\.(css)$/i.test(assetInfo.name)) {
            return `css/[name][extname]`;
          }
          return `assets/[name][extname]`;
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
    // Add legacy browser support with explicit targets
    legacy({
      targets: ['defaults', 'not IE 11'],
      modernPolyfills: true
    }),
    // Custom plugin to handle non-module scripts
    {
      name: 'handle-non-module-scripts',
      transformIndexHtml(html) {
        // Replace dist/js/ references with ./js/ for Vite to process them
        return html
            .replace(/src="dist\/js\//g, 'src="./js/')
            .replace(/href="dist\/css\//g, 'href="./css/')
            .replace(/src="dist\/img\//g, 'src="./img/')
            .replace(/href="dist\/img\//g, 'href="./img/')
            .replace(/src="dist\/fonts\//g, 'src="./fonts/')
            .replace(/href="dist\/fonts\//g, 'href="./fonts/')
            // Also handle direct CSS references without dist/ prefix
            .replace(/href="css\//g, 'href="./css/');
      }
    },
    {
      name: 'handle-missing-app-js',
      resolveId(id, importer) {
        if (id === './js/app.js' && importer && importer.includes('streams.html')) {
          // Return false to signal that this import should be treated as external
          // This will prevent Vite from trying to resolve it during build
          return false;
        }
      }
    },
    // Custom plugin to copy CSS files
    {
      name: 'copy-css-files',
      async writeBundle() {
        const fs = await import('fs/promises');
        const path = await import('path');

        try {
          // Create dist/css directory if it doesn't exist
          await fs.mkdir('dist/css', { recursive: true });

          // Read all files from web/css
          const cssFiles = await fs.readdir('css');

          // Copy each CSS file to dist/css
          for (const file of cssFiles) {
            if (file.endsWith('.css')) {
              await fs.copyFile(
                  path.join('css', file),
                  path.join('dist/css', file)
              );
              console.log(`Copied ${file} to dist/css/`);
            }
          }
        } catch (error) {
          console.error('Error copying CSS files:', error);
        }
      }
    },
    // Gzip compression for static assets - generates .gz files alongside originals
    viteCompression({
      verbose: true,
      disable: false,
      threshold: 0, // Compress all files (needed for embedded devices with gzip-only assets)
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
      // Add aliases for the dist/js paths
      'dist/js': resolve(__dirname, 'js'),
      'dist/css': resolve(__dirname, 'css'),
      'dist/img': resolve(__dirname, 'img'),
      'dist/fonts': resolve(__dirname, 'fonts'),

      // Add React to Preact aliases
      'react': '@preact/compat',
      'react-dom/test-utils': '@preact/compat/test-utils',
      'react-dom': '@preact/compat',
      'react/jsx-runtime': '@preact/compat/jsx-runtime'
    },
  },
});
