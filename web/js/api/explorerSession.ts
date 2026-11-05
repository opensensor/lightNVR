import { validateSession } from '../utils/auth-utils.ts';
import type { SessionInfo } from '../types/core.ts';
import type { components } from '../../generated/api/schema';
import { validateOpenApiDocument } from '../../generated/api/validators.js';

export type ExplorerDocument = components['schemas']['OpenApiDocument'];
export type ExplorerStatus = 'loading' | 'denied' | 'unavailable' | 'partial' | 'empty';

export function explorerUrls(pageUrl: string) {
  const page = new URL(pageUrl);
  if (!['http:', 'https:'].includes(page.protocol)) throw new Error('Unsupported explorer origin');
  const base = new URL('./', page);
  return {
    base: base.href,
    spec: new URL('api/openapi.json', base).href,
    verify: new URL('api/auth/verify', base).href,
  };
}

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

/** Reject remote references before the vendor resolver can see them. */
export function prepareExplorerDocument(value: unknown, specUrl: string): ExplorerDocument {
  if (!validateOpenApiDocument(value)) throw new Error('Invalid API reference');
  const expectedServer = new URL('../', specUrl);
  const inspect = (node: unknown): void => {
    if (Array.isArray(node)) { node.forEach(inspect); return; }
    if (!record(node)) return;
    for (const [key, child] of Object.entries(node)) {
      if (['$ref', '$dynamicRef', 'operationRef'].includes(key) && typeof child === 'string' && !child.startsWith('#')) {
        throw new Error('Remote references are disabled');
      }
      if (key === 'servers' && Array.isArray(child)) {
        for (const server of child) {
          if (!record(server) || typeof server.url !== 'string' || new URL(server.url, specUrl).href !== expectedServer.href) {
            throw new Error('Unexpected API server');
          }
        }
      }
      inspect(child);
    }
  };
  inspect(value);
  return { ...value, servers: [{ url: expectedServer.href }] };
}

/** Pinned Swagger UI 5.34.1 configuration. No Swagger-originated HTTP is needed.
 * https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/
 * Component names are checked against the installed standalone distribution.
 */
export function readOnlySwaggerOptions(spec: ExplorerDocument, domNode: HTMLElement) {
  return {
    spec, domNode, layout: 'BaseLayout',
    supportedSubmitMethods: [], tryItOutEnabled: false,
    persistAuthorization: false, queryConfigEnabled: false,
    validatorUrl: null, configUrl: null, url: '', urls: [],
    withCredentials: false, deepLinking: false,
    displayRequestDuration: false, defaultModelsExpandDepth: -1,
    requestInterceptor: () => { throw new Error('API reference requests are disabled'); },
    plugins: [() => ({ components: {
      authorizeBtn: () => null,
      AuthorizeBtnContainer: () => null,
      authorizeOperationBtn: () => null,
      authorizationPopup: () => null,
      auths: () => null,
    } })],
  };
}

interface ExplorerDependencies {
  pageUrl: string;
  clear: () => void;
  status: (state: ExplorerStatus) => void;
  render: (document: ExplorerDocument) => void;
  prepare?: () => Promise<void>;
  verify?: (options: { signal: AbortSignal; verifyUrl: string }) => Promise<SessionInfo>;
  fetch?: typeof globalThis.fetch;
  timeout?: number;
}

function identity(session: SessionInfo): string | null {
  if (!session.valid || session.must_change_password) return null;
  return JSON.stringify([session.id, session.username, session.role, session.role_id, session.demo_mode, session.auth_enabled]);
}

/** Every boundary clears synchronously; even transports ignoring abort cannot
 * publish a response belonging to an older generation or a different identity.
 */
export function createExplorerSession(deps: ExplorerDependencies) {
  const urls = explorerUrls(deps.pageUrl);
  const verify = deps.verify ?? validateSession;
  const fetcher = deps.fetch ?? globalThis.fetch;
  let generation = 0;
  let controller: AbortController | undefined;
  let disposed = false;

  function clear(state: ExplorerStatus = 'loading'): void {
    generation++;
    controller?.abort();
    controller = undefined;
    deps.clear();
    deps.status(state);
  }

  async function reload(identityRetry = 0): Promise<void> {
    if (disposed) return;
    clear();
    const requestGeneration = generation;
    const current = new AbortController();
    controller = current;
    const deadline = setTimeout(() => {
      current.abort();
      if (!disposed && requestGeneration === generation) { deps.clear(); deps.status('unavailable'); }
    }, deps.timeout ?? 10000);
    const active = () => !disposed && !current.signal.aborted && requestGeneration === generation;
    try {
      await deps.prepare?.();
      if (!active()) return;
      const before = await verify({ signal: current.signal, verifyUrl: urls.verify });
      if (!active()) return;
      if (!before.valid) { deps.status(before.status === 'invalid' ? 'denied' : 'unavailable'); return; }
      if (identity(before) === null) { deps.status('denied'); return; }
      const response = await fetcher(urls.spec, {
        method: 'GET', credentials: 'same-origin', cache: 'no-store',
        redirect: 'error', signal: current.signal, headers: { Accept: 'application/json' },
      });
      if (!active()) return;
      if (!response.ok) {
        deps.status(response.status === 401 || response.status === 403 ? 'denied' : 'unavailable');
        return;
      }
      // Redirects are disabled; never let a response choose another spec base.
      if (response.url && response.url !== urls.spec) throw new Error('Unexpected API reference URL');
      const payload: unknown = await response.json();
      if (!active()) return;
      const document = prepareExplorerDocument(payload, response.url || urls.spec);
      const after = await verify({ signal: current.signal, verifyUrl: urls.verify });
      if (!active()) return;
      if (!after.valid || identity(after) === null) {
        deps.status(after.status === 'unavailable' ? 'unavailable' : 'denied');
        return;
      }
      if (identity(before) !== identity(after)) {
        clear('unavailable');
        if (identityRetry === 0) await reload(1);
        return;
      }
      const hasOperations = Object.values(document.paths).some(item => record(item) &&
        ['get', 'put', 'post', 'delete', 'patch', 'head', 'options', 'trace'].some(method => record(item[method])));
      deps.status(hasOperations ? 'partial' : 'empty');
      if (hasOperations) deps.render(document);
    } catch {
      if (!disposed && requestGeneration === generation) {
        deps.clear();
        deps.status('unavailable');
      }
    } finally {
      clearTimeout(deadline);
      if (controller === current) controller = undefined;
      // Abort on a deadline, even if an injected fetch did not reject promptly.
      if (!disposed && requestGeneration === generation && current.signal.aborted) {
        deps.clear(); deps.status('unavailable');
      }
    }
  }

  return {
    reload: () => reload(), clear,
    dispose() { if (!disposed) { disposed = true; clear('denied'); } },
  };
}

/** Hidden/bfcache pages contain no personalized document. Re-entry starts fresh. */
export function bindExplorerLifecycle(session: ReturnType<typeof createExplorerSession>, win: Window, doc: Document): () => void {
  const reload = () => { if (doc.visibilityState === 'hidden') session.clear(); else void session.reload(); };
  const hidden = () => session.clear();
  const logout = () => session.clear('denied');
  const visibility = () => doc.visibilityState === 'hidden' ? hidden() : reload();
  const storage = (event: StorageEvent) => { if (event.key === null || event.key === 'auth') reload(); };
  const broadcast = (event: MessageEvent<unknown>) => { if (event.data === 'cleared') logout(); };
  let channel: BroadcastChannel | undefined;
  try {
    if (typeof BroadcastChannel === 'function') {
      channel = new BroadcastChannel('lightnvr-auth');
      channel.addEventListener('message', broadcast);
    }
  } catch { /* The bounded visible-page check still covers cookie-only changes. */ }
  const periodic = win.setInterval(() => { if (doc.visibilityState === 'visible') reload(); }, 30000);
  win.addEventListener('lightnvr:auth-cleared', logout);
  win.addEventListener('storage', storage);
  win.addEventListener('focus', reload);
  win.addEventListener('pagehide', hidden);
  win.addEventListener('pageshow', reload);
  doc.addEventListener('visibilitychange', visibility);
  return () => {
    win.clearInterval(periodic);
    channel?.removeEventListener('message', broadcast);
    channel?.close();
    win.removeEventListener('lightnvr:auth-cleared', logout);
    win.removeEventListener('storage', storage);
    win.removeEventListener('focus', reload);
    win.removeEventListener('pagehide', hidden);
    win.removeEventListener('pageshow', reload);
    doc.removeEventListener('visibilitychange', visibility);
    session.dispose();
  };
}
