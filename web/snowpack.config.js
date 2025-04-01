// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    // Mount directories to match the structure in the HTML files
    './js': '/js',
    './css': '/css',
    './img': '/img',
    './fonts': '/fonts',
    // Mount HTML files at the root
    '.': {
      url: '/',
      static: true,
      resolve: false
    }
  },
  plugins: [
    '@snowpack/plugin-postcss',
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    // Set the port for the dev server
    port: 8080,
    // Open the browser on start
    open: 'none',
    // Output style
    output: 'stream',
  },
  buildOptions: {
    // Output directory for the build
    out: './dist',
    // Base URL for the site
    baseUrl: './',
    // Clean the output directory before building
    clean: true,
    // Use source maps
    sourcemap: true,
    // Ensure assets are correctly referenced
    metaUrlPath: '_snowpack'
  },
  // Disable optimization to avoid path conflicts
  optimize: {}
};