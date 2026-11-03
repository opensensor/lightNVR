import {
  getPtzCapabilities, savePtzPreset, setPtzHome, ptzMove, ptzStop, ptzHome, ptzGotoPreset
} from '../js/components/preact/ptzActions.ts';

const originalFetch = global.fetch;

describe('PTZ position actions', () => {
  afterEach(() => {
    global.fetch = originalFetch;
  });

  test('saves a named preset and refreshes the list for immediate use', async () => {
    const updatedPresets = [{ token: 'preset-42', name: 'Front door' }];
    global.fetch = jest.fn()
      .mockResolvedValueOnce({ ok: true, json: async () => ({ success: true, token: 'preset-42' }) })
      .mockResolvedValueOnce({ ok: true, json: async () => ({ presets: updatedPresets }) });
    const onPresets = jest.fn();

    await savePtzPreset('Front door / 1', 'Front door', onPresets);

    expect(global.fetch).toHaveBeenNthCalledWith(1,
      '/api/streams/Front%20door%20%2F%201/ptz/preset', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: 'Front door' })
      });
    expect(global.fetch).toHaveBeenNthCalledWith(2,
      '/api/streams/Front%20door%20%2F%201/ptz/presets', undefined);
    expect(onPresets).toHaveBeenCalledWith(updatedPresets);
  });

  test('sets the current position as home', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: true, json: async () => ({ success: true })
    });

    await setPtzHome('Lobby');

    expect(global.fetch).toHaveBeenCalledWith(
      '/api/streams/Lobby/ptz/set-home', { method: 'POST' });
  });

  test('loads the capability fields that gate the controls', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: true, json: async () => ({ has_presets: false, has_home_position: true })
    });

    await expect(getPtzCapabilities('Lobby')).resolves.toMatchObject({
      has_presets: false, has_home_position: true
    });
    expect(global.fetch).toHaveBeenCalledWith('/api/streams/Lobby/ptz/capabilities', undefined);
  });

  test('exposes a 403 so the panel can hide both actions', async () => {
    global.fetch = jest.fn().mockResolvedValue({ ok: false, status: 403 });
    const onPresets = jest.fn();

    await expect(savePtzPreset('Lobby', 'Front door', onPresets))
      .rejects.toMatchObject({ status: 403 });
    expect(onPresets).not.toHaveBeenCalled();
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  test('does not replace presets when refresh fails after saving', async () => {
    global.fetch = jest.fn()
      .mockResolvedValueOnce({ ok: true, json: async () => ({ success: true }) })
      .mockResolvedValueOnce({ ok: false, status: 502 });
    const onPresets = jest.fn();

    await expect(savePtzPreset('Lobby', 'Front door', onPresets))
      .rejects.toMatchObject({ status: 502, stage: 'refresh' });
    expect(onPresets).not.toHaveBeenCalled();
  });

  test('movement actions post to the right endpoints', async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: true, json: async () => ({ success: true })
    });

    await ptzMove('Lobby', 0.5, -0.25, 0);
    await ptzStop('Lobby');
    await ptzHome('Lobby');
    await ptzGotoPreset('Lobby', 'preset-1');

    expect(global.fetch).toHaveBeenNthCalledWith(1, '/api/streams/Lobby/ptz/move', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ pan: 0.5, tilt: -0.25, zoom: 0 })
    });
    expect(global.fetch).toHaveBeenNthCalledWith(2, '/api/streams/Lobby/ptz/stop', { method: 'POST' });
    expect(global.fetch).toHaveBeenNthCalledWith(3, '/api/streams/Lobby/ptz/home', { method: 'POST' });
    expect(global.fetch).toHaveBeenNthCalledWith(4, '/api/streams/Lobby/ptz/goto-preset', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: 'preset-1' })
    });
  });

  test('movement actions reject on HTTP errors instead of parsing the body', async () => {
    const json = jest.fn();
    global.fetch = jest.fn().mockResolvedValue({ ok: false, status: 500, json });

    await expect(ptzMove('Lobby', 1, 0, 0)).rejects.toMatchObject({ status: 500 });
    await expect(ptzGotoPreset('Lobby', 'x')).rejects.toMatchObject({ status: 500 });
    expect(json).not.toHaveBeenCalled();
  });
});
