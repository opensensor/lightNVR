/**
 * LightNVR Web Interface Motion Recording Page
 * Entry point for the motion recording page
 */

import { render } from 'preact';
import { MotionView } from '../components/preact/MotionView.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import { Header } from "../components/preact/Header.jsx";
import { Footer } from "../components/preact/Footer.jsx";
import { ToastContainer } from "../components/preact/ToastContainer.jsx";

// Render the MotionView component when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  // Get the container element
  const container = document.getElementById('main-content');

  if (container) {
    render(
      <QueryClientProvider client={queryClient}>
        <Header />
        <ToastContainer />
        <MotionView />
        <Footer />
      </QueryClientProvider>,
      container
    );
  }
});

