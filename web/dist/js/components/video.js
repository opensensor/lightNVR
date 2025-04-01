/**
 * LightNVR Web Interface Video Module
 * Main entry point for video functionality
 */

// Import all video modules
document.addEventListener('DOMContentLoaded', function() {
    // Load all required scripts in order
    const scripts = [
        'video-utils.js',
        'video-detection.js',
        'video-player.js',
        'video-layout.js',
        'video-recordings.js'
    ];
    
    // Function to load a script
    function loadScript(src) {
        return new Promise((resolve, reject) => {
            const script = document.createElement('script');
            script.src = `js/components/${src}`;
            script.onload = () => resolve();
            script.onerror = () => reject(new Error(`Failed to load script: ${src}`));
            document.head.appendChild(script);
        });
    }
    
    // Load scripts sequentially
    async function loadScripts() {
        for (const script of scripts) {
            try {
                await loadScript(script);
                console.log(`Loaded ${script}`);
            } catch (error) {
                console.error(error);
            }
        }
        
        // Initialize any global video functionality here
        console.log('All video modules loaded');
    }
    
    // Start loading scripts
    loadScripts();
});
