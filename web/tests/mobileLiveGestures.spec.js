import {
  fullscreenSwipeAction,
  resistedPullDistance,
  shouldIgnoreTileGestureTarget,
  shouldShowGestureTip,
} from '../js/components/preact/mobileLiveGestures.js';

describe('mobile live gestures', () => {
  test('classifies fullscreen navigation and exit swipes', () => {
    expect(fullscreenSwipeAction(-90, 10)).toBe('next');
    expect(fullscreenSwipeAction(90, -5)).toBe('previous');
    expect(fullscreenSwipeAction(12, 100)).toBe('exit');
    expect(fullscreenSwipeAction(35, 15)).toBeNull();
    expect(fullscreenSwipeAction(-90, 10, 1.5)).toBeNull();
  });

  test('applies resistance and a hard cap to pull distance', () => {
    expect(resistedPullDistance(-20)).toBe(0);
    expect(resistedPullDistance(80)).toBe(40);
    expect(resistedPullDistance(400)).toBe(96);
  });

  test('does not start a tile gesture from interactive chrome', () => {
    const target = {
      closest: jest.fn((selector) => selector.includes('button') ? {} : null),
    };
    expect(shouldIgnoreTileGestureTarget(target)).toBe(true);
    expect(shouldIgnoreTileGestureTarget({ closest: () => null })).toBe(false);
  });

  test('shows a contextual tip once after its threshold', () => {
    const values = new Map();
    const storage = {
      getItem: (key) => values.get(key) ?? null,
      setItem: (key, value) => values.set(key, value),
    };

    expect(shouldShowGestureTip('double-tap', 3, storage)).toBe(false);
    expect(shouldShowGestureTip('double-tap', 3, storage)).toBe(false);
    expect(shouldShowGestureTip('double-tap', 3, storage)).toBe(true);
    expect(shouldShowGestureTip('double-tap', 3, storage)).toBe(false);
  });
});
