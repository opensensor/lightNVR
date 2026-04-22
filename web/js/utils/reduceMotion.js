/**
 * Reduce-motion preference utility for LightNVR.
 *
 * PRD: UXD_01 §5.4 — Theme polish, task T4.
 *
 * Three-state preference stored in localStorage:
 *   'auto' — honor `prefers-reduced-motion: reduce` (default)
 *   'on'   — force reduced motion regardless of system preference
 *   'off'  — force full motion regardless of system preference
 *
 * The preference is applied to <html data-reduce-motion="true|false"> so a
 * single CSS rule in web/css/theme/scrollbar.css can neutralize animations
 * and transitions globally when motion is reduced.
 *
 * Callers:
 *   - applyReduceMotion() is called once on app boot (see main.js-equivalent
 *     boot calls and theme-init.js inline script — we re-apply from the JS
 *     entry to keep things deterministic and to cover the async render path).
 *   - SettingsView.jsx calls setReduceMotionPref(value) when the user picks
 *     a different option, then calls applyReduceMotion() to reflect it.
 */

export const REDUCE_MOTION_STORAGE_KEY = 'lightnvr.reduceMotion';
export const REDUCE_MOTION_VALUES = Object.freeze(['auto', 'on', 'off']);
const DEFAULT_PREF = 'auto';

/**
 * Read the user's stored preference. Returns one of REDUCE_MOTION_VALUES.
 * Any unknown / missing value falls back to 'auto'.
 * @returns {'auto' | 'on' | 'off'}
 */
export function getReduceMotionPref() {
  try {
    if (typeof localStorage === 'undefined') return DEFAULT_PREF;
    const stored = localStorage.getItem(REDUCE_MOTION_STORAGE_KEY);
    if (REDUCE_MOTION_VALUES.includes(stored)) {
      return stored;
    }
    return DEFAULT_PREF;
  } catch (_err) {
    return DEFAULT_PREF;
  }
}

/**
 * Persist a preference value. Invalid inputs are coerced to 'auto'.
 * Does NOT re-apply the attribute — callers that want the change visible
 * should call applyReduceMotion() right after.
 * @param {'auto' | 'on' | 'off'} value
 */
export function setReduceMotionPref(value) {
  const normalized = REDUCE_MOTION_VALUES.includes(value) ? value : DEFAULT_PREF;
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(REDUCE_MOTION_STORAGE_KEY, normalized);
    }
  } catch (_err) {
    // Storage may be unavailable (private mode, quota, etc.) — silent fallback.
  }
  return normalized;
}

/**
 * Resolve the preference to a concrete boolean. 'auto' consults matchMedia.
 * Exported separately for components that want to show an "effective" state.
 * @returns {boolean} true when motion should be reduced
 */
export function isReduceMotionActive() {
  const pref = getReduceMotionPref();
  if (pref === 'on') return true;
  if (pref === 'off') return false;
  // 'auto': defer to the OS/browser setting.
  try {
    return typeof window !== 'undefined'
      && typeof window.matchMedia === 'function'
      && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  } catch (_err) {
    return false;
  }
}

/**
 * Apply the resolved preference to <html> as a data attribute so CSS can
 * key off it. Safe to call multiple times (idempotent).
 * @returns {boolean} the effective state that was applied
 */
export function applyReduceMotion() {
  const active = isReduceMotionActive();
  try {
    if (typeof document !== 'undefined' && document.documentElement) {
      document.documentElement.setAttribute('data-reduce-motion', active ? 'true' : 'false');
    }
  } catch (_err) {
    // No-op if document isn't available (SSR / test env).
  }
  return active;
}

// Keep the effective state synced with OS-level changes when the user has
// selected 'auto'. Safe in SSR/Jest where matchMedia may be missing.
let _mediaListenerInstalled = false;
export function installReduceMotionMediaListener() {
  if (_mediaListenerInstalled) return;
  try {
    if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') return;
    const mql = window.matchMedia('(prefers-reduced-motion: reduce)');
    const onChange = () => {
      // Only re-apply when the user is on 'auto' — other values are explicit.
      if (getReduceMotionPref() === 'auto') {
        applyReduceMotion();
      }
    };
    if (typeof mql.addEventListener === 'function') {
      mql.addEventListener('change', onChange);
    } else if (typeof mql.addListener === 'function') {
      // Safari < 14 fallback
      mql.addListener(onChange);
    }
    _mediaListenerInstalled = true;
  } catch (_err) {
    // Silent — preference-resolution still works on the next applyReduceMotion().
  }
}
