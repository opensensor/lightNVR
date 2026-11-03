import { render } from 'preact';

import { PlatesView } from '../components/preact/lpr/PlatesView.tsx';
import { Footer } from '../components/preact/Footer.tsx';
import { Header } from '../components/preact/Header.tsx';
import { ToastContainer } from '../components/preact/ToastContainer.tsx';
import { initI18n } from '../i18n.ts';
import { QueryClientProvider, queryClient } from '../query-client.ts';
import { setupSessionValidation } from '../utils/auth-utils.ts';
import { AuthGate } from '../components/preact/AuthGate.tsx';

document.addEventListener('DOMContentLoaded', async () => {
  await initI18n();
  setupSessionValidation();
  const container = document.getElementById('main-content');
  if (!container) return;
  render(
    <QueryClientProvider client={queryClient}>
      <AuthGate>
        <Header />
        <ToastContainer />
        <PlatesView />
        <Footer />
      </AuthGate>
    </QueryClientProvider>,
    container,
  );
});
