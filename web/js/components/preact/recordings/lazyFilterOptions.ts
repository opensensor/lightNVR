export const FILTER_OPTIONS_IDLE = 'idle';
export const FILTER_OPTIONS_LOADING = 'loading';
export const FILTER_OPTIONS_LOADED = 'loaded';
export const FILTER_OPTIONS_ERROR = 'error';

export type FilterOptionsStatus =
  | typeof FILTER_OPTIONS_IDLE
  | typeof FILTER_OPTIONS_LOADING
  | typeof FILTER_OPTIONS_LOADED
  | typeof FILTER_OPTIONS_ERROR;

export function shouldLoadFilterOptions(isExpanded: boolean, status: FilterOptionsStatus) {
  return isExpanded && status === FILTER_OPTIONS_IDLE;
}

export function areFilterOptionsInteractive(status: FilterOptionsStatus) {
  return status === FILTER_OPTIONS_LOADED;
}

export const FILTER_OPTIONS_STALE_TIME_MS = 5 * 60 * 1000;

/**
 * Wrap a filter-option loader so a fresh result is reused across sidebar
 * mounts (collapse/expand, page navigation) and concurrent callers share one
 * in-flight request. Failures are never cached, so a retry always refetches.
 *
 * @param {() => T | Promise<T>} loader Fetches the options
 * @param {{ staleTime?: number, now?: () => number }} [options]
 * @returns {(() => Promise<T>) & { invalidate: () => void }}
 */
export function createCachedFilterOptionsLoader<T>(
  loader: () => T | Promise<T>,
  { staleTime = FILTER_OPTIONS_STALE_TIME_MS, now = () => Date.now() }: { staleTime?: number; now?: () => number } = {}
) {
  let cached: { value: T; loadedAt: number } | null = null;
  let inFlight: Promise<T> | null = null;

  const load = (): Promise<T> => {
    if (cached && now() - cached.loadedAt < staleTime) {
      return Promise.resolve(cached.value) as Promise<T>;
    }
    if (inFlight) return inFlight;

    let request: Promise<T>;
    try {
      request = Promise.resolve(loader()) as Promise<T>;
    } catch (error) {
      return Promise.reject(error);
    }
    inFlight = request
      .then((value: T) => {
        cached = { value, loadedAt: now() };
        return value;
      })
      .finally(() => {
        inFlight = null;
      });
    return inFlight;
  };

  load.invalidate = () => {
    cached = null;
  };

  return load;
}
