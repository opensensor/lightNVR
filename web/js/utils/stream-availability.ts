import type { StreamAvailabilityFilter } from '../types/core.ts';

const STREAM_AVAILABILITY_VALUES = new Set<string | null>([
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
export function getInitialStreamAvailability(search?: string | null): StreamAvailabilityFilter {
  const locationSearch = search ?? (
    typeof window !== 'undefined' ? window.location.search : ''
  );
  const fromUrl = new URLSearchParams(locationSearch).get('availability');
  return STREAM_AVAILABILITY_VALUES.has(fromUrl) ? fromUrl as StreamAvailabilityFilter : 'all';
}

export function hasStreamSummaryFilters(search: string | null | undefined, availability: string): boolean {
  return Boolean(search?.trim()) || availability !== 'all';
}
