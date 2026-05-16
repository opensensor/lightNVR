/**
 * AuthTab — Authentication, session/device management, WebRTC toggle,
 * trusted-proxy allowlist, MFA enforcement, login rate limiting.
 *
 * The session + trusted-device listing blocks live here too (they are
 * clearly an "auth surface" even though they aren't part of `settings`
 * state — their data lives behind `authSessionsData` / `trustedDevicesData`
 * queries owned by the parent and passed in as props).
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

import { AsyncButton } from '../AsyncButton.jsx';

export function AuthTab({
  settings,
  handleInputChange,
  canModifySettings,
  t,
  authSessionsData,
  authSessionsLoading,
  authSessionsIsError,
  authSessionsError,
  revokeSessionMutation,
  trustedDevicesData,
  trustedDevicesLoading,
  trustedDevicesIsError,
  trustedDevicesError,
  revokeTrustedDeviceMutation,
  formatTimestamp,
}) {
  return (
    <div class="space-y-6">
      {/* Authentication & access */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.authentication') || 'Authentication & Access'}</h3>
        <div data-setting-label={t('settings.enableAuthentication')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-auth-enabled" class="font-medium">{t('settings.enableAuthentication')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-auth-enabled"
              name="authEnabled"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.authEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
        </div>
        <div data-setting-label={t('auth.demoMode')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-demo-mode" class="font-medium">{t('auth.demoMode')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-demo-mode"
              name="demoMode"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.demoMode}
              onChange={handleInputChange}
              disabled={!canModifySettings || !settings.authEnabled}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.demoModeHelp')}</span>
          </div>
        </div>

        {/* Dashboard view methods (#397) — WebRTC / HLS / MSE as independent
            checkboxes.  Internally each is stored as a *_disabled flag; the
            UI presents them as positive "enable" toggles so the control
            direction matches user intent.  Backend rejects the save if all
            three would end up disabled. */}
        {(() => {
          const viewMethods = [
            { key: 'webrtcDisabled', id: 'setting-webrtc-enabled', label: t('settings.viewMethodWebrtc') },
            { key: 'hlsDisabled',    id: 'setting-hls-enabled',    label: t('settings.viewMethodHls') },
            { key: 'mseDisabled',    id: 'setting-mse-enabled',    label: t('settings.viewMethodMse') },
          ];
          const enabledCount = viewMethods.filter(m => !settings[m.key]).length;
          return (
            <div data-setting-label={t('settings.enabledViewMethods')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
              <label class="font-medium">{t('settings.enabledViewMethods')}</label>
              <div class="col-span-2 flex flex-col gap-2">
                {viewMethods.map(m => (
                  <label key={m.key} for={m.id} class="inline-flex items-center gap-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id={m.id}
                      class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                      style={{ accentColor: 'hsl(var(--primary))' }}
                      checked={!settings[m.key]}
                      onChange={(e) => handleInputChange({
                        target: { name: m.key, type: 'checkbox', checked: !e.target.checked }
                      })}
                      /* Block the user from unchecking the last enabled method
                         client-side; backend also rejects this, but disabling
                         the control avoids an error round-trip. */
                      disabled={!canModifySettings || (enabledCount === 1 && !settings[m.key])}
                    />
                    <span class="text-sm">{m.label}</span>
                  </label>
                ))}
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enabledViewMethodsHelp')}</span>
              </div>
            </div>
          );
        })()}
        <div data-setting-label={t('settings.sessionIdleTimeoutHours')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-auth-timeout" class="font-medium">{t('settings.sessionIdleTimeoutHours')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-auth-timeout"
              name="authTimeoutHours"
              min="1"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.authTimeoutHours}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.sessionIdleTimeoutHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.absoluteSessionLifetimeHours')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-auth-absolute-timeout" class="font-medium">{t('settings.absoluteSessionLifetimeHours')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-auth-absolute-timeout"
              name="authAbsoluteTimeoutHours"
              min="1"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.authAbsoluteTimeoutHours}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.absoluteSessionLifetimeHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.rememberDeviceDays')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-trusted-device-days" class="font-medium">{t('settings.rememberDeviceDays')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-trusted-device-days"
              name="trustedDeviceDays"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.trustedDeviceDays}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.rememberDeviceDaysHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.trustedProxyCidrs')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
          <label for="setting-trusted-proxy-cidrs" class="font-medium">{t('settings.trustedProxyCidrs')}</label>
          <div class="col-span-2">
            <textarea
              id="setting-trusted-proxy-cidrs"
              name="trustedProxyCidrs"
              rows="3"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-2xl disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.trustedProxyCidrs}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="127.0.0.1/32,::1/128"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.trustedProxyCidrsHelpBefore')} <code>X-Forwarded-For</code> / <code>X-Real-IP</code> {t('settings.trustedProxyCidrsHelpAfter')}</span>
          </div>
        </div>
      </div>

      {/* Session & device management */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.sessionAndDeviceManagement')}</h3>
        <div class="mb-6">
          <h4 class="font-medium mb-2">{t('settings.activeSessions')}</h4>
          <div class="space-y-3">
            {authSessionsLoading ? (
              <p class="text-sm text-muted-foreground">{t('settings.loadingSessions')}</p>
            ) : authSessionsIsError ? (
              <p class="text-sm text-destructive">{t('settings.failedToLoadSessions', { message: authSessionsError?.message || '' })}</p>
            ) : ((authSessionsData?.sessions || []).length === 0 ? (
              <p class="text-sm text-muted-foreground">{t('settings.noActiveSessionsFound')}</p>
            ) : (
              (authSessionsData?.sessions || []).map((session) => (
                <div key={session.id} class="border border-border rounded p-3 flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
                  <div class="text-sm">
                    <div class="font-medium">{session.current ? t('settings.currentSession') : t('settings.sessionNumber', { id: session.id })}</div>
                    <div class="text-muted-foreground">{t('settings.lastActive')}: {formatTimestamp(session.last_activity_at)}</div>
                    <div class="text-muted-foreground">{t('settings.idleExpiry')}: {formatTimestamp(session.idle_expires_at)} · {t('settings.absoluteExpiry')}: {formatTimestamp(session.expires_at)}</div>
                    <div class="text-muted-foreground">{session.ip_address || t('settings.unknownIp')} · {session.user_agent || t('settings.unknownBrowser')}</div>
                  </div>
                  {/* AsyncButton shows a spinner + self-disables while the
                      DELETE is in flight so users don't rapid-tap and fire
                      duplicate revokes (#399 / PRD UXD_01 §5.1 / T1). */}
                  <AsyncButton
                    type="button"
                    className="px-3 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm self-start"
                    onClick={() => revokeSessionMutation.mutateAsync(session.id)}
                  >
                    {t('settings.revoke')}
                  </AsyncButton>
                </div>
              ))
            ))}
          </div>
        </div>
        <div>
          <h4 class="font-medium mb-2">{t('settings.trustedDevices')}</h4>
          <div class="space-y-3">
            {trustedDevicesLoading ? (
              <p class="text-sm text-muted-foreground">{t('settings.loadingTrustedDevices')}</p>
            ) : trustedDevicesIsError ? (
              <p class="text-sm text-destructive">{t('settings.failedToLoadTrustedDevices', { message: trustedDevicesError?.message || '' })}</p>
            ) : ((trustedDevicesData?.trusted_devices || []).length === 0 ? (
              <p class="text-sm text-muted-foreground">{t('settings.noRememberedDevicesFound')}</p>
            ) : (
              (trustedDevicesData?.trusted_devices || []).map((device) => (
                <div key={device.id} class="border border-border rounded p-3 flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
                  <div class="text-sm">
                    <div class="font-medium">{device.current ? t('settings.thisDevice') : t('settings.trustedDeviceNumber', { id: device.id })}</div>
                    <div class="text-muted-foreground">{t('settings.lastUsed')}: {formatTimestamp(device.last_used_at)}</div>
                    <div class="text-muted-foreground">{t('settings.expires')}: {formatTimestamp(device.expires_at)}</div>
                    <div class="text-muted-foreground">{device.ip_address || t('settings.unknownIp')} · {device.user_agent || t('settings.unknownBrowser')}</div>
                  </div>
                  {/* AsyncButton — same pending-guard pattern as the
                      session revoke button above. */}
                  <AsyncButton
                    type="button"
                    className="px-3 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm self-start"
                    onClick={() => revokeTrustedDeviceMutation.mutateAsync(device.id)}
                  >
                    {t('settings.revoke')}
                  </AsyncButton>
                </div>
              ))
            ))}
          </div>
        </div>
      </div>

      {/* Login security */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.loginSecurity')}</h3>
        <div data-setting-label={t('settings.forceMfaOnLogin')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-force-mfa" class="font-medium">{t('settings.forceMfaOnLogin')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-force-mfa"
              name="forceMfaOnLogin"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.forceMfaOnLogin}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.forceMfaOnLoginHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.loginRateLimiting')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-rate-limit-enabled" class="font-medium">{t('settings.loginRateLimiting')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-rate-limit-enabled"
              name="loginRateLimitEnabled"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.loginRateLimitEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.loginRateLimitingHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.maxLoginAttempts')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-rate-limit-max" class="font-medium">{t('settings.maxLoginAttempts')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-rate-limit-max"
              name="loginRateLimitMaxAttempts"
              min="1"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.loginRateLimitMaxAttempts}
              onChange={handleInputChange}
              disabled={!canModifySettings || !settings.loginRateLimitEnabled}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.maxLoginAttemptsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.rateLimitWindowSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-rate-limit-window" class="font-medium">{t('settings.rateLimitWindowSeconds')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-rate-limit-window"
              name="loginRateLimitWindowSeconds"
              min="10"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.loginRateLimitWindowSeconds}
              onChange={handleInputChange}
              disabled={!canModifySettings || !settings.loginRateLimitEnabled}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.rateLimitWindowSecondsHelp')}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AuthTab;
