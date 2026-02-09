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
  const [totpRequired, setTotpRequired] = useState(false);
  const [totpCode, setTotpCode] = useState('');
  const [totpToken, setTotpToken] = useState('');

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
        const data = await response.json();

        // Check if TOTP verification is required
        if (data.totp_required && data.totp_token) {
          setTotpRequired(true);
          setTotpToken(data.totp_token);
          setIsLoggingIn(false);
          setErrorMessage('');
          return;
        }

        // Successful login (no TOTP required)
        console.log('Login successful, proceeding to redirect');

        // Get redirect URL from query parameter if it exists
        const urlParams = new URLSearchParams(window.location.search);
        const redirectUrl = urlParams.get('redirect');
        const targetUrl = redirectUrl || '/index.html';

        // Redirect immediately
        window.location.href = targetUrl;
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

  // Handle TOTP code submission
  const handleTotpSubmit = async (e) => {
    e.preventDefault();

    if (!totpCode || totpCode.length !== 6) {
      setErrorMessage('Please enter a 6-digit verification code');
      return;
    }

    setIsLoggingIn(true);
    setErrorMessage('');

    try {
      const response = await fetch('/api/auth/login/totp', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ totp_token: totpToken, code: totpCode }),
      });

      if (response.ok) {
        console.log('TOTP verification successful, proceeding to redirect');

        const urlParams = new URLSearchParams(window.location.search);
        const redirectUrl = urlParams.get('redirect');
        const targetUrl = redirectUrl || '/index.html';

        window.location.href = targetUrl;
      } else {
        const data = await response.json();
        setIsLoggingIn(false);

        if (response.status === 401 && data.error && data.error.includes('expired')) {
          // Token expired, go back to password step
          setTotpRequired(false);
          setTotpToken('');
          setTotpCode('');
          setErrorMessage('MFA session expired. Please login again.');
        } else {
          setErrorMessage(data.error || 'Invalid verification code');
          setTotpCode('');
        }
      }
    } catch (error) {
      console.error('TOTP verification error:', error);
      setIsLoggingIn(false);
      setErrorMessage('An error occurred during verification. Please try again.');
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

        {!totpRequired ? (
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
        ) : (
          <form id="totp-form" className="space-y-6" onSubmit={handleTotpSubmit}>
            <div className="text-center mb-4">
              <p className="text-sm text-muted-foreground">
                Enter the 6-digit code from your authenticator app
              </p>
            </div>
            <div className="form-group">
              <label htmlFor="totp-code" className="block text-sm font-medium mb-1">Verification Code</label>
              <input
                  type="text"
                  id="totp-code"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white text-center text-2xl tracking-widest"
                  placeholder="000000"
                  value={totpCode}
                  onChange={(e) => setTotpCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                  maxLength="6"
                  pattern="[0-9]{6}"
                  autoComplete="one-time-code"
                  autoFocus
                  required
              />
            </div>
            <div className="form-group">
              <button
                  type="submit"
                  className="btn-primary w-full focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                  disabled={isLoggingIn || totpCode.length !== 6}
              >
                {isLoggingIn ? 'Verifying...' : 'Verify'}
              </button>
            </div>
            <div className="form-group text-center">
              <button
                  type="button"
                  className="text-sm text-muted-foreground hover:underline"
                  onClick={() => {
                    setTotpRequired(false);
                    setTotpToken('');
                    setTotpCode('');
                    setErrorMessage('');
                  }}
              >
                ← Back to login
              </button>
            </div>
          </form>
        )}

        {!totpRequired && (
          <div className="mt-6 text-center text-sm text-muted-foreground">
            <p>Default credentials: admin / admin</p>
            <p className="mt-2">You can change this password in Users after login</p>
          </div>
        )}
      </div>
    </section>
  );
}
