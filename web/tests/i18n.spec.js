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
            { code: 'pt-BR', nativeName: 'Português (Brasil)' },
          ],
        }));
      }

      if (url.endsWith('/locales/en.json')) {
        return Promise.resolve(createJsonResponse({
          'nav.settings': 'Settings',
          'login.rememberDevice': 'Remember this device for {days} days',
        }));
      }

      if (url.endsWith('/locales/pt-BR.json')) {
        return Promise.resolve(createJsonResponse({
          'nav.settings': 'Configurações',
          'login.rememberDevice': 'Lembrar este dispositivo por {days} dias',
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
    expect(resolveSupportedLocale('pt-PT')).toBe('pt-BR');
    expect(resolveSupportedLocale('de-DE')).toBe('en');
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
});