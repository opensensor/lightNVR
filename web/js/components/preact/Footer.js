/**
 * LightNVR Web Interface Footer Component
 * Preact component for the site footer
 */

import { h } from '../../preact.min.js';
import { useState } from '../../preact.hooks.module.js';
import { html } from '../../html-helper.js';
import { fetchSystemVersion } from './utils.js';

/**
 * Footer component
 * @param {Object} props - Component props
 * @param {string} props.version - System version
 * @returns {JSX.Element} Footer component
 */
export function Footer({ version = '' }) {
  const [year] = useState(new Date().getFullYear());
  
  return html`
    <footer class="bg-gray-800 text-white py-4 px-4 mt-4 shadow-inner">
      <div class="container mx-auto flex flex-col sm:flex-row justify-between items-center">
        <div class="text-center sm:text-left mb-2 sm:mb-0">
          <p class="text-sm">© ${year} LightNVR</p>
          <p class="text-xs text-gray-400 mt-1 hidden sm:block">Lightweight Network Video Recorder</p>
        </div>
        <div class="flex flex-col sm:flex-row items-center">
          <span class="text-sm mb-2 sm:mb-0 sm:mr-4">Version ${version}</span>
          <div class="flex space-x-4">
            <a href="https://github.com/lightnvr/lightnvr" class="text-blue-300 hover:text-blue-100 text-sm" target="_blank" rel="noopener noreferrer">GitHub</a>
            <a href="docs/TROUBLESHOOTING.md" class="text-blue-300 hover:text-blue-100 text-sm" target="_blank">Help</a>
          </div>
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
  
  // Fetch system version and render the Footer component
  fetchSystemVersion().then(version => {
    import('../../preact.min.js').then(({ render }) => {
      render(html`<${Footer} version=${version} />`, footerContainer);
    });
  });
}
