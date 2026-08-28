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
import { initI18n } from '../i18n.js';
import { AuthGate } from '../components/preact/AuthGate.jsx';

/**
 * Main App component that conditionally renders WebRTCView or LiveView
 * based on whether WebRTC is disabled in settings
 */
function App() {
    const [viewFlags, setViewFlags] = useState({
        audioDisabled: false,
        autoDisabled: false,
        webrtcDisabled: false,
        hlsDisabled: false,
        mseDisabled: false,
    });
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        // Fetch the three view-method flags (#397). go2rtc being off
        // forces WebRTC+MSE off regardless of the user's configuration.
        async function loadViewFlags() {
            try {
                const response = await fetch('/api/client-config');
                if (!response.ok) {
                    console.error('Failed to fetch settings:', response.status, response.statusText);
                    setIsLoading(false);
                    return;
                }
                const settings = await response.json();
                const go2rtcOff = settings.go2rtc_enabled === false;
                setViewFlags({
                    audioDisabled:  !!settings.audio_disabled,
                    autoDisabled:   !!settings.auto_disabled,
                    webrtcDisabled: !!settings.webrtc_disabled || go2rtcOff,
                    hlsDisabled:    !!settings.hls_disabled,
                    mseDisabled:    !!settings.mse_disabled  || go2rtcOff,
                });
            } catch (error) {
                console.error('Error loading view-method flags:', error);
            } finally {
                setIsLoading(false);
            }
        }

        loadViewFlags();
    }, []);

    if (isLoading) {
        return <div className="loading">Loading...</div>;
    }

    return (
        <>
            <Header />
            <ToastContainer />
            <LiveView
                audioDisabled={viewFlags.audioDisabled}
                isAutoDisabled={viewFlags.autoDisabled}
                isWebRTCDisabled={viewFlags.webrtcDisabled}
                isHlsDisabled={viewFlags.hlsDisabled}
                isMseDisabled={viewFlags.mseDisabled}
            />
            <Footer />
        </>
    );
}

// Render the App component when the DOM is loaded
document.addEventListener('DOMContentLoaded', async () => {
    await initI18n();
    // Setup session validation (checks every 5 minutes)
    setupSessionValidation();

    // Get the container element
    const container = document.getElementById('main-content');

    if (container) {
        render(
            <QueryClientProvider client={queryClient}>
                <AuthGate>
                    <App />
                </AuthGate>
            </QueryClientProvider>,
            container
        );
    }
});
