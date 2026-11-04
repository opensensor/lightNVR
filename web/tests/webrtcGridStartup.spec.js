// Exercise the real cell's startup effect and shared gate without a camera or
// browser. Only DOM/RTC/network boundaries and unrelated UI hooks are stubbed.
import path from 'node:path';
import { buildSync } from 'esbuild';
import * as gateModule from '../js/utils/stream-connection-gate.ts';

const output = buildSync({
  absWorkingDir: path.resolve(__dirname, '..'),
  entryPoints: ['js/components/preact/WebRTCVideoCell.tsx'],
  bundle: true, format: 'cjs', platform: 'node', external: ['*'],
  jsx: 'automatic', jsxImportSource: 'preact', write: false,
}).outputFiles[0].text;

let gate, connectionMs, cells, peers, requests;
const noop = () => {};
const t = key => key;

class Peer {
  constructor() {
    this.iceConnectionState = 'new';
    this.iceGatheringState = 'complete';
    this.signalingState = 'stable';
    this.setRemoteDescription = jest.fn(async () => {});
    peers.push(this);
  }
  addTransceiver() { return {}; }
  async createOffer() { return { type: 'offer', sdp: 'offer' }; }
  async setLocalDescription(offer) { this.localDescription = offer; }
  async getStats() { return new Map(); }
  close() { this.signalingState = 'closed'; this.iceConnectionState = 'closed'; }
}

function mount(name) {
  const state = [], effects = [], cleanups = [];
  const hooks = {
    useState(initial) {
      const cell = { value: typeof initial === 'function' ? initial() : initial };
      state.push(cell);
      return [cell.value, next => { cell.value = typeof next === 'function' ? next(cell.value) : next; }];
    },
    useRef: current => ({ current }),
    useCallback: fn => fn,
    useEffect: fn => effects.push(fn),
  };
  const stubs = {
    'preact/hooks': hooks,
    'webrtc-adapter': {},
    'i18n.ts': { useI18n: () => ({ t }) },
    'query-client.ts': { useQueryClient: () => ({}) },
    'settings-utils.ts': {
      getGo2rtcBaseUrl: async () => '/go2rtc',
      getWebRTCTimeouts: async () => ({ connectionMs, iceRecoveryMs: 5000 }),
    },
    'stream-connection-gate.ts': { ...gateModule, streamConnectionGate: gate },
    'useVideoZoom.ts': { useVideoZoom: () => ({}) },
    'MobileTileGestures.tsx': { useMobileTileGestures: () => ({}) },
    'useAutoRetry.ts': { useAutoRetry: noop },
    'player-telemetry.ts': { createPlayerTelemetry: () => ({ destroy: noop }) },
  };
  const loaded = { exports: {} };
  const componentDir = path.resolve(__dirname, '../js/components/preact');
  Function('module', 'exports', 'require', output)(loaded, loaded.exports, id => {
    if (id === 'preact/jsx-runtime') return require(id);
    const stub = stubs[id] || stubs[path.basename(id)];
    if (stub) return stub;
    if (id.endsWith('.tsx')) return {};
    return require(path.resolve(componentDir, id));
  });
  const tree = loaded.exports.WebRTCVideoCell({
    stream: { name, status: 'Running', codec: 'hevc' }, streamId: name, audioDisabled: true,
  });
  const attachVideo = node => {
    if (!node || typeof node !== 'object') return;
    if (Array.isArray(node)) return node.forEach(attachVideo);
    if (node.type === 'video') node.ref.current = {
      srcObject: null, addEventListener: noop, removeEventListener: noop,
    };
    attachVideo(node.props?.children);
  };
  attachVideo(tree);
  effects.forEach(fn => { const cleanup = fn(); if (cleanup) cleanups.push(cleanup); });
  const cell = { state, unmount: () => cleanups.splice(0).forEach(fn => fn()) };
  cells.push(cell);
  return cell;
}

function deferred(signal) {
  let resolve, reject;
  const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
  const abort = () => reject(new DOMException('Aborted', 'AbortError'));
  if (signal.aborted) abort();
  else signal.addEventListener('abort', abort, { once: true });
  return { promise, resolve, reject };
}

function respond(request, { status = 200, body = 'answer' } = {}) {
  request.resolve({ ok: status === 200, status, text: async () => body });
}

beforeEach(() => {
  jest.useFakeTimers();
  gate = new gateModule.StreamConnectionGate(6, 10000);
  connectionMs = 30000;
  cells = []; peers = []; requests = [];
  jest.spyOn(globalThis, 'fetch').mockImplementation(async (url, { signal }) => {
    if (url === '/api/ice-servers') return { ok: true, json: async () => ({ ice_servers: [] }) };
    const request = { ...deferred(signal), url, signal };
    requests.push(request);
    return request.promise;
  });
  globalThis.RTCPeerConnection = Peer;
  globalThis.RTCSessionDescription = class { constructor(value) { Object.assign(this, value); } };
  for (const method of ['log', 'warn', 'error', 'debug']) jest.spyOn(console, method).mockImplementation(noop);
});

afterEach(async () => {
  cells.forEach(cell => cell.unmount());
  await jest.advanceTimersByTimeAsync(0);
  expect(gate.getStats()).toEqual({ active: 0, queued: 0 });
  jest.clearAllTimers();
  jest.useRealTimers();
  jest.restoreAllMocks();
  delete globalThis.RTCPeerConnection;
  delete globalThis.RTCSessionDescription;
});

test('four cameras can finish signaling after ten seconds within the configured deadline', async () => {
  ['2k-a', '2k-b', '4k-a', '4k-b'].forEach(mount);
  await jest.advanceTimersByTimeAsync(0);
  expect(requests).toHaveLength(4);
  respond(requests[0]); respond(requests[1]);
  await jest.advanceTimersByTimeAsync(12000);
  expect(requests.slice(2).every(request => !request.signal.aborted)).toBe(true);
  respond(requests[2]); respond(requests[3]);
  await jest.advanceTimersByTimeAsync(0);
  peers.forEach(peer => expect(peer.setRemoteDescription).toHaveBeenCalledTimes(1));
});

test('a queued camera gets its connection budget when its offer starts', async () => {
  gate = new gateModule.StreamConnectionGate(1, 60000);
  mount('first'); const queued = mount('queued');
  await jest.advanceTimersByTimeAsync(20000);
  respond(requests[0]);
  await jest.advanceTimersByTimeAsync(0);
  expect(requests).toHaveLength(2);
  await jest.advanceTimersByTimeAsync(15000);
  expect(queued.state.some(cell => cell.value === 'live.connectionTimeoutCheckNetwork')).toBe(false);
  expect(requests[1].signal.aborted).toBe(false);
  respond(requests[1]);
  await jest.advanceTimersByTimeAsync(0);
  expect(peers[1].setRemoteDescription).toHaveBeenCalledTimes(1);
});

test('the gate retains cancellation and its slot until the SDP body is read', async () => {
  mount('slow-body');
  await jest.advanceTimersByTimeAsync(0);
  const body = deferred(requests[0].signal);
  requests[0].resolve({ ok: true, status: 200, text: () => body.promise });
  await jest.advanceTimersByTimeAsync(0);
  expect(gate.getStats().active).toBe(1);
  cells[0].unmount();
  await jest.advanceTimersByTimeAsync(0);
  expect(requests[0].signal.aborted).toBe(true);
  expect(peers[0].setRemoteDescription).not.toHaveBeenCalled();
});

test('a terminal signaling failure closes the peer immediately', async () => {
  mount('failure');
  await jest.advanceTimersByTimeAsync(0);
  respond(requests[0], { status: 500, body: 'streams: codecs not matched' });
  await jest.advanceTimersByTimeAsync(0);
  expect(peers[0].signalingState).toBe('closed');
});

test('the configured connection deadline aborts requests and closes the peer', async () => {
  connectionMs = 45000;
  mount('stuck');
  await jest.advanceTimersByTimeAsync(44000);
  expect(requests[0].signal.aborted).toBe(false);
  await jest.advanceTimersByTimeAsync(1000);
  expect(requests[0].signal.aborted).toBe(true);
  expect(peers[0].signalingState).toBe('closed');
  expect(peers[0].setRemoteDescription).not.toHaveBeenCalled();
});

test('a stalled SDP body is aborted at the configured deadline', async () => {
  mount('stuck-body');
  await jest.advanceTimersByTimeAsync(0);
  const body = deferred(requests[0].signal);
  requests[0].resolve({ ok: true, status: 200, text: () => body.promise });
  await jest.advanceTimersByTimeAsync(30000);
  expect(requests[0].signal.aborted).toBe(true);
  expect(peers[0].signalingState).toBe('closed');
  expect(peers[0].setRemoteDescription).not.toHaveBeenCalled();
});

test('unmounting a queued tile prevents its offer from being sent', async () => {
  gate = new gateModule.StreamConnectionGate(1, 10000);
  mount('first'); const queued = mount('queued');
  await jest.advanceTimersByTimeAsync(0);
  expect(gate.getStats()).toEqual({ active: 1, queued: 1 });
  queued.unmount();
  respond(requests[0]);
  await jest.advanceTimersByTimeAsync(0);
  expect(requests).toHaveLength(1);
  expect(peers[1].signalingState).toBe('closed');
});

test('unmounting during retry backoff prevents any further offer', async () => {
  const cell = mount('retry');
  await jest.advanceTimersByTimeAsync(0);
  respond(requests[0], { status: 500, body: 'stream not ready' });
  await jest.advanceTimersByTimeAsync(0);
  cell.unmount();
  await jest.advanceTimersByTimeAsync(2000);
  expect(requests).toHaveLength(1);
  expect(peers[0].signalingState).toBe('closed');
});

test('the startup deadline leaves an established peer connected', async () => {
  const cell = mount('connected');
  await jest.advanceTimersByTimeAsync(0);
  respond(requests[0]);
  await jest.advanceTimersByTimeAsync(0);
  peers[0].iceConnectionState = 'connected';
  peers[0].oniceconnectionstatechange();
  await jest.advanceTimersByTimeAsync(31000);
  expect(peers[0].signalingState).toBe('stable');
  expect(cell.state.some(state => state.value === 'live.connectionTimeoutCheckNetwork')).toBe(false);
});
