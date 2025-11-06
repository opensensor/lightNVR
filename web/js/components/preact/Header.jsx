/**
 * LightNVR Web Interface Header Component
 * Preact component for the site header
 */

import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import {VERSION} from '../../version.js';

/**
 * Header component
 * @param {Object} props - Component props
 * @param {string} props.version - System version
 * @returns {JSX.Element} Header component
 */
export function Header({ version = VERSION }) {
  // Get active navigation from data attribute on header container
  const headerContainer = document.getElementById('header-container');
  const activeNav = headerContainer?.dataset?.activeNav || '';
  const [username, setUsername] = useState('');
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  // Get the current username from localStorage
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

  // Force navigation function to bypass React cleanup issues
  const forceNavigation = (href, e) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }

    // Use setTimeout with 0 delay to move this to the end of the event queue
    // This helps ensure the navigation happens after any other event handlers
    setTimeout(() => {
      window.location.href = href;
    }, 0);

    return false; // Prevent default behavior
  };

  // Render navigation item
  const renderNavItem = (item) => {
    const isActive = activeNav === item.id;
    const baseClasses = "no-underline rounded transition-colors";
    const desktopClasses = "px-3 py-2";
    const mobileClasses = "block w-full px-4 py-3 text-left";
    const activeClass = isActive ? 'bg-[hsl(var(--primary))] text-[hsl(var(--primary-foreground))]' : 'text-[hsl(var(--card-foreground))] hover:bg-[hsl(var(--primary)/0.8)] hover:text-[hsl(var(--primary-foreground))]';

    return (
        <li className={mobileMenuOpen ? "w-full" : "mx-1"}>
          <a
              href={item.href}
              id={item.id}
              className={`${baseClasses} ${mobileMenuOpen ? mobileClasses : desktopClasses} ${activeClass}`}
              onClick={(e) => {
                // Force navigation and prevent default behavior
                forceNavigation(item.href, e);

                // Close mobile menu if open
                if (mobileMenuOpen) {
                  toggleMobileMenu();
                }
              }}
          >
            {item.label}
          </a>
        </li>
    );
  };

  return (
      <header className="py-2 shadow-md mb-4 w-full" style={{ position: 'relative', zIndex: 20, backgroundColor: 'hsl(var(--card))', color: 'hsl(var(--card-foreground))' }}>
        <div className="container mx-auto px-4 flex justify-between items-center">
          <div className="logo flex items-center">
            <h1 className="text-xl font-bold m-0">LightNVR</h1>
            <span className="version text-xs ml-2" style={{color: 'hsl(var(--muted-foreground))'}}>v{version}</span>
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden md:block" style={{ position: 'relative', zIndex: 20 }}>
            <ul className="flex list-none m-0 p-0">
              {navItems.map(renderNavItem)}
            </ul>
          </nav>

          {/* User Menu (Desktop) */}
          <div className="user-menu hidden md:flex items-center">
            <span className="mr-2">{username}</span>
            <a
              href="/logout"
              className="logout-link no-underline px-3 py-1 rounded transition-colors"
              style={{
                color: 'hsl(var(--card-foreground))',
                backgroundColor: 'transparent'
              }}
              onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
              onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
            >
              Logout
            </a>
          </div>

          {/* Mobile Menu Button */}
          <button
              className="md:hidden p-2 focus:outline-none"
              style={{color: 'hsl(var(--card-foreground))'}}
              onClick={toggleMobileMenu}
              aria-label="Toggle menu"
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d={mobileMenuOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"} />
            </svg>
          </button>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
            <div className="md:hidden mt-2 border-t pt-2 container mx-auto px-4" style={{borderColor: 'hsl(var(--border))'}}>
              <ul className="list-none m-0 p-0 flex flex-col w-full">
                {navItems.map(renderNavItem)}
                <li className="w-full mt-2 pt-2 border-t" style={{borderColor: 'hsl(var(--border))'}}>
                  <div className="flex justify-between items-center px-4 py-2">
                    <span>{username}</span>
                    <a
                      href="/logout"
                      className="logout-link no-underline px-3 py-1 rounded transition-colors"
                      style={{
                        color: 'hsl(var(--card-foreground))',
                        backgroundColor: 'transparent'
                      }}
                      onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.8)'}
                      onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                    >
                      Logout
                    </a>
                  </div>
                </li>
              </ul>
            </div>
        )}
      </header>
  );
}