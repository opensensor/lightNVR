/**
 * LightNVR Web Interface Streams Page
 * Entry point for the streams page
 */

import { render } from 'preact';
import { StreamsView } from '../components/preact/StreamsView.jsx';
import { QueryClientProvider, queryClient } from '../query-client.js';
import {Header} from "../components/preact/Header.jsx";
import {Footer} from "../components/preact/Footer.jsx";
import { ToastContainer } from "../components/preact/ToastContainer.jsx";

// Render the StreamsView component when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  // Get the container element
  const container = document.getElementById('main-content');

  if (container) {
    render(
      <QueryClientProvider client={queryClient}>
        <Header />
        <ToastContainer />
        <StreamsView />
        <Footer />
      </QueryClientProvider>,
      container
    );
  }
});
