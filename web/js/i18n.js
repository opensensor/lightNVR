import { useEffect, useState } from 'preact/hooks';

export const DEFAULT_LOCALE = 'en';
export const AUTO_LOCALE = 'auto';

const LOCALE_STORAGE_KEY = 'lightnvr-locale-preference';
const FALLBACK_MANIFEST = {
  locales: [
    { code: 'en', name: 'English', nativeName: 'English' },
    { code: 'pt-BR', name: 'Portuguese (Brazil)', nativeName: 'Português (Brasil)' },
  ],
};

let manifest = FALLBACK_MANIFEST;
let manifestPromise = null;
let initPromise = null;
let currentLocale = DEFAULT_LOCALE;
let localePreference = null;
let translations = {};
let languageChangeListenerRegistered = false;

const listeners = new Set();

function getLocalesUrl(fileName) {
  const origin = typeof window !== 'undefined' && window.location?.origin
    ? window.location.origin
    : 'http://localhost';
  return new URL(`/locales/${fileName}`, origin).toString();
}

function normalizeLocaleCode(value) {
  return typeof value === 'string' ? value.trim().replace('_', '-') : '';
}

function getLocaleCandidates() {
  if (typeof navigator === 'undefined') {
    return [DEFAULT_LOCALE];
  }

  const candidates = Array.isArray(navigator.languages) && navigator.languages.length > 0
    ? navigator.languages
    : [navigator.language || DEFAULT_LOCALE];

  return candidates.map(normalizeLocaleCode).filter(Boolean);
}

export function resolveSupportedLocale(candidates) {
  const supportedLocales = getAvailableLocales().map((locale) => locale.code);
  const supportedLower = new Map(supportedLocales.map((locale) => [locale.toLowerCase(), locale]));
  const values = Array.isArray(candidates) ? candidates : [candidates];

  for (const candidate of values.map(normalizeLocaleCode).filter(Boolean)) {
    const exactMatch = supportedLower.get(candidate.toLowerCase());
    if (exactMatch) {
      return exactMatch;
    }

    const baseLanguage = candidate.split('-')[0].toLowerCase();
    const baseMatch = supportedLocales.find((locale) => locale.toLowerCase().split('-')[0] === baseLanguage);
    if (baseMatch) {
      return baseMatch;
    }
  }

  return DEFAULT_LOCALE;
}

export function getBrowserPreferredLocale() {
  return resolveSupportedLocale(getLocaleCandidates());
}

function getStoredLocalePreference() {
  try {
    const stored = localStorage.getItem(LOCALE_STORAGE_KEY);
    return stored ? resolveSupportedLocale(stored) : null;
  } catch {
    return null;
  }
}

function persistLocalePreference(preference) {
  try {
    if (preference) {
      localStorage.setItem(LOCALE_STORAGE_KEY, preference);
    } else {
      localStorage.removeItem(LOCALE_STORAGE_KEY);
    }
  } catch {
    // Ignore storage failures.
  }
}

async function loadManifest() {
  if (!manifestPromise) {
    manifestPromise = fetch(getLocalesUrl('manifest.json'), { cache: 'no-cache' })
      .then((response) => response.ok ? response.json() : FALLBACK_MANIFEST)
      .catch(() => FALLBACK_MANIFEST)
      .then((data) => {
        manifest = data?.locales?.length ? data : FALLBACK_MANIFEST;
        return manifest;
      });
  }

  return manifestPromise;
}

async function loadTranslations(locale) {
  const response = await fetch(getLocalesUrl(`${locale}.json`), { cache: 'no-cache' });
  if (response.ok) {
    return response.json();
  }

  if (locale !== DEFAULT_LOCALE) {
    return loadTranslations(DEFAULT_LOCALE);
  }

  return {};
}

function notifyListeners() {
  if (typeof document !== 'undefined') {
    document.documentElement.lang = currentLocale;
    document.dispatchEvent?.(new CustomEvent('lightnvr:localechange', { detail: { locale: currentLocale } }));
  }

  listeners.forEach((listener) => listener());
}

async function applyLocale(locale) {
  currentLocale = locale;
  translations = await loadTranslations(locale);
  notifyListeners();
  return currentLocale;
}

function ensureLanguageChangeListener() {
  if (languageChangeListenerRegistered || typeof window === 'undefined') {
    return;
  }

  window.addEventListener('languagechange', () => {
    if (!localePreference) {
      void applyLocale(getBrowserPreferredLocale());
    }
  });
  languageChangeListenerRegistered = true;
}

export async function initI18n() {
  if (!initPromise) {
    initPromise = (async () => {
      await loadManifest();
      ensureLanguageChangeListener();
      localePreference = getStoredLocalePreference();
      const initialLocale = localePreference || getBrowserPreferredLocale();
      await applyLocale(initialLocale);
      return currentLocale;
    })().catch(async () => {
      manifest = FALLBACK_MANIFEST;
      await applyLocale(DEFAULT_LOCALE);
      return currentLocale;
    });
  }

  return initPromise;
}

export async function setLocalePreference(preference) {
  await loadManifest();
  localePreference = preference ? resolveSupportedLocale(preference) : null;
  persistLocalePreference(localePreference);
  return applyLocale(localePreference || getBrowserPreferredLocale());
}

export function getLocale() {
  return currentLocale;
}

export function getLocalePreference() {
  return localePreference;
}

export function getAvailableLocales() {
  return manifest.locales || FALLBACK_MANIFEST.locales;
}

export function t(key, params = {}) {
  const template = translations[key] || key;
  return template.replace(/\{([^}]+)\}/g, (_, name) => {
    const value = params[name.trim()];
    return value === undefined || value === null ? '' : String(value);
  });
}

export function subscribeToLocaleChange(listener) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useI18n() {
  const [, setVersion] = useState(0);

  useEffect(() => {
    let mounted = true;
    void initI18n().finally(() => {
      if (mounted) {
        setVersion((value) => value + 1);
      }
    });

    const unsubscribe = subscribeToLocaleChange(() => {
      setVersion((value) => value + 1);
    });

    return () => {
      mounted = false;
      unsubscribe();
    };
  }, []);

  return {
    t,
    locale: currentLocale,
    localePreference,
    availableLocales: getAvailableLocales(),
    setLocalePreference,
    AUTO_LOCALE,
  };
}

export function __resetI18nStateForTests() {
  manifest = FALLBACK_MANIFEST;
  manifestPromise = null;
  initPromise = null;
  currentLocale = DEFAULT_LOCALE;
  localePreference = null;
  translations = {};
  languageChangeListenerRegistered = false;
  listeners.clear();
}