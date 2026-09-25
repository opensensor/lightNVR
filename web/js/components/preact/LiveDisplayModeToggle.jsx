import { useI18n } from '../../i18n.js';
import { LIVE_DISPLAY_MODES, normalizeLiveDisplayMode } from './useLiveDisplayMode.js';

const LABEL_KEYS = {
  fit: 'live.displayModeFit',
  fill: 'live.displayModeFill',
};

const DESCRIPTION_KEYS = {
  fit: 'live.displayModeFitDescription',
  fill: 'live.displayModeFillDescription',
};

function DisplayModeIcon({ mode }) {
  if (mode === 'fill') {
    // Frame that overflows the tile: arrows pushing out of the corners.
    return (
      <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <rect x="3" y="5" width="18" height="14" rx="2" />
        <path d="M8 10l-3 2 3 2M16 10l3 2-3 2" />
      </svg>
    );
  }
  // Letterboxed frame inside the tile.
  return (
    <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <rect x="3" y="5" width="18" height="14" rx="2" />
      <rect x="6" y="9" width="12" height="6" rx="1" />
    </svg>
  );
}

/**
 * Fit / Fill segmented control for the live view toolbar (#619).
 * Fit keeps the whole frame visible (letterboxed); Fill fills the tile and
 * crops the frame edges.
 */
export function LiveDisplayModeToggle({ mode, onChange }) {
  const { t } = useI18n();
  const current = normalizeLiveDisplayMode(mode);
  return (
    <div
      className="inline-flex overflow-hidden rounded-full shadow-sm"
      role="group"
      aria-label={t('live.displayMode')}
      data-testid="live-display-mode"
    >
      {LIVE_DISPLAY_MODES.map((value) => {
        const active = value === current;
        const description = t(DESCRIPTION_KEYS[value]);
        return (
          <button
            key={value}
            type="button"
            className={`inline-flex items-center gap-1 px-2.5 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary ${active ? 'bg-primary text-primary-foreground' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}`}
            onClick={() => onChange(value)}
            aria-pressed={active}
            title={description}
            aria-label={description}
            data-display-mode-option={value}
          >
            <DisplayModeIcon mode={value} />
            <span>{t(LABEL_KEYS[value])}</span>
          </button>
        );
      })}
    </div>
  );
}

export default LiveDisplayModeToggle;
