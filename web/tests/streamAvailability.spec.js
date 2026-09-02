import fs from 'node:fs';
import path from 'node:path';
import {
  getInitialStreamAvailability,
  hasStreamSummaryFilters,
} from '../js/utils/stream-availability.js';

describe('stream Configuration availability (#582)', () => {
  test('shows all streams by default and honors explicit URL filters', () => {
    expect(getInitialStreamAvailability('')).toBe('all');
    expect(getInitialStreamAvailability('?view=inventory')).toBe('all');
    expect(getInitialStreamAvailability('?availability=disabled')).toBe('disabled');
    expect(getInitialStreamAvailability('?availability=never_connected')).toBe('never_connected');
    expect(getInitialStreamAvailability('?availability=invalid')).toBe('all');
  });

  test('does not inherit the legacy persisted live-only filter', () => {
    const previousWindow = global.window;
    global.window = {
      location: { search: '' },
      localStorage: { getItem: jest.fn(() => 'live') },
    };

    try {
      expect(getInitialStreamAvailability()).toBe('all');
      expect(global.window.localStorage.getItem).not.toHaveBeenCalled();
    } finally {
      if (previousWindow === undefined) delete global.window;
      else global.window = previousWindow;
    }
  });

  test('distinguishes an empty inventory from an empty filtered result', () => {
    expect(hasStreamSummaryFilters('', 'all')).toBe(false);
    expect(hasStreamSummaryFilters('', 'disabled')).toBe(true);
    expect(hasStreamSummaryFilters('garage', 'all')).toBe(true);
  });

  test('keeps the availability picker outside the empty-result loader', () => {
    const source = fs.readFileSync(
      path.resolve(__dirname, '../js/components/preact/StreamsView.jsx'),
      'utf8'
    );
    const picker = source.indexOf('id="streams-availability"');
    const loader = source.indexOf('<ContentLoader', picker);

    expect(picker).toBeGreaterThan(-1);
    expect(loader).toBeGreaterThan(picker);
  });
});
