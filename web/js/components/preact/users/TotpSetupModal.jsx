/**
 * TOTP MFA Setup Modal Component
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
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
  const [totpConfigured, setTotpConfigured] = useState(false);
  const canvasRef = useRef(null);

  const stopPropagation = (e) => e.stopPropagation();

  // Fetch current TOTP status on mount
  useEffect(() => {
    fetchStatus();
  }, []);

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

  const fetchStatus = async () => {
    try {
      const res = await fetch(`/api/auth/users/${user.id}/totp/status`, {
        headers: getAuthHeaders()
      });
      if (res.ok) {
        const data = await res.json();
        setTotpEnabled(data.totp_enabled);
        setTotpConfigured(data.totp_configured);
        setStep('status');
      } else {
        // TOTP columns may not exist yet
        setTotpEnabled(false);
        setTotpConfigured(false);
        setStep('status');
      }
    } catch (err) {
      console.error('Error fetching TOTP status:', err);
      setStep('status');
    }
  };

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
        setTotpConfigured(false);
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
    <div className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick={stopPropagation}>
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

