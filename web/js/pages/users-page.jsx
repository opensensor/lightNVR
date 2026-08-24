/**
 * LightNVR Web Interface Users Page
 * Entry point for the users page
 */

import { render } from 'preact';
import { UsersView } from '../components/preact/UsersView.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import {Header} from "../components/preact/Header.jsx";
import {Footer} from "../components/preact/Footer.jsx";
import { ToastContainer } from "../components/preact/ToastContainer.jsx";
import { setupSessionValidation } from '../utils/auth-utils.js';
import { initI18n } from '../i18n.js';
import { AuthGate } from '../components/preact/AuthGate.jsx';

// Render the UsersView component when the DOM is loaded
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
          <Header />
          <ToastContainer />
          <UsersView />
          <Footer />
        </AuthGate>
      </QueryClientProvider>,
      container
    );
  }
});
