import { useEffect, useState } from 'preact/hooks';
import {
  applyReduceMotion,
  installReduceMotionMediaListener,
} from './utils/reduceMotion.ts';

export const DEFAULT_LOCALE = 'en';
export const AUTO_LOCALE = 'auto';

export interface LocaleInfo {
  code: string;
  name: string;
  nativeName: string;
}

export interface LocaleManifest {
  locales: LocaleInfo[];
}

export type TextDirection = 'ltr' | 'rtl';

/** Placeholder values for t(). null and undefined render as an empty string. */
export type TranslationParams = Record<string, unknown>;

type TranslationTable = Record<string, string>;

const LOCALE_STORAGE_KEY = 'lightnvr-locale-preference';
const FALLBACK_MANIFEST: LocaleManifest = {
  locales: [
    { code: 'en', name: 'English', nativeName: 'English' },
    { code: 'es', name: 'Spanish', nativeName: 'Español' },
    { code: 'de', name: 'German', nativeName: 'Deutsch' },
    { code: 'fr', name: 'French', nativeName: 'Français' },
    { code: 'fil', name: 'Filipino', nativeName: 'Filipino' },
    { code: 'it', name: 'Italian', nativeName: 'Italiano' },
    { code: 'ja', name: 'Japanese', nativeName: '日本語' },
    { code: 'ko', name: 'Korean', nativeName: '한국어' },
    { code: 'zh', name: 'Chinese', nativeName: '中文' },
    { code: 'zh-TW', name: 'Chinese (Traditional)', nativeName: '繁體中文' },
    { code: 'pt-BR', name: 'Portuguese (Brazil)', nativeName: 'Português (Brasil)' },
    { code: 'pt-PT', name: 'Portuguese (Portugal)', nativeName: 'Português (Portugal)' },
    { code: 'nl', name: 'Dutch', nativeName: 'Nederlands' },
    { code: 'pl', name: 'Polish', nativeName: 'Polski' },
    { code: 'tr', name: 'Turkish', nativeName: 'Türkçe' },
    { code: 'uk', name: 'Ukrainian', nativeName: 'Українська' },
    { code: 'ru', name: 'Russian', nativeName: 'Русский' },
    { code: 'ar', name: 'Arabic', nativeName: 'العربية' },
  ],
};

let manifest = FALLBACK_MANIFEST;
let manifestPromise: Promise<LocaleManifest> | null = null;
let initPromise: Promise<string> | null = null;
let currentLocale: string = DEFAULT_LOCALE;
let localePreference: string | null = null;
const RTL_LANGUAGES = new Set(['ar', 'he', 'fa', 'ur']);
let translations: TranslationTable = {};
let languageChangeListenerRegistered = false;
let translationsCache = new Map<string, Promise<TranslationTable | null>>();

const listeners = new Set<() => void>();

export function getLocalesUrl(fileName: string): string {
  const origin = typeof window !== 'undefined' && window.location?.origin
    ? window.location.origin
    : 'http://localhost';
  return new URL(`/locales/${fileName}`, origin).toString();
}

function normalizeLocaleCode(value: unknown): string {
  return typeof value === 'string' ? value.trim().replace('_', '-') : '';
}

function getLocaleCandidates() {
  if (typeof navigator === 'undefined') {
    return [DEFAULT_LOCALE];
  }

  const candidates: readonly string[] = Array.isArray(navigator.languages) && navigator.languages.length > 0
    ? navigator.languages
    : [navigator.language || DEFAULT_LOCALE];

  return candidates.map(normalizeLocaleCode).filter(Boolean);
}

export function resolveSupportedLocale(candidates: string | readonly string[] | null | undefined): string {
  const supportedLocales = getAvailableLocales().map((locale) => locale.code);
  const supportedLower = new Map(supportedLocales.map((locale) => [locale.toLowerCase(), locale]));
  const values = Array.isArray(candidates) ? candidates : [candidates];

  for (const candidate of values.map(normalizeLocaleCode).filter(Boolean)) {
    const exactMatch = supportedLower.get(candidate.toLowerCase());
    if (exactMatch) {
      return exactMatch;
    }

    // Browsers may identify Filipino preferences with the Tagalog code.
    const candidateLanguage = candidate.split('-')[0]!.toLowerCase();
    const baseLanguage = candidateLanguage === 'tl' ? 'fil' : candidateLanguage;
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

function persistLocalePreference(preference: string | null): void {
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
      .then((data: LocaleManifest | null) => {
        manifest = data?.locales?.length ? data : FALLBACK_MANIFEST;
        return manifest;
      });
  }

  return manifestPromise;
}

async function fetchTranslations(locale: string) {
  if (translationsCache.has(locale)) {
    return translationsCache.get(locale);
  }

  const request = fetch(getLocalesUrl(`${locale}.json`), { cache: 'no-cache' })
    .then((response) => response.ok ? response.json() : null)
    .catch(() => null);

  translationsCache.set(locale, request);
  return request;
}

async function loadTranslations(locale: string): Promise<TranslationTable> {
  const fallbackTranslations = await fetchTranslations(DEFAULT_LOCALE) || {};

  if (locale === DEFAULT_LOCALE) {
    return fallbackTranslations;
  }

  const localizedTranslations = await fetchTranslations(locale);
  if (localizedTranslations) {
    return { ...fallbackTranslations, ...localizedTranslations };
  }

  return fallbackTranslations;
}

function notifyListeners() {
  if (typeof document !== 'undefined') {
    document.documentElement.lang = currentLocale;
    document.documentElement.dir = getTextDirection(currentLocale);
    document.dispatchEvent?.(new CustomEvent('lightnvr:localechange', { detail: { locale: currentLocale } }));
  }

  listeners.forEach((listener) => listener());
}

async function applyLocale(locale: string): Promise<string> {
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
    // UXD T4 — reduce-motion: re-apply on boot (the inline theme-init script
    // already sets the attribute before paint, but calling this here covers
    // any state drift from storage writes elsewhere in the session) and
    // install the media-query listener so 'auto' tracks OS changes live.
    try {
      applyReduceMotion();
      installReduceMotionMediaListener();
    } catch (_err) { /* boot must not fail on preference plumbing */ }

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

export async function setLocalePreference(preference?: string | null): Promise<string> {
  await loadManifest();
  localePreference = preference ? resolveSupportedLocale(preference) : null;
  persistLocalePreference(localePreference);
  return applyLocale(localePreference || getBrowserPreferredLocale());
}

/**
 * Writing direction for a locale ('rtl' for Arabic, Hebrew, Persian, Urdu).
 * Components that pin a region to LTR (PTZ pad, timeline track, transport
 * controls) use this to restore the page direction for nested text UI.
 */
export function getTextDirection(locale: string | null | undefined = currentLocale): TextDirection {
  return RTL_LANGUAGES.has(String(locale || '').split('-')[0]!) ? 'rtl' : 'ltr';
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

export function t(key: string, params?: TranslationParams): string;
/** @deprecated t() has no fallback argument. A string here is not used as the text; see StreamsView.tsx. */
export function t(key: string, ignored: string): string;
export function t(key: string, params: TranslationParams | string = {}): string {
  const template = translations[key] || key;
  return template.replace(/\{([^}]+)\}/g, (_, name: string) => {
    const value = (params as TranslationParams)[name.trim()];
    return value === undefined || value === null ? '' : String(value);
  });
}

export function subscribeToLocaleChange(listener: () => void) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useI18n() {
  const [, setVersion] = useState(0); // intentional: state value unused; setVersion used only to force re-render

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
    dir: getTextDirection(currentLocale),
    localePreference,
    availableLocales: getAvailableLocales(),
    setLocalePreference,
    AUTO_LOCALE,
    getLocalesUrl,
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
  translationsCache = new Map();
  listeners.clear();
}
