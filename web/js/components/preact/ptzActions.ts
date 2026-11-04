import type { PtzActionResult, PtzCapabilities, PtzError, PtzPreset, PtzPresetsResponse } from '../../types/live.ts';

const ptzUrl = (streamName: string, action: string) =>
  `/api/streams/${encodeURIComponent(streamName)}/ptz/${action}`;

async function ptzRequest<T extends PtzActionResult = PtzActionResult>(streamName: string, action: string, options?: RequestInit): Promise<T> {
  const response = await fetch(ptzUrl(streamName, action), options);
  if (!response.ok) {
    const error: PtzError = new Error(`PTZ ${action} failed (${response.status})`);
    error.status = response.status;
    throw error;
  }

  const data: T = await response.json();
  if (data.success === false) {
    throw new Error(`PTZ ${action} failed`);
  }
  return data;
}

export function getPtzCapabilities(streamName: string) {
  return ptzRequest<PtzCapabilities>(streamName, 'capabilities');
}

export function getPtzPresets(streamName: string) {
  return ptzRequest<PtzPresetsResponse>(streamName, 'presets');
}

export async function savePtzPreset(streamName: string, name: string, onPresets: (presets: PtzPreset[]) => void) {
  const result = await ptzRequest(streamName, 'preset', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name })
  });
  let data: PtzPresetsResponse;
  try {
    data = await getPtzPresets(streamName);
  } catch (error) {
    (error as PtzError).stage = 'refresh';
    throw error;
  }
  onPresets(data.presets || []);
  return result;
}

export function setPtzHome(streamName: string) {
  return ptzRequest(streamName, 'set-home', { method: 'POST' });
}

const postJson = (body: unknown): RequestInit => ({
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(body)
});

export function ptzMove(streamName: string, pan: number, tilt: number, zoom: number) {
  return ptzRequest(streamName, 'move', postJson({ pan, tilt, zoom }));
}

export function ptzStop(streamName: string) {
  return ptzRequest(streamName, 'stop', { method: 'POST' });
}

export function ptzHome(streamName: string) {
  return ptzRequest(streamName, 'home', { method: 'POST' });
}

export function ptzGotoPreset(streamName: string, token: string) {
  return ptzRequest(streamName, 'goto-preset', postJson({ token }));
}
