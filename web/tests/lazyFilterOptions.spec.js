import {
  FILTER_OPTIONS_ERROR,
  FILTER_OPTIONS_IDLE,
  FILTER_OPTIONS_LOADED,
  FILTER_OPTIONS_LOADING,
  areFilterOptionsInteractive,
  shouldLoadFilterOptions
} from '../js/components/preact/recordings/lazyFilterOptions.js';

describe('lazy recordings filter options', () => {
  test('do not load while their accordion section is collapsed', () => {
    expect(shouldLoadFilterOptions(false, FILTER_OPTIONS_IDLE)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_IDLE)).toBe(true);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_LOADING)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_LOADED)).toBe(false);
    expect(shouldLoadFilterOptions(true, FILTER_OPTIONS_ERROR)).toBe(false);
  });

  test('keep the native select disabled until its options are present', () => {
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_IDLE)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_LOADING)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_ERROR)).toBe(false);
    expect(areFilterOptionsInteractive(FILTER_OPTIONS_LOADED)).toBe(true);
  });
});
