import { useEffect, useState } from 'preact/hooks';

export const ALWAYS_FULLSCREEN_ON_TAP_KEY = 'lightnvr-always-fullscreen-on-tap';

export function readAlwaysFullscreenOnTap(storage) {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    return selectedStorage?.getItem(ALWAYS_FULLSCREEN_ON_TAP_KEY) === 'true';
  } catch {
    return false;
  }
}

export function writeAlwaysFullscreenOnTap(value, storage) {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    selectedStorage?.setItem(ALWAYS_FULLSCREEN_ON_TAP_KEY, String(Boolean(value)));
  } catch {
    // Storage can be disabled in private browsing; keep the in-memory choice.
  }
}

export function useAlwaysFullscreenOnTap() {
  const [enabled, setEnabled] = useState(() => readAlwaysFullscreenOnTap());
  useEffect(() => writeAlwaysFullscreenOnTap(enabled), [enabled]);
  return [enabled, setEnabled];
}

export function shouldEnterFullscreenFromTap(event, enabled, zoomed = false, documentObject = globalThis.document) {
  if (!enabled || zoomed || documentObject?.fullscreenElement || documentObject?.webkitFullscreenElement) {
    return false;
  }
  const target = event?.target;
  if (target && typeof target.closest === 'function'
      && target.closest('button, a, input, select, textarea, [role="button"], [role="menu"]')) {
    return false;
  }
  return true;
}
