import { useI18n } from '../../i18n.ts';

export interface LiveLastImageToggleProps {
  enabled: boolean;
  onChange: (enabled: boolean) => unknown;
}

/**
 * Live view toolbar toggle (#974): show the last known image of a camera
 * behind the error overlay of a tile that cannot play.
 */
export function LiveLastImageToggle({ enabled, onChange }: LiveLastImageToggleProps) {
  const { t } = useI18n();
  const label = t('live.showLastImageWhenOffline');
  return (
    <button
      type="button"
      className={`rounded-full p-2 focus:outline-none focus:ring-2 focus:ring-primary ${enabled ? 'bg-primary text-primary-foreground hover:bg-primary/90' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}`}
      onClick={() => onChange(!enabled)}
      title={label}
      aria-label={label}
      aria-pressed={enabled}
      data-testid="live-last-image-toggle"
    >
      {/* Picture with a clock badge: a still image from earlier. */}
      <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <path d="M13 19H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" />
        <path d="M3 15l4-4 3 3 2-2" />
        <circle cx="18" cy="17" r="4" />
        <path d="M18 15.5V17l1 1" />
      </svg>
      <span className="live-mobile-label">{t('live.lastImageShort')}</span>
    </button>
  );
}

export default LiveLastImageToggle;
