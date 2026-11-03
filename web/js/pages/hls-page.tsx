import '../sentry.ts';
/**
 * LightNVR Web Interface Live View Page
 * Entry point for the live view page with WebRTC/HLS support
 */

import { render } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { LiveView } from '../components/preact/LiveView.tsx';
import { ToastContainer } from "../components/preact/ToastContainer.tsx";
import { QueryClientProvider, queryClient } from '../query-client.ts';
import { Header } from "../components/preact/Header.tsx";
import { Footer } from "../components/preact/Footer.tsx";
import { setupSessionValidation } from '../utils/auth-utils.ts';
import { initI18n } from '../i18n.ts';
import { AuthGate } from '../components/preact/AuthGate.tsx';
import type { ClientConfig } from '../types/core.ts';

/**
 * Main App component. Always renders the HLS variant of the LiveView.
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
                const settings: ClientConfig = await response.json();
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
                variant="hls"
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
