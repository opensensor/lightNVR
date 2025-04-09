/**
 * LightNVR Timeline View Component
 * Loads the Timeline page component into the main content area
 */

import { render } from 'preact';
import { html } from '../../../html-helper.js';
import { TimelinePage } from './TimelinePage.js';
import { QueryClientProvider, queryClient } from '../../../query-client.js';

/**
 * Load TimelineView component
 */
export function loadTimelineView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;

  // Clear any existing content
  mainContent.innerHTML = '';

  // Render the TimelinePage component to the container wrapped with QueryClientProvider
  render(
    html`<${QueryClientProvider} client=${queryClient}>
      <${TimelinePage} />
    <//>`,
    mainContent
  );
}