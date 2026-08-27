import { parseEptzConfig } from './eptz-config.js';

export const EPTZ_VIEW_MODES = Object.freeze(['raw', 'dewarp', 'panorama', 'dual']);

const clamp = (value, minimum, maximum, fallback) => {
  const parsed = Number(value);
  return Number.isFinite(parsed)
    ? Math.min(maximum, Math.max(minimum, parsed)) : fallback;
};

export const wrapEptzDegrees = (value) =>
  ((Number(value) + 180) % 360 + 360) % 360 - 180;

export function defaultEptzView(config = {}) {
  const primaryYaw = wrapEptzDegrees(config.defaultYaw || 0);
  const tilt = clamp(config.defaultTilt, -90, 30, -45);
  const fov = clamp(config.defaultViewFov, 20, 120, 75);
  return {
    mode: 'dewarp',
    presetUuid: '',
    primary: { yaw: primaryYaw, tilt, fov },
    secondary: { yaw: wrapEptzDegrees(primaryYaw + 180), tilt, fov },
  };
}

export function normalizeEptzView(value, config = {}) {
  const fallback = defaultEptzView(config);
  return {
    mode: EPTZ_VIEW_MODES.includes(value?.mode) ? value.mode : fallback.mode,
    presetUuid: typeof value?.presetUuid === 'string' ? value.presetUuid : '',
    primary: {
      yaw: wrapEptzDegrees(value?.primary?.yaw ?? value?.yaw ?? fallback.primary.yaw),
      tilt: clamp(value?.primary?.tilt ?? value?.tilt, -90, 30, fallback.primary.tilt),
      fov: clamp(value?.primary?.fov ?? value?.fov, 20, 120, fallback.primary.fov),
    },
    secondary: {
      yaw: wrapEptzDegrees(value?.secondary?.yaw ?? value?.secondary_yaw ?? fallback.secondary.yaw),
      tilt: clamp(value?.secondary?.tilt ?? value?.secondary_tilt, -90, 30, fallback.secondary.tilt),
      fov: clamp(value?.secondary?.fov ?? value?.secondary_view_fov, 20, 120, fallback.secondary.fov),
    },
  };
}

export function eptzViewStorageKey(cameraUuid, streamName = '') {
  return `lightnvr-eptz-view:${cameraUuid || streamName || 'camera'}`;
}

export function readEptzView(cameraUuid, streamName, config = {}) {
  const fallback = defaultEptzView(config);
  if (typeof localStorage === 'undefined') return fallback;
  try {
    const current = localStorage.getItem(eptzViewStorageKey(cameraUuid, streamName));
    const legacy = streamName
      ? localStorage.getItem(eptzViewStorageKey('', streamName)) : null;
    return normalizeEptzView(JSON.parse(current || legacy || 'null'), config);
  } catch {
    return fallback;
  }
}

export function writeEptzView(cameraUuid, streamName, value, config = {}) {
  const normalized = normalizeEptzView(value, config);
  if (typeof localStorage !== 'undefined') {
    try {
      localStorage.setItem(
        eptzViewStorageKey(cameraUuid, streamName), JSON.stringify(normalized));
    } catch { /* persistence is best effort */ }
  }
  return normalized;
}

export function eptzViewFromPreset(preset, config = {}) {
  return normalizeEptzView({
    mode: preset?.mode,
    presetUuid: preset?.uuid || '',
    primary: {
      yaw: preset?.yaw,
      tilt: preset?.tilt,
      fov: preset?.view_fov,
    },
    secondary: {
      yaw: preset?.secondary_yaw,
      tilt: preset?.secondary_tilt,
      fov: preset?.secondary_view_fov,
    },
  }, config);
}

export function eptzPresetPayload(name, isShared, state) {
  const normalized = normalizeEptzView(state);
  return {
    name: name.trim(),
    is_shared: Boolean(isShared),
    mode: normalized.mode,
    yaw: normalized.primary.yaw,
    tilt: normalized.primary.tilt,
    view_fov: normalized.primary.fov,
    secondary_yaw: normalized.secondary.yaw,
    secondary_tilt: normalized.secondary.tilt,
    secondary_view_fov: normalized.secondary.fov,
  };
}

export function eptzLayoutSlot(stream) {
  const config = parseEptzConfig(stream?.eptz_config);
  const state = readEptzView(stream?.camera_uuid, stream?.name, config);
  return {
    camera_uuid: stream.camera_uuid,
    eptz_mode: state.mode,
    eptz_preset_uuid: state.presetUuid || null,
    eptz_view: {
      yaw: state.primary.yaw,
      tilt: state.primary.tilt,
      fov: state.primary.fov,
      secondary_yaw: state.secondary.yaw,
      secondary_tilt: state.secondary.tilt,
      secondary_view_fov: state.secondary.fov,
    },
  };
}

export function applyEptzLayoutSlot(slot, stream) {
  if (!slot || !stream) return null;
  const config = parseEptzConfig(stream.eptz_config);
  const state = writeEptzView(stream.camera_uuid, stream.name, {
    mode: slot.eptz_mode,
    presetUuid: slot.eptz_preset_uuid || '',
    primary: slot.eptz_view,
    secondary: {
      yaw: slot.eptz_view?.secondary_yaw,
      tilt: slot.eptz_view?.secondary_tilt,
      fov: slot.eptz_view?.secondary_view_fov,
    },
  }, config);
  if (typeof window !== 'undefined') {
    window.dispatchEvent(new CustomEvent('lightnvr:eptz-layout-applied', {
      detail: { cameraUuid: stream.camera_uuid, state },
    }));
  }
  return state;
}
