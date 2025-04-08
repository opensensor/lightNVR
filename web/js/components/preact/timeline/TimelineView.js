/**
 * LightNVR Timeline View Component
 * Loads the Timeline page component into the main content area
 */

import { render } from 'preact';
import { html } from '../../../html-helper.js';
import { TimelinePage } from './TimelinePage.js';

/**
 * Load TimelineView component
 */
export function loadTimelineView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;

  // Clear any existing content
  mainContent.innerHTML = '';

  // Render the TimelinePage component to the container
  render(html`<${TimelinePage} />`, mainContent);
}