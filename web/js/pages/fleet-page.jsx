/**
 * LightNVR Fleet Page
 * Server-paginated camera inventory and organization workspace.
 */

import { render } from 'preact';
import { QueryClientProvider, queryClient } from '../query-client.js';
import { FleetView } from '../components/preact/FleetView.jsx';
import { Header } from '../components/preact/Header.jsx';
import { Footer } from '../components/preact/Footer.jsx';
import { ToastContainer } from '../components/preact/ToastContainer.jsx';
import { setupSessionValidation } from '../utils/auth-utils.js';
import { initI18n } from '../i18n.js';

document.addEventListener('DOMContentLoaded', async () => {
  await initI18n();
  setupSessionValidation();

  const container = document.getElementById('main-content');
  if (container) {
    render(
      <QueryClientProvider client={queryClient}>
        <Header />
        <ToastContainer />
        <FleetView />
        <Footer />
      </QueryClientProvider>,
      container
    );
  }
});
