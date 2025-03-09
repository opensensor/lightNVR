/**
 * LightNVR Web Interface JavaScript
 * Main entry point that loads all required modules
 */

// Load core functionality
document.write('<script src="/js/core.js"></script>');

// Load authentication module
document.write('<script src="/js/components/auth.js"></script>');

// Load UI components
document.write('<script src="/js/components/ui.js"></script>');

// Load stream management
document.write('<script src="/js/components/streams.js"></script>');

// Load video player
document.write('<script src="/js/components/video.js"></script>');

// Load recordings management
document.write('<script src="/js/components/recordings.js"></script>');

// Load settings management
document.write('<script src="/js/components/settings.js"></script>');

// Load system management
document.write('<script src="/js/components/system.js"></script>');

// Load page handlers
document.write('<script src="/js/pages/pages.js"></script>');

console.log('LightNVR Web Interface modules loaded');
