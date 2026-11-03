// @ts-nocheck
import '../sentry.ts';
/**
 * LightNVR Web Interface Login Page
 * Entry point for the login page
 */

import { render } from 'preact';
import { LoginView } from '../components/preact/LoginView.tsx';
import { QueryClientProvider, queryClient } from '../query-client.ts';
import { ToastContainer } from "../components/preact/ToastContainer.tsx";
import { initI18n } from '../i18n.ts';

// Render the LoginView component when the DOM is loaded
document.addEventListener('DOMContentLoaded', async () => {
  await initI18n();
  // Get the container element
  const container = document.getElementById('main-content');

  if (container) {
    // Render the LoginView component
    render(
      <QueryClientProvider client={queryClient}>
        <ToastContainer />
        <LoginView />
      </QueryClientProvider>,
      container
    );
  }
});
