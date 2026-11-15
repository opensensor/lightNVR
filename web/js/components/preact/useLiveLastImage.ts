import { useEffect, useState } from 'preact/hooks';
import type { Dispatch, StateUpdater } from 'preact/hooks';

/**
 * Live view preference (#974): show a camera's last known image behind the
 * error overlay of a tile that cannot play. On by default; persisted per
 * browser like the other live view preferences. Off means no extra requests.
 */
export const LIVE_LAST_IMAGE_KEY = 'lightnvr-live-show-last-image';
export const DEFAULT_LIVE_LAST_IMAGE = true;

export function readLiveLastImage(storage?: Storage | null): boolean {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    const stored = selectedStorage?.getItem(LIVE_LAST_IMAGE_KEY);
    if (stored === 'true') return true;
    if (stored === 'false') return false;
    return DEFAULT_LIVE_LAST_IMAGE;
  } catch {
    return DEFAULT_LIVE_LAST_IMAGE;
  }
}

export function writeLiveLastImage(value: unknown, storage?: Storage | null): void {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    selectedStorage?.setItem(LIVE_LAST_IMAGE_KEY, String(Boolean(value)));
  } catch {
    // Storage can be disabled in private browsing; keep the in-memory choice.
  }
}

export function useLiveLastImage(): [boolean, Dispatch<StateUpdater<boolean>>] {
  const [enabled, setEnabled] = useState(() => readLiveLastImage());
  useEffect(() => writeLiveLastImage(enabled), [enabled]);
  return [enabled, setEnabled];
}
