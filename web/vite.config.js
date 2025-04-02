// Vite Configuration File
// Migrated from Snowpack configuration

import { defineConfig } from 'vite';
import { resolve } from 'path';
import legacy from '@vitejs/plugin-legacy';

export default defineConfig({
  // Base public path when served in production
  base: './',
  
  // Configure the build
  build: {
    // Output directory for the build (equivalent to Snowpack's out)
    outDir: 'dist',
    
    // Enable source maps
    sourcemap: true,
    
    // Ensure assets are correctly referenced
    assetsDir: 'assets',
    
    // Clean the output directory before building
    emptyOutDir: true,
    
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
    // Add legacy browser support
    legacy({
      targets: ['defaults', 'not IE 11']
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
          .replace(/href="dist\/fonts\//g, 'href="./fonts/');
      }
    }
  ],
  
  // Configure CSS
  css: {
    // PostCSS configuration is loaded from postcss.config.js
    postcss: true,
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
    },
  },
});
