/**
 * LightNVR Web Interface Login Page
 * Entry point for the login page
 */

import { render } from 'preact';
import { LoginView } from '../components/preact/LoginView.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import { ToastContainer } from "../components/preact/ToastContainer.jsx";

// Render the LoginView component when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
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
