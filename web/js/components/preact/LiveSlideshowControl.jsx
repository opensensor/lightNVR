import { useI18n } from '../../i18n.js';
import { LIVE_SLIDESHOW_INTERVALS } from './useLiveSlideshow.js';

/**
 * Slideshow toggle for the live view toolbar, with an interval picker shown
 * while it is on. In a 1x1 layout it cycles cameras; otherwise grid pages.
 */
export function LiveSlideshowControl({ enabled, interval, isSingleStream, onToggle, onIntervalChange }) {
  const { t } = useI18n();
  const label = enabled
    ? t('live.slideshowStop')
    : t(isSingleStream ? 'live.slideshowStartCameras' : 'live.slideshowStartPages');
  return (
    <div className="inline-flex items-center gap-1" data-testid="live-slideshow">
      <button
        type="button"
        className={`rounded-full p-2 focus:outline-none focus:ring-2 focus:ring-primary ${enabled ? 'bg-primary text-primary-foreground hover:bg-primary/90' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}`}
        onClick={() => onToggle(!enabled)}
        title={label}
        aria-label={label}
        aria-pressed={enabled}
      >
        <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <rect x="2" y="4" width="20" height="14" rx="2" />
          {enabled
            ? <path d="M10 8v6M14 8v6" />
            : <path d="M10 8l5 3-5 3z" />}
          <path d="M8 21h8" />
        </svg>
      </button>
      {enabled && (
        <select
          className="px-2 py-2 border border-border rounded-md bg-background text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-primary"
          value={interval}
          onChange={(event) => onIntervalChange(event.currentTarget.value)}
          title={t('live.slideshowInterval')}
          aria-label={t('live.slideshowInterval')}
        >
          {LIVE_SLIDESHOW_INTERVALS.map((seconds) => (
            <option key={seconds} value={seconds}>{t('live.slideshowSeconds', { seconds })}</option>
          ))}
        </select>
      )}
    </div>
  );
}

export default LiveSlideshowControl;
