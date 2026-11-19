/**
 * Last known image behind a live tile's error overlay (#974): the recording
 * fallback, the one-shot frame capture, the caption time, the per-browser
 * preference, and the controller that bounds network cost per tile.
 */
import {
  LAST_FRAME_OVERLAY_BACKGROUND,
  LAST_FRAME_RECORDING_TTL_MS,
  canvasToBlob,
  captureVideoFrame,
  createLastFrameController,
  formatLastImageTime,
  lastFrameOverlayBackground,
  loadRecordingFallback,
  pickNewestCompletedRecording,
  recordingEndMilliseconds,
  recordingThumbnailUrl,
  recordingsListUrl,
} from '../js/components/preact/live/lastFrame.ts';
import {
  DEFAULT_LIVE_LAST_IMAGE,
  LIVE_LAST_IMAGE_KEY,
  readLiveLastImage,
  writeLiveLastImage,
} from '../js/components/preact/useLiveLastImage.ts';

const flush = async () => {
  for (let i = 0; i < 10; i++) await new Promise((resolve) => setImmediate(resolve));
};

const recording = (id, endUnix, extra = {}) => ({
  id,
  end_time_unix: endUnix,
  end_time: endUnix ? new Date(endUnix * 1000).toISOString() : null,
  in_progress: false,
  ...extra,
});

function httpError(status) {
  const error = new Error(`HTTP ${status}`);
  error.status = status;
  return error;
}

describe('recording fallback', () => {
  test('builds the existing list and thumbnail endpoints', () => {
    expect(recordingsListUrl('Garage Cam')).toBe('/api/recordings?stream=Garage+Cam&sort=start_time&order=desc&limit=5');
    expect(recordingThumbnailUrl(42, 2)).toBe('/api/recordings/thumbnail/42/2');
  });

  test('picks the newest recording with end_time > 0 and skips in-progress ones', () => {
    const recordings = [
      recording(9, 0, { in_progress: true, end_time: null, end_time_unix: null }),
      recording(8, 1700000300, { in_progress: true }),
      recording(7, 1700000200),
      recording(6, 1700000100),
    ];
    expect(pickNewestCompletedRecording(recordings).id).toBe(7);
    // Does not rely on the server's ordering.
    expect(pickNewestCompletedRecording([recording(1, 100), recording(2, 300), recording(3, 200)]).id).toBe(2);
    expect(pickNewestCompletedRecording([recording(1, 0)])).toBeNull();
    expect(pickNewestCompletedRecording([])).toBeNull();
    expect(pickNewestCompletedRecording(undefined)).toBeNull();
    expect(recordingEndMilliseconds(recording(1, 1700000000))).toBe(1700000000 * 1000);
    expect(recordingEndMilliseconds({ end_time_unix: null, end_time: null })).toBe(0);
  });

  test('uses thumbnail index 2 with the recording end_time as the caption time', async () => {
    const fetchRecordings = jest.fn().mockResolvedValue({ recordings: [recording(7, 1700000200)] });
    const loadThumbnail = jest.fn().mockResolvedValue('ok');
    await expect(loadRecordingFallback('garage', { fetchRecordings, loadThumbnail })).resolves.toEqual({
      url: '/api/recordings/thumbnail/7/2',
      capturedAt: 1700000200 * 1000,
      source: 'recording',
    });
    expect(fetchRecordings).toHaveBeenCalledTimes(1);
    expect(fetchRecordings).toHaveBeenCalledWith('garage');
    expect(loadThumbnail).toHaveBeenCalledTimes(1);
  });

  test.each([400, 404])('falls back from index 2 to index 0 on %s', async (status) => {
    const fetchRecordings = jest.fn().mockResolvedValue({ recordings: [recording(7, 1700000200)] });
    const loadThumbnail = jest.fn()
      .mockRejectedValueOnce(httpError(status))
      .mockResolvedValueOnce('ok');
    const image = await loadRecordingFallback('garage', { fetchRecordings, loadThumbnail });
    expect(loadThumbnail.mock.calls.map(([url]) => url)).toEqual([
      '/api/recordings/thumbnail/7/2',
      '/api/recordings/thumbnail/7/0',
    ]);
    expect(image.url).toBe('/api/recordings/thumbnail/7/0');
  });

  test('returns nothing when every thumbnail request fails', async () => {
    const fetchRecordings = jest.fn().mockResolvedValue({ recordings: [recording(7, 1700000200)] });
    const loadThumbnail = jest.fn().mockRejectedValue(httpError(404));
    await expect(loadRecordingFallback('garage', { fetchRecordings, loadThumbnail })).resolves.toBeNull();
    expect(loadThumbnail).toHaveBeenCalledTimes(2);
  });

  test('returns nothing, without thumbnail requests, when there are no recordings', async () => {
    const loadThumbnail = jest.fn();
    await expect(loadRecordingFallback('garage', {
      fetchRecordings: jest.fn().mockResolvedValue({ recordings: [] }),
      loadThumbnail,
    })).resolves.toBeNull();
    await expect(loadRecordingFallback('garage', {
      fetchRecordings: jest.fn().mockResolvedValue({ recordings: [recording(9, 0, { in_progress: true })] }),
      loadThumbnail,
    })).resolves.toBeNull();
    await expect(loadRecordingFallback('garage', {
      fetchRecordings: jest.fn().mockRejectedValue(httpError(500)),
      loadThumbnail,
    })).resolves.toBeNull();
    expect(loadThumbnail).not.toHaveBeenCalled();
  });
});

describe('frame capture', () => {
  function fakeDocument({ drawImage = jest.fn(), context = true } = {}) {
    const canvas = {
      width: 0,
      height: 0,
      getContext: jest.fn(() => (context ? { drawImage } : null)),
    };
    return { doc: { createElement: jest.fn(() => canvas) }, canvas, drawImage };
  }

  test('skips quietly when the video has no decoded frame', () => {
    const { doc } = fakeDocument();
    expect(captureVideoFrame({ videoWidth: 0, videoHeight: 0, readyState: 4 }, doc)).toBeNull();
    expect(captureVideoFrame({ videoWidth: 640, videoHeight: 360, readyState: 1 }, doc)).toBeNull();
    expect(captureVideoFrame(null, doc)).toBeNull();
    expect(captureVideoFrame({ videoWidth: 640, videoHeight: 360, readyState: 4 }, null)).toBeNull();
    expect(doc.createElement).not.toHaveBeenCalled();
  });

  test('draws the current frame once, scaled to the maximum width', () => {
    const { doc, canvas, drawImage } = fakeDocument();
    const video = { videoWidth: 2560, videoHeight: 1440, readyState: 4 };
    expect(captureVideoFrame(video, doc, 1280)).toBe(canvas);
    expect(canvas.width).toBe(1280);
    expect(canvas.height).toBe(720);
    expect(drawImage).toHaveBeenCalledTimes(1);
    expect(drawImage).toHaveBeenCalledWith(video, 0, 0, 1280, 720);
  });

  test('never throws when drawing fails or there is no 2D context', () => {
    const throwing = fakeDocument({ drawImage: jest.fn(() => { throw new Error('InvalidStateError'); }) });
    const video = { videoWidth: 640, videoHeight: 360, readyState: 4 };
    expect(() => captureVideoFrame(video, throwing.doc)).not.toThrow();
    expect(captureVideoFrame(video, throwing.doc)).toBeNull();
    expect(captureVideoFrame(video, fakeDocument({ context: false }).doc)).toBeNull();
  });

  test('a tainted canvas resolves to null instead of throwing', async () => {
    const tainted = { toBlob: () => { throw new Error('SecurityError'); } };
    await expect(canvasToBlob(tainted)).resolves.toBeNull();
    await expect(canvasToBlob(null)).resolves.toBeNull();
    await expect(canvasToBlob({ toBlob: (done) => done(null) })).resolves.toBeNull();
    const blob = { size: 10 };
    await expect(canvasToBlob({ toBlob: (done) => done(blob) })).resolves.toBe(blob);
  });
});

describe('caption time', () => {
  const now = new Date(2026, 10, 14, 22, 50).getTime();

  test('shows only the time for an image from today', () => {
    const today = new Date(2026, 10, 14, 22, 41).getTime();
    const expected = new Intl.DateTimeFormat('en-US', { hour: 'numeric', minute: '2-digit' }).format(new Date(today));
    expect(formatLastImageTime(today, 'en-US', now)).toBe(expected);
  });

  test('includes the date when the image is not from today', () => {
    const yesterday = new Date(2026, 10, 13, 22, 41).getTime();
    const text = formatLastImageTime(yesterday, 'en-US', now);
    expect(text).toContain('Nov');
    expect(text).toContain('13');
    expect(text).not.toContain('2026');
    expect(formatLastImageTime(new Date(2025, 10, 13, 22, 41).getTime(), 'en-US', now)).toContain('2025');
  });

  test('tolerates an unknown locale and a bad timestamp', () => {
    expect(() => formatLastImageTime(now, 'not a locale!', now)).not.toThrow();
    expect(formatLastImageTime(NaN, 'en-US', now)).toBe('');
  });

  test('the overlay background only turns semi-transparent with an image', () => {
    expect(lastFrameOverlayBackground(null, 'rgba(0, 0, 0, 0.8)')).toBe('rgba(0, 0, 0, 0.8)');
    expect(lastFrameOverlayBackground({ url: 'x', capturedAt: 1, source: 'frame' }, 'rgba(0, 0, 0, 0.8)'))
      .toBe(LAST_FRAME_OVERLAY_BACKGROUND);
  });
});

describe('last image preference', () => {
  function memoryStorage() {
    const values = new Map();
    return { getItem: (key) => values.get(key) ?? null, setItem: (key, value) => values.set(key, value) };
  }

  test('defaults to on and persists per browser', () => {
    const storage = memoryStorage();
    expect(DEFAULT_LIVE_LAST_IMAGE).toBe(true);
    expect(readLiveLastImage(storage)).toBe(true);
    writeLiveLastImage(false, storage);
    expect(storage.getItem(LIVE_LAST_IMAGE_KEY)).toBe('false');
    expect(readLiveLastImage(storage)).toBe(false);
    writeLiveLastImage(true, storage);
    expect(readLiveLastImage(storage)).toBe(true);
    storage.setItem(LIVE_LAST_IMAGE_KEY, 'garbage');
    expect(readLiveLastImage(storage)).toBe(true);
  });

  test('tolerates storage that throws', () => {
    const broken = { getItem: () => { throw new Error('disabled'); }, setItem: () => { throw new Error('disabled'); } };
    expect(readLiveLastImage(broken)).toBe(true);
    expect(() => writeLiveLastImage(false, broken)).not.toThrow();
  });
});

describe('last frame controller', () => {
  const RECORDING_IMAGE = { url: '/api/recordings/thumbnail/7/2', capturedAt: 1700000200000, source: 'recording' };
  const playingVideo = { videoWidth: 640, videoHeight: 360, readyState: 4 };

  function setup({ recordingImage = RECORDING_IMAGE, blob = { size: 10 }, enabled = true } = {}) {
    let clock = 1000;
    let urlCount = 0;
    const changes = [];
    const deps = {
      loadRecordingFallback: jest.fn().mockResolvedValue(recordingImage),
      captureFrame: jest.fn((video) => (video && video.videoWidth ? { canvas: true } : null)),
      toBlob: jest.fn().mockResolvedValue(blob),
      createObjectURL: jest.fn(() => `blob:frame-${++urlCount}`),
      revokeObjectURL: jest.fn(),
      now: () => clock,
    };
    const controller = createLastFrameController((image) => changes.push(image), deps, enabled);
    controller.setStream('garage');
    return {
      controller,
      deps,
      changes,
      latest: () => (changes.length ? changes[changes.length - 1] : null),
      advance: (ms) => { clock += ms; },
      at: () => clock,
    };
  }

  test('a tile that played shows its last frame, stamped with the capture time', async () => {
    const t = setup();
    t.controller.recovered(); // "playing"
    t.advance(5000);
    const capturedAt = t.at();
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.latest()).toEqual({ url: 'blob:frame-1', capturedAt, source: 'frame' });
    expect(t.deps.captureFrame).toHaveBeenCalledTimes(1);
    expect(t.deps.loadRecordingFallback).not.toHaveBeenCalled();
  });

  test('a frame captured at teardown, before the error, is the one shown', async () => {
    const t = setup();
    t.controller.recovered();
    expect(t.controller.capture(playingVideo)).toBe(true);
    // The cell then resets the video and raises its error.
    t.controller.setError(true, { videoWidth: 0, videoHeight: 0, readyState: 0 });
    await flush();
    expect(t.latest().source).toBe('frame');
    expect(t.deps.captureFrame).toHaveBeenCalledTimes(1);
    expect(t.deps.loadRecordingFallback).not.toHaveBeenCalled();
  });

  test('a failure-teardown capture is not encoded while the tile is still healthy', async () => {
    const t = setup();
    t.controller.recovered();
    const capturedAt = t.at();
    expect(t.controller.capture(playingVideo)).toBe(true);
    await flush();
    // Only the canvas draw happened; no JPEG encode, no object URL, nothing shown.
    expect(t.deps.toBlob).not.toHaveBeenCalled();
    expect(t.deps.createObjectURL).not.toHaveBeenCalled();
    expect(t.latest()).toBeNull();
    t.advance(3000);
    t.controller.setError(true, { videoWidth: 0, videoHeight: 0, readyState: 0 });
    await flush();
    expect(t.deps.toBlob).toHaveBeenCalledTimes(1);
    expect(t.latest()).toEqual({ url: 'blob:frame-1', capturedAt, source: 'frame' });
  });

  test('a capture followed by recovery is dropped without ever being encoded', async () => {
    const t = setup();
    t.controller.recovered();
    t.controller.capture(playingVideo);
    t.controller.recovered(); // the reconnect played again before any error
    await flush();
    expect(t.deps.toBlob).not.toHaveBeenCalled();
    expect(t.latest()).toBeNull();
    // A later outage on a reset video falls back to the recording, not the stale canvas.
    t.controller.setError(true, { videoWidth: 0, videoHeight: 0, readyState: 0 });
    await flush();
    expect(t.deps.toBlob).not.toHaveBeenCalled();
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });

  test('a tile that never played falls back to the newest recording', async () => {
    const t = setup();
    t.controller.setError(true, { videoWidth: 0, videoHeight: 0, readyState: 0 });
    await flush();
    expect(t.deps.captureFrame).not.toHaveBeenCalled();
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledTimes(1);
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledWith('garage');
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });

  test('a tainted or empty capture skips quietly to the recording fallback', async () => {
    const t = setup({ blob: null });
    t.controller.recovered();
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.deps.createObjectURL).not.toHaveBeenCalled();
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledTimes(1);
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });

  test('no recordings: nothing is shown', async () => {
    const t = setup({ recordingImage: null });
    t.controller.setError(true, null);
    await flush();
    expect(t.changes).toEqual([]);
  });

  test('does not fetch while healthy, nor refetch while already in error or across retry cycles', async () => {
    const t = setup();
    await flush();
    expect(t.deps.loadRecordingFallback).not.toHaveBeenCalled();

    t.controller.setError(true, null);
    t.controller.setError(true, null); // re-render while in error
    await flush();
    // Auto-retry clears the error, the attempt fails, and the error returns.
    for (let i = 0; i < 5; i++) {
      t.controller.setError(false);
      t.advance(10000);
      t.controller.setError(true, null);
      await flush();
    }
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledTimes(1);
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });

  test('a lookup still in flight is not duplicated by a retry cycle', async () => {
    const t = setup();
    let resolveLookup;
    t.deps.loadRecordingFallback.mockImplementation(() => new Promise((resolve) => { resolveLookup = resolve; }));
    t.controller.setError(true, null);
    await flush();
    t.controller.setError(false);
    t.controller.setError(true, null);
    await flush();
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledTimes(1);
    resolveLookup(RECORDING_IMAGE);
    await flush();
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });

  test('refreshes the recording lookup only after the TTL during a long outage', async () => {
    const t = setup();
    t.controller.setError(true, null);
    await flush();
    t.controller.setError(false);
    t.advance(LAST_FRAME_RECORDING_TTL_MS + 1);
    t.controller.setError(true, null);
    await flush();
    expect(t.deps.loadRecordingFallback).toHaveBeenCalledTimes(2);
  });

  test('the image is hidden while the error is cleared and removed when the stream plays', async () => {
    const t = setup();
    t.controller.recovered();
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.latest().url).toBe('blob:frame-1');

    t.controller.setError(false);
    expect(t.latest()).toBeNull();
    t.controller.recovered();
    expect(t.deps.revokeObjectURL).toHaveBeenCalledWith('blob:frame-1');

    // The next outage captures a fresh frame.
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.latest().url).toBe('blob:frame-2');
  });

  test('a retry does not re-stamp the frozen frame with a new time', async () => {
    const t = setup();
    t.controller.recovered();
    t.controller.setError(true, playingVideo);
    await flush();
    const first = t.latest();
    t.controller.setError(false);
    t.advance(60000);
    // The retry's teardown and the next error see the same frozen frame.
    expect(t.controller.capture(playingVideo)).toBe(false);
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.latest()).toBe(first);
    expect(t.deps.captureFrame).toHaveBeenCalledTimes(1);
  });

  test('disabled: no capture and no fetch at all', async () => {
    const t = setup({ enabled: false });
    t.controller.recovered();
    expect(t.controller.capture(playingVideo)).toBe(false);
    t.controller.setError(true, playingVideo);
    await flush();
    expect(t.deps.captureFrame).not.toHaveBeenCalled();
    expect(t.deps.loadRecordingFallback).not.toHaveBeenCalled();
    expect(t.changes).toEqual([]);
  });

  test('turning the toggle off removes the image and revokes the frame URL', async () => {
    const t = setup();
    t.controller.recovered();
    t.controller.setError(true, playingVideo);
    await flush();
    t.controller.setEnabled(false);
    expect(t.latest()).toBeNull();
    expect(t.deps.revokeObjectURL).toHaveBeenCalledWith('blob:frame-1');
  });

  test('results that arrive after recovery or unmount are dropped', async () => {
    const t = setup();
    let resolveLookup;
    t.deps.loadRecordingFallback.mockImplementation(() => new Promise((resolve) => { resolveLookup = resolve; }));
    t.controller.setError(true, null);
    await flush();
    t.controller.setError(false);
    t.controller.recovered();
    resolveLookup(RECORDING_IMAGE);
    await flush();
    expect(t.changes).toEqual([]);

    const u = setup();
    let resolveBlob;
    const blobPromise = new Promise((resolve) => { resolveBlob = resolve; });
    u.deps.toBlob.mockImplementation(() => blobPromise);
    u.controller.recovered();
    u.controller.setError(true, playingVideo);
    await flush();
    u.controller.dispose();
    resolveBlob(new Blob(['frame'], { type: 'image/png' }));
    await flush();
    expect(u.deps.createObjectURL).not.toHaveBeenCalled();
    expect(u.changes).toEqual([]);

    const v = setup();
    let resolveUnmountLookup;
    v.deps.loadRecordingFallback.mockImplementation(
      () => new Promise((resolve) => { resolveUnmountLookup = resolve; }),
    );
    v.controller.setError(true, null);
    await flush();
    v.controller.dispose();
    resolveUnmountLookup(RECORDING_IMAGE);
    await flush();
    expect(v.changes).toEqual([]);
  });

  test('switching streams discards the previous camera\'s image', async () => {
    const t = setup();
    t.controller.recovered();
    t.controller.setError(true, playingVideo);
    await flush();
    t.controller.setStream('porch');
    await flush();
    expect(t.deps.revokeObjectURL).toHaveBeenCalledWith('blob:frame-1');
    expect(t.deps.loadRecordingFallback).toHaveBeenLastCalledWith('porch');
    expect(t.latest()).toEqual(RECORDING_IMAGE);
  });
});

describe('cells capture only on failure paths', () => {
  const fs = require('fs');
  const path = require('path');
  const read = (name) => fs.readFileSync(path.join(__dirname, '../js/components/preact', name), 'utf8');
  const between = (source, start, end) => {
    const from = source.indexOf(start);
    expect(from).toBeGreaterThanOrEqual(0);
    const to = source.indexOf(end, from + start.length);
    expect(to).toBeGreaterThan(from);
    return source.slice(from, to);
  };

  // These teardowns also run on healthy setting changes (audioDisabled,
  // useSubStream, ...), so they must never capture (#975 review).
  test('generic teardown paths do not capture', () => {
    expect(between(read('MSEVideoCell.tsx'), 'const cleanup = () => {', '\n  };')).not.toContain('lastFrame.capture');
    expect(between(read('WebRTCVideoCell.tsx'), 'const cleanupWebRTCResources = () => {', 'return cleanupWebRTCResources'))
      .not.toContain('lastFrame.capture');
    expect(between(read('HLSVideoCell.tsx'), '// Destroy HLS.js instance', 'hlsPlayerRef.current = null;'))
      .not.toContain('lastFrame.capture');
  });

  // The controller tests above cover capture before failure teardown and
  // recovery. A source-text call count would fail whenever a valid failure
  // path is added without catching a behavior regression.
});
