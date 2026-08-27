export const DEFAULT_EPTZ_CONFIG = Object.freeze({
  version: 1,
  projection: 'equidistant',
  mount: 'ceiling',
  centerX: 0.5,
  centerY: 0.5,
  radius: 0.5,
  fov: 190,
  rotation: 0,
  defaultYaw: 0,
  defaultTilt: -45,
  defaultViewFov: 75,
});

const clamp = (value, minimum, maximum, fallback) => {
  const number = Number(value);
  return Number.isFinite(number)
    ? Math.min(maximum, Math.max(minimum, number))
    : fallback;
};

export function parseEptzConfig(raw) {
  if (!raw) return { enabled: false, ...DEFAULT_EPTZ_CONFIG };

  try {
    const value = typeof raw === 'string' ? JSON.parse(raw) : raw;
    if (!value || value.version !== 1 || value.projection !== 'equidistant' ||
        value.mount !== 'ceiling') {
      return { enabled: false, ...DEFAULT_EPTZ_CONFIG };
    }
    return {
      enabled: true,
      ...DEFAULT_EPTZ_CONFIG,
      centerX: clamp(value.centerX, 0, 1, DEFAULT_EPTZ_CONFIG.centerX),
      centerY: clamp(value.centerY, 0, 1, DEFAULT_EPTZ_CONFIG.centerY),
      radius: clamp(value.radius, 0.05, 1, DEFAULT_EPTZ_CONFIG.radius),
      fov: clamp(value.fov, 160, 360, DEFAULT_EPTZ_CONFIG.fov),
      rotation: clamp(value.rotation, -360, 360, DEFAULT_EPTZ_CONFIG.rotation),
      defaultYaw: clamp(value.defaultYaw, -360, 360, DEFAULT_EPTZ_CONFIG.defaultYaw),
      defaultTilt: clamp(value.defaultTilt, -90, 30, DEFAULT_EPTZ_CONFIG.defaultTilt),
      defaultViewFov: clamp(value.defaultViewFov, 20, 120, DEFAULT_EPTZ_CONFIG.defaultViewFov),
    };
  } catch {
    return { enabled: false, ...DEFAULT_EPTZ_CONFIG };
  }
}

export function serializeEptzConfig(value) {
  if (!value?.enabled) return '';
  const normalized = parseEptzConfig({
    ...DEFAULT_EPTZ_CONFIG,
    ...value,
    version: 1,
    projection: 'equidistant',
    mount: 'ceiling',
  });
  const { enabled: _enabled, ...document } = normalized;
  return JSON.stringify(document);
}

export function isEptzEnabled(raw) {
  return parseEptzConfig(raw).enabled;
}

export function eptzFormFields(raw) {
  const config = parseEptzConfig(raw);
  return {
    eptzEnabled: config.enabled,
    eptzCenterX: config.centerX,
    eptzCenterY: config.centerY,
    eptzRadius: config.radius,
    eptzFov: config.fov,
    eptzRotation: config.rotation,
    eptzDefaultYaw: config.defaultYaw,
    eptzDefaultTilt: config.defaultTilt,
    eptzDefaultViewFov: config.defaultViewFov,
  };
}

export function eptzConfigFromForm(stream) {
  return {
    enabled: !!stream.eptzEnabled,
    centerX: stream.eptzCenterX,
    centerY: stream.eptzCenterY,
    radius: stream.eptzRadius,
    fov: stream.eptzFov,
    rotation: stream.eptzRotation,
    defaultYaw: stream.eptzDefaultYaw,
    defaultTilt: stream.eptzDefaultTilt,
    defaultViewFov: stream.eptzDefaultViewFov,
  };
}
