/**
 * LightNVR Web Interface Footer Component
 * Preact component for the site footer
 */

import { h } from '../../preact.min.js';
import { useState } from '../../preact.hooks.module.js';
import { html } from '../../html-helper.js';

/**
 * Footer component
 * @param {Object} props - Component props
 * @returns {JSX.Element} Footer component
 */
export function Footer() {
  const [year] = useState(new Date().getFullYear());

  return html`
    <footer class="bg-gray-800 text-white py-3 px-4 mt-4 shadow-inner">
      <div class="container mx-auto flex flex-col sm:flex-row justify-between items-center">
        <div class="text-center sm:text-left mb-2 sm:mb-0">
          <p class="text-sm">© ${year} LightNVR ·
            <a href="https://github.com/opensensor/lightnvr" class="text-blue-300 hover:text-blue-100 text-sm" target="_blank" rel="noopener noreferrer">GitHub</a>
          </p>
          <p class="text-xs text-gray-400 mt-1 hidden sm:block">Lightweight Network Video Recorder</p>
        </div>
      </div>
    </footer>
  `;
}

/**
 * Load footer
 */
export function loadFooter() {
  const footerContainer = document.getElementById('footer-container');
  if (!footerContainer) return;

  // First render the footer with an empty version to ensure it appears immediately
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${Footer} />`, footerContainer);

    // Then fetch the version and update the footer
    fetchSystemVersion()
      .then(version => {
        if (version) {
          render(html`<${Footer} />`, footerContainer);
        }
      })
      .catch(error => {
        console.error('Error loading system version for footer:', error);
        // Footer is already rendered with empty version, so no need to re-render
      });
  }).catch(error => {
    console.error('Error importing Preact for footer:', error);
  });
}
