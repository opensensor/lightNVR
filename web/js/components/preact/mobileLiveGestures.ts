export const LONG_PRESS_DELAY_MS = 550;
export const TILE_CHROME_TIMEOUT_MS = 4000;
export const PULL_REFRESH_THRESHOLD_PX = 64;

const GESTURE_TIP_PREFIX = 'lightnvr-gesture-tip';

export function resistedPullDistance(deltaY: number, maxDistance: number = 96): number {
  if (!Number.isFinite(deltaY) || deltaY <= 0) return 0;
  return Math.min(maxDistance, deltaY * 0.5);
}

export function fullscreenSwipeAction(deltaX: number, deltaY: number, zoomScale: number | string = 1): 'exit' | 'next' | 'previous' | null {
  if (Number(zoomScale) > 1.01) return null;

  const horizontal = Math.abs(deltaX);
  const vertical = Math.abs(deltaY);

  if (deltaY >= 80 && vertical > horizontal * 1.2) return 'exit';
  if (horizontal < 60 || horizontal <= vertical * 1.2) return null;
  return deltaX < 0 ? 'next' : 'previous';
}

export function shouldIgnoreTileGestureTarget(target: EventTarget | null | undefined): boolean {
  return Boolean(target && typeof (target as Element).closest === 'function' && (target as Element).closest(
    'button, a, input, select, textarea, [role="button"], [role="menu"], [data-gesture-ignore]'
  ));
}

/**
 * Count a contextual opportunity and return true exactly once, when the tip
 * should be shown. Storage failures are deliberately non-fatal.
 */
export function shouldShowGestureTip(name: string, threshold: number = 1, storage: Storage | null | undefined = globalThis.localStorage): boolean {
  const countKey = `${GESTURE_TIP_PREFIX}:${name}:count`;
  const shownKey = `${GESTURE_TIP_PREFIX}:${name}:shown`;

  try {
    if (storage?.getItem(shownKey) === 'true') return false;
    const nextCount = (Number.parseInt(storage?.getItem(countKey) || '0', 10) || 0) + 1;
    storage?.setItem(countKey, String(nextCount));
    if (nextCount < threshold) return false;
    storage?.setItem(shownKey, 'true');
    return true;
  } catch {
    return false;
  }
}
