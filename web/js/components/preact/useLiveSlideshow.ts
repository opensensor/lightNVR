import { useCallback, useEffect, useMemo, useRef, useState } from 'preact/hooks';
import type { LiveStream } from '../../types/live.ts';

export interface LiveSlideshowState {
  enabled: boolean;
  interval: number;
}

interface SlideshowPosition {
  isSingleStream: boolean;
  streamNames: string[];
  selectedStream: string;
  currentPage: number;
  maxStreams: number;
}

/** `{selectedStream}` in 1x1 mode, `{page}` in grid mode. */
export type SlideshowTarget =
  | { selectedStream: string; page?: undefined }
  | { page: number; selectedStream?: undefined };

interface SlideshowLatest extends SlideshowPosition {
  setSelectedStream: (name: string) => void;
  setCurrentPage: (page: number) => void;
}

export interface UseLiveSlideshowOptions {
  paused?: boolean;
  isSingleStream: boolean;
  orderedStreams: LiveStream[];
  selectedStream: string;
  setSelectedStream: (name: string) => void;
  currentPage: number;
  setCurrentPage: (page: number) => void;
  maxStreams: number;
}

/**
 * Live view slideshow: auto-advance the live grid on a timer.
 *
 *   1x1 layout  - cycles through the cameras one at a time.
 *   larger grid - cycles through the pages of the grid.
 *
 * The preference is persisted in localStorage and mirrored to the URL as
 * ?slideshow=<seconds>, so a wall display or kiosk can bookmark it.
 */
export const LIVE_SLIDESHOW_KEY = 'lightnvr-live-slideshow';
export const LIVE_SLIDESHOW_INTERVAL_KEY = 'lightnvr-live-slideshow-interval';
export const LIVE_SLIDESHOW_URL_PARAM = 'slideshow';
export const LIVE_SLIDESHOW_INTERVALS = Object.freeze([5, 10, 15, 30, 60]);
export const DEFAULT_LIVE_SLIDESHOW_INTERVAL = 10;

export function normalizeSlideshowInterval(value: unknown): number {
  const seconds = Number.parseInt(value as string, 10);
  return LIVE_SLIDESHOW_INTERVALS.includes(seconds) ? seconds : DEFAULT_LIVE_SLIDESHOW_INTERVAL;
}

/**
 * Initial slideshow state. A ?slideshow=<seconds> URL param wins over the
 * stored preference; ?slideshow=0 turns it off.
 *
 * @returns {{enabled: boolean, interval: number}}
 */
export function readLiveSlideshow(storage?: Storage | null, search?: string | null): LiveSlideshowState {
  try {
    const param = new URLSearchParams(search ?? globalThis.location?.search ?? '').get(LIVE_SLIDESHOW_URL_PARAM);
    if (param !== null) {
      if (param === '0' || param === 'off') {
        return { enabled: false, interval: DEFAULT_LIVE_SLIDESHOW_INTERVAL };
      }
      return { enabled: true, interval: normalizeSlideshowInterval(param) };
    }
  } catch {
    // Fall through to the stored preference.
  }
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    return {
      enabled: selectedStorage?.getItem(LIVE_SLIDESHOW_KEY) === 'true',
      interval: normalizeSlideshowInterval(selectedStorage?.getItem(LIVE_SLIDESHOW_INTERVAL_KEY)),
    };
  } catch {
    return { enabled: false, interval: DEFAULT_LIVE_SLIDESHOW_INTERVAL };
  }
}

export function writeLiveSlideshow({ enabled, interval }: LiveSlideshowState, storage?: Storage | null): void {
  try {
    const selectedStorage = storage ?? globalThis.localStorage;
    selectedStorage?.setItem(LIVE_SLIDESHOW_KEY, String(!!enabled));
    selectedStorage?.setItem(LIVE_SLIDESHOW_INTERVAL_KEY, String(normalizeSlideshowInterval(interval)));
  } catch {
    // Storage can be disabled in private browsing; keep the in-memory choice.
  }
}

/** Number of slides the slideshow would cycle through. */
export function slideshowSlideCount({ isSingleStream, streamCount, maxStreams }: { isSingleStream: boolean; streamCount: number; maxStreams: number }): number {
  if (streamCount <= 0) return 0;
  if (isSingleStream) return streamCount;
  return Math.ceil(streamCount / Math.max(1, maxStreams));
}

/**
 * The next slide after the current one, wrapping around at the end.
 * Returns {selectedStream} in 1x1 mode, {page} in grid mode, or null when
 * there is nothing to cycle to.
 */
export function nextSlideshowTarget({ isSingleStream, streamNames, selectedStream, currentPage, maxStreams }: SlideshowPosition): SlideshowTarget | null {
  const slides = slideshowSlideCount({ isSingleStream, streamCount: streamNames.length, maxStreams });
  if (slides < 2) return null;
  if (isSingleStream) {
    const index = streamNames.indexOf(selectedStream);
    return { selectedStream: streamNames[(index + 1) % streamNames.length] as string };
  }
  return { page: (Math.max(0, currentPage) + 1) % slides };
}

function useDocumentVisible() {
  const [visible, setVisible] = useState(() => typeof document === 'undefined' || !document.hidden);
  useEffect(() => {
    const onChange = () => setVisible(!document.hidden);
    document.addEventListener('visibilitychange', onChange);
    return () => document.removeEventListener('visibilitychange', onChange);
  }, []);
  return visible;
}

/**
 * Drive the live view slideshow. The countdown restarts whenever the visible
 * slide changes, so manual paging or camera selection gets a full interval.
 * It also stops while `paused` is set (reorder mode, a tile in native
 * fullscreen) and while the browser tab is hidden.
 */
export function useLiveSlideshow({
  paused = false,
  isSingleStream,
  orderedStreams,
  selectedStream,
  setSelectedStream,
  currentPage,
  setCurrentPage,
  maxStreams,
}: UseLiveSlideshowOptions) {
  const [state, setState] = useState(() => readLiveSlideshow());
  const visible = useDocumentVisible();

  useEffect(() => {
    writeLiveSlideshow(state);
    // URL stringifies the Location; its types only accept a string or URL.
    const url = new URL(window.location as unknown as string);
    if (state.enabled) url.searchParams.set(LIVE_SLIDESHOW_URL_PARAM, String(state.interval));
    else url.searchParams.delete(LIVE_SLIDESHOW_URL_PARAM);
    window.history.replaceState({}, '', url);
  }, [state]);

  // Streams are refetched periodically; key the timer on the names so a
  // refresh with the same cameras does not restart the countdown.
  const streamNames = useMemo(() => orderedStreams.map((stream) => stream.name), [orderedStreams]);
  const namesKey = streamNames.join('\n');
  const latest = useRef<SlideshowLatest | null>(null);
  latest.current = { isSingleStream, streamNames, selectedStream, currentPage, maxStreams, setSelectedStream, setCurrentPage };

  const slideCount = slideshowSlideCount({ isSingleStream, streamCount: streamNames.length, maxStreams });
  const canCycle = slideCount > 1;
  const running = state.enabled && canCycle && !paused && visible;

  useEffect(() => {
    if (!running) return undefined;
    const timer = setTimeout(() => {
      const current = latest.current!;
      const target = nextSlideshowTarget(current);
      if (!target) return;
      if (target.selectedStream !== undefined) current.setSelectedStream(target.selectedStream);
      else current.setCurrentPage(target.page);
    }, state.interval * 1000);
    return () => clearTimeout(timer);
  }, [running, state.interval, isSingleStream, selectedStream, currentPage, maxStreams, namesKey]);

  const setEnabled = useCallback((enabled: boolean) => setState((s) => ({ ...s, enabled: !!enabled })), []);
  const setInterval = useCallback((interval: unknown) =>
    setState((s) => ({ ...s, interval: normalizeSlideshowInterval(interval) })), []);

  return {
    enabled: state.enabled,
    interval: state.interval,
    canCycle,
    running,
    setEnabled,
    setInterval,
  };
}
