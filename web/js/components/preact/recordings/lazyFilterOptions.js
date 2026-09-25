export const FILTER_OPTIONS_IDLE = 'idle';
export const FILTER_OPTIONS_LOADING = 'loading';
export const FILTER_OPTIONS_LOADED = 'loaded';
export const FILTER_OPTIONS_ERROR = 'error';

export function shouldLoadFilterOptions(isExpanded, status) {
  return isExpanded && status === FILTER_OPTIONS_IDLE;
}

export function areFilterOptionsInteractive(status) {
  return status === FILTER_OPTIONS_LOADED;
}

export const FILTER_OPTIONS_STALE_TIME_MS = 5 * 60 * 1000;

/**
 * Wrap a filter-option loader so a fresh result is reused across sidebar
 * mounts (collapse/expand, page navigation) and concurrent callers share one
 * in-flight request. Failures are never cached, so a retry always refetches.
 *
 * @param {() => Promise<any>} loader Fetches the options
 * @param {{ staleTime?: number, now?: () => number }} [options]
 * @returns {(() => Promise<any>) & { invalidate: () => void }}
 */
export function createCachedFilterOptionsLoader(
  loader,
  { staleTime = FILTER_OPTIONS_STALE_TIME_MS, now = () => Date.now() } = {}
) {
  let cached = null;
  let inFlight = null;

  const load = () => {
    if (cached && now() - cached.loadedAt < staleTime) {
      return Promise.resolve(cached.value);
    }
    if (inFlight) return inFlight;

    let request;
    try {
      request = Promise.resolve(loader());
    } catch (error) {
      return Promise.reject(error);
    }
    inFlight = request
      .then((value) => {
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
