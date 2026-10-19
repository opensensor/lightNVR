const ptzUrl = (streamName, action) =>
  `/api/streams/${encodeURIComponent(streamName)}/ptz/${action}`;

async function ptzRequest(streamName, action, options) {
  const response = await fetch(ptzUrl(streamName, action), options);
  if (!response.ok) {
    const error = new Error(`PTZ ${action} failed (${response.status})`);
    error.status = response.status;
    throw error;
  }

  const data = await response.json();
  if (data.success === false) {
    throw new Error(`PTZ ${action} failed`);
  }
  return data;
}

export function getPtzCapabilities(streamName) {
  return ptzRequest(streamName, 'capabilities');
}

export function getPtzPresets(streamName) {
  return ptzRequest(streamName, 'presets');
}

export async function savePtzPreset(streamName, name, onPresets) {
  const result = await ptzRequest(streamName, 'preset', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name })
  });
  let data;
  try {
    data = await getPtzPresets(streamName);
  } catch (error) {
    error.stage = 'refresh';
    throw error;
  }
  onPresets(data.presets || []);
  return result;
}

export function setPtzHome(streamName) {
  return ptzRequest(streamName, 'set-home', { method: 'POST' });
}
