/**
 * LightNVR Web Interface LoginView Component
 * Preact component for the login page
 */

import { useState, useRef, useEffect } from 'preact/hooks';

/**
 * LoginView component
 * @returns {JSX.Element} LoginView component
 */
export function LoginView() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');
  const redirectTimerRef = useRef(null);

  // Check URL for error, auth_required, or logout parameter
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('error')) {
      setErrorMessage('Invalid username or password');
    } else if (urlParams.has('auth_required') && urlParams.has('logout')) {
      setErrorMessage('You have been successfully logged out.');
    } else if (urlParams.has('auth_required')) {
      const reason = urlParams.get('reason');
      if (reason === 'session_expired') {
        setErrorMessage('Your session has expired. Please log in again.');
      } else {
        setErrorMessage('Authentication required. Please log in to continue.');
      }
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

  // Function to check if browser might be blocking redirects
  const checkBrowserRedirectSupport = () => {
    // Check if running in a sandboxed iframe which might block navigation
    const isSandboxed = window !== window.top;

    // Check if there are any service workers that might intercept navigation
    const hasServiceWorker = 'serviceWorker' in navigator;

    // Log potential issues
    if (isSandboxed) {
      console.warn('Login page is running in an iframe, which might block navigation');
    }

    if (hasServiceWorker) {
      console.log('Service Worker API is available, checking for active service workers');
      navigator.serviceWorker.getRegistrations().then(registrations => {
        if (registrations.length > 0) {
          console.warn(`${registrations.length} service worker(s) detected which might intercept navigation`);
        } else {
          console.log('No active service workers detected');
        }
      });
    }

    return { isSandboxed, hasServiceWorker };
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

      if (response.ok) {
        // Successful login
        console.log('Login successful, proceeding to redirect');

        // Try multiple redirect approaches
        try {
          // Get redirect URL from query parameter if it exists
          const urlParams = new URLSearchParams(window.location.search);
          const redirectUrl = urlParams.get('redirect');

          // Add timestamp to prevent caching issues
          const targetUrl = redirectUrl
              ? redirectUrl
              : `/index.html`;


          // Set a fallback timer in case the redirect doesn't happen immediately
          redirectTimerRef.current = setTimeout(() => {
            console.log('Fallback: trying window.location.href');
            window.location.href = targetUrl;

            // If that also doesn't work, try replace
            redirectTimerRef.current = setTimeout(() => {
              console.log('Fallback: trying window.location.replace');
              window.location.replace(targetUrl);
            }, 500);
          }, 500);
        } catch (redirectError) {
          console.error('Redirect error:', redirectError);
          // Last resort: try a different approach
          window.location.replace('index.html');
        }
      } else {
        // Failed login
        setIsLoggingIn(false);
        setErrorMessage('Invalid username or password');
        localStorage.removeItem('auth');
      }
    } catch (error) {
      console.error('Login error:', error);
      // Reset login state on error
      setIsLoggingIn(false);
      setErrorMessage('An error occurred during login. Please try again.');
      localStorage.removeItem('auth');
    }
  };

  // Determine error message class based on content
  const getErrorMessageClass = () => {
    const baseClass = "mb-4 p-3 rounded-lg ";

    // Check for success messages
    const isSuccess = (
      errorMessage.includes('successfully logged out') ||
      errorMessage.includes('Click the button below') ||
      errorMessage.includes('Login successful')
    );

    return baseClass + (
      isSuccess
        ? 'badge-success'
        : 'badge-danger'
    );
  };

  return (
    <section id="login-page" className="page flex items-center justify-center min-h-screen">
      <div className="login-container w-full max-w-md p-6 bg-card text-card-foreground rounded-lg shadow-lg">
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold">LightNVR</h1>
          <p className="text-muted-foreground">Please sign in to continue</p>
        </div>

        {errorMessage && (
          <div className={getErrorMessageClass()}>
            {errorMessage}
          </div>
        )}

        <form id="login-form" className="space-y-6" action="/api/auth/login" method="POST" onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="username" className="block text-sm font-medium mb-1">Username</label>
            <input
                type="text"
                id="username"
                name="username"
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                placeholder="Enter your username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                autoComplete="username"
            />
          </div>
          <div className="form-group">
            <label htmlFor="password" className="block text-sm font-medium mb-1">Password</label>
            <input
                type="password"
                id="password"
                name="password"
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                placeholder="Enter your password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                autoComplete="current-password"
            />
          </div>
          <div className="form-group">
            <button
                type="submit"
                className="btn-primary w-full focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                disabled={isLoggingIn}
            >
              {isLoggingIn ? 'Signing in...' : 'Sign In'}
            </button>
          </div>
        </form>

        <div className="mt-6 text-center text-sm text-muted-foreground">
          <p>Default credentials: admin / admin</p>
          <p className="mt-2">You can change these in Settings after login</p>
        </div>
      </div>
    </section>
  );
}
