import { render } from 'preact';

import { InvestigationView } from '../components/preact/investigation/InvestigationView.jsx';
import { Footer } from '../components/preact/Footer.jsx';
import { Header } from '../components/preact/Header.jsx';
import { ToastContainer } from '../components/preact/ToastContainer.jsx';
import { initI18n } from '../i18n.js';
import { QueryClientProvider, queryClient } from '../query-client.js';
import { setupSessionValidation } from '../utils/auth-utils.js';

document.addEventListener('DOMContentLoaded', async () => {
  await initI18n();
  setupSessionValidation();
  const container = document.getElementById('main-content');
  if (!container) return;
  render(
    <QueryClientProvider client={queryClient}>
      <Header />
      <ToastContainer />
      <InvestigationView />
      <Footer />
    </QueryClientProvider>,
    container,
  );
});
