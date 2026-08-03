import { useCallback, useEffect, useState } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';

export function ManualRecordingButton({ streamName }) {
  const [status, setStatus] = useState(null);
  const [busy, setBusy] = useState(false);

  const statusUrl = `/api/streams/${encodeURIComponent(streamName)}/recording`;
  const refresh = useCallback(async () => {
    try {
      const response = await fetch(statusUrl, { credentials: 'same-origin' });
      if (!response.ok) return;
      setStatus(await response.json());
    } catch {
      // A transient status failure should not disrupt live video controls.
    }
  }, [statusUrl]);

  useEffect(() => {
    refresh();
    const timer = setInterval(refresh, 5000);
    return () => clearInterval(timer);
  }, [refresh]);

  const isManual = status?.capture_method === 'manual' && status?.recording_active;
  const isOtherRecording = status?.recording_active && !isManual;
  const disabled = busy || !status?.manual_control_allowed || isOtherRecording;
  const title = !status?.manual_control_allowed
    ? status?.manual_control_reason === 'continuous_recording'
      ? 'Manual recording is unavailable while continuous recording is enabled'
      : 'Manual recording requires operator access'
    : isOtherRecording
      ? `${status.capture_method || 'Another'} recording is active`
      : isManual ? 'Stop manual recording' : 'Start manual recording';

  const toggle = async () => {
    if (disabled) return;
    setBusy(true);
    try {
      const action = isManual ? 'stop' : 'start';
      const response = await fetch(statusUrl, {
        method: 'POST',
        credentials: 'same-origin',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action })
      });
      const data = await response.json().catch(() => ({}));
      if (!response.ok) {
        throw new Error(data.error || data.message || `HTTP ${response.status}`);
      }
      showStatusMessage(
        action === 'start' ? 'Manual recording started' : 'Manual recording stopped',
        'success'
      );
      await refresh();
    } catch (error) {
      showStatusMessage(`Manual recording failed: ${error.message}`, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  return (
    <button
      type="button"
      className="manual-recording-btn"
      title={title}
      aria-label={title}
      aria-pressed={isManual}
      disabled={disabled}
      onClick={toggle}
      style={{
        width: '36px',
        height: '36px',
        padding: '5px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        border: 'none',
        borderRadius: '4px',
        color: 'white',
        backgroundColor: isManual ? 'rgba(220, 38, 38, 0.9)' : 'transparent',
        cursor: disabled ? 'not-allowed' : 'pointer',
        opacity: disabled ? 0.55 : 1
      }}
    >
      <span
        aria-hidden="true"
        style={{
          width: isManual ? '12px' : '14px',
          height: isManual ? '12px' : '14px',
          borderRadius: isManual ? '2px' : '50%',
          background: isOtherRecording ? '#f59e0b' : '#ef4444'
        }}
      />
    </button>
  );
}
