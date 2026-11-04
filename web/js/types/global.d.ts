/**
 * Window properties written and read by the core modules (auth-utils.ts,
 * fetch-utils.ts). Globals that belong to a single feature (toasts, batch
 * delete, modal context) are declared by the domain that owns them.
 */
export {};

declare global {
  interface Window {
    /** Set by setupSessionValidation when /api/auth/verify reports auth_enabled: false. */
    _authDisabled?: boolean;
    /** Set by validateSession from /api/auth/verify. */
    _demoMode?: boolean;
  }
}
