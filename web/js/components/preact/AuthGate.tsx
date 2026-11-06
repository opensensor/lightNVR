import type { ComponentChildren } from 'preact';
import { useEffect, useState, useRef } from 'preact/hooks';

import { useI18n } from '../../i18n.ts';
import {
  clearAuthState,
  redirectToLogin,
  validateSession,
} from '../../utils/auth-utils.ts';
import { changePassword } from '../../api/authentication.ts';
import { validateForcedPasswordChange } from './forcedPasswordChange.ts';

type SessionInfo = Awaited<ReturnType<typeof validateSession>>;

export function RequiredPasswordChange({ session }: { session: SessionInfo }) {
  const { t } = useI18n();
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmation, setConfirmation] = useState('');
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);
  const requestRef = useRef<AbortController | null>(null);
  useEffect(() => () => requestRef.current?.abort(), [session.id]);

  useEffect(() => {
    document.title = `${t('passwordChangeRequired.title')} - LightNVR`;
  }, [t]);

  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    if (requestRef.current && !requestRef.current.signal.aborted) return;
    const validationError = validateForcedPasswordChange(
      currentPassword,
      newPassword,
      confirmation,
    );
    if (validationError) {
      setError(t(`passwordChangeRequired.error.${validationError}`));
      return;
    }

    const controller = new AbortController();
    requestRef.current = controller;
    setSaving(true); setError('');
    try {
      if (session.id === undefined) throw new Error('Missing identity');
      await changePassword(session.id, {old_password: currentPassword, new_password: newPassword}, {signal: controller.signal});
      if (controller.signal.aborted) return;
      setCurrentPassword(''); setNewPassword(''); setConfirmation('');
      clearAuthState();
      window.location.assign('/login.html?password_changed=true');
    } catch {
      if (!controller.signal.aborted) setError(t('passwordChangeRequired.error.generic'));
    } finally {
      controller.abort();
      if (requestRef.current === controller) { requestRef.current = null; setSaving(false); }
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
              /* eslint-disable-line jsx-a11y/no-autofocus -- moves focus to the first field of a newly opened dialog or step */ autoFocus
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
              minLength={8}
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

export function AuthGate({ children }: { children?: ComponentChildren }) {
  const { t } = useI18n();
  const [session, setSession] = useState<SessionInfo | null>(null);
  const [unavailable, setUnavailable] = useState(false);
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    let active = true;
    const controller = new AbortController();
    setUnavailable(false);
    const cleared = () => {
      controller.abort();
      setSession(null);
      setUnavailable(false);
    };
    window.addEventListener('lightnvr:auth-cleared', cleared);
    validateSession({signal: controller.signal}).then((result) => {
      if (!active || controller.signal.aborted) return;
      if (result.status === 'invalid') {
        clearAuthState();
        redirectToLogin('session_expired');
        return;
      }
      if (!result.valid) {
        setUnavailable(true);
        return;
      }
      setSession(result);
    });
    return () => {
      active = false;
      controller.abort();
      window.removeEventListener('lightnvr:auth-cleared', cleared);
    };
  }, [attempt]);

  if (!session && unavailable) {
    return <section className="flex min-h-screen flex-col items-center justify-center gap-4" role="alert">
      <p>{t('auth.sessionVerificationUnavailable')}</p>
      <button type="button" className="btn btn-primary" onClick={() => setAttempt(value => value + 1)}>
        {t('common.retry')}
      </button>
    </section>;
  }

  if (!session) {
    return <div className="flex min-h-screen items-center justify-center">{t('common.loading')}</div>;
  }

  if (session.must_change_password && !session.demo_mode) {
    return <RequiredPasswordChange session={session} />;
  }

  return children;
}
