/**
 * PlatesView behaviour tests. There is no DOM renderer in this project (see
 * jest.config.cjs), so the view is bundled with esbuild (Jest's Babel config
 * has no JSX transform), the app modules around it are stubbed, and
 * preact/hooks is replaced with a small deps-aware shim. The view is re-run as
 * a "component" until its state settles and the returned vnode tree is
 * inspected; handlers are called directly with fake events. lprSearch.js is
 * bundled for real, so requests go through the mocked global fetch.
 */
const path = require('path');
const { buildSync } = require('esbuild');

const CAMERA = '2e41848e-6400-4d49-9a18-1887e691e761';
const OTHER_CAMERA = 'a17596d5-e8c7-4100-bac7-1179c9be1dfd';
const NOW = new Date(2026, 9, 29, 12, 0, 0).getTime();

const harness = { slots: [], cursor: 0, pendingEffects: [], dirty: false };

function depsChanged(prev, next) {
  if (!prev || !next) return true;
  return prev.length !== next.length || prev.some((value, i) => !Object.is(value, next[i]));
}

const hooksShim = {
  useState: (initial) => {
    const slot = harness.cursor++;
    if (!harness.slots[slot]) {
      const cell = { value: typeof initial === 'function' ? initial() : initial };
      cell.set = (next) => {
        const resolved = typeof next === 'function' ? next(cell.value) : next;
        if (!Object.is(resolved, cell.value)) {
          cell.value = resolved;
          harness.dirty = true;
        }
      };
      harness.slots[slot] = cell;
    }
    return [harness.slots[slot].value, harness.slots[slot].set];
  },
  useRef: (initial) => {
    const slot = harness.cursor++;
    if (!harness.slots[slot]) harness.slots[slot] = { current: initial };
    return harness.slots[slot];
  },
  useMemo: (fn, deps) => {
    const slot = harness.cursor++;
    const cell = harness.slots[slot];
    if (!cell || depsChanged(cell.deps, deps)) harness.slots[slot] = { value: fn(), deps };
    return harness.slots[slot].value;
  },
  useEffect: (fn, deps) => {
    const slot = harness.cursor++;
    const cell = harness.slots[slot] || (harness.slots[slot] = { deps: undefined, cleanup: null });
    if (depsChanged(cell.deps, deps)) {
      cell.deps = deps;
      harness.pendingEffects.push(() => {
        if (cell.cleanup) cell.cleanup();
        cell.cleanup = fn() || null;
      });
    }
  },
};

const query = { data: undefined, isLoading: false, error: null };
const useQuery = jest.fn(() => query);
const fetchAllStreamSummaries = jest.fn();
const handleAuthenticationFailure = jest.fn();
const showStatusMessage = jest.fn();
function ConfirmDialog() { return null; }

function loadPlatesView() {
  const output = buildSync({
    absWorkingDir: path.resolve(__dirname, '..'),
    entryPoints: ['js/components/preact/lpr/PlatesView.jsx'],
    bundle: true,
    format: 'cjs',
    platform: 'node',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    write: false,
    logLevel: 'silent',
    external: [
      'preact/hooks',
      '*/query-client.js',
      '*/i18n.js',
      '*/fetch-utils.js',
      '*/stream-summaries.js',
      '*/ModalDialog.jsx',
      '*/ToastContainer.jsx',
    ],
  }).outputFiles[0].text;
  const stubs = {
    'preact/hooks': hooksShim,
    'query-client.js': { useQuery },
    'i18n.js': {
      useI18n: () => ({ t: (key, params) => (params ? `${key}:${JSON.stringify(params)}` : key) }),
    },
    'fetch-utils.js': { handleAuthenticationFailure },
    'stream-summaries.js': { fetchAllStreamSummaries },
    'ModalDialog.jsx': { ConfirmDialog },
    'ToastContainer.jsx': { showStatusMessage },
  };
  const requireWithStubs = (id) => {
    const stub = Object.keys(stubs).find((suffix) => id === suffix || id.endsWith(`/${suffix}`));
    return stub ? stubs[stub] : require(id);
  };
  const loaded = { exports: {} };
  Function('module', 'exports', 'require', output)(loaded, loaded.exports, requireWithStubs);
  return loaded.exports;
}

const { PlatesView } = loadPlatesView();

/* ---------- rendering and tree queries ---------- */

let tree = null;

function settle() {
  for (let pass = 0; pass < 25; pass += 1) {
    harness.dirty = false;
    harness.cursor = 0;
    tree = PlatesView();
    const effects = harness.pendingEffects.splice(0);
    effects.forEach((run) => run());
    if (!harness.dirty && harness.pendingEffects.length === 0) return tree;
  }
  throw new Error('PlatesView did not settle');
}

function unmount() {
  harness.slots.forEach((cell) => cell?.cleanup?.());
}

function mount({ cameras = [{ camera_uuid: CAMERA, name: 'Driveway' }] } = {}) {
  harness.slots = [];
  harness.pendingEffects = [];
  query.data = cameras;
  query.isLoading = false;
  query.error = null;
  return settle();
}

// Expands plain function components (FieldErrors) so their output is searchable;
// ConfirmDialog stays a vnode so its props can be inspected.
function children(node) {
  if (typeof node.type === 'function' && node.type !== ConfirmDialog) return [node.type(node.props)];
  return [node.props?.children];
}

function collect(predicate, node, found) {
  if (node === null || node === undefined || typeof node !== 'object') return found;
  if (Array.isArray(node)) {
    node.forEach((child) => collect(predicate, child, found));
    return found;
  }
  if (predicate(node)) found.push(node);
  children(node).forEach((child) => collect(predicate, child, found));
  return found;
}

const findAll = (predicate, root = tree) => collect(predicate, root, []);
const find = (predicate, root = tree) => findAll(predicate, root)[0];
const byId = (id) => find((n) => typeof n.type === 'string' && n.props?.id === id);

function text(node) {
  if (node === null || node === undefined || typeof node === 'boolean') return '';
  if (typeof node === 'string' || typeof node === 'number') return String(node);
  if (Array.isArray(node)) return node.map(text).join('');
  return children(node).map(text).join('');
}

const tableRows = () => findAll((n) => n.type === 'tbody')
  .flatMap((body) => findAll((n) => n.type === 'tr', body));
const button = (label, scope) => find((n) => n.type === 'button' && text(n) === label, scope);
const dialog = () => find((n) => n.type === ConfirmDialog);
const alertText = () => text(find((n) => n.props?.role === 'alert'));

/* ---------- interactions ---------- */

function input(id, value) {
  const node = byId(id);
  const handler = node.props.onInput || node.props.onChange;
  handler({ currentTarget: { value } });
  settle();
}

async function submit() {
  const preventDefault = jest.fn();
  await find((n) => n.type === 'form').props.onSubmit({ preventDefault });
  settle();
  return preventDefault;
}

async function click(node) {
  await node.props.onClick();
  settle();
}

/* ---------- fetch ---------- */

const json = (status, body) => new Response(JSON.stringify(body), {
  status,
  headers: { 'Content-Type': 'application/json' },
});

const read = (overrides = {}) => ({
  id: 'b1a7c0de-0000-4000-8000-000000000001',
  camera_uuid: CAMERA,
  stream_name: 'Driveway (old name)',
  observed_at: new Date(2026, 9, 29, 9, 5, 7).getTime(),
  plate: 'ABC1234',
  confidence: 0.97,
  attributes: { direction: 'inbound', vehicle_color: 'blue', vehicle_type: 'car' },
  ...overrides,
});

function requestBody(call) {
  return JSON.parse(call[1].body);
}

beforeEach(() => {
  jest.spyOn(Date, 'now').mockReturnValue(NOW);
  global.fetch = jest.fn();
  global.window = {};
  handleAuthenticationFailure.mockClear();
  showStatusMessage.mockClear();
  useQuery.mockClear();
  fetchAllStreamSummaries.mockReset();
});

afterEach(() => {
  jest.restoreAllMocks();
  delete global.fetch;
  delete global.window;
});

/* ---------- tests ---------- */

describe('PlatesView filters', () => {
  test('discovers cameras with LPR permissions instead of live-view permissions', async () => {
    mount();
    const options = useQuery.mock.calls[0][0];
    const controller = new AbortController();
    await options.queryFn({ signal: controller.signal });
    expect(fetchAllStreamSummaries).toHaveBeenCalledWith({
      surface: 'lpr', signal: controller.signal,
    });
  });

  test('selects the first camera once streams load and prompts for a search', () => {
    harness.slots = [];
    harness.pendingEffects = [];
    query.data = undefined;
    query.isLoading = true;
    query.error = null;
    settle();
    expect(byId('lpr-camera').props.disabled).toBe(true);
    expect(text(byId('lpr-camera'))).toBe('lpr.loadingCameras');

    query.data = [
      { name: 'No identity' },
      { camera_uuid: OTHER_CAMERA, name: 'Front' },
      { camera_uuid: CAMERA, name: 'Driveway' },
    ];
    query.isLoading = false;
    settle();
    expect(byId('lpr-camera').props.value).toBe(OTHER_CAMERA);
    expect(findAll((n) => n.type === 'option').map((o) => o.props.value)).toEqual([OTHER_CAMERA, CAMERA]);
    expect(text(tree)).toContain('lpr.prompt');
    expect(button('lpr.export')).toBeDefined();
  });

  test('explains when there are no cameras or the camera list fails', () => {
    mount({ cameras: [] });
    expect(text(byId('lpr-camera'))).toBe('lpr.noCameras');
    expect(button('lpr.export')).toBeUndefined();

    query.error = new Error('boom');
    settle();
    expect(text(tree)).toContain('lpr.error.cameras');
  });

  test('shows field errors and sends nothing when the filters are invalid', async () => {
    mount();
    input('lpr-plate', 'ab');
    input('lpr-limit', '500');
    input('lpr-end', '2026-10-27T12:00');
    const preventDefault = await submit();

    expect(preventDefault).toHaveBeenCalled();
    expect(global.fetch).not.toHaveBeenCalled();
    expect(byId('lpr-plate').props['aria-invalid']).toBe(true);
    expect(byId('lpr-plate').props['aria-describedby']).toBe('lpr-plate-error');
    expect(text(byId('lpr-plate-error'))).toBe('lpr.validation.plateTooShort:{"min":3}');
    expect(text(byId('lpr-limit-error'))).toBe('lpr.validation.limit:{"max":100}');
    expect(text(byId('lpr-end-error'))).toBe('lpr.validation.rangeOrder');
    expect(byId('lpr-start-error')).toBeUndefined();
  });

  test('a fixed field drops its error on the next search', async () => {
    mount();
    input('lpr-plate', 'ab');
    await submit();
    expect(byId('lpr-plate-error')).toBeDefined();

    global.fetch.mockResolvedValue(json(200, { reads: [], count: 0 }));
    input('lpr-match-exact', 'exact');
    await submit();
    expect(byId('lpr-plate-error')).toBeUndefined();
    expect(byId('lpr-plate').props['aria-describedby']).toBe('lpr-plate-hint');
    expect(requestBody(global.fetch.mock.calls[0])).toMatchObject({ match: 'exact', plate: 'ab' });
  });
});

describe('PlatesView search results', () => {
  test('posts the filters and renders each read', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, {
      reads: [read(), read({ id: 'b1a7c0de-0000-4000-8000-000000000002', plate: 'XYZ987', confidence: null, attributes: {} })],
      count: 2,
    }));
    input('lpr-plate', ' abc 12 ');
    await submit();

    const [url, init] = global.fetch.mock.calls[0];
    expect(url).toBe('/api/lpr/search');
    expect(init).toMatchObject({ method: 'POST', credentials: 'same-origin' });
    expect(requestBody(global.fetch.mock.calls[0])).toEqual({
      camera_uuid: CAMERA,
      start_at: NOW - 24 * 60 * 60 * 1000,
      end_at: NOW,
      limit: 50,
      match: 'partial',
      plate: 'abc 12',
    });

    const rows = tableRows();
    expect(rows).toHaveLength(2);
    const cells = (row) => findAll((n) => n.type === 'td' || n.type === 'th', row).map(text);
    const first = cells(rows[0]);
    expect(first.slice(0, 1)).toEqual(['ABC1234']);
    expect(first.slice(2, 6)).toEqual(['Driveway', '97%', 'inbound', 'blue car']);
    expect(cells(rows[1]).slice(2, 6)).toEqual(['Driveway', '—', '—', '—']);

    const link = find((n) => n.type === 'a' && n.props.href?.startsWith('timeline.html'), rows[0]);
    expect(link.props.href).toBe(`timeline.html?camera_uuid=${CAMERA}&stream=Driveway&date=2026-10-29&time=09%3A05%3A07`);
    expect(link.props.href).not.toContain('ABC');
    expect(link.props['aria-label']).toBe('lpr.viewRecordingFor:{"plate":"ABC1234"}');
    expect(find((n) => n.type === 'caption')).toBeDefined();
    expect(text(find((n) => n.type === 'caption'))).toBe('lpr.resultsCaption:{"count":2}');
  });

  test('falls back to the stored stream name when the camera is no longer listed', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, { reads: [read({ camera_uuid: OTHER_CAMERA })] }));
    await submit();
    const cells = findAll((n) => n.type === 'td', tableRows()[0]).map(text);
    expect(cells[1]).toBe('Driveway (old name)');
  });

  test('renders an associated recording link independently of a camera rename', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, { reads: [read({ recording_id: 42 })] }));
    await submit();
    const link = find((n) => n.type === 'a' && n.props.href?.startsWith('timeline.html'), tableRows()[0]);
    expect(link.props.href).toBe('timeline.html?ids=42&date=2026-10-29&time=09%3A05%3A07');
  });

  test('shows the empty state when nothing matched', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, { reads: [], count: 0 }));
    await submit();
    expect(tableRows()).toHaveLength(0);
    expect(text(tree)).toContain('lpr.emptyTitle');
    expect(text(tree)).not.toContain('lpr.prompt');
  });

  test('treats a malformed success body as no results', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, { reads: 'nope' }));
    await submit();
    expect(text(tree)).toContain('lpr.emptyTitle');
  });

  test('disables the search button while a search is running', async () => {
    mount();
    let resolve;
    global.fetch.mockReturnValue(new Promise((r) => { resolve = r; }));
    const pending = find((n) => n.type === 'form').props.onSubmit({ preventDefault() {} });
    settle();
    const searching = find((n) => n.type === 'button' && n.props.type === 'submit');
    expect(searching.props.disabled).toBe(true);
    expect(text(searching)).toBe('lpr.searching');

    resolve(json(200, { reads: [] }));
    await pending;
    settle();
    expect(find((n) => n.type === 'button' && n.props.type === 'submit').props.disabled).toBe(false);
  });

  test('a newer search aborts the previous one and only its results are shown', async () => {
    mount();
    const signals = [];
    let resolveFirst;
    global.fetch
      .mockImplementationOnce((_url, init) => {
        signals.push(init.signal);
        return new Promise((r) => { resolveFirst = r; });
      })
      .mockImplementationOnce((_url, init) => {
        signals.push(init.signal);
        return Promise.resolve(json(200, { reads: [read({ plate: 'NEWER1' })] }));
      });

    const first = find((n) => n.type === 'form').props.onSubmit({ preventDefault() {} });
    await submit();
    expect(signals[0].aborted).toBe(true);
    expect(signals[1].aborted).toBe(false);

    resolveFirst(json(200, { reads: [read({ plate: 'STALE1' })] }));
    await first;
    settle();
    expect(tableRows().map((row) => text(row.props.children[0]))).toEqual(['NEWER1']);
    expect(find((n) => n.type === 'button' && n.props.type === 'submit').props.disabled).toBe(false);
  });

  test('aborts an in-flight search when the page unmounts', async () => {
    mount();
    let signal;
    global.fetch.mockImplementation((_url, init) => {
      signal = init.signal;
      return new Promise(() => {});
    });
    find((n) => n.type === 'form').props.onSubmit({ preventDefault() {} });
    settle();
    unmount();
    expect(signal.aborted).toBe(true);
  });
});

describe('PlatesView search errors', () => {
  test.each([
    [400, 'lpr.error.invalid'],
    [403, 'lpr.error.searchForbidden'],
    [503, 'lpr.error.unavailable'],
    [500, 'lpr.error.generic'],
  ])('a %i response shows %s and clears earlier results', async (status, message) => {
    mount();
    global.fetch.mockResolvedValueOnce(json(200, { reads: [read()] }));
    await submit();
    expect(tableRows()).toHaveLength(1);

    global.fetch.mockResolvedValueOnce(json(status, { error: 'nope' }));
    await submit();
    expect(alertText()).toBe(message);
    expect(tableRows()).toHaveLength(0);
    expect(text(tree)).not.toContain('lpr.prompt');
  });

  test('a network failure shows the generic error', async () => {
    mount();
    global.fetch.mockRejectedValue(new TypeError('Failed to fetch'));
    await submit();
    expect(alertText()).toBe('lpr.error.generic');
  });

  test('401 sends the user to login, without an inline error', async () => {
    mount();
    global.fetch.mockResolvedValue(json(401, { error: 'Unauthorized' }));
    await submit();
    expect(handleAuthenticationFailure).toHaveBeenCalledTimes(1);
    expect(find((n) => n.props?.role === 'alert')).toBeUndefined();
  });

  test.each(['_authDisabled', '_demoMode'])('401 does not redirect when window.%s is set', async (flag) => {
    global.window[flag] = true;
    mount();
    global.fetch.mockResolvedValue(json(401, {}));
    await submit();
    expect(handleAuthenticationFailure).not.toHaveBeenCalled();
  });

  test('a new search clears the previous error', async () => {
    mount();
    global.fetch.mockResolvedValueOnce(json(503, {}));
    await submit();
    expect(alertText()).toBe('lpr.error.unavailable');
    global.fetch.mockResolvedValueOnce(json(200, { reads: [] }));
    await submit();
    expect(find((n) => n.props?.role === 'alert')).toBeUndefined();
  });
});

describe('PlatesView export', () => {
  let anchor;

  beforeEach(() => {
    anchor = { click: jest.fn(), remove: jest.fn() };
    global.document = {
      createElement: jest.fn(() => anchor),
      body: { appendChild: jest.fn() },
    };
    jest.spyOn(URL, 'createObjectURL').mockReturnValue('blob:lpr-export');
    jest.spyOn(URL, 'revokeObjectURL').mockImplementation(() => {});
    jest.useFakeTimers({ doNotFake: ['Date'] });
  });

  afterEach(() => {
    jest.useRealTimers();
    delete global.document;
  });

  test('downloads up to 1000 reads as a JSON file and releases the blob URL', async () => {
    mount();
    global.fetch.mockResolvedValue(json(200, { reads: [read()], count: 1 }));
    input('lpr-limit', '10');
    await click(button('lpr.export'));

    expect(global.fetch.mock.calls[0][0]).toBe('/api/lpr/export');
    expect(requestBody(global.fetch.mock.calls[0]).limit).toBe(1000);
    expect(anchor.href).toBe('blob:lpr-export');
    expect(anchor.download).toBe('lightnvr-lpr-export.json');
    expect(anchor.click).toHaveBeenCalled();
    expect(anchor.remove).toHaveBeenCalled();
    expect(URL.revokeObjectURL).not.toHaveBeenCalled();
    jest.runAllTimers();
    expect(URL.revokeObjectURL).toHaveBeenCalledWith('blob:lpr-export');
    expect(tableRows()).toHaveLength(0);
  });

  test('validates before exporting', async () => {
    mount();
    input('lpr-plate', 'a');
    await click(button('lpr.export'));
    expect(global.fetch).not.toHaveBeenCalled();
    expect(byId('lpr-plate-error')).toBeDefined();
  });

  test('a 403 hides Export for that camera only, without an error', async () => {
    mount({ cameras: [{ camera_uuid: CAMERA, name: 'Driveway' }, { camera_uuid: OTHER_CAMERA, name: 'Front' }] });
    global.fetch.mockResolvedValue(json(403, { error: 'Forbidden' }));
    await click(button('lpr.export'));
    expect(button('lpr.export')).toBeUndefined();
    expect(find((n) => n.props?.role === 'alert')).toBeUndefined();

    input('lpr-camera', OTHER_CAMERA);
    expect(button('lpr.export')).toBeDefined();
  });

  test('other failures show an error and keep Export available', async () => {
    mount();
    global.fetch.mockResolvedValueOnce(json(503, {}));
    await click(button('lpr.export'));
    expect(alertText()).toBe('lpr.error.unavailable');
    expect(button('lpr.export')).toBeDefined();

    global.fetch.mockRejectedValueOnce(new TypeError('offline'));
    await click(button('lpr.export'));
    expect(alertText()).toBe('lpr.error.generic');
    expect(text(button('lpr.export'))).toBe('lpr.export');
  });

  test('401 during export sends the user to login', async () => {
    mount();
    global.fetch.mockResolvedValue(json(401, {}));
    await click(button('lpr.export'));
    expect(handleAuthenticationFailure).toHaveBeenCalledTimes(1);
  });
});

describe('PlatesView delete', () => {
  const second = read({ id: 'b1a7c0de-0000-4000-8000-000000000002', plate: 'XYZ987' });

  async function searchTwoReads() {
    mount();
    global.fetch.mockResolvedValueOnce(json(200, { reads: [read(), second] }));
    await submit();
    expect(tableRows()).toHaveLength(2);
  }

  const deleteButton = (row) => find((n) => n.type === 'button' && text(n) === 'common.delete', row);

  test('asks for confirmation naming the plate, then deletes only that read', async () => {
    await searchTwoReads();
    expect(dialog().props.isOpen).toBe(false);

    await click(deleteButton(tableRows()[1]));
    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(dialog().props.isOpen).toBe(true);
    expect(dialog().props.message).toBe('lpr.deleteMessage:{"plate":"XYZ987"}');
    expect(dialog().props.variant).toBe('danger');

    global.fetch.mockResolvedValueOnce(json(200, { deleted: true }));
    await dialog().props.onConfirm();
    settle();
    expect(global.fetch.mock.calls[1][0]).toBe(`/api/lpr/reads/${second.id}`);
    expect(global.fetch.mock.calls[1][1]).toMatchObject({ method: 'DELETE', credentials: 'same-origin' });
    expect(tableRows()).toHaveLength(1);
    expect(text(tableRows()[0].props.children[0])).toBe('ABC1234');
    expect(showStatusMessage).toHaveBeenCalledWith('lpr.deleted', 'success');
  });

  test('cancelling closes the dialog without deleting', async () => {
    await searchTwoReads();
    await click(deleteButton(tableRows()[0]));
    dialog().props.onClose();
    settle();
    expect(dialog().props.isOpen).toBe(false);
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  test('a read that is already gone is removed from the list', async () => {
    await searchTwoReads();
    await click(deleteButton(tableRows()[0]));
    global.fetch.mockResolvedValueOnce(json(404, { error: 'LPR read not found' }));
    await dialog().props.onConfirm();
    settle();
    expect(tableRows()).toHaveLength(1);
  });

  test('a 403 hides Delete for the camera and keeps the read', async () => {
    await searchTwoReads();
    await click(deleteButton(tableRows()[0]));
    global.fetch.mockResolvedValueOnce(json(403, {}));
    await dialog().props.onConfirm();
    settle();
    expect(tableRows()).toHaveLength(2);
    expect(tableRows().map(deleteButton)).toEqual([undefined, undefined]);
    expect(find((n) => n.props?.role === 'alert')).toBeUndefined();
    expect(showStatusMessage).not.toHaveBeenCalled();
  });

  test.each([
    ['a server error', () => json(500, {})],
    ['a network failure', () => Promise.reject(new TypeError('offline'))],
  ])('%s keeps the read and shows an error', async (_name, response) => {
    await searchTwoReads();
    await click(deleteButton(tableRows()[0]));
    global.fetch.mockImplementationOnce(response);
    await dialog().props.onConfirm();
    settle();
    expect(tableRows()).toHaveLength(2);
    expect(alertText()).toBe('lpr.error.deleteFailed');
  });

  test('401 during delete sends the user to login', async () => {
    await searchTwoReads();
    await click(deleteButton(tableRows()[0]));
    global.fetch.mockResolvedValueOnce(json(401, {}));
    await dialog().props.onConfirm();
    settle();
    expect(handleAuthenticationFailure).toHaveBeenCalledTimes(1);
    expect(tableRows()).toHaveLength(2);
  });
});

describe('PlatesView mobile layout', () => {
  const second = read({
    id: 'b1a7c0de-0000-4000-8000-000000000002', plate: 'XYZ987', confidence: null, attributes: {},
  });
  const cards = () => findAll((n) => n.type === 'li');
  const cardButton = (card, label) => find((n) => n.type === 'button' && text(n) === label, card);

  async function searchTwoReads() {
    mount();
    global.fetch.mockResolvedValueOnce(json(200, { reads: [read(), second] }));
    await submit();
  }

  test('renders one card per read below md and keeps the table for wider screens', async () => {
    await searchTwoReads();
    const list = find((n) => n.type === 'ul');
    expect(list.props.className).toContain('md:hidden');
    expect(list.props['aria-label']).toBe('lpr.resultsCaption:{"count":2}');
    const table = find((n) => n.type === 'table');
    const tableWrapper = find((n) => n.type === 'div' && findAll((c) => c === table, n).length > 0
      && n.props.className?.includes('md:block'));
    expect(tableWrapper.props.className).toContain('hidden');
    expect(cards()).toHaveLength(2);
    expect(tableRows()).toHaveLength(2);
  });

  test('a card shows the plate, confidence, time and joined details', async () => {
    await searchTwoReads();
    const [first, other] = cards();
    const paragraphs = (card) => findAll((n) => n.type === 'p', card).map(text);
    expect(paragraphs(first)).toEqual([
      'ABC1234',
      'lpr.column.confidence: 97%',
      expect.stringContaining('2026'),
      'Driveway · inbound · blue car',
    ]);
    // Missing confidence and attributes are left out rather than shown as dashes.
    expect(paragraphs(other)).toEqual(['XYZ987', expect.stringContaining('2026'), 'Driveway']);
    const srLabel = find((n) => n.type === 'span' && n.props.className === 'sr-only', first);
    expect(text(srLabel)).toBe('lpr.column.confidence: ');
  });

  test('card actions link to the recording and confirm deletes for that read', async () => {
    await searchTwoReads();
    const [, other] = cards();
    const link = find((n) => n.type === 'a', other);
    expect(link.props.href).toMatch(/^timeline\.html\?stream=Driveway&/);
    expect(link.props.href).not.toContain('XYZ');
    expect(link.props['aria-label']).toBe('lpr.viewRecordingFor:{"plate":"XYZ987"}');

    await click(cardButton(other, 'common.delete'));
    expect(dialog().props.isOpen).toBe(true);
    expect(dialog().props.message).toBe('lpr.deleteMessage:{"plate":"XYZ987"}');
    global.fetch.mockResolvedValueOnce(json(200, { deleted: true }));
    await dialog().props.onConfirm();
    settle();
    expect(cards().map((card) => text(findAll((n) => n.type === 'p', card)[0]))).toEqual(['ABC1234']);
  });

  test('a delete refusal hides Delete on the cards too', async () => {
    await searchTwoReads();
    await click(cardButton(cards()[0], 'common.delete'));
    global.fetch.mockResolvedValueOnce(json(403, {}));
    await dialog().props.onConfirm();
    settle();
    expect(cards().map((card) => cardButton(card, 'common.delete'))).toEqual([undefined, undefined]);
  });

  test('a card with no recording link and no delete permission has no action row', async () => {
    mount();
    const unlinked = read({ observed_at: undefined });
    global.fetch.mockResolvedValueOnce(json(200, { reads: [unlinked] }));
    await submit();
    await click(cardButton(cards()[0], 'common.delete'));
    global.fetch.mockResolvedValueOnce(json(403, {}));
    await dialog().props.onConfirm();
    settle();
    expect(findAll((n) => n.type === 'a' || n.type === 'button', cards()[0])).toEqual([]);
  });

  test('no cards render before a search or when nothing matched', async () => {
    mount();
    expect(find((n) => n.type === 'ul')).toBeUndefined();
    global.fetch.mockResolvedValueOnce(json(200, { reads: [] }));
    await submit();
    expect(find((n) => n.type === 'ul')).toBeUndefined();
  });

  test('the view tabs are not a <nav> element, which layout.css styles as header links', () => {
    mount();
    expect(find((n) => n.type === 'nav')).toBeUndefined();
    const tabs = find((n) => n.props?.role === 'navigation');
    expect(tabs.props['aria-label']).toBe('recordings.views');
    expect(tabs.props.className).toContain('overflow-x-auto');
    const links = findAll((n) => n.type === 'a', tabs);
    expect(links.map((a) => a.props.href)).toEqual(['recordings.html', 'timeline.html', 'investigation.html', 'plates.html']);
    expect(links.map((a) => a.props['aria-current'])).toEqual([undefined, undefined, undefined, 'page']);
    links.forEach((a) => expect(a.props.className).toContain('shrink-0'));
  });
});
