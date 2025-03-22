/**
 * LightNVR Web Interface Footer Component
 * Preact component for the site footer
 */

import { h } from '../../preact.min.js';
import { useState, useEffect } from '../../preact.hooks.module.js';
import { html } from '../../preact-app.js';

/**
 * Footer component
 * @returns {JSX.Element} Footer component
 */
export function Footer() {
  const [year] = useState(new Date().getFullYear());
  const [version, setVersion] = useState('');
  
  // Fetch system version on mount
  useEffect(() => {
    fetchSystemVersion();
  }, []);
  
  // Fetch system version from API
  async function fetchSystemVersion() {
    try {
      const response = await fetch('/api/system');
      if (!response.ok) {
        throw new Error('Failed to load system information');
      }
      
      const data = await response.json();
      if (data.version) {
        setVersion(data.version);
      }
    } catch (error) {
      console.error('Error loading system version:', error);
    }
  }
  
  return html`
    <footer class="bg-gray-800 text-white py-3 px-4 flex justify-between items-center text-sm mt-4">
      <div>© ${year} LightNVR</div>
      <div>Version ${version}</div>
    </footer>
  `;
}

/**
 * Load footer
 */
export function loadFooter() {
  const footerContainer = document.getElementById('footer-container');
  if (!footerContainer) return;
  
  // Render the Footer component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${Footer} />`, footerContainer);
  });
}
