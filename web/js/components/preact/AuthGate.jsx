import { useEffect, useState } from 'preact/hooks';

import { useI18n } from '../../i18n.js';
import {
  clearAuthState,
  redirectToLogin,
  validateSession,
} from '../../utils/auth-utils.js';
import { validateForcedPasswordChange } from './forcedPasswordChange.js';

export function RequiredPasswordChange({ session }) {
  const { t } = useI18n();
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmation, setConfirmation] = useState('');
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    document.title = `${t('passwordChangeRequired.title')} - LightNVR`;
  }, [t]);

  const handleSubmit = async (event) => {
    event.preventDefault();
    const validationError = validateForcedPasswordChange(
      currentPassword,
      newPassword,
      confirmation,
    );
    if (validationError) {
      setError(t(`passwordChangeRequired.error.${validationError}`));
      return;
    }

    setSaving(true);
    setError('');
    try {
      const response = await fetch(`/api/auth/users/${session.id}/password`, {
        method: 'PUT',
        credentials: 'same-origin',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          old_password: currentPassword,
          new_password: newPassword,
        }),
      });

      if (!response.ok) {
        const payload = await response.json().catch(() => ({}));
        throw new Error(payload.error || t('passwordChangeRequired.error.generic'));
      }

      clearAuthState();
      window.location.assign('/login.html?password_changed=true');
    } catch (requestError) {
      setError(requestError.message || t('passwordChangeRequired.error.generic'));
      setSaving(false);
    }
  };

  return (
    <section className="flex min-h-screen items-center justify-center px-4 py-10">
      <div className="w-full max-w-md rounded-lg bg-card p-6 text-card-foreground shadow-lg">
        <div className="mb-6 text-center">
          <div className="mb-3 text-4xl" aria-hidden="true">🔐</div>
          <h1 className="text-2xl font-bold">{t('passwordChangeRequired.title')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('passwordChangeRequired.description')}
          </p>
        </div>

        {error && (
          <div className="badge-danger mb-4 rounded-lg p-3" role="alert">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="mb-1 block text-sm font-medium" htmlFor="required-current-password">
              {t('passwordChangeRequired.currentPassword')}
            </label>
            <input
              id="required-current-password"
              type="password"
              autoComplete="current-password"
              className="w-full rounded border border-input bg-background p-2 text-foreground"
              value={currentPassword}
              onInput={(event) => setCurrentPassword(event.currentTarget.value)}
              disabled={saving}
              autoFocus
            />
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium" htmlFor="required-new-password">
              {t('passwordChangeRequired.newPassword')}
            </label>
            <input
              id="required-new-password"
              type="password"
              autoComplete="new-password"
              minLength="8"
              className="w-full rounded border border-input bg-background p-2 text-foreground"
              value={newPassword}
              onInput={(event) => setNewPassword(event.currentTarget.value)}
              disabled={saving}
            />
            <p className="mt-1 text-xs text-muted-foreground">
              {t('passwordChangeRequired.passwordHelp')}
            </p>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium" htmlFor="required-confirm-password">
              {t('passwordChangeRequired.confirmPassword')}
            </label>
            <input
              id="required-confirm-password"
              type="password"
              autoComplete="new-password"
              className="w-full rounded border border-input bg-background p-2 text-foreground"
              value={confirmation}
              onInput={(event) => setConfirmation(event.currentTarget.value)}
              disabled={saving}
            />
          </div>

          <button
            type="submit"
            className="btn btn-primary w-full"
            disabled={saving}
          >
            {saving
              ? t('passwordChangeRequired.saving')
              : t('passwordChangeRequired.submit')}
          </button>
        </form>
      </div>
    </section>
  );
}

export function AuthGate({ children }) {
  const { t } = useI18n();
  const [session, setSession] = useState(null);

  useEffect(() => {
    let active = true;
    validateSession().then((result) => {
      if (!active) return;
      if (!result.valid) {
        clearAuthState();
        redirectToLogin('session_expired');
        return;
      }
      setSession(result);
    });
    return () => { active = false; };
  }, []);

  if (!session) {
    return <div className="flex min-h-screen items-center justify-center">{t('common.loading')}</div>;
  }

  if (session.must_change_password && !session.demo_mode) {
    return <RequiredPasswordChange session={session} />;
  }

  return children;
}
