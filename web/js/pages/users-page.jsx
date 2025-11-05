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

// Render the UsersView component when the DOM is loaded
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
        <UsersView />
        <Footer />
      </QueryClientProvider>,
      container
    );
  }
});
