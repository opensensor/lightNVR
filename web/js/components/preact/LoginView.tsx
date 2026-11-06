/**
 * LightNVR Web Interface LoginView Component
 * Preact component for the login page
 */

import { useState, useRef, useEffect } from 'preact/hooks';
import { useI18n } from '../../i18n.ts';
import LanguageSelector from './common/LanguageSelector.tsx';

import { login, completeTotpLogin, getLoginConfig, logout, AuthenticationRequestError } from '../../api/authentication.ts';
import type { LoginRequest } from '../../api/authentication.ts';

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
function safeRedirectPath(url: string | null): string {
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
  const [configState, setConfigState] = useState<'loading' | 'ready' | 'unavailable'>('loading');
  const [configAttempt, setConfigAttempt] = useState(0);
  const abortControllerRef = useRef<AbortController>(null);
  const staleSessionLogoutRef = useRef<Promise<unknown>>(null);
  const { t, locale } = useI18n();

  useEffect(() => {
    document.title = `${t('login.signIn')} - LightNVR`;
  }, [locale, t]);

  // Clear any stale server-side session on login page load.
  // The session cookie is HttpOnly so JavaScript cannot clear it directly;
  // calling the logout endpoint lets the server expire it via Set-Cookie.
  // The promise is kept so login submission can wait for it: the logout
  // response carries "Set-Cookie: session=; Max-Age=0", and if it arrives
  // after the login response it deletes the freshly issued session cookie,
  // bouncing the user straight back to the login page.
  useEffect(() => {
    staleSessionLogoutRef.current =
      logout()
        .catch(() => {})
        .finally(() => { staleSessionLogoutRef.current = null; });
  }, []);

  // A missing/malformed configuration is unavailable, never a guessed MFA policy.
  useEffect(() => {
    const controller = new AbortController();
    setConfigState('loading');
    getLoginConfig({signal: controller.signal}).then(data => {
      if (controller.signal.aborted) return;
      setForceMfaEnabled(data.force_mfa_on_login);
      setRememberDeviceEnabled(data.remember_device_enabled);
      setTrustedDeviceDays(data.trusted_device_days);
      setConfigState('ready');
    }).catch(() => { if (!controller.signal.aborted) setConfigState('unavailable'); });
    return () => controller.abort();
  }, [configAttempt]);

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
    } else if (urlParams.has('password_changed')) {
      setErrorMessage(t('login.passwordChanged'));
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

  // A pending request owns the state until cancelled. Never retry credential mutations.
  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (isLoggingIn || configState !== 'ready') return;
    if (!username || !password) {
      setErrorMessage(t('login.error.enterUsernamePassword')); return;
    }
    if (forceMfaEnabled && forceMfaTotpCode && !/^[0-9]{6}$/.test(forceMfaTotpCode)) {
      setErrorMessage(t('login.error.codeSixDigits')); return;
    }
    abortControllerRef.current?.abort();
    const controller = new AbortController();
    abortControllerRef.current = controller;
    setIsLoggingIn(true); setErrorMessage('');
    try {
      // Do not let a late logout response clear the freshly issued login cookie.
      await staleSessionLogoutRef.current;
      if (controller.signal.aborted) return;
      const body: LoginRequest = {username, password};
      if (forceMfaEnabled && forceMfaTotpCode) body.totp_code = forceMfaTotpCode;
      if (rememberDeviceEnabled) body.remember_device = rememberDevice;
      const data = await login(body, {signal: controller.signal});
      if (controller.signal.aborted) return;
      setPassword(''); setForceMfaTotpCode('');
      if (data.kind === 'challenge') {
        setTotpRequired(true); setTotpToken(data.token); return;
      }
      const params = new URLSearchParams(window.location.search);
      window.location.replace(data.mustChangePassword ? '/index.html' : safeRedirectPath(params.get('redirect')));
    } catch (error: unknown) {
      if (controller.signal.aborted) return;
      setForceMfaTotpCode('');
      setErrorMessage(t(error instanceof AuthenticationRequestError && error.status === 429 ? 'login.error.rateLimited' :
        error instanceof AuthenticationRequestError && error.status === 401 ? 'login.error.invalidCredentials' : 'login.error.generic'));
    } finally {
      if (!controller.signal.aborted) setIsLoggingIn(false);
    }
  };

  const handleTotpSubmit = async (e: Event) => {
    e.preventDefault();
    if (isLoggingIn) return;
    if (!/^[0-9]{6}$/.test(totpCode)) {
      setErrorMessage(t('login.error.enterVerificationCode')); return;
    }
    abortControllerRef.current?.abort();
    const controller = new AbortController();
    abortControllerRef.current = controller;
    setIsLoggingIn(true); setErrorMessage('');
    try {
      await completeTotpLogin({totp_token: totpToken, code: totpCode, remember_device: rememberDevice}, {signal: controller.signal});
      if (controller.signal.aborted) return;
      setTotpToken(''); setTotpCode('');
      const params = new URLSearchParams(window.location.search);
      window.location.replace(safeRedirectPath(params.get('redirect')));
    } catch (error: unknown) {
      if (controller.signal.aborted) return;
      setTotpCode('');
      // Both expired challenges and invalid/replayed codes can return 401. Do
      // not classify by a server-provided string; Back to login restarts safely.
      setErrorMessage(t(error instanceof AuthenticationRequestError && error.status === 429 ? 'login.error.rateLimited' :
        error instanceof AuthenticationRequestError && error.status === 401 ? 'login.error.invalidVerificationCode' : 'login.error.generic'));
    } finally {
      if (!controller.signal.aborted) setIsLoggingIn(false);
    }
  };

  // Determine notification message class (success or error) based on content
  const getNotificationClass = () => {
    const baseClass = "mb-4 p-3 rounded-lg ";

    // Check for success messages
    const isSuccess = (
      errorMessage === t('login.error.loggedOut') ||
      errorMessage === t('login.passwordChanged')
    );

    return baseClass + (
      isSuccess
        ? 'badge-success'
        : 'badge-danger'
    );
  };

  const forceMfaCodeLength = forceMfaTotpCode.replace(/[^0-9]/g, '').length;
  const hasPartialForceMfaCode = forceMfaEnabled && forceMfaCodeLength > 0 && forceMfaCodeLength !== 6;
  const isPrimarySubmitDisabled = configState !== 'ready' || isLoggingIn || !username || !password || hasPartialForceMfaCode;

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

        {configState === 'unavailable' && <div role="alert" className="mb-4">
          <p>{t('login.error.generic')}</p>
          <button type="button" className="btn-secondary" onClick={() => setConfigAttempt(value => value + 1)}>{t('common.retry')}</button>
        </div>}
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
                  onChange={(e) => setUsername((e.target as HTMLInputElement).value)}
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
                  onChange={(e) => setPassword((e.target as HTMLInputElement).value)}
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
                    onChange={(e) => setForceMfaTotpCode((e.target as HTMLInputElement).value.replace(/[^0-9]/g, '').slice(0, 6))}
                    maxLength={6}
                    pattern="[0-9]{6}"
                    autoComplete="one-time-code"
                />
                <span className="hint text-sm text-muted-foreground block mt-1">{t('login.verificationHint')}</span>
              </div>
            )}
            {rememberDeviceEnabled && (
              <label className="touch-target flex cursor-pointer items-center gap-2 text-sm text-muted-foreground">
                <input
                  type="checkbox"
                  checked={rememberDevice}
                  onChange={(e) => setRememberDevice((e.target as HTMLInputElement).checked)}
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
                  onChange={(e) => setTotpCode((e.target as HTMLInputElement).value.replace(/[^0-9]/g, '').slice(0, 6))}
                  maxLength={6}
                  pattern="[0-9]{6}"
                  autoComplete="one-time-code"
                  /* eslint-disable-line jsx-a11y/no-autofocus -- moves focus to the first field of a newly opened dialog or step */ autoFocus
                  required
              />
            </div>
            {rememberDeviceEnabled && (
              <label className="touch-target flex cursor-pointer items-center gap-2 text-sm text-muted-foreground">
                <input
                  type="checkbox"
                  checked={rememberDevice}
                  onChange={(e) => setRememberDevice((e.target as HTMLInputElement).checked)}
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
                    abortControllerRef.current?.abort();
                    setIsLoggingIn(false);
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

        <ul className="list-none flex justify-center mt-4 p-0">
          <LanguageSelector/>
        </ul>
      </div>
    </section>
  );
}
