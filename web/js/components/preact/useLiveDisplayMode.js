import { useCallback, useEffect, useState } from 'preact/hooks';

/**
 * Live view display mode: how each grid tile scales its stream (#619).
 *
 *   fit  - object-fit: contain. The whole frame is visible, letterboxed.
 *   fill - object-fit: cover. The tile is filled and the frame edges are cropped.
 *
 * "fit" is the historical behaviour, so it stays the default for existing users.
 * The choice is persisted like the other live view preferences.
 */
export const LIVE_DISPLAY_MODE_KEY = 'lightnvr-live-display-mode';
export const LIVE_DISPLAY_MODES = Object.freeze(['fit', 'fill']);
export const DEFAULT_LIVE_DISPLAY_MODE = 'fit';

export function normalizeLiveDisplayMode(value) {
  const mode = typeof value === 'string' ? value.trim().toLowerCase() : '';
  return LIVE_DISPLAY_MODES.includes(mode) ? mode : DEFAULT_LIVE_DISPLAY_MODE;
}

export function readLiveDisplayMode(storage) {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    return normalizeLiveDisplayMode(selectedStorage?.getItem(LIVE_DISPLAY_MODE_KEY));
  } catch {
    return DEFAULT_LIVE_DISPLAY_MODE;
  }
}

export function writeLiveDisplayMode(mode, storage) {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    selectedStorage?.setItem(LIVE_DISPLAY_MODE_KEY, normalizeLiveDisplayMode(mode));
  } catch {
    // Storage can be disabled in private browsing; keep the in-memory choice.
  }
}

export function nextLiveDisplayMode(mode) {
  return normalizeLiveDisplayMode(mode) === 'fit' ? 'fill' : 'fit';
}

export function objectFitForLiveDisplayMode(mode) {
  return normalizeLiveDisplayMode(mode) === 'fill' ? 'cover' : 'contain';
}

/**
 * Rectangle (in box pixels) that a video frame occupies inside a box of
 * boxWidth x boxHeight under the given object-fit. With "contain" the frame is
 * letterboxed inside the box; with "cover" it overflows the box, so the offsets
 * go negative. Overlays that map normalized frame coordinates (detections,
 * zones) onto screen pixels need this to stay aligned in both display modes.
 *
 * @returns {{drawWidth: number, drawHeight: number, offsetX: number, offsetY: number}}
 */
export function displayedVideoBox(boxWidth, boxHeight, videoWidth, videoHeight, objectFit = 'contain') {
  if (!boxWidth || !boxHeight || !videoWidth || !videoHeight) {
    return { drawWidth: boxWidth || 0, drawHeight: boxHeight || 0, offsetX: 0, offsetY: 0 };
  }
  const videoAspect = videoWidth / videoHeight;
  const boxAspect = boxWidth / boxHeight;
  // contain: the wider side of the frame touches the box edges.
  // cover: the narrower side touches the box edges and the other overflows.
  const widthLimited = objectFit === 'cover'
    ? videoAspect < boxAspect
    : videoAspect > boxAspect;
  if (widthLimited) {
    const drawHeight = boxWidth / videoAspect;
    return { drawWidth: boxWidth, drawHeight, offsetX: 0, offsetY: (boxHeight - drawHeight) / 2 };
  }
  const drawWidth = boxHeight * videoAspect;
  return { drawWidth, drawHeight: boxHeight, offsetX: (boxWidth - drawWidth) / 2, offsetY: 0 };
}

export function useLiveDisplayMode() {
  const [mode, setMode] = useState(() => readLiveDisplayMode());
  useEffect(() => writeLiveDisplayMode(mode), [mode]);
  const setDisplayMode = useCallback((next) => setMode(normalizeLiveDisplayMode(next)), []);
  return [mode, setDisplayMode];
}
