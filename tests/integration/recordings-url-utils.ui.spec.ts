import { test, expect } from '@playwright/test';
import { urlUtils } from '../../web/js/components/preact/recordings/urlUtils.js';

// The imported browser implementation is still JS until the recordings rollout.
// Treat its broad Object result as unknown and require the object envelope that
// these existing value assertions consume. No declaration invents a return type.
const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null && !Array.isArray(value);

function readUrlState(): (Record<string, unknown> & { filters: Record<string, unknown> }) | null {
  const state: unknown = urlUtils.getFiltersFromUrl();
  if (state === null) return null;
  if (!isRecord(state) || !isRecord(state.filters)) {
    throw new Error('Expected URL state with a filters object');
  }
  return { ...state, filters: state.filters };
}

const setWindowLocation = (url: string) => {
  const browserWindow = {
    location: new URL(url),
    history: {
      replaceState: (_state: unknown, _title: string, nextUrl: string): void => {
        browserWindow.location = new URL(nextUrl, browserWindow.location.href);
      }
    }
  };
  Object.defineProperty(globalThis, 'window', { configurable: true, writable: true, value: browserWindow });
  Object.defineProperty(globalThis, 'history', { configurable: true, writable: true, value: browserWindow.history });
  return browserWindow;
};

test('parses comma-separated multi-select filters from the URL', () => {
  setWindowLocation('http://localhost/recordings?stream=cam1,cam2&detection_label=person,car&tag=urgent,review&capture_method=scheduled,manual&detection=1');

  const state = readUrlState();

  expect(state?.filters.streamIds).toEqual(['cam1', 'cam2']);
  expect(state?.filters.detectionLabels).toEqual(['person', 'car']);
  expect(state?.filters.tags).toEqual(['urgent', 'review']);
  expect(state?.filters.captureMethods).toEqual(['scheduled', 'manual']);
  expect(state?.filters.recordingType).toBe('detection');
});

test('builds one active-filter badge per selected value', () => {
  const activeFilters = urlUtils.getActiveFiltersDisplay({
    ...urlUtils.createDefaultFilters(),
    streamIds: ['cam1', 'cam2'],
    detectionLabels: ['person'],
    tags: ['urgent', 'review'],
    captureMethods: ['manual']
  });

  expect(activeFilters).toEqual([
    { key: 'streamIds', value: 'cam1', label: 'Stream: cam1' },
    { key: 'streamIds', value: 'cam2', label: 'Stream: cam2' },
    { key: 'detectionLabels', value: 'person', label: 'Object: person' },
    { key: 'tags', value: 'urgent', label: 'Tag: urgent' },
    { key: 'tags', value: 'review', label: 'Tag: review' },
    { key: 'captureMethods', value: 'manual', label: 'Capture: Manual' }
  ]);
});

test('serializes multi-select filters back into the URL', () => {
  const browserWindow = setWindowLocation('http://localhost/recordings');

  urlUtils.updateUrlWithFilters(
    {
      ...urlUtils.createDefaultFilters(),
      streamIds: ['cam1', 'cam2'],
      detectionLabels: ['person', 'car'],
      tags: ['urgent'],
      captureMethods: ['scheduled', 'manual']
    },
    { currentPage: 2, pageSize: 50 },
    'start_time',
    'desc'
  );

  const params = browserWindow.location.searchParams;
  expect(params.get('stream')).toBe('cam1,cam2');
  expect(params.get('detection_label')).toBe('person,car');
  expect(params.get('tag')).toBe('urgent');
  expect(params.get('capture_method')).toBe('scheduled,manual');
  expect(params.get('page')).toBe('2');
  expect(params.get('limit')).toBe('50');
});

test('supports the all-items pagination mode in URL state', () => {
  const browserWindow = setWindowLocation('http://localhost/recordings?limit=all&page=4');

  const state = readUrlState();
  expect(state?.limit).toBe(20);
  expect(state?.showAll).toBe(true);
  expect(state?.page).toBe(1);

  urlUtils.updateUrlWithFilters(
    urlUtils.createDefaultFilters(),
    { currentPage: 1, pageSize: 20, showAll: true },
    'start_time',
    'desc'
  );

  const params = browserWindow.location.searchParams;
  expect(params.get('limit')).toBe('all');
  expect(params.get('page')).toBe('1');
});
