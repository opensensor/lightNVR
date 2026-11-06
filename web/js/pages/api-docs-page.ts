import {
  bindExplorerLifecycle, createExplorerSession, explorerUrls, readOnlySwaggerOptions,
} from '../api/explorerSession.ts';
import type { ExplorerDocument, ExplorerStatus } from '../api/explorerSession.ts';

interface SwaggerInstance {
  specActions: { updateJsonSpec: (spec: object) => void; updateSpec: (spec: string) => void; updateResolved: (spec: object) => void };
}
type SwaggerFactory = (options: ReturnType<typeof readOnlySwaggerOptions>) => SwaggerInstance;
declare global { interface Window { SwaggerUIBundle?: SwaggerFactory } }

const urls = explorerUrls(window.location.href);
const host = document.getElementById('api-reference');
const statusNode = document.getElementById('docs-status');
if (!host || !statusNode) throw new Error('API reference elements missing');
const referenceHost = host;
const referenceStatus = statusNode;
let instance: SwaggerInstance | undefined;
let vendorLoad: Promise<void> | undefined;
let strings: Record<string, string> = {};
let currentStatus: ExplorerStatus = 'loading';

const statusKeys: Record<ExplorerStatus, string> = {
  loading: 'apiDocs.loading', denied: 'apiDocs.denied', unavailable: 'apiDocs.unavailable',
  partial: 'apiDocs.partial', empty: 'apiDocs.empty',
};
const fallback: Record<string, string> = {
  'apiDocs.title': 'API reference', 'apiDocs.intro': 'Read-only API documentation for your current session.',
  'apiDocs.loading': 'Verifying your session and loading available operations…',
  'apiDocs.denied': 'Sign in with an account permitted to view the API reference.',
  'apiDocs.unavailable': 'Unable to load the API reference. Try again.',
  'apiDocs.partial': 'This reference covers selected APIs. Missing operations do not indicate denied access.',
  'apiDocs.empty': 'No API operations are available in this reference for the current session.',
  'common.refresh': 'Refresh', 'nav.live': 'Live View', 'auth.login': 'Login', 'auth.logout': 'Logout',
};
function text(key: string): string { return strings[key] ?? fallback[key] ?? key; }
function updateLabels(): void {
  document.querySelectorAll<HTMLElement>('[data-text]').forEach(element => {
    element.textContent = text(element.dataset.text ?? '');
  });
  document.title = `${text('apiDocs.title')} — LightNVR`;
  referenceHost.setAttribute('aria-label', text('apiDocs.title'));
  setStatus(currentStatus);
}
function setStatus(state: ExplorerStatus): void {
  currentStatus = state;
  referenceStatus.textContent = text(statusKeys[state]);
  referenceStatus.setAttribute('role', state === 'denied' || state === 'unavailable' ? 'alert' : 'status');
  referenceHost.setAttribute('aria-busy', String(state === 'loading'));
}
function clearDocument(): void {
  // Detach the whole vendor mount before asking React to clear its store. Its
  // deferred work can only touch a detached node, never restore old DOM.
  referenceHost.replaceChildren();
  const previous = instance;
  instance = undefined;
  if (previous) {
    previous.specActions.updateJsonSpec({});
    previous.specActions.updateSpec('');
    previous.specActions.updateResolved({});
  }
}
function loadVendor(): Promise<void> {
  if (window.SwaggerUIBundle) return Promise.resolve();
  if (!vendorLoad) vendorLoad = new Promise<void>((resolve, reject) => {
    const script = document.createElement('script');
    script.src = new URL('api-docs-vendor/swagger-ui-bundle-5.34.1.js', urls.base).href;
    script.onload = () => window.SwaggerUIBundle ? resolve() : reject(new Error('API reference unavailable'));
    script.onerror = () => { vendorLoad = undefined; script.remove(); reject(new Error('API reference unavailable')); };
    document.head.append(script);
  });
  return vendorLoad;
}
function renderDocument(spec: ExplorerDocument): void {
  const factory = window.SwaggerUIBundle;
  if (!factory) throw new Error('API reference unavailable');
  const mount = document.createElement('div');
  referenceHost.append(mount);
  instance = factory(readOnlySwaggerOptions(spec, mount));
}

/** Local catalogs use the same preference as the app, resolved under this page's
 * directory so a reverse-proxy prefix also works for translated status text.
 */
async function loadLanguage(): Promise<void> {
  const fetchJson = async (name: string): Promise<unknown> => {
    const response = await fetch(new URL(`locales/${name}`, urls.base), { credentials: 'same-origin', redirect: 'error' });
    return response.ok ? response.json() : null;
  };
  const table = (value: unknown): Record<string, string> => {
    if (value === null || typeof value !== 'object' || Array.isArray(value)) return {};
    return Object.fromEntries(Object.entries(value).filter((entry): entry is [string, string] => typeof entry[1] === 'string'));
  };
  try {
    const manifest = await fetchJson('manifest.json');
    const locales = manifest !== null && typeof manifest === 'object' && 'locales' in manifest && Array.isArray(manifest.locales)
      ? manifest.locales.flatMap((item: unknown) => item !== null && typeof item === 'object' && 'code' in item && typeof item.code === 'string' && /^[A-Za-z-]+$/.test(item.code) ? [item.code] : []) : ['en'];
    let preference: string | null = null;
    try { preference = localStorage.getItem('lightnvr-locale-preference'); } catch { /* Browser storage can be disabled. */ }
    const candidates = preference && preference !== 'auto' ? [preference] : [...navigator.languages];
    const locale = candidates.map(candidate => locales.find(code => code.toLowerCase() === candidate.toLowerCase()) ??
      locales.find(code => code.toLowerCase().split('-')[0] === candidate.toLowerCase().split('-')[0])).find(Boolean) ?? 'en';
    const english = table(await fetchJson('en.json'));
    strings = locale === 'en' ? english : { ...english, ...table(await fetchJson(`${locale}.json`)) };
    document.documentElement.lang = locale;
    document.documentElement.dir = locale === 'ar' ? 'rtl' : 'ltr';
  } catch { strings = {}; }
  updateLabels();
}

const session = createExplorerSession({
  pageUrl: window.location.href, clear: clearDocument, status: setStatus,
  prepare: loadVendor, render: renderDocument,
});
bindExplorerLifecycle(session, window, document);
document.getElementById('docs-refresh')?.addEventListener('click', () => { void session.reload(); });
document.getElementById('docs-logout')?.addEventListener('click', () => session.clear('denied'));
// The reference cannot navigate to third-party URLs embedded in descriptions.
referenceHost.addEventListener('click', event => {
  const anchor = event.target instanceof Element ? event.target.closest('a[href]') : null;
  if (anchor instanceof HTMLAnchorElement && anchor.origin !== window.location.origin) event.preventDefault();
});
window.addEventListener('languagechange', () => { void loadLanguage(); });
window.addEventListener('storage', event => { if (event.key === 'lightnvr-locale-preference') void loadLanguage(); });
void loadLanguage();
if (document.visibilityState !== 'hidden') void session.reload();
