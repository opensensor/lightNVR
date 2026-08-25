import { useI18n } from '../../i18n.js';
import { healthStateForStream, recordingModesForStream } from './liveTileStatus.js';

function RecordingGlyph({ modes }) {
  const hasConstant = modes.includes('constant');
  const hasDetection = modes.includes('detection');
  const hasSchedule = modes.includes('schedule');
  const active = modes.length > 0;

  return (
    <svg viewBox="0 0 24 24" className="h-5 w-5" aria-hidden="true">
      <circle
        cx="12"
        cy="12"
        r={hasConstant ? 6 : 5}
        fill={active ? '#ef4444' : 'none'}
        stroke={active ? '#fecaca' : '#cbd5e1'}
        strokeWidth="2"
      />
      {hasDetection && (
        <>
          <path d="M4.5 9.5A8.4 8.4 0 0 1 8 5.3" fill="none" stroke="white" strokeWidth="1.7" strokeLinecap="round" />
          <path d="M19.5 9.5A8.4 8.4 0 0 0 16 5.3" fill="none" stroke="white" strokeWidth="1.7" strokeLinecap="round" />
        </>
      )}
      {hasSchedule && (
        <>
          <circle cx="12" cy="12" r="3.5" fill="rgba(127,29,29,0.72)" stroke="white" strokeWidth="1" />
          <path d="M12 9.7v2.6l1.8 1" fill="none" stroke="white" strokeWidth="1.2" strokeLinecap="round" strokeLinejoin="round" />
        </>
      )}
    </svg>
  );
}

function HealthGlyph({ state }) {
  if (state === 'healthy') {
    return (
      <svg viewBox="0 0 24 24" className="h-5 w-5" aria-hidden="true">
        <circle cx="12" cy="12" r="8" fill="#16a34a" />
        <path d="m8.2 12.1 2.4 2.4 5.2-5.2" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      </svg>
    );
  }
  if (state === 'offline') {
    return (
      <svg viewBox="0 0 24 24" className="h-5 w-5" aria-hidden="true">
        <path d="M12 3.2 21 19H3L12 3.2Z" fill="#dc2626" stroke="#fecaca" strokeWidth="1" />
        <path d="M12 8v5.3M12 16.5h.01" fill="none" stroke="white" strokeWidth="2" strokeLinecap="round" />
      </svg>
    );
  }
  return (
    <svg viewBox="0 0 24 24" className="h-5 w-5" aria-hidden="true">
      <circle cx="12" cy="12" r="8" fill="#d97706" />
      <circle cx="12" cy="12" r="2" fill="white" />
    </svg>
  );
}

export function LiveTileStatus({ stream, isPlaying, isLoading, error }) {
  const { t } = useI18n();
  const modes = recordingModesForStream(stream);
  const health = healthStateForStream(stream, { isPlaying, isLoading, error });
  const recordingLabel = modes.length > 0
    ? modes.map((mode) => t(`live.recordingMode.${mode}`)).join(' + ')
    : t('live.recordingMode.none');
  const healthLabel = t(`live.health.${health}`);

  return (
    <div className="pointer-events-none absolute right-2 top-2 z-30 flex gap-1 rounded-full bg-black/65 p-1 shadow backdrop-blur-sm">
      <span
        className="flex h-9 w-9 items-center justify-center rounded-full sm:h-7 sm:w-7"
        role="img"
        aria-label={recordingLabel}
        title={recordingLabel}
      >
        <RecordingGlyph modes={modes} />
      </span>
      <span
        className="flex h-9 w-9 items-center justify-center rounded-full sm:h-7 sm:w-7"
        role="img"
        aria-label={healthLabel}
        title={healthLabel}
      >
        <HealthGlyph state={health} />
      </span>
    </div>
  );
}

export default LiveTileStatus;
