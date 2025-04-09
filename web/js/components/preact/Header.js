/**
 * LightNVR Web Interface Header Component
 * Preact component for the site header
 */

import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { html } from '../../html-helper.js';
import { fetchSystemVersion } from './utils.js';

/**
 * Header component
 * @param {Object} props - Component props
 * @param {string} props.activeNav - ID of the active navigation item
 * @param {string} props.version - System version
 * @returns {JSX.Element} Header component
 */
export function Header({ activeNav = '', version = '' }) {
  const [username, setUsername] = useState('');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  // Get the currently logged in username from localStorage
  useEffect(() => {
    const auth = localStorage.getItem('auth');
    if (auth) {
      try {
        // Decode the base64 auth string (username:password)
        const decoded = atob(auth);
        // Extract the username (everything before the colon)
        const extractedUsername = decoded.split(':')[0];
        setUsername(extractedUsername);
      } catch (error) {
        console.error('Error decoding auth token:', error);
        setUsername('User');
      }
    } else {
      setUsername('User');
    }
  }, []);

  // Handle logout
  const handleLogout = (e) => {
    e.preventDefault();

    // Clear localStorage
    localStorage.removeItem('auth');

    // Clear cookies
    document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
    document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";

    // Call the logout endpoint to clear browser's basic auth cache
    fetch('/api/auth/logout', {
      method: 'POST'
    }).then(() => {
      // For Chrome, we need to make a request with invalid credentials to clear the auth cache
      // Create an iframe to make a request with invalid credentials
      const iframe = document.createElement('iframe');
      iframe.style.display = 'none';
      document.body.appendChild(iframe);
      
      // Set a timeout to redirect after attempting to clear auth cache
      setTimeout(() => {
        // Redirect to login page with auth_required parameter to prevent auto-redirect
        window.location.href = 'login.html?auth_required=true&logout=true';
      }, 100);

      // Try to load a protected resource with invalid credentials in the iframe
      if (iframe.contentWindow) {
        iframe.contentWindow.location.href = '/api/auth/verify';
      }
    }).catch(() => {
      // Redirect even if the request fails
      window.location.href = 'login.html?auth_required=true&logout=true';
    });
  };

  // Toggle mobile menu
  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen);
  };

  // Special handling for Live View link to handle both index.html and root URL
  const getLiveViewHref = () => {
    // Check if we're on the root URL or index.html
    const isRoot = window.location.pathname === '/' || window.location.pathname.endsWith('/');
    
    // If we're on the root URL, stay on the root URL
    if (isRoot) {
      return './';
    }
    
    // Otherwise, default to index.html
    return 'index.html';
  };

  // Navigation items - don't preserve query parameters when navigating via header
  const navItems = [
    { id: 'nav-live', href: getLiveViewHref(), label: 'Live View' },
    { id: 'nav-recordings', href: 'recordings.html', label: 'Recordings' },
    { id: 'nav-streams', href: 'streams.html', label: 'Streams' },
    { id: 'nav-settings', href: 'settings.html', label: 'Settings' },
    { id: 'nav-users', href: 'users.html', label: 'Users' },
    { id: 'nav-system', href: 'system.html', label: 'System' }
  ];

  // Render navigation item
  const renderNavItem = (item) => {
    const isActive = activeNav === item.id;
    const baseClasses = "text-white no-underline rounded transition-colors";
    const desktopClasses = "px-3 py-2";
    const mobileClasses = "block w-full px-4 py-3 text-left";
    const activeClass = isActive ? 'bg-blue-600' : 'hover:bg-blue-700';

    return html`
      <li class=${mobileMenuOpen ? "w-full" : "mx-1"}>
        <a
          href="${item.href}"
          id=${item.id}
          class=${`${baseClasses} ${mobileMenuOpen ? mobileClasses : desktopClasses} ${activeClass}`}
          onClick=${mobileMenuOpen ? toggleMobileMenu : null}
        >
          ${item.label}
        </a>
      </li>
    `;
  };

  return html`
    <header class="bg-gray-800 text-white py-2 shadow-md mb-4 w-full">
      <div class="container mx-auto px-4 flex justify-between items-center">
        <div class="logo flex items-center">
          <h1 class="text-xl font-bold m-0">LightNVR</h1>
          <span class="version text-blue-200 text-xs ml-2">v${version}</span>
        </div>

        <!-- Desktop Navigation -->
        <nav class="hidden md:block">
          <ul class="flex list-none m-0 p-0">
            ${navItems.map(renderNavItem)}
          </ul>
        </nav>

        <!-- User Menu (Desktop) -->
        <div class="user-menu hidden md:flex items-center">
          <span class="mr-2">${username}</span>
          <a href="#" onClick=${handleLogout} class="text-white no-underline hover:bg-blue-700 px-3 py-1 rounded transition-colors">Logout</a>
        </div>

        <!-- Mobile Menu Button -->
        <button
          class="md:hidden text-white p-2 focus:outline-none"
          onClick=${toggleMobileMenu}
          aria-label="Toggle menu"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d=${mobileMenuOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"} />
          </svg>
        </button>
      </div>

      <!-- Mobile Navigation -->
      ${mobileMenuOpen ? html`
        <div class="md:hidden mt-2 border-t border-gray-700 pt-2 container mx-auto px-4">
          <ul class="list-none m-0 p-0 flex flex-col w-full">
            ${navItems.map(renderNavItem)}
            <li class="w-full mt-2 pt-2 border-t border-gray-700">
              <div class="flex justify-between items-center px-4 py-2">
                <span>${username}</span>
                <a href="#" onClick=${handleLogout} class="text-white no-underline hover:bg-blue-700 px-3 py-1 rounded transition-colors">Logout</a>
              </div>
            </li>
          </ul>
        </div>
      ` : null}
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

  // Get the version
  const version = fetchSystemVersion();

  // Render the header with version
  import('preact').then(({ render }) => {
    render(html`<${Header} activeNav=${activeNav} version=${version} />`, headerContainer);
  }).catch(error => {
    console.error('Error importing Preact:', error);
  });
}
