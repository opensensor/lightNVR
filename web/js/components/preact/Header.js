/**
 * LightNVR Web Interface Header Component
 * Preact component for the site header
 */

import { h } from '../../preact.min.js';
import { useState } from '../../preact.hooks.module.js';
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
  const [username, setUsername] = useState('Admin');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  
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
  
  // Toggle mobile menu
  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen);
  };
  
  // Navigation items
  const navItems = [
    { id: 'nav-live', href: 'index.html', label: 'Live View' },
    { id: 'nav-recordings', href: 'recordings.html', label: 'Recordings' },
    { id: 'nav-streams', href: 'streams.html', label: 'Streams' },
    { id: 'nav-settings', href: 'settings.html', label: 'Settings' },
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
          href="${item.href}?t=${new Date().getTime()}" 
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
    <header class="bg-gray-800 text-white py-2 px-4 shadow-md mb-4">
      <div class="flex justify-between items-center">
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
          <span class="mr-4">${username}</span>
          <a href="#" onClick=${handleLogout} class="text-white no-underline hover:underline">Logout</a>
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
        <div class="md:hidden mt-2 border-t border-gray-700 pt-2">
          <ul class="list-none m-0 p-0 flex flex-col">
            ${navItems.map(renderNavItem)}
            <li class="w-full mt-2 pt-2 border-t border-gray-700">
              <div class="flex justify-between items-center px-4 py-2">
                <span>${username}</span>
                <a href="#" onClick=${handleLogout} class="text-white no-underline hover:underline">Logout</a>
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
  
  // Fetch system version and render the Header component
  fetchSystemVersion().then(version => {
    import('../../preact.min.js').then(({ render }) => {
      render(html`<${Header} activeNav=${activeNav} version=${version} />`, headerContainer);
    });
  });
}
