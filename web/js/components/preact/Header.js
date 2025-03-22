/**
 * LightNVR Web Interface Header Component
 * Preact component for the site header
 */

import { h } from '../../preact.min.js';
import { useState, useEffect } from '../../preact.hooks.module.js';
import { html } from '../../preact-app.js';

/**
 * Header component
 * @param {Object} props - Component props
 * @param {string} props.activeNav - ID of the active navigation item
 * @returns {JSX.Element} Header component
 */
export function Header({ activeNav = '' }) {
  const [version, setVersion] = useState('');
  const [username, setUsername] = useState('Admin');
  
  // Fetch system version on mount
  useEffect(() => {
    fetchSystemVersion();
  }, []);
  
  // Handle logout
  const handleLogout = (e) => {
    e.preventDefault();
    
    // Clear localStorage
    localStorage.removeItem('auth');
    
    // Call the logout endpoint to clear browser's basic auth cache
    fetch('/api/auth/logout', {
      method: 'POST'
    }).then(() => {
      // Redirect to login page
      window.location.href = 'login.html?logout=true';
    }).catch(() => {
      // Redirect even if the request fails
      window.location.href = 'login.html?logout=true';
    });
  };
  
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
    <header class="bg-gray-800 text-white py-2 px-4 flex justify-between items-center shadow-md mb-4">
      <div class="logo">
        <h1 class="text-xl font-bold m-0">LightNVR</h1>
        <span class="version text-blue-200 text-xs ml-2">v${version}</span>
      </div>
      <nav>
        <ul class="flex list-none m-0 p-0">
          <li class="mx-1">
            <a 
              href="live.html" 
              id="nav-live" 
              class=${`text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-live' ? 'bg-blue-600' : 'hover:bg-blue-700'}`}
            >
              Live View
            </a>
          </li>
          <li class="mx-1">
            <a 
              href="recordings.html" 
              id="nav-recordings" 
              class=${`text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-recordings' ? 'bg-blue-600' : 'hover:bg-blue-700'}`}
            >
              Recordings
            </a>
          </li>
          <li class="mx-1">
            <a 
              href="streams.html" 
              id="nav-streams" 
              class=${`text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-streams' ? 'bg-blue-600' : 'hover:bg-blue-700'}`}
            >
              Streams
            </a>
          </li>
          <li class="mx-1">
            <a 
              href="settings.html" 
              id="nav-settings" 
              class=${`text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-settings' ? 'bg-blue-600' : 'hover:bg-blue-700'}`}
            >
              Settings
            </a>
          </li>
          <li class="mx-1">
            <a 
              href="system.html" 
              id="nav-system" 
              class=${`text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-system' ? 'bg-blue-600' : 'hover:bg-blue-700'}`}
            >
              System
            </a>
          </li>
        </ul>
      </nav>
      <div class="user-menu">
        <span class="mr-4">${username}</span>
        <a href="#" onClick=${handleLogout} class="text-white no-underline hover:underline">Logout</a>
      </div>
    </header>
  `;
}

/**
 * Load header with active navigation item
 * @param {string} activeNav - ID of the active navigation item
 */
export function loadHeader(activeNav = '') {
  const headerContainer = document.getElementById('header-container');
  if (!headerContainer) return;
  
  // Render the Header component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${Header} activeNav=${activeNav} />`, headerContainer);
  });
}
