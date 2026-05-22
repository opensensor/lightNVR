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
import { initI18n } from '../i18n.js';

/**
 * Main App component that conditionally renders WebRTCView or LiveView
 * based on whether WebRTC is disabled in settings
 */
function App() {
    const [viewFlags, setViewFlags] = useState({
        webrtcDisabled: false,
        hlsDisabled: false,
        mseDisabled: false,
    });
    const [isLoading, setIsLoading] = useState(true);
    const [showWizard, setShowWizard] = useState(false);

    useEffect(() => {
        // Check setup wizard status and view-method settings in parallel.
        async function init() {
            try {
                const [settingsRes, setupRes] = await Promise.all([
                    fetch('/api/settings'),
                    fetch('/api/setup/status'),
                ]);

                if (settingsRes.ok) {
                    const settings = await settingsRes.json();
                    // WebRTC requires go2rtc. If go2rtc is off, treat WebRTC
                    // and MSE as unavailable regardless of the user's flags.
                    // #397
                    const go2rtcOff = settings.go2rtcEnabled === false || settings.go2rtc_enabled === false;
                    
                    setViewFlags({
                        // Membaca versi camelCase agar sinkron dengan AuthTab
                        webrtcDisabled: settings.webrtcDisabled !== undefined ? !!settings.webrtcDisabled || go2rtcOff : !!settings.webrtc_disabled || go2rtcOff,
                        hlsDisabled:    settings.hlsDisabled !== undefined ? !!settings.hlsDisabled : !!settings.hls_disabled,
                        mseDisabled:    settings.mseDisabled !== undefined ? !!settings.mseDisabled || go2rtcOff : !!settings.mse_disabled || go2rtcOff,
                    });
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

    // Choose the initial view: prefer WebRTC, fall back to whichever of
    // HLS / MSE remain. The LiveView component itself handles HLS↔MSE
    // tab switching and will hide tabs for disabled methods. #397
    const useWebRTC = !viewFlags.webrtcDisabled;
    if (!useWebRTC) {
        document.title = 'HLS View - LightNVR';
    } else {
        document.title = 'Live View - LightNVR';
    }

    return (
        <>
            {showWizard && <SetupWizard onClose={() => setShowWizard(false)} />}
            {useWebRTC
                ? <WebRTCView 
                    isWebRTCDisabled={viewFlags.webrtcDisabled}
                    isHlsDisabled={viewFlags.hlsDisabled}
                    isMseDisabled={viewFlags.mseDisabled}
                  />
                : <LiveView
                    isWebRTCDisabled={viewFlags.webrtcDisabled}
                    isHlsDisabled={viewFlags.hlsDisabled}
                    isMseDisabled={viewFlags.mseDisabled}
                  />}
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
                <Header />
                <ToastContainer />
                <App />
                <Footer />
            </QueryClientProvider>,
            container
        );
    }
});
