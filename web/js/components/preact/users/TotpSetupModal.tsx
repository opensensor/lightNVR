/**
 * TOTP MFA Setup Modal Component
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import QRCode from 'qrcode';
import { useI18n } from '../../../i18n.ts';
import { AsyncButton } from '../AsyncButton.tsx';
import type { User } from '../../../types/core.ts';
import type { GetAuthHeaders } from '../../../types/users.ts';
import { getTotpStatus, setupTotp, verifyTotp, disableTotp, AuthenticationRequestError } from '../../../api/authentication.ts';
import { useEscapeKey } from '../../../hooks/useEscapeKey.ts';

export interface TotpSetupModalProps {
  user: Pick<User, 'id' | 'username'>;
  onClose: () => void;
  onSuccess?: () => unknown;
  getAuthHeaders: GetAuthHeaders;
}

/**
 * TOTP Setup Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.user - User to configure TOTP for
 * @param {Function} props.onClose - Function to close the modal
 * @param {Function} props.onSuccess - Function called after TOTP is enabled/disabled
 * @param {Function} props.getAuthHeaders - Function to get auth headers
 * @returns {JSX.Element} TOTP setup modal
 */
export function TotpSetupModal({ user, onClose, onSuccess, getAuthHeaders }: TotpSetupModalProps) {
  useEscapeKey(() => close());
  const [step, setStep] = useState('loading'); // 'loading', 'status', 'setup', 'verify', 'enabled'
  const [otpauthUri, setOtpauthUri] = useState('');
  const [secret, setSecret] = useState('');
  const [verifyCode, setVerifyCode] = useState('');
  const [error, setError] = useState('');
  const [totpEnabled, setTotpEnabled] = useState(false);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const { t } = useI18n();

  const context = useRef<AbortController | null>(null);
  const pending = useRef(false);
  const stopPropagation = (e: Event) => e.stopPropagation();
  const clearEnrollment = () => { setSecret(''); setOtpauthUri(''); setVerifyCode(''); };
  const close = () => { context.current?.abort(); clearEnrollment(); onClose(); };

  const fetchStatus = useCallback(async () => {
    context.current?.abort();
    const controller = new AbortController();
    context.current = controller;
    pending.current = false;
    setSecret(''); setOtpauthUri(''); setVerifyCode(''); setError(''); setStep('loading');
    try {
      const data = await getTotpStatus(user.id, {headers: getAuthHeaders(), signal: controller.signal});
      if (controller.signal.aborted) return;
      setTotpEnabled(data.totp_enabled); setStep('status');
    } catch {
      if (!controller.signal.aborted) { setStep('unavailable'); setError(t('users.networkErrorDuringMfaSetup')); }
    }
  }, [user.id, getAuthHeaders, t]);

  useEffect(() => {
    void fetchStatus();
    return () => { context.current?.abort(); };
  }, [fetchStatus]);

  useEffect(() => {
    let active = true;
    if (otpauthUri && canvasRef.current) {
      QRCode.toCanvas(canvasRef.current, otpauthUri, {
        width: 256, margin: 2, color: { dark: '#000000', light: '#ffffff' }
      }).catch(() => { if (active) setError(t('users.mfaSetupFailed')); });
    }
    return () => { active = false; };
  }, [otpauthUri, step, t]);

  const mutate = async (action: 'setup' | 'verify' | 'disable') => {
    const controller = context.current;
    if (pending.current || !controller || controller.signal.aborted) return;
    pending.current = true; setError('');
    const options = {headers: getAuthHeaders(), signal: controller.signal};
    try {
      if (action === 'setup') {
        const data = await setupTotp(user.id, options);
        if (controller.signal.aborted) return;
        setOtpauthUri(data.otpauth_uri); setSecret(data.secret); setStep('setup');
      } else {
        if (action === 'verify') await verifyTotp(user.id, verifyCode, options);
        else await disableTotp(user.id, options);
        if (controller.signal.aborted) return;
        clearEnrollment(); setTotpEnabled(action === 'verify');
        setStep(action === 'verify' ? 'enabled' : 'status');
        onSuccess?.();
      }
    } catch (error: unknown) {
      if (controller.signal.aborted) return;
      setVerifyCode('');
      const status = error instanceof AuthenticationRequestError ? error.status : undefined;
      setError(t(status === 429 ? 'login.error.rateLimited' : status === 401 ? 'users.invalidVerificationCode' : 'users.mfaSetupFailed'));
      // An ambiguous mutation may have applied. Refresh the real state before
      // offering another mutation; never infer disabled/enabled from an error.
      if (status !== 401 && status !== 429) { clearEnrollment(); setStep('unavailable'); }
    } finally {
      if (context.current === controller) pending.current = false;
    }
  };
  const handleSetup = () => mutate('setup');
  const handleDisable = () => mutate('disable');
  const handleVerify = async (e: Event) => {
    e.preventDefault();
    if (!/^[0-9]{6}$/.test(verifyCode)) { setError(t('users.enterSixDigitCode')); return; }
    await mutate('verify');
  };

  return (
    <div role="presentation" className="app-modal-overlay fixed inset-0 bg-black/50 flex items-center justify-center " onClick={close}>
      <div role="presentation" className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">{t('users.mfaSetupNamed', { username: user.username })}</h2>

        {error && (
          <div className="mb-4 p-3 rounded-lg badge-danger">{error}</div>
        )}

        {step === 'loading' && (
          <div className="text-center py-4">{t('common.loading')}</div>
        )}

        {step === 'unavailable' && <AsyncButton className="btn-secondary w-full" onClick={fetchStatus}>{t('common.retry')}</AsyncButton>}

        {step === 'status' && !totpEnabled && (
          <div>
            <p className="mb-4 text-sm text-muted-foreground">
              {t('users.mfaDescription')}
            </p>
            {/* AsyncButton — locks + spins while the POST /totp/setup is in
                flight so the #399 rapid-tap double-submit can't fire twice
                and generate two secrets. */}
            <AsyncButton className="btn-primary w-full" onClick={handleSetup}>
              {t('users.setupTwoFactorAuthentication')}
            </AsyncButton>
          </div>
        )}

        {step === 'status' && totpEnabled && (
          <div>
            <div className="mb-4 p-3 rounded-lg badge-success">
              {t('users.twoFactorEnabled')}
            </div>
            {/* AsyncButton — pending-guarded disable prevents duplicate
                POST /totp/disable calls. */}
            <AsyncButton className="btn-danger w-full" onClick={handleDisable}>
              {t('users.disableTwoFactorAuthentication')}
            </AsyncButton>
          </div>
        )}

        {step === 'setup' && (
          <div>
            <p className="mb-3 text-sm">
              {t('users.scanQrCode')}
            </p>
            <div className="flex justify-center mb-3 bg-white p-2 rounded">
              <canvas ref={canvasRef}></canvas>
            </div>
            <div className="mb-4">
              <p className="text-xs text-muted-foreground mb-1">{t('users.enterKeyManually')}</p>
              <code className="block p-2 bg-muted text-foreground rounded text-sm font-mono break-all select-all">
                {secret}
              </code>
            </div>
            {/* Swapped the form's submit button to AsyncButton so the
                verify POST is pending-guarded (#399 / PRD UXD_01 §5.1 / T1).
                Keep the <form> for Enter-to-submit; onSubmit still routes
                through handleVerify, and AsyncButton's onClick funnels the
                same call. The button's own handleClick preventDefaults
                while pending so a second Enter press is dropped. */}
            <form onSubmit={handleVerify}>
              <label className="block text-sm font-medium mb-1">{t('users.enterVerificationCode')}</label>
              <input
                type="text"
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary text-center text-xl tracking-widest mb-3"
                placeholder="000000"
                value={verifyCode}
                onChange={(e) => setVerifyCode((e.target as HTMLInputElement).value.replace(/[^0-9]/g, '').slice(0, 6))}
                maxLength={6}
                /* eslint-disable-line jsx-a11y/no-autofocus -- moves focus to the first field of a newly opened dialog or step */ autoFocus
                required
              />
              <AsyncButton
                type="submit"
                className="btn-primary w-full"
                disabled={verifyCode.length !== 6}
                onClick={handleVerify}
              >
                {t('users.verifyAndEnable')}
              </AsyncButton>
            </form>
          </div>
        )}

        {step === 'enabled' && (
          <div>
            <div className="mb-4 p-3 rounded-lg badge-success">
              {t('users.twoFactorEnabledSuccess')}
            </div>
            <p className="mb-4 text-sm text-muted-foreground">
              {t('users.mfaNowRequired')}
            </p>
          </div>
        )}

        <div className="flex justify-end mt-4">
          <button className="btn-secondary" onClick={close}>
            {step === 'enabled' ? t('common.done') : t('common.close')}
          </button>
        </div>
      </div>
    </div>
  );
}

