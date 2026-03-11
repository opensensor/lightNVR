import { forceNavigation } from '../js/utils/navigation-utils.js';

describe('navigation-utils', () => {
  const originalWindow = global.window;

  beforeEach(() => {
    jest.useFakeTimers();
    global.window = { location: { href: 'index.html' } };
  });

  afterEach(() => {
    jest.runOnlyPendingTimers();
    jest.useRealTimers();
    global.window = originalWindow;
  });

  test('defers navigation until the current event queue finishes', () => {
    const event = {
      preventDefault: jest.fn(),
      stopPropagation: jest.fn()
    };

    const result = forceNavigation('timeline.html?stream=front_door', event);

    expect(result).toBe(false);
    expect(event.preventDefault).toHaveBeenCalledTimes(1);
    expect(event.stopPropagation).toHaveBeenCalledTimes(1);
    expect(window.location.href).toBe('index.html');

    jest.runAllTimers();

    expect(window.location.href).toBe('timeline.html?stream=front_door');
  });

  test('does nothing when no destination URL is provided', () => {
    forceNavigation('', null);

    jest.runAllTimers();

    expect(window.location.href).toBe('index.html');
  });
});