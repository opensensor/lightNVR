/**
 * LightNVR Web Interface WebRTC Check
 * Checks if WebRTC is disabled and redirects to HLS view if needed
 */

// Function to check if WebRTC is disabled and redirect if needed
async function checkWebRTCDisabled() {
    try {
        // Only check on index.html or root path
        const currentPath = window.location.pathname;
        if (currentPath !== '/' && currentPath !== '/index.html') {
            return;
        }

        // Fetch settings to check if WebRTC is disabled
        const response = await fetch('/api/settings');
        if (!response.ok) {
            console.error('Failed to fetch settings:', response.status, response.statusText);
            return;
        }

        const settings = await response.json();
        
        // If WebRTC is disabled, redirect to HLS view
        if (settings.webrtc_disabled) {
            console.log('WebRTC is disabled, redirecting to HLS view');
            
            // Preserve query parameters
            const queryParams = window.location.search;
            window.location.href = '/hls.html' + queryParams;
        }
    } catch (error) {
        console.error('Error checking WebRTC status:', error);
    }
}

// Run the check when the page loads
document.addEventListener('DOMContentLoaded', checkWebRTCDisabled);
