/**
 * TOTP MFA Setup Modal Component
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import QRCode from 'qrcode';

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
        setError(data.error || 'Failed to set up TOTP');
      }
    } catch (err) {
      setError('Network error during TOTP setup');
    }
  };

  const handleVerify = async (e) => {
    e.preventDefault();
    setError('');
    if (verifyCode.length !== 6) {
      setError('Please enter a 6-digit code');
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
        setError(data.error || 'Invalid verification code');
        setVerifyCode('');
      }
    } catch (err) {
      setError('Network error during verification');
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
        setError(data.error || 'Failed to disable TOTP');
      }
    } catch (err) {
      setError('Network error during disable');
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">MFA Setup — {user.username}</h2>

        {error && (
          <div className="mb-4 p-3 rounded-lg badge-danger">{error}</div>
        )}

        {step === 'loading' && (
          <div className="text-center py-4">Loading...</div>
        )}

        {step === 'status' && !totpEnabled && (
          <div>
            <p className="mb-4 text-sm text-muted-foreground">
              Two-factor authentication adds an extra layer of security.
              You'll need an authenticator app like Google Authenticator or Authy.
            </p>
            <button className="btn-primary w-full" onClick={handleSetup}>
              Set Up Two-Factor Authentication
            </button>
          </div>
        )}

        {step === 'status' && totpEnabled && (
          <div>
            <div className="mb-4 p-3 rounded-lg badge-success">
              ✓ Two-factor authentication is enabled
            </div>
            <button className="btn-danger w-full" onClick={handleDisable}>
              Disable Two-Factor Authentication
            </button>
          </div>
        )}

        {step === 'setup' && (
          <div>
            <p className="mb-3 text-sm">
              Scan this QR code with your authenticator app:
            </p>
            <div className="flex justify-center mb-3 bg-white p-2 rounded">
              <canvas ref={canvasRef}></canvas>
            </div>
            <div className="mb-4">
              <p className="text-xs text-muted-foreground mb-1">Or enter this key manually:</p>
              <code className="block p-2 bg-muted text-foreground rounded text-sm font-mono break-all select-all">
                {secret}
              </code>
            </div>
            <form onSubmit={handleVerify}>
              <label className="block text-sm font-medium mb-1">Enter verification code:</label>
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
              <button
                type="submit"
                className="btn-primary w-full"
                disabled={verifyCode.length !== 6}
              >
                Verify & Enable
              </button>
            </form>
          </div>
        )}

        {step === 'enabled' && (
          <div>
            <div className="mb-4 p-3 rounded-lg badge-success">
              ✓ Two-factor authentication has been enabled successfully!
            </div>
            <p className="mb-4 text-sm text-muted-foreground">
              You will now be required to enter a verification code from your authenticator app each time you log in.
            </p>
          </div>
        )}

        <div className="flex justify-end mt-4">
          <button className="btn-secondary" onClick={onClose}>
            {step === 'enabled' ? 'Done' : 'Close'}
          </button>
        </div>
      </div>
    </div>
  );
}

