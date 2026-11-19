import { useI18n } from '../../../i18n.ts';
import { formatLastImageTime } from './lastFrame.ts';
import type { LastFrameImage } from '../../../types/live.ts';

export interface LastFrameFallbackProps {
  image: LastFrameImage | null | undefined;
}

/**
 * The camera's last known image, rendered as the first child of a live tile's
 * error overlay (#974). The overlay is positioned with a z-index, so the
 * negative z-index paints the image above the overlay's own background but
 * below its error text and retry button. The brightness filter does the
 * dimming that the opaque background did before. Object-fit follows the Fit /
 * Fill display mode via live.css.
 */
export function LastFrameBackdrop({ image }: LastFrameFallbackProps) {
  if (!image) return null;
  return (
    <img
      key={image.url}
      className="last-frame-image"
      src={image.url}
      alt=""
      aria-hidden="true"
      draggable={false}
      data-testid="last-frame-image"
      data-last-frame-source={image.source}
      // A cache miss that fails after the preload: hide it, never show a broken icon.
      onError={(event) => { event.currentTarget.style.display = 'none'; }}
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        zIndex: -1,
        pointerEvents: 'none',
        filter: 'brightness(0.45)',
      }}
    />
  );
}

/** "Last image · <time>" under the retry button, so the image is never taken for live video. */
export function LastFrameCaption({ image }: LastFrameFallbackProps) {
  const { t, locale } = useI18n();
  if (!image) return null;
  return (
    <p
      className="last-frame-caption"
      data-testid="last-frame-caption"
      style={{
        margin: '12px 0 0',
        fontSize: '12px',
        lineHeight: '1.3',
        color: 'rgba(255, 255, 255, 0.85)',
        textAlign: 'center',
      }}
    >
      {t('live.lastImageCaption', { time: formatLastImageTime(image.capturedAt, locale) })}
    </p>
  );
}
