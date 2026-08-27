import {
  DEFAULT_EPTZ_CONFIG,
  eptzConfigFromForm,
  eptzFormFields,
  isEptzEnabled,
  parseEptzConfig,
  serializeEptzConfig,
} from '../js/utils/eptz-config.js';

describe('fisheye ePTZ configuration', () => {
  test('uses an empty string as the disabled representation', () => {
    expect(isEptzEnabled('')).toBe(false);
    expect(serializeEptzConfig({ enabled: false })).toBe('');
    expect(eptzFormFields('').eptzEnabled).toBe(false);
  });

  test('round-trips a version-1 equidistant ceiling calibration', () => {
    const serialized = serializeEptzConfig({
      enabled: true,
      centerX: 0.503,
      centerY: 0.491,
      radius: 0.472,
      fov: 195,
      rotation: 12,
      defaultYaw: 25,
      defaultTilt: -40,
      defaultViewFov: 68,
    });
    const parsed = parseEptzConfig(serialized);
    expect(parsed).toMatchObject({
      enabled: true,
      projection: 'equidistant',
      mount: 'ceiling',
      centerX: 0.503,
      centerY: 0.491,
      radius: 0.472,
      fov: 195,
      defaultTilt: -40,
    });
  });

  test('rejects unknown versions and projection models safely', () => {
    expect(parseEptzConfig(JSON.stringify({
      ...DEFAULT_EPTZ_CONFIG,
      version: 2,
    })).enabled).toBe(false);
    expect(parseEptzConfig(JSON.stringify({
      ...DEFAULT_EPTZ_CONFIG,
      projection: 'equisolid',
    })).enabled).toBe(false);
  });

  test('clamps calibration values before saving', () => {
    const form = eptzFormFields(serializeEptzConfig({
      enabled: true,
      centerX: 4,
      centerY: -2,
      radius: 8,
      fov: 999,
      rotation: 0,
      defaultYaw: 0,
      defaultTilt: -45,
      defaultViewFov: 1,
    }));
    const serialized = serializeEptzConfig(eptzConfigFromForm(form));
    expect(parseEptzConfig(serialized)).toMatchObject({
      centerX: 1,
      centerY: 0,
      radius: 1,
      fov: 360,
      defaultViewFov: 20,
    });
  });
});
