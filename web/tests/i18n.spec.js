import fs from 'node:fs';
import path from 'node:path';
import {
  __resetI18nStateForTests,
  getBrowserPreferredLocale,
  getLocale,
  getLocalePreference,
  initI18n,
  resolveSupportedLocale,
  setLocalePreference,
  t,
} from '../js/i18n.js';

const ptBrCatalog = JSON.parse(fs.readFileSync(
  path.resolve(__dirname, '../public/locales/pt-BR.json'),
  'utf8',
));

const filCatalog = JSON.parse(fs.readFileSync(
  path.resolve(__dirname, '../public/locales/fil.json'),
  'utf8',
));
const localeManifest = JSON.parse(fs.readFileSync(
  path.resolve(__dirname, '../public/locales/manifest.json'),
  'utf8',
));

test('Filipino is selectable and has a flag and valid translation placeholders', () => {
  expect(localeManifest.locales).toContainEqual({
    code: 'fil', name: 'Filipino', nativeName: 'Filipino',
  });
  const flag = fs.readFileSync(path.resolve(__dirname, '../public/locales/fil.png'));
  expect(flag.subarray(0, 8)).toEqual(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]));

  const englishCatalog = JSON.parse(fs.readFileSync(
    path.resolve(__dirname, '../public/locales/en.json'), 'utf8',
  ));
  const placeholders = (value) => (value.match(/\{[^}]+\}/g) || []).sort();
  for (const [key, value] of Object.entries(filCatalog)) {
    expect(englishCatalog).toHaveProperty([key]);
    expect(typeof value).toBe('string');
    expect(value.trim()).not.toBe('');
    expect(placeholders(value)).toEqual(placeholders(englishCatalog[key]));
  }
});

const REPORTED_PT_BR_TRANSLATIONS = {
  'availability.live': 'Ao vivo agora',
  'availability.neverConnected': 'Nunca conectada',
  'live.navigator.title': 'Navegador',
  'live.navigator.subtitle': 'Locais, layouts e atividade',
  'live.navigator.search': 'Buscar uma câmera...',
  'live.plan.emptyTitle': 'Crie uma visualização espacial das câmeras',
  'investigation.experimental': 'Espaço de trabalho experimental',
  'investigation.title': 'Investigação',
  'investigation.query': 'Janela de investigação',
  'investigation.cameraSelection': 'Seleção de câmeras',
  'investigation.applyWindow': 'Aplicar intervalo de tempo',
  'investigation.minimumConfidence': 'Confiança mínima',
  'investigation.metadataRegionSearch': 'Pesquisa por região de metadados',
  'investigation.resultCount': '{count} eventos correspondentes',
  'investigation.noResults': 'Nenhum evento persistido corresponde à câmera, ao período e aos filtros selecionados.',
  'investigation.bookmarks.save': 'Salvar marcador',
  'investigation.actions.open': 'Proteger / baixar',
  'streams.configuration': 'Configuração',
  'streams.inventory': 'Inventário',
  'fleet.title': 'Inventário de Câmeras',
  'fleet.searchPlaceholder': 'Pesquisar nome, endereço, local, tag, fabricante ou modelo...',
  'fleet.paginationSummary': 'Exibindo {first}–{last} de {total}',
  'events.title': 'Eventos e Roteamento',
  'events.accessDenied': 'Acesso negado - privilégios insuficientes',
  'settings.reduceMotion': 'Reduzir movimento',
  'workspaces.title': 'Espaços de trabalho do operador',
  'workspaces.liveNavigator': 'Navegador ao vivo aprimorado',
  'workspaces.investigationDescription': 'Pesquise, correlacione, proteja e exporte evidências gravadas.',
};

test('pt-BR includes the reported operator workspace translations', () => {
  expect(ptBrCatalog).toMatchObject(REPORTED_PT_BR_TRANSLATIONS);
});

test('the footer keeps the LightNVR brand out of localization', () => {
  const footerSource = fs.readFileSync(
    path.resolve(__dirname, '../js/components/preact/Footer.jsx'),
    'utf8',
  );

  expect(footerSource).toContain('LightNVR © {year}');
  expect(footerSource).not.toContain("t('footer.tagline')");
});

function createJsonResponse(payload) {
  return {
    ok: true,
    json: async () => payload,
  };
}

describe('i18n locale selection', () => {
  beforeEach(() => {
    __resetI18nStateForTests();

    global.CustomEvent = class CustomEvent {
      constructor(type, init = {}) {
        this.type = type;
        this.detail = init.detail;
      }
    };

    global.window = {
      location: { origin: 'http://localhost' },
      addEventListener: jest.fn(),
    };

    global.document = {
      documentElement: { lang: '' },
      dispatchEvent: jest.fn(),
    };

    global.navigator = {
      languages: ['pt-BR', 'en-US'],
      language: 'pt-BR',
    };

    const storage = new Map();
    global.localStorage = {
      getItem: jest.fn((key) => storage.get(key) ?? null),
      setItem: jest.fn((key, value) => storage.set(key, value)),
      removeItem: jest.fn((key) => storage.delete(key)),
    };

    global.fetch = jest.fn((url) => {
      if (url.endsWith('/locales/manifest.json')) {
        return Promise.resolve(createJsonResponse({
          locales: [
            { code: 'en', nativeName: 'English' },
            { code: 'es', nativeName: 'Español' },
            { code: 'de', nativeName: 'Deutsch' },
            { code: 'fil', nativeName: 'Filipino' },
            { code: 'pt-BR', nativeName: 'Português (Brasil)' },
          ],
        }));
      }

      if (url.endsWith('/locales/en.json')) {
        return Promise.resolve(createJsonResponse({
          'nav.settings': 'Settings',
          'login.rememberDevice': 'Remember this device for {days} days',
          'englishOnly': 'English fallback',
        }));
      }

      if (url.endsWith('/locales/pt-BR.json')) {
        return Promise.resolve(createJsonResponse({
          'nav.settings': 'Configurações',
          'login.rememberDevice': 'Lembrar este dispositivo por {days} dias',
        }));
      }

      if (url.endsWith('/locales/fil.json')) {
        return Promise.resolve(createJsonResponse(filCatalog));
      }

      if (url.endsWith('/locales/es.json')) {
        return Promise.resolve(createJsonResponse({
          'nav.settings': 'Configuración',
        }));
      }

      return Promise.resolve({ ok: false, json: async () => ({}) });
    });
  });

  afterEach(() => {
    delete global.CustomEvent;
    delete global.window;
    delete global.document;
    delete global.navigator;
    delete global.localStorage;
    delete global.fetch;
  });

  test('matches supported locales exactly and by base language', () => {
    expect(resolveSupportedLocale('pt-BR')).toBe('pt-BR');
    expect(resolveSupportedLocale('pt-PT')).toBe('pt-PT');
    expect(resolveSupportedLocale('de-DE')).toBe('de');
  });

  test('defaults to browser preference on initialization', async () => {
    await initI18n();

    expect(getBrowserPreferredLocale()).toBe('pt-BR');
    expect(getLocale()).toBe('pt-BR');
    expect(getLocalePreference()).toBeNull();
    expect(document.documentElement.lang).toBe('pt-BR');
    expect(t('nav.settings')).toBe('Configurações');
    expect(t('login.rememberDevice', { days: 30 })).toBe('Lembrar este dispositivo por 30 dias');
  });

  test.each(['fil', 'fil-PH', 'FIL_ph', 'tl', 'tl-PH', 'TL_ph'])(
    'recognizes Filipino or Tagalog browser preference %s', async (language) => {
      navigator.languages = [language, 'en-US'];
      await initI18n();

      expect(getBrowserPreferredLocale()).toBe('fil');
      expect(getLocale()).toBe('fil');
      expect(getLocalePreference()).toBeNull();
      expect(document.documentElement.lang).toBe('fil');
      expect(t('nav.settings')).toBe('Mga Setting');
      expect(t('login.rememberDevice', { days: 30 })).toBe('Tandaan ang device na ito sa loob ng 30 araw');
      expect(t('englishOnly')).toBe('English fallback');
      expect(fetch).toHaveBeenCalledWith('http://localhost/locales/fil.json', { cache: 'no-cache' });
    },
  );

  test('preserves browser preference order when resolving language aliases', () => {
    expect(resolveSupportedLocale(['tl-PH', 'en-US'])).toBe('fil');
    expect(resolveSupportedLocale(['en-US', 'tl-PH'])).toBe('en');
    expect(resolveSupportedLocale(['ceb-PH', 'fil-PH'])).toBe('fil');
    expect(resolveSupportedLocale('ceb-PH')).toBe('en');
  });

  test('persists Filipino selected via Tagalog and restores it on initialization', async () => {
    await setLocalePreference('tl-PH');
    expect(getLocalePreference()).toBe('fil');
    expect(localStorage.setItem).toHaveBeenCalledWith('lightnvr-locale-preference', 'fil');

    __resetI18nStateForTests();
    await initI18n();
    expect(getLocale()).toBe('fil');
    expect(getLocalePreference()).toBe('fil');
    expect(t('nav.settings')).toBe('Mga Setting');

    await setLocalePreference(null);
    expect(getLocale()).toBe('pt-BR');
    expect(getLocalePreference()).toBeNull();
  });

  test('recognizes Filipino when the manifest cannot be loaded', async () => {
    const originalFetch = global.fetch;
    global.fetch = jest.fn((url, options) => url.endsWith('/locales/manifest.json')
      ? Promise.reject(new Error('manifest unavailable'))
      : originalFetch(url, options));
    navigator.languages = ['tl-PH', 'en-US'];

    await initI18n();
    expect(getLocale()).toBe('fil');
    expect(t('nav.settings')).toBe('Mga Setting');
  });

  test('stores manual overrides and can return to browser default', async () => {
    await initI18n();
    await setLocalePreference('en');

    expect(getLocale()).toBe('en');
    expect(getLocalePreference()).toBe('en');
    expect(localStorage.setItem).toHaveBeenCalledWith('lightnvr-locale-preference', 'en');
    expect(t('nav.settings')).toBe('Settings');

    await setLocalePreference(null);

    expect(getLocale()).toBe('pt-BR');
    expect(getLocalePreference()).toBeNull();
    expect(localStorage.removeItem).toHaveBeenCalledWith('lightnvr-locale-preference');
  });

  test('merges partial locale catalogs with English fallback strings', async () => {
    await initI18n();
    await setLocalePreference('es-MX');

    expect(getLocale()).toBe('es');
    expect(getLocalePreference()).toBe('es');
    expect(t('nav.settings')).toBe('Configuración');
    expect(t('login.rememberDevice', { days: 30 })).toBe('Remember this device for 30 days');
  });

  test('handles complete locale loading failure gracefully', async () => {
    // Override fetch for this test to simulate failures for both requested and fallback locales.
    global.fetch = jest.fn((url) => {
      if (url.endsWith('/locales/manifest.json')) {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            locales: [
              { code: 'en', nativeName: 'English' },
              { code: 'es', nativeName: 'Español' },
            ],
          }),
        });
      }

      // Simulate failure to load both the requested locale and the English fallback.
      if (url.endsWith('/locales/es.json') || url.endsWith('/locales/en.json')) {
        return Promise.resolve({
          ok: false,
          json: async () => ({}),
        });
      }

      return Promise.resolve({
        ok: false,
        json: async () => ({}),
      });
    });

    await expect(initI18n()).resolves.toBe('en');

    // Even if translations fail to load, t should still be callable without throwing.
    expect(() => t('nav.settings')).not.toThrow();
  });
});
