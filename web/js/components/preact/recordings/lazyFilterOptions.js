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
