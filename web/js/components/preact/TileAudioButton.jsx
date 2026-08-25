import { useI18n } from '../../i18n.js';

export function TileAudioButton({ enabled, onToggle, disabled = false }) {
  const { t } = useI18n();
  const label = t(enabled ? 'live.muteCameraAudio' : 'live.unmuteCameraAudio');

  return (
    <button
      type="button"
      className={`audio-toggle-btn ${enabled ? 'active' : ''}`}
      title={label}
      aria-label={label}
      aria-pressed={enabled}
      disabled={disabled}
      onClick={onToggle}
      style={{
        backgroundColor: enabled ? 'rgba(34, 197, 94, 0.8)' : 'transparent',
        border: 'none',
        padding: '5px',
        borderRadius: '4px',
        color: 'white',
        cursor: disabled ? 'not-allowed' : 'pointer',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      {enabled ? (
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14" />
        </svg>
      ) : (
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
          <line x1="23" y1="9" x2="17" y2="15" />
          <line x1="17" y1="9" x2="23" y2="15" />
        </svg>
      )}
    </button>
  );
}

export default TileAudioButton;
