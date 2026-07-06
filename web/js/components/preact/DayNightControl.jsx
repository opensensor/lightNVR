/**
 * Day/Night mode control for ONVIF cameras (IR cut filter)
 *
 * Renders a toolbar button that opens a small popover with Auto / Day / Night
 * options, backed by GET/PUT /api/streams/{name}/daynight (ONVIF Imaging
 * service, e.g. thingino cameras). Renders nothing for non-ONVIF streams.
 */

import { useState, useRef, useEffect } from 'preact/hooks';
import { useI18n } from '../../i18n.js';
import { showStatusMessage } from './ToastContainer.jsx';

const MODES = ['auto', 'day', 'night'];

function SunIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="4"/>
      <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>
    </svg>
  );
}

function MoonIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
  );
}

function AutoIcon() {
  // Half sun / half moon to convey "auto"
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M12 3a9 9 0 1 0 0 18z" fill="white" stroke="none"/>
      <circle cx="12" cy="12" r="9"/>
    </svg>
  );
}

export function DayNightControl({ stream }) {
  const { t } = useI18n();
  const [open, setOpen] = useState(false);
  const [mode, setMode] = useState(null);      // 'auto' | 'day' | 'night' | 'unknown' | null (not yet loaded)
  const [loading, setLoading] = useState(false);
  const [pending, setPending] = useState(null); // mode currently being applied
  const containerRef = useRef(null);

  // Close the popover on outside clicks
  useEffect(() => {
    if (!open) return undefined;
    const handler = (e) => {
      if (containerRef.current && !containerRef.current.contains(e.target)) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [open]);

  if (!stream?.isOnvif) {
    return null;
  }

  const fetchMode = async () => {
    setLoading(true);
    try {
      const response = await fetch(`/api/streams/${encodeURIComponent(stream.name)}/daynight`);
      const data = await response.json().catch(() => null);
      if (!response.ok || !data?.success) {
        throw new Error(data?.error || `HTTP ${response.status}`);
      }
      setMode(data.mode || 'unknown');
    } catch (err) {
      setMode('unknown');
      showStatusMessage(`${t('live.dayNightLoadFailed')}: ${err.message}`, 'error', 5000);
    } finally {
      setLoading(false);
    }
  };

  const toggleOpen = () => {
    const next = !open;
    setOpen(next);
    if (next && mode === null && !loading) {
      fetchMode();
    }
  };

  const applyMode = async (nextMode) => {
    if (pending) return;
    setPending(nextMode);
    try {
      const response = await fetch(`/api/streams/${encodeURIComponent(stream.name)}/daynight`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mode: nextMode })
      });
      const data = await response.json().catch(() => null);
      if (!response.ok || !data?.success) {
        throw new Error(data?.error || `HTTP ${response.status}`);
      }
      setMode(data.mode || nextMode);
      showStatusMessage(t('live.dayNightApplied', { mode: t(`live.dayNightMode_${data.mode || nextMode}`) }), 'success', 3000);
    } catch (err) {
      showStatusMessage(`${t('live.dayNightApplyFailed')}: ${err.message}`, 'error', 5000);
    } finally {
      setPending(null);
    }
  };

  const CurrentIcon = mode === 'day' ? SunIcon : mode === 'night' ? MoonIcon : AutoIcon;

  return (
    <span ref={containerRef} style={{ position: 'relative', display: 'inline-flex' }}>
      <button
        className={`daynight-toggle-btn ${open ? 'active' : ''}`}
        title={t('live.dayNightControl')}
        onClick={toggleOpen}
        style={{
          backgroundColor: open ? 'rgba(59, 130, 246, 0.8)' : 'transparent',
          border: 'none',
          padding: '5px',
          borderRadius: '4px',
          color: 'white',
          cursor: 'pointer',
          transition: 'background-color 0.2s ease'
        }}
        onMouseOver={(e) => !open && (e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.2)')}
        onMouseOut={(e) => !open && (e.currentTarget.style.backgroundColor = 'transparent')}
      >
        <CurrentIcon />
      </button>

      {open && (
        <div
          style={{
            position: 'absolute',
            bottom: '40px',
            right: 0,
            backgroundColor: 'rgba(0, 0, 0, 0.85)',
            borderRadius: '8px',
            padding: '8px',
            display: 'flex',
            flexDirection: 'column',
            gap: '4px',
            minWidth: '110px',
            zIndex: 30
          }}
        >
          <div style={{ color: 'white', fontSize: '12px', opacity: 0.7, padding: '0 4px 4px' }}>
            {loading ? t('live.dayNightLoading') : t('live.dayNightControl')}
          </div>
          {MODES.map((m) => (
            <button
              key={m}
              onClick={() => applyMode(m)}
              disabled={loading || pending !== null}
              style={{
                backgroundColor: mode === m ? 'rgba(59, 130, 246, 0.8)' : 'rgba(255, 255, 255, 0.1)',
                border: 'none',
                borderRadius: '4px',
                color: 'white',
                cursor: loading || pending !== null ? 'wait' : 'pointer',
                padding: '6px 10px',
                fontSize: '13px',
                textAlign: 'left',
                opacity: pending && pending !== m ? 0.5 : 1
              }}
            >
              {pending === m ? t('live.dayNightApplying') : t(`live.dayNightMode_${m}`)}
            </button>
          ))}
        </div>
      )}
    </span>
  );
}

export default DayNightControl;
