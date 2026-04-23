/**
 * TOTP MFA Setup Modal Component
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import QRCode from 'qrcode';
import { useI18n } from '../../../i18n.js';
import { AsyncButton } from '../AsyncButton.jsx';

/**
 * TOTP Setup Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.user - User to configure TOTP for
 * @param {Function} props.onClose - Function to close the modal
 * @param {Function} props.onSuccess - Function called after TOTP is enabled/disabled
 * @param {Function} props.getAuthHeaders - Function to get auth headers
 * @returns {JSX.Element} TOTP setup modal
 */
export function TotpSetupModal({ user, onClose, onSuccess, getAuthHeaders }) {
  const [step, setStep] = useState('loading'); // 'loading', 'status', 'setup', 'verify', 'enabled'
  const [otpauthUri, setOtpauthUri] = useState('');
  const [secret, setSecret] = useState('');
  const [verifyCode, setVerifyCode] = useState('');
  const [error, setError] = useState('');
  const [totpEnabled, setTotpEnabled] = useState(false);
  const canvasRef = useRef(null);
  const { t } = useI18n();

  const stopPropagation = (e) => e.stopPropagation();

  const fetchStatus = useCallback(async () => {
    try {
      const res = await fetch(`/api/auth/users/${user.id}/totp/status`, {
        headers: getAuthHeaders()
      });
      if (res.ok) {
        const data = await res.json();
        setTotpEnabled(data.totp_enabled);
        setStep('status');
      } else {
        // TOTP columns may not exist yet
        setTotpEnabled(false);
        setStep('status');
      }
    } catch (err) {
      console.error('Error fetching TOTP status:', err);
      setStep('status');
    }
  }, [user.id, getAuthHeaders]);

  // Fetch current TOTP status on mount or when dependencies change
  useEffect(() => {
    fetchStatus();
  }, [fetchStatus]);

  // Render QR code when otpauthUri changes
  useEffect(() => {
    if (otpauthUri && canvasRef.current) {
      QRCode.toCanvas(canvasRef.current, otpauthUri, {
        width: 256,
        margin: 2,
        color: { dark: '#000000', light: '#ffffff' }
      }).catch(err => console.error('QR Code error:', err));
    }
  }, [otpauthUri, step]);

  const handleSetup = async () => {
    setError('');
    try {
      const res = await fetch(`/api/auth/users/${user.id}/totp/setup`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() }
      });
      if (res.ok) {
        const data = await res.json();
        setOtpauthUri(data.otpauth_uri);
        setSecret(data.secret);
        setStep('setup');
      } else {
        const data = await res.json();
        setError(data.error || t('users.mfaSetupFailed'));
      }
    } catch (err) {
      setError(t('users.networkErrorDuringMfaSetup'));
    }
  };

  const handleVerify = async (e) => {
    e.preventDefault();
    setError('');
    if (verifyCode.length !== 6) {
      setError(t('users.enterSixDigitCode'));
      return;
    }
    try {
      const res = await fetch(`/api/auth/users/${user.id}/totp/verify`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify({ code: verifyCode })
      });
      if (res.ok) {
        setStep('enabled');
        if (onSuccess) onSuccess();
      } else {
        const data = await res.json();
        setError(data.error || t('users.invalidVerificationCode'));
        setVerifyCode('');
      }
    } catch (err) {
      setError(t('users.networkErrorDuringVerification'));
    }
  };

  const handleDisable = async () => {
    setError('');
    try {
      const res = await fetch(`/api/auth/users/${user.id}/totp/disable`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() }
      });
      if (res.ok) {
        setTotpEnabled(false);
        setStep('status');
        if (onSuccess) onSuccess();
      } else {
        const data = await res.json();
        setError(data.error || t('users.disableMfaFailed'));
      }
    } catch (err) {
      setError(t('users.networkErrorDuringDisableMfa'));
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">{t('users.mfaSetupNamed', { username: user.username })}</h2>

        {error && (
          <div className="mb-4 p-3 rounded-lg badge-danger">{error}</div>
        )}

        {step === 'loading' && (
          <div className="text-center py-4">{t('common.loading')}</div>
        )}

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
                onChange={(e) => setVerifyCode(e.target.value.replace(/[^0-9]/g, '').slice(0, 6))}
                maxLength="6"
                autoFocus
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
          <button className="btn-secondary" onClick={onClose}>
            {step === 'enabled' ? t('common.done') : t('common.close')}
          </button>
        </div>
      </div>
    </div>
  );
}

