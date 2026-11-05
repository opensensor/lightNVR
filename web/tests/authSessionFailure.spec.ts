import {clearAuthState, setupSessionValidation, validateSession} from '../js/utils/auth-utils.ts';

const validBody = {authenticated: true, id: 7, username: 'fixture', role: 'viewer', is_active: true};
const response = (body: unknown, status = 200) => new Response(JSON.stringify(body), {status});

describe('session verification failures', () => {
  let events: EventTarget;
  beforeEach(() => {
    jest.useFakeTimers();
    for (const method of ['log', 'debug', 'warn', 'error'] as const) jest.spyOn(console, method).mockImplementation(() => {});
    const values = new Map<string, string>();
    const storage: Storage = {
      get length() {return values.size;},
      clear: () => values.clear(), getItem: key => values.get(key) ?? null,
      setItem: (key, value) => {values.set(key, value);}, removeItem: key => {values.delete(key);},
      key: index => [...values.keys()][index] ?? null,
    };
    events = new EventTarget();
    Object.defineProperty(globalThis, 'localStorage', {configurable: true, value: storage});
    Object.defineProperty(globalThis, 'document', {configurable: true, value: {cookie: ''}});
    Object.defineProperty(globalThis, 'window', {configurable: true, value: {
      location: {pathname: '/streams.html', search: '', href: '/streams.html'},
      addEventListener: events.addEventListener.bind(events),
      removeEventListener: events.removeEventListener.bind(events),
      dispatchEvent: events.dispatchEvent.bind(events),
    }});
    clearAuthState();
    localStorage.setItem('auth', 'fixture-credential');
  });
  afterEach(async () => {
    events.dispatchEvent(new Event('pagehide'));
    window.location.pathname = '/login.html';
    setupSessionValidation();
    await jest.advanceTimersByTimeAsync(0);
    jest.clearAllTimers();
    jest.useRealTimers();
    jest.restoreAllMocks();
    Reflect.deleteProperty(globalThis, 'window');
    Reflect.deleteProperty(globalThis, 'document');
    Reflect.deleteProperty(globalThis, 'localStorage');
  });

  it.each([503, 429])('returns unavailable for HTTP %s without erasing credentials', async status => {
    jest.spyOn(globalThis, 'fetch').mockResolvedValue(response({error: 'temporary'}, status));
    const pending = validateSession();
    await jest.advanceTimersByTimeAsync(1000);
    expect(await pending).toEqual({valid: false, status: 'unavailable'});
    expect(localStorage.getItem('auth')).toBe('fixture-credential');
  });

  it('does not clear credentials or redirect after transient periodic verification', async () => {
    jest.spyOn(globalThis, 'fetch').mockResolvedValue(response({error: 'temporary'}, 503));
    setupSessionValidation(5000);
    await jest.advanceTimersByTimeAsync(1000);
    expect(localStorage.getItem('auth')).toBe('fixture-credential');
    expect(window.location.href).toBe('/streams.html');
  });

  it.each([401, 403])('clears and redirects on definite HTTP %s rejection', async status => {
    jest.spyOn(globalThis, 'fetch').mockResolvedValue(response({}, status));
    setupSessionValidation();
    await jest.advanceTimersByTimeAsync(0);
    expect(localStorage.getItem('auth')).toBeNull();
    expect(window.location.href).toContain('/login.html?auth_required=true&reason=session_expired');
  });

  it.each([{}, {authenticated: true}, {authenticated: true, role: 'superadmin'},
    {authenticated: 'true', role: 'admin'}, {authenticated: false, demo_mode: true, role: 'admin'}])(
    'never authenticates malformed verification %j', async body => {
      jest.spyOn(globalThis, 'fetch').mockResolvedValue(response(body));
      expect(await validateSession()).toEqual({valid: false, status: 'unavailable'});
      expect(window._authDisabled).toBe(false);
    });

  it('preserves credentials on network failures', async () => {
    jest.spyOn(globalThis, 'fetch').mockRejectedValue(new TypeError('Network unavailable'));
    expect(await validateSession()).toEqual({valid: false, status: 'unavailable'});
    expect(localStorage.getItem('auth')).toBe('fixture-credential');
  });

  it('bounds a stalled verification and does not overlap periodic requests', async () => {
    let aborted = 0;
    const fetch = jest.spyOn(globalThis, 'fetch').mockImplementation((_url, options) => new Promise((_resolve, reject) => {
      options?.signal?.addEventListener('abort', () => {aborted++; reject(new DOMException('Aborted', 'AbortError'));}, {once: true});
    }));
    setupSessionValidation(2000);
    await jest.advanceTimersByTimeAsync(9000);
    expect(fetch).toHaveBeenCalledTimes(1);
    await jest.advanceTimersByTimeAsync(1000);
    expect(aborted).toBe(1);
    expect(localStorage.getItem('auth')).toBe('fixture-credential');
    expect(window.location.href).toBe('/streams.html');
  });

  it('keeps the deadline active while reading a stalled response body', async () => {
    jest.spyOn(globalThis, 'fetch').mockImplementation((_url, options) => Promise.resolve(new Response(new ReadableStream({
      start(controller) {
        options?.signal?.addEventListener('abort', () => controller.error(new DOMException('Aborted', 'AbortError')), {once: true});
      },
    }))));
    const pending = validateSession();
    await jest.advanceTimersByTimeAsync(10000);
    expect(await pending).toEqual({valid: false, status: 'unavailable'});
    expect(localStorage.getItem('auth')).toBe('fixture-credential');
    expect(jest.getTimerCount()).toBe(0);
  });

  it('cancels promptly when its caller unmounts and removes its deadline', async () => {
    jest.spyOn(globalThis, 'fetch').mockImplementation((_url, options) => new Promise((_resolve, reject) => {
      options?.signal?.addEventListener('abort', () => reject(new DOMException('Aborted', 'AbortError')), {once: true});
    }));
    const controller = new AbortController();
    const pending = validateSession({signal: controller.signal});
    controller.abort();
    expect(await pending).toEqual({valid: false, status: 'unavailable'});
    expect(jest.getTimerCount()).toBe(0);
  });

  it('validates invisible HttpOnly sessions and previously disabled auth', async () => {
    localStorage.clear();
    window._authDisabled = true;
    const fetch = jest.spyOn(globalThis, 'fetch').mockResolvedValue(response(validBody));
    setupSessionValidation(2000);
    await jest.advanceTimersByTimeAsync(2000);
    expect(fetch).toHaveBeenCalledTimes(2);
    expect(window._authDisabled).toBe(false);
  });

  it('skips login pages and pauses requests while a page is hidden', async () => {
    const fetch = jest.spyOn(globalThis, 'fetch').mockImplementation(() => Promise.resolve(response(validBody)));
    window.location.pathname = '/login.html';
    expect(setupSessionValidation(2000)).toBeNull();
    await jest.advanceTimersByTimeAsync(4000);
    expect(fetch).not.toHaveBeenCalled();
    window.location.pathname = '/streams.html';
    setupSessionValidation(2000);
    await jest.advanceTimersByTimeAsync(0);
    events.dispatchEvent(new Event('pagehide'));
    await jest.advanceTimersByTimeAsync(4000);
    expect(fetch).toHaveBeenCalledTimes(1);
    events.dispatchEvent(new Event('pageshow'));
    await jest.advanceTimersByTimeAsync(0);
    expect(fetch).toHaveBeenCalledTimes(2);
  });

  it('does not restore a session from a reply received after explicit logout', async () => {
    let resolveResponse!: (value: Response) => void;
    jest.spyOn(globalThis, 'fetch').mockImplementation(() => new Promise(resolve => {resolveResponse = resolve;}));
    const pending = validateSession();
    clearAuthState();
    resolveResponse(response({...validBody, auth_enabled: false, role: 'admin'}));
    expect(await pending).toEqual({valid: false, status: 'unavailable'});
    expect(localStorage.getItem('auth')).toBeNull();
    expect(window._authDisabled).toBe(false);
  });

  it.each([
    {authenticated: true, role: 'admin', auth_enabled: false},
    {authenticated: false, role: 'viewer', demo_mode: true},
  ])('preserves verified no-auth and anonymous demo modes %j', async body => {
    jest.spyOn(globalThis, 'fetch').mockResolvedValue(response(body));
    expect(await validateSession()).toMatchObject({valid: true, status: 'valid', role: body.role});
  });

  it('keeps successful verification distinct from explicit rejection', async () => {
    jest.spyOn(globalThis, 'fetch')
      .mockResolvedValueOnce(response(validBody))
      .mockResolvedValueOnce(response({error: 'expired'}, 401));
    expect(await validateSession()).toMatchObject({valid: true, status: 'valid', role: 'viewer'});
    expect(await validateSession()).toEqual({valid: false, status: 'invalid'});
  });
});
