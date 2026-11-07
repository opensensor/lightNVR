import type { PtzActionResult, PtzCapabilities, PtzError, PtzPreset, PtzPresetsResponse } from '../../types/live.ts';

import {deviceRequest, type DeviceOperation} from '../../api/device.ts';
const operations:Record<string,DeviceOperation>={capabilities:'getPtzCapabilities',presets:'getPtzPresets',preset:'setPtzPreset','set-home':'setPtzHome',move:'movePtz',stop:'stopPtz',home:'homePtz','goto-preset':'gotoPtzPreset'};
async function ptzRequest<T extends PtzActionResult = PtzActionResult>(streamName:string,action:string,options?:RequestInit):Promise<T>{
  return await deviceRequest(operations[action]!,{name:streamName,body:typeof options?.body==='string'?JSON.parse(options.body):undefined}) as T;
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
