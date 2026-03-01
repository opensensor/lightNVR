/**
 * LightNVR Web Interface Live View Page
 * Entry point for the live view page with WebRTC/HLS support
 */

import { render } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { LiveView } from '../components/preact/LiveView.jsx';
import { WebRTCView } from '../components/preact/WebRTCView.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import { Header } from "../components/preact/Header.jsx";
import { Footer } from "../components/preact/Footer.jsx";
import { ToastContainer } from "../components/preact/ToastContainer.jsx";
import { setupSessionValidation } from '../utils/auth-utils.js';
import { SetupWizard } from '../components/preact/SetupWizard.jsx';

/**
 * Main App component that conditionally renders WebRTCView or LiveView
 * based on whether WebRTC is disabled in settings
 */
function App() {
    const [isWebRTCDisabled, setIsWebRTCDisabled] = useState(false);
    const [isLoading, setIsLoading] = useState(true);
    const [showWizard, setShowWizard] = useState(false);

    useEffect(() => {
        // Check setup wizard status and WebRTC settings in parallel
        async function init() {
            try {
                const [settingsRes, setupRes] = await Promise.all([
                    fetch('/api/settings'),
                    fetch('/api/setup/status'),
                ]);

                if (settingsRes.ok) {
                    const settings = await settingsRes.json();
                    if (settings.webrtc_disabled || settings.go2rtc_enabled === false) {
                        console.log('WebRTC is disabled' + (settings.go2rtc_enabled === false ? ' (go2rtc disabled)' : '') + ', using HLS view');
                        setIsWebRTCDisabled(true);
                        document.title = 'HLS View - LightNVR';
                    } else {
                        console.log('WebRTC is enabled, using WebRTC view');
                        setIsWebRTCDisabled(false);
                    }
                } else {
                    console.error('Failed to fetch settings:', settingsRes.status, settingsRes.statusText);
                }

                if (setupRes.ok) {
                    const setupData = await setupRes.json();
                    if (!setupData.complete) {
                        setShowWizard(true);
                    }
                }
            } catch (error) {
                console.error('Error during init:', error);
            } finally {
                setIsLoading(false);
            }
        }

        init();
    }, []);

    if (isLoading) {
        return <div className="loading">Loading...</div>;
    }

    return (
        <>
            {showWizard && <SetupWizard onClose={() => setShowWizard(false)} />}
            {isWebRTCDisabled ? <LiveView isWebRTCDisabled={true} /> : <WebRTCView />}
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
                <Header />
                <ToastContainer />
                <App />
                <Footer />
            </QueryClientProvider>,
            container
        );
    }
});
