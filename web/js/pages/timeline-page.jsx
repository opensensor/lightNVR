/**
 * LightNVR Web Interface Users Page
 * Entry point for the users page
 */

import { render } from 'preact';
import { TimelinePage } from '../components/preact/timeline/TimelinePage.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import {Header} from "../components/preact/Header.jsx";
import {Footer} from "../components/preact/Footer.jsx";
import { ToastContainer } from "../components/preact/ToastContainer.jsx";

// Render the UsersView component when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    // Get the container element
    const container = document.getElementById('main-content');

    if (container) {
        render(
            <QueryClientProvider client={queryClient}>
                <Header />
                <ToastContainer />
                <TimelinePage />
                <Footer />
            </QueryClientProvider>,
            container
        );
    }
});
