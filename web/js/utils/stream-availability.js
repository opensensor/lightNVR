const STREAM_AVAILABILITY_VALUES = new Set([
  'live',
  'offline',
  'never_connected',
  'disabled',
  'all',
]);

/**
 * Configuration is an administrative inventory, so it must show every stream
 * unless the operator explicitly requests an availability filter in the URL.
 */
export function getInitialStreamAvailability(search) {
  const locationSearch = search ?? (
    typeof window !== 'undefined' ? window.location.search : ''
  );
  const fromUrl = new URLSearchParams(locationSearch).get('availability');
  return STREAM_AVAILABILITY_VALUES.has(fromUrl) ? fromUrl : 'all';
}

export function hasStreamSummaryFilters(search, availability) {
  return Boolean(search?.trim()) || availability !== 'all';
}
