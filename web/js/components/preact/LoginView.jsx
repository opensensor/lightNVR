/**
 * LightNVR Web Interface LoginView Component
 * Preact component for the login page
 */

import { useState, useRef, useEffect } from 'preact/hooks';

/**
 * Returns a safe same-origin redirect path from a candidate URL string.
 *
 * Uses the browser's URL constructor to parse the candidate, then checks
 * that the resolved origin matches the current page's origin.  Only the
 * pathname + search + hash components are returned – never any user-supplied
 * host or scheme – which prevents both open-redirect and javascript: XSS.
 *
 * @param {string|null} url  Raw value from the `redirect` query parameter
 * @returns {string}  A safe relative path, defaulting to '/index.html'
 */
function safeRedirectPath(url) {
  if (!url || typeof url !== 'string') return '/index.html';
  try {
    // Resolve against the current origin so relative paths work too.
    const parsed = new URL(url, window.location.origin);
    // Reject anything that resolves to a different origin (open-redirect)
    // or a non-http(s) scheme (e.g. javascript:).
    if (parsed.origin !== window.location.origin) return '/index.html';
    // Return only the path components – never the (potentially attacker-
    // supplied) host or scheme.
    return (parsed.pathname + parsed.search + parsed.hash) || '/index.html';
  } catch (_) {
    return '/index.html';
  }
}

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
  const [forceMfaEnabled, setForceMfaEnabled] = useState(false);
  const [forceMfaTotpCode, setForceMfaTotpCode] = useState('');
  const abortControllerRef = useRef(null);

  // Fetch login config to determine if force MFA is enabled
  useEffect(() => {
    async function fetchLoginConfig() {
      try {
        const response = await fetch('/api/auth/login/config');
        if (response.ok) {
          const data = await response.json();
          setForceMfaEnabled(data.force_mfa_on_login || false);
        }
      } catch (error) {
        console.warn('Failed to fetch login config:', error);
      }
    }
    fetchLoginConfig();
  }, []);

  // Check URL for error, auth_required, or logout parameter
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.has('error')) {
      const errorType = urlParams.get('error');
      if (errorType === 'rate_limited') {
        setErrorMessage('Too many login attempts. Please try again later.');
      } else {
        setErrorMessage('Invalid username or password');
      }
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

  // Cancel any in-flight login request when the component unmounts
  useEffect(() => {
    return () => {
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    };
  }, []);

  // Handle login form submission
  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!username || !password) {
      setErrorMessage('Please enter both username and password');
      return;
    }

    // When force MFA is enabled, require TOTP code in the same step
    if (forceMfaEnabled && (!forceMfaTotpCode || forceMfaTotpCode.length !== 6)) {
      setErrorMessage('Please enter your 6-digit verification code');
      return;
    }

    // Abort any previous in-flight login request before starting a new one
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }

    setIsLoggingIn(true);
    setErrorMessage('');

    try {
      const authString = btoa(`${username}:${password}`);

      // Build login request body
      const loginBody = { username, password };
      if (forceMfaEnabled && forceMfaTotpCode) {
        loginBody.totp_code = forceMfaTotpCode;
      }

      // Make login request with an explicit timeout using AbortController
      const controller = new AbortController();
      abortControllerRef.current = controller;
      const timeoutId = setTimeout(() => controller.abort(), 10000);

      let response;
      try {
        response = await fetch('/api/auth/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Basic ${authString}`
          },
          body: JSON.stringify(loginBody),
          signal: controller.signal
        });
      } finally {
        clearTimeout(timeoutId);
      }

      if (response.ok) {
        const data = await response.json();

        // Check if TOTP verification is required (two-step flow, only when force MFA is off)
        if (data.totp_required && data.totp_token) {
          setTotpRequired(true);
          setTotpToken(data.totp_token);
          setIsLoggingIn(false);
          setErrorMessage('');
          return;
        }

        // Successful login (no TOTP required or force MFA verified)
        console.log('Login successful, proceeding to redirect');

        // Redirect to the requested page, or the index if none / unsafe.
        const urlParams = new URLSearchParams(window.location.search);
        window.location.href = safeRedirectPath(urlParams.get('redirect'));
      } else {
        // Failed login
        setIsLoggingIn(false);
        if (response.status === 429) {
          setErrorMessage('Too many login attempts. Please try again later.');
        } else {
          setErrorMessage('Invalid credentials');
        }
        if (forceMfaEnabled) {
          setForceMfaTotpCode('');
        }
      }
    } catch (error) {
      console.error('Login error:', error);
      // Reset login state on error
      setIsLoggingIn(false);
      setErrorMessage('An error occurred during login. Please try again.');
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

        const redirectParams = new URLSearchParams(window.location.search);
        window.location.href = safeRedirectPath(redirectParams.get('redirect'));
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

  // Determine notification message class (success or error) based on content
  const getNotificationClass = () => {
    const baseClass = "mb-4 p-3 rounded-lg ";

    // Check for success messages
    const isSuccess = (
      errorMessage.includes('successfully logged out')
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
          <div className={getNotificationClass()}>
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
            {forceMfaEnabled && (
              <div className="form-group">
                <label htmlFor="totp-code-force" className="block text-sm font-medium mb-1">Verification Code</label>
                <input
                    type="text"
                    id="totp-code-force"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white text-center text-2xl tracking-widest"
                    placeholder="000000"
                    value={forceMfaTotpCode}
                    onChange={(e) => setForceMfaTotpCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                    maxLength="6"
                    pattern="[0-9]{6}"
                    autoComplete="one-time-code"
                    required
                />
                <span className="hint text-sm text-muted-foreground block mt-1">Enter the 6-digit code from your authenticator app</span>
              </div>
            )}
            <div className="form-group">
              <button
                  type="submit"
                  className="btn-primary w-full focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                  disabled={isLoggingIn || (forceMfaEnabled && forceMfaTotpCode.length !== 6)}
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
