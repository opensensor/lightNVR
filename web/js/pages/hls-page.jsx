/**
 * LightNVR Web Interface Live View Page
 * Entry point for the live view page with WebRTC/HLS support
 */

import { render } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { LiveView } from '../components/preact/LiveView.jsx';
import { ToastContainer } from "../components/preact/ToastContainer.jsx";
import { QueryClientProvider, queryClient } from '../query-client.js';
import { Header } from "../components/preact/Header.jsx";
import { Footer } from "../components/preact/Footer.jsx";
import { setupSessionValidation } from '../utils/auth-utils.js';

/**
 * Main App component that conditionally renders WebRTCView or LiveView
 * based on whether WebRTC is disabled in settings
 */
function App() {
    const [isWebRTCDisabled, setIsWebRTCDisabled] = useState(false);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        // Check if WebRTC is disabled in settings
        async function checkWebRTCStatus() {
            try {
                const response = await fetch('/api/settings');
                if (!response.ok) {
                    console.error('Failed to fetch settings:', response.status, response.statusText);
                    setIsLoading(false);
                    return;
                }

                const settings = await response.json();

                if (settings.webrtc_disabled) {
                    console.log('WebRTC is disabled, using HLS view');
                    setIsWebRTCDisabled(true);
                } else {
                    console.log('WebRTC is enabled, using WebRTC view');
                    setIsWebRTCDisabled(false);
                }
            } catch (error) {
                console.error('Error checking WebRTC status:', error);
            } finally {
                setIsLoading(false);
            }
        }

        checkWebRTCStatus();
    }, []);

    if (isLoading) {
        return <div className="loading">Loading...</div>;
    }

    return (
        <>
            <Header />
            <ToastContainer />
            <LiveView isWebRTCDisabled={isWebRTCDisabled} />
            <Footer />
        </>
    );
}

// Render the App component when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    // Setup session validation (checks every 5 minutes)
    setupSessionValidation();

    // Get the container element
    const container = document.getElementById('main-content');

    if (container) {
        render(
            <QueryClientProvider client={queryClient}>
                <App />
            </QueryClientProvider>,
            container
        );
    }
});
