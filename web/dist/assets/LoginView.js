/**
 * LightNVR Web Interface LoginView Component
 * Preact component for the login page
 */

import { h } from 'preact';
import { html } from '../../html-helper.js';
import { useState, useRef, useEffect } from 'preact/hooks';
import { showStatusMessage } from './UI.js';
import { enhancedFetch, createRequestController } from '../../fetch-utils.js';

/**
 * LoginView component
 * @returns {JSX.Element} LoginView component
 */
export function LoginView() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');
  const [redirectAttempts, setRedirectAttempts] = useState(0);
  const redirectTimerRef = useRef(null);

  // Check URL for error, auth_required, or logout parameter
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('error')) {
      setErrorMessage('Invalid username or password');
    } else if (urlParams.has('auth_required') && urlParams.has('logout')) {
      setErrorMessage('You have been successfully logged out.');
    } else if (urlParams.has('auth_required')) {
      setErrorMessage('Authentication required. Please log in to continue.');
    } else if (urlParams.has('logout')) {
      setErrorMessage('You have been successfully logged out.');
    }
  }, []);

  // Request controller for cancelling requests
  const requestControllerRef = useRef(null);

  // Cleanup function for any timers
  useEffect(() => {
    return () => {
      if (redirectTimerRef.current) {
        clearTimeout(redirectTimerRef.current);
      }
    };
  }, []);

  // Handle successful login with improved reliability
  const handleSuccessfulLogin = () => {
    // Get redirect URL from query parameter if it exists
    const urlParams = new URLSearchParams(window.location.search);
    const redirectUrl = urlParams.get('redirect');

    // Add timestamp to prevent caching issues
    const timestamp = new Date().getTime();
    const targetUrl = redirectUrl
        ? `${redirectUrl}${redirectUrl.includes('?') ? '&' : '?'}t=${timestamp}`
        : `/index.html?t=${timestamp}`;

    console.log(`Login successful, redirecting to: ${targetUrl}`);

    // First, try using window.location.href
    try {
      window.location.href = targetUrl;
    } catch (error) {
      console.error('Error redirecting via location.href:', error);
    }

    // Set a backup timeout to check if redirection happened
    redirectTimerRef.current = setTimeout(() => {
      // Check if we're still on the login page
      if (window.location.pathname.includes('login.html')) {
        console.log('Still on login page, trying alternate redirection method');

        // Increment redirect attempts
        setRedirectAttempts(prev => {
          const newCount = prev + 1;

          // If we've tried multiple times, try different approaches
          if (newCount <= 3) {
            try {
              // Try window.location assign
              window.location.assign(targetUrl);

              // Also try reload as a fallback
              redirectTimerRef.current = setTimeout(() => {
                if (window.location.pathname.includes('login.html')) {
                  console.log('Still on login page after assign, trying location.replace');
                  window.location.replace(targetUrl);
                }
              }, 1000);
            } catch (error) {
              console.error('Error during alternate redirection:', error);

              // Last resort - show a manual redirect button
              if (newCount >= 3) {
                showStatusMessage('Please click the "Go to Dashboard" button to continue', 'info', 10000);
                setErrorMessage('Login successful! Click the button below to continue.');

                // Create a redirect button (this will be handled by showing the message in the UI)
                const redirectButton = document.createElement('button');
                redirectButton.textContent = 'Go to Dashboard';
                redirectButton.className = 'w-full px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors';
                redirectButton.onclick = () => {
                  window.location = targetUrl;
                };

                // Find the login form and append the button
                const loginForm = document.getElementById('login-form');
                if (loginForm && loginForm.parentNode) {
                  loginForm.parentNode.appendChild(redirectButton);
                }
              }
            }
          }
          return newCount;
        });
      }
    }, 500); // Check after 500ms if redirection worked
  };

  // Handle login form submission
  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!username || !password) {
      setErrorMessage('Please enter both username and password');
      return;
    }

    setIsLoggingIn(true);
    setErrorMessage('');

    try {
      // Store credentials in localStorage for future requests
      const authString = btoa(`${username}:${password}`);
      localStorage.setItem('auth', authString);

      // Make login request
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Basic ${authString}`
        },
        body: JSON.stringify({ username, password }),
        timeout: 10000
      });

      if (response.ok || response.status === 302) {
        // Successful login
        console.log('Login successful, proceeding to redirect');
        // Set a flag to indicate authenticated status
        sessionStorage.setItem('auth_confirmed', 'true');
        handleSuccessfulLogin();
      } else {
        // Failed login
        setIsLoggingIn(false);
        setErrorMessage('Invalid username or password');
        localStorage.removeItem('auth');
      }
    } catch (error) {
      console.error('Login error:', error);

      // If it's a timeout error, proceed anyway with stored credentials
      if (error.message === 'Request timed out' && localStorage.getItem('auth')) {
        console.log('Login request timed out, proceeding with stored credentials');
        handleSuccessfulLogin();
      }
      // For other errors, also try to proceed if we have credentials
      else if (localStorage.getItem('auth')) {
        console.log('Login API error, but proceeding with stored credentials');
        handleSuccessfulLogin();
      } else {
        setIsLoggingIn(false);
        setErrorMessage('Login failed. Please try again.');
      }
    }
  };

  return html`
    <section id="login-page" class="page flex items-center justify-center min-h-screen">
      <div class="login-container w-full max-w-md p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg">
        <div class="text-center mb-8">
          <h1 class="text-2xl font-bold">LightNVR</h1>
          <p class="text-gray-600 dark:text-gray-400">Please sign in to continue</p>
        </div>

        ${errorMessage && html`
          <div class=${`mb-4 p-3 rounded-lg ${
              errorMessage.includes('successfully logged out') || errorMessage.includes('Click the button below')
                  ? 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200'
                  : 'bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200'
          }`}>
            ${errorMessage}
          </div>
        `}

        <form id="login-form" class="space-y-6" action="/api/auth/login" method="POST" onSubmit=${handleSubmit}>
          <div class="form-group">
            <label for="username" class="block text-sm font-medium mb-1">Username</label>
            <input
                type="text"
                id="username"
                name="username"
                class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                placeholder="Enter your username"
                value=${username}
                onChange=${e => setUsername(e.target.value)}
                required
                autocomplete="username"
            />
          </div>
          <div class="form-group">
            <label for="password" class="block text-sm font-medium mb-1">Password</label>
            <input
                type="password"
                id="password"
                name="password"
                class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                placeholder="Enter your password"
                value=${password}
                onChange=${e => setPassword(e.target.value)}
                required
                autocomplete="current-password"
            />
          </div>
          <div class="form-group">
            <button
                type="submit"
                class="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                disabled=${isLoggingIn}
            >
              ${isLoggingIn ? 'Signing in...' : 'Sign In'}
            </button>
          </div>
        </form>

        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <p>Default credentials: admin / admin</p>
          <p class="mt-2">You can change these in Settings after login</p>
        </div>
      </div>
    </section>
  `;
}

/**
 * Load LoginView component
 */
export function loadLoginView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;

  // Render the LoginView component to the container
  import('preact').then(({ render }) => {
    render(html`<${LoginView} />`, mainContent);
  });
}