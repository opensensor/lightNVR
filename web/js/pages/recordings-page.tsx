import '../sentry.ts';
/**
 * LightNVR Web Interface Streams Page
 * Entry point for the streams page
 */

import { render } from 'preact';
import { RecordingsView } from '../components/preact/RecordingsView.tsx';
import { QueryClientProvider, queryClient } from '../query-client.ts';
import { Header } from "../components/preact/Header.tsx";
import { Footer } from "../components/preact/Footer.tsx";
import { ToastContainer } from "../components/preact/ToastContainer.tsx";
import { BatchDeleteModal } from "../components/preact/BatchDeleteModal.tsx";
import { ModalProvider } from "../components/preact/UI.tsx";
import { setupSessionValidation } from '../utils/auth-utils.ts';
import { initI18n } from '../i18n.ts';
import { AuthGate } from '../components/preact/AuthGate.tsx';

// Render the StreamsView component when the DOM is loaded
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
                    <ModalProvider>
                        <Header />
                        <ToastContainer />
                        <BatchDeleteModal />
                        <RecordingsView />
                        <Footer />
                    </ModalProvider>
                </AuthGate>
            </QueryClientProvider>,
            container
        );
    }
});
