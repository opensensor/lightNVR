import {
  EPTZ_VIEW_MODES,
  applyEptzLayoutSlot,
  eptzLayoutSlot,
  eptzPresetPayload,
  eptzViewFromPreset,
  normalizeEptzView,
  readEptzView,
  writeEptzView,
} from '../js/utils/eptz-view.js';

describe('operator ePTZ view state', () => {
  const storage = new Map();
  beforeAll(() => {
    global.localStorage = {
      getItem: (key) => storage.get(key) ?? null,
      setItem: (key, value) => storage.set(key, String(value)),
      removeItem: (key) => storage.delete(key),
      clear: () => storage.clear(),
    };
  });
  beforeEach(() => localStorage.clear());

  test('normalizes all modern renderer modes and dual views', () => {
    expect(EPTZ_VIEW_MODES).toEqual(['raw', 'dewarp', 'panorama', 'dual']);
    expect(normalizeEptzView({
      mode: 'dual',
      primary: { yaw: 200, tilt: -50, fov: 70 },
      secondary: { yaw: -200, tilt: -35, fov: 60 },
    })).toEqual({
      mode: 'dual',
      presetUuid: '',
      primary: { yaw: -160, tilt: -50, fov: 70 },
      secondary: { yaw: 160, tilt: -35, fov: 60 },
    });
  });

  test('round-trips a server preset and payload', () => {
    const state = eptzViewFromPreset({
      uuid: 'preset-1', mode: 'panorama', yaw: 12, tilt: -40, view_fov: 80,
      secondary_yaw: -168, secondary_tilt: -35, secondary_view_fov: 65,
    });
    expect(eptzPresetPayload(' Lobby ', true, state)).toEqual({
      name: 'Lobby', is_shared: true, mode: 'panorama',
      yaw: 12, tilt: -40, view_fov: 80,
      secondary_yaw: -168, secondary_tilt: -35, secondary_view_fov: 65,
    });
  });

  test('persists ePTZ state inside saved Live layout slots', () => {
    const stream = { camera_uuid: 'camera-a', name: 'Lobby' };
    writeEptzView(stream.camera_uuid, stream.name, {
      mode: 'dual', presetUuid: 'preset-a',
      primary: { yaw: 15, tilt: -45, fov: 70 },
      secondary: { yaw: -165, tilt: -40, fov: 65 },
    });
    const slot = eptzLayoutSlot(stream);
    localStorage.clear();
    applyEptzLayoutSlot(slot, stream);
    expect(readEptzView(stream.camera_uuid, stream.name)).toMatchObject({
      mode: 'dual', presetUuid: 'preset-a',
      primary: { yaw: 15, tilt: -45, fov: 70 },
      secondary: { yaw: -165, tilt: -40, fov: 65 },
    });
  });
});
