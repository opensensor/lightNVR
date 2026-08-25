import {
  exitNativeFullscreen,
  getNativeFullscreenElement,
  requestNativeFullscreen,
} from '../js/components/preact/fullscreenApi.js';

function classListDouble() {
  const values = new Set();
  return {
    add: (value) => values.add(value),
    remove: (value) => values.delete(value),
    contains: (value) => values.has(value),
  };
}

describe('fullscreen fallback', () => {
  const originalDocument = global.document;
  const originalEvent = global.Event;

  afterEach(() => {
    global.document = originalDocument;
    global.Event = originalEvent;
  });

  test('uses a reversible pseudo-fullscreen when the native API is absent', async () => {
    const listeners = [];
    global.Event = class Event { constructor(type) { this.type = type; } };
    global.document = {
      fullscreenElement: null,
      webkitFullscreenElement: null,
      body: { classList: classListDouble() },
      dispatchEvent: (event) => listeners.push(event.type),
    };
    const element = { classList: classListDouble() };

    await requestNativeFullscreen(element);
    expect(getNativeFullscreenElement()).toBe(element);
    expect(element.classList.contains('pseudo-native-fullscreen')).toBe(true);
    expect(document.body.classList.contains('pseudo-native-fullscreen-active')).toBe(true);

    await exitNativeFullscreen();
    expect(getNativeFullscreenElement()).toBeNull();
    expect(element.classList.contains('pseudo-native-fullscreen')).toBe(false);
    expect(listeners).toEqual([
      'lightnvr:pseudo-fullscreenchange',
      'lightnvr:pseudo-fullscreenchange',
    ]);
  });

  test('falls back when a browser exposes but rejects element fullscreen', async () => {
    global.Event = class Event { constructor(type) { this.type = type; } };
    global.document = {
      fullscreenElement: null,
      webkitFullscreenElement: null,
      body: { classList: classListDouble() },
      dispatchEvent: jest.fn(),
    };
    const element = {
      classList: classListDouble(),
      requestFullscreen: () => Promise.reject(new Error('not supported')),
    };

    await expect(requestNativeFullscreen(element)).resolves.toBeUndefined();
    expect(getNativeFullscreenElement()).toBe(element);
    await exitNativeFullscreen();
  });
});
