/**
 * LightNVR Web Interface LoginView Component
 * Preact component for the login page
 */

import { useState, useRef, useEffect } from 'preact/hooks';
import { useI18n } from '../../i18n.js';

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
  const [rememberDeviceEnabled, setRememberDeviceEnabled] = useState(false);
  const [trustedDeviceDays, setTrustedDeviceDays] = useState(30);
  const [rememberDevice, setRememberDevice] = useState(false);
  const abortControllerRef = useRef(null);
  const { t, locale } = useI18n();

  useEffect(() => {
    document.title = `${t('login.signIn')} - LightNVR`;
  }, [locale, t]);

  // Fetch login config to determine if force MFA is enabled
  useEffect(() => {
    async function fetchLoginConfig() {
      try {
        const response = await fetch('/api/auth/login/config');
        if (response.ok) {
          const data = await response.json();
          setForceMfaEnabled(data.force_mfa_on_login || false);
          setRememberDeviceEnabled(data.remember_device_enabled || false);
          setTrustedDeviceDays(data.trusted_device_days || 30);
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
        setErrorMessage(t('login.error.rateLimited'));
      } else {
        setErrorMessage(t('login.error.invalidUsernamePassword'));
      }
    } else if (urlParams.has('auth_required') && urlParams.has('logout')) {
      setErrorMessage(t('login.error.loggedOut'));
    } else if (urlParams.has('auth_required')) {
      const reason = urlParams.get('reason');
      if (reason === 'session_expired') {
        setErrorMessage(t('login.error.sessionExpired'));
      } else {
        setErrorMessage(t('login.error.authRequired'));
      }
    } else if (urlParams.has('logout')) {
      setErrorMessage(t('login.error.loggedOut'));
    } else {
      setErrorMessage('');
    }
  }, [locale, t]);

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
      setErrorMessage(t('login.error.enterUsernamePassword'));
      return;
    }

    // When force MFA is enabled and a TOTP code is provided, validate it's 6 digits
    if (forceMfaEnabled && forceMfaTotpCode && forceMfaTotpCode.length > 0 && forceMfaTotpCode.length !== 6) {
      setErrorMessage(t('login.error.codeSixDigits'));
      return;
    }

    // Abort any previous in-flight login request before starting a new one
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
    }

    setIsLoggingIn(true);
    setErrorMessage('');

    try {
      // Build login request body
      const loginBody = { username, password };
      if (forceMfaEnabled && forceMfaTotpCode) {
        loginBody.totp_code = forceMfaTotpCode;
      }
      if (rememberDeviceEnabled) {
        loginBody.remember_device = rememberDevice;
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
            'Content-Type': 'application/json'
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
          setErrorMessage(t('login.error.rateLimited'));
        } else {
          setErrorMessage(t('login.error.invalidCredentials'));
        }
        if (forceMfaEnabled) {
          setForceMfaTotpCode('');
        }
      }
    } catch (error) {
      console.error('Login error:', error);
      // Reset login state on error
      setIsLoggingIn(false);
      setErrorMessage(t('login.error.generic'));
    }
  };

  // Handle TOTP code submission
  const handleTotpSubmit = async (e) => {
    e.preventDefault();

    if (!totpCode || totpCode.length !== 6) {
      setErrorMessage(t('login.error.enterVerificationCode'));
      return;
    }

    setIsLoggingIn(true);
    setErrorMessage('');

    try {
      const response = await fetch('/api/auth/login/totp', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ totp_token: totpToken, code: totpCode, remember_device: rememberDevice }),
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
          setErrorMessage(t('login.error.mfaExpired'));
        } else {
          setErrorMessage(data.error || t('login.error.invalidVerificationCode'));
          setTotpCode('');
        }
      }
    } catch (error) {
      console.error('TOTP verification error:', error);
      setIsLoggingIn(false);
      setErrorMessage(t('login.error.generic'));
    }
  };

  // Determine notification message class (success or error) based on content
  const getNotificationClass = () => {
    const baseClass = "mb-4 p-3 rounded-lg ";

    // Check for success messages
    const isSuccess = (
      errorMessage === t('login.error.loggedOut')
    );

    return baseClass + (
      isSuccess
        ? 'badge-success'
        : 'badge-danger'
    );
  };

  const forceMfaCodeLength = forceMfaTotpCode.replace(/[^0-9]/g, '').length;
  const hasPartialForceMfaCode = forceMfaEnabled && forceMfaCodeLength > 0 && forceMfaCodeLength !== 6;
  const isPrimarySubmitDisabled = isLoggingIn || !username || !password || hasPartialForceMfaCode;

  return (
    <section id="login-page" className="page flex items-center justify-center min-h-screen">
      <div className="login-container w-full max-w-md p-6 bg-card text-card-foreground rounded-lg shadow-lg">
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold">LightNVR</h1>
          <p className="text-muted-foreground">{t('login.subtitle')}</p>
        </div>

        {errorMessage && (
          <div className={getNotificationClass()}>
            {errorMessage}
          </div>
        )}

        {!totpRequired ? (
          <form id="login-form" className="space-y-6" onSubmit={handleSubmit}>
            <div className="form-group">
              <label htmlFor="username" className="block text-sm font-medium mb-1">{t('login.username')}</label>
              <input
                  type="text"
                  id="username"
                  name="username"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  placeholder={t('login.usernamePlaceholder')}
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  required
                  autoComplete="username"
              />
            </div>
            <div className="form-group">
              <label htmlFor="password" className="block text-sm font-medium mb-1">{t('login.password')}</label>
              <input
                  type="password"
                  id="password"
                  name="password"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  placeholder={t('login.passwordPlaceholder')}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  autoComplete="current-password"
              />
            </div>
            {forceMfaEnabled && (
              <div className="form-group">
                <label htmlFor="totp-code-force" className="block text-sm font-medium mb-1">{t('login.verificationCode')}</label>
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
                />
                <span className="hint text-sm text-muted-foreground block mt-1">{t('login.verificationHint')}</span>
              </div>
            )}
            {rememberDeviceEnabled && (
              <label className="flex items-center gap-2 text-sm text-muted-foreground">
                <input
                  type="checkbox"
                  checked={rememberDevice}
                  onChange={(e) => setRememberDevice(e.target.checked)}
                  disabled={isLoggingIn}
                />
                <span>{t('login.rememberDevice', { days: trustedDeviceDays })}</span>
              </label>
            )}
            <div className="form-group">
              <button
                  type="submit"
                  className="btn-primary w-full focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                  disabled={isPrimarySubmitDisabled}
              >
                {isLoggingIn ? t('login.signingIn') : t('login.signIn')}
              </button>
            </div>
          </form>
        ) : (
          <form id="totp-form" className="space-y-6" onSubmit={handleTotpSubmit}>
            <div className="text-center mb-4">
              <p className="text-sm text-muted-foreground">
                {t('login.enterAuthenticatorCode')}
              </p>
            </div>
            <div className="form-group">
              <label htmlFor="totp-code" className="block text-sm font-medium mb-1">{t('login.verificationCode')}</label>
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
            {rememberDeviceEnabled && (
              <label className="flex items-center gap-2 text-sm text-muted-foreground">
                <input
                  type="checkbox"
                  checked={rememberDevice}
                  onChange={(e) => setRememberDevice(e.target.checked)}
                  disabled={isLoggingIn}
                />
                <span>{t('login.rememberDevice', { days: trustedDeviceDays })}</span>
              </label>
            )}
            <div className="form-group">
              <button
                  type="submit"
                  className="btn-primary w-full focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                  disabled={isLoggingIn || totpCode.length !== 6}
              >
                {isLoggingIn ? t('login.verifying') : t('login.verify')}
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
                {t('login.backToLogin')}
              </button>
            </div>
          </form>
        )}

        {!totpRequired && (
          <div className="mt-6 text-center text-sm text-muted-foreground">
            <p>{t('login.defaultCredentials', { username: 'admin', password: 'admin' })}</p>
            <p className="mt-2">{t('login.changePasswordHint')}</p>
          </div>
        )}
      </div>
    </section>
  );
}
