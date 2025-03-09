/**
 * LightNVR Web Interface Debug Functionality
 * Contains functionality for debugging and development
 */

/**
 * Load debug information
 */
function loadDebugInfo() {
    const debugInfo = document.getElementById('debug-info');
    if (!debugInfo) return;

    debugInfo.textContent = 'Loading debug information...';

    // Simulate API call with delay
    setTimeout(() => {
        debugInfo.textContent = `Environment: Development
Node.js Version: 18.15.0
Database: SQLite 3.36.0
FFmpeg Version: 4.4.2
OS: Linux 5.15.0-52-generic x86_64
Memory Usage: 78.4 MB
Uptime: 1d 4h 23m`;
    }, 500);
}
