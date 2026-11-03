import { useEffect, useState } from 'preact/hooks';
import type { Dispatch, StateUpdater } from 'preact/hooks';

interface FullscreenDocumentLike {
  fullscreenElement?: Element | null;
  webkitFullscreenElement?: Element | null;
}

/** Tile gesture events; only `target` is read. */
interface GestureEventLike {
  target?: EventTarget | null;
}

export const ALWAYS_FULLSCREEN_ON_TAP_KEY = 'lightnvr-always-fullscreen-on-tap';

export function readAlwaysFullscreenOnTap(storage?: Storage | null): boolean {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    return selectedStorage?.getItem(ALWAYS_FULLSCREEN_ON_TAP_KEY) === 'true';
  } catch {
    return false;
  }
}

export function writeAlwaysFullscreenOnTap(value: unknown, storage?: Storage | null): void {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    selectedStorage?.setItem(ALWAYS_FULLSCREEN_ON_TAP_KEY, String(Boolean(value)));
  } catch {
    // Storage can be disabled in private browsing; keep the in-memory choice.
  }
}

export function useAlwaysFullscreenOnTap(): [boolean, Dispatch<StateUpdater<boolean>>] {
  const [enabled, setEnabled] = useState(() => readAlwaysFullscreenOnTap());
  useEffect(() => writeAlwaysFullscreenOnTap(enabled), [enabled]);
  return [enabled, setEnabled];
}

function isFullscreenGestureTarget(event?: GestureEventLike | null) {
  const target = event?.target as Element | null | undefined;
  return !(target && typeof target.closest === 'function'
    && target.closest('button, a, input, select, textarea, [role="button"], [role="menu"]'));
}

function currentFullscreenElement(documentObject?: FullscreenDocumentLike | null) {
  return documentObject?.fullscreenElement || documentObject?.webkitFullscreenElement || null;
}

/**
 * With "fullscreen on single tap" enabled a tap is a toggle: it enters
 * fullscreen and, once this tile is fullscreen, the next tap exits again.
 * (Double-click is disabled in this mode, so without this the only way out
 * was Escape or the toolbar button.) A tap never exits when something else
 * owns fullscreen, and taps on controls or a zoomed tile are left alone.
 */
export function shouldToggleFullscreenFromTap(
  event: GestureEventLike | null | undefined,
  enabled: boolean,
  zoomed: boolean = false,
  cellElement: Element | null = null,
  fullscreenElement: Element | null = currentFullscreenElement(globalThis.document)
): boolean {
  if (!enabled || zoomed) return false;
  if (fullscreenElement && fullscreenElement !== cellElement) return false;
  return isFullscreenGestureTarget(event);
}

/**
 * Double-click is a fullscreen toggle, unlike the optional single-tap entry
 * gesture. It must remain active while a tile is already fullscreen so the
 * second double-click can reach the shared exitFullscreen path.
 */
export function shouldToggleFullscreenFromDoubleClick(
  event: GestureEventLike | null | undefined,
  alwaysFullscreenOnTap: boolean,
  zoomed: boolean = false
): boolean {
  return !alwaysFullscreenOnTap && !zoomed && isFullscreenGestureTarget(event);
}
