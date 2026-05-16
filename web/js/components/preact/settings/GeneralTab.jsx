/**
 * GeneralTab — General settings (log level, syslog, web port/bind IP,
 * web thread pool, max streams, rerun setup wizard).
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

import { showStatusMessage } from '../ToastContainer.jsx';

export function GeneralTab({ settings, handleInputChange, canModifySettings, restartNotice, t }) {
  return (
    <div class="space-y-6">
      {/* Logging */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.general')}</h3>
        <div data-setting-label={t('settings.logLevel')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-log-level" class="font-medium">{t('settings.logLevel')}</label>
          <select
            id="setting-log-level"
            name="logLevel"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.logLevel}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          >
            <option value="0">{t('settings.logLevelError')}</option>
            <option value="1">{t('settings.logLevelWarning')}</option>
            <option value="2">{t('settings.logLevelInfo')}</option>
            <option value="3">{t('settings.logLevelDebug')}</option>
          </select>
        </div>
        <div data-setting-label={t('settings.enableSyslog')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-syslog-enabled" class="font-medium">{t('settings.enableSyslog')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-syslog-enabled"
              name="syslogEnabled"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.syslogEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableSyslogHelp')}</span>
          </div>
        </div>
        {settings.syslogEnabled && (
          <>
            <div data-setting-label={t('settings.syslogIdent')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-syslog-ident" class="font-medium">{t('settings.syslogIdent')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-syslog-ident"
                  name="syslogIdent"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.syslogIdent}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder="lightnvr"
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.syslogIdentHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.syslogFacility')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-syslog-facility" class="font-medium">{t('settings.syslogFacility')}</label>
              <div class="col-span-2">
                <select
                  id="setting-syslog-facility"
                  name="syslogFacility"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.syslogFacility}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                >
                  <option value="LOG_USER">LOG_USER</option>
                  <option value="LOG_DAEMON">LOG_DAEMON</option>
                  <option value="LOG_LOCAL0">LOG_LOCAL0</option>
                  <option value="LOG_LOCAL1">LOG_LOCAL1</option>
                  <option value="LOG_LOCAL2">LOG_LOCAL2</option>
                  <option value="LOG_LOCAL3">LOG_LOCAL3</option>
                  <option value="LOG_LOCAL4">LOG_LOCAL4</option>
                  <option value="LOG_LOCAL5">LOG_LOCAL5</option>
                  <option value="LOG_LOCAL6">LOG_LOCAL6</option>
                  <option value="LOG_LOCAL7">LOG_LOCAL7</option>
                </select>
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.syslogFacilityHelp')}</span>
              </div>
            </div>
          </>
        )}
      </div>

      {/* Web interface */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.webInterface')}</h3>
        {restartNotice && (
          <div class="mb-4 rounded border border-yellow-300 bg-yellow-50 dark:bg-yellow-900/20 px-4 py-3 text-sm text-yellow-900 dark:text-yellow-100">
            <strong>{t('settings.pendingRestartLabel')}</strong> {restartNotice}
          </div>
        )}
        <div data-setting-label={t('settings.webPort')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-web-port" class="font-medium">{t('settings.webPort')}</label>
          <input
            type="number"
            id="setting-web-port"
            name="webPort"
            min="1"
            max="65535"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.webPort}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.webBindIp')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-web-bind-ip" class="font-medium">{t('settings.webBindIp')}</label>
          <input
            type="text"
            id="setting-web-bind-ip"
            name="webBindIp"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.webBindIp}
            onChange={handleInputChange}
            disabled={!canModifySettings}
            placeholder="0.0.0.0"
          />
        </div>
        <div data-setting-label={t('settings.threadPoolSize')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-thread-pool" class="font-medium">
            {t('settings.threadPoolSize')}
            <span class="ml-1 text-xs text-muted-foreground">({t('settings.requiresRestart')})</span>
          </label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-thread-pool"
              name="webThreadPoolSize"
              min="2"
              max="128"
              placeholder={t('settings.threadPoolPlaceholder')}
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.webThreadPoolSize}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <p class="text-xs text-muted-foreground mt-1">
              {t('settings.threadPoolHelp')}
            </p>
          </div>
        </div>
        <div data-setting-label={t('settings.maxStreams')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-max-streams" class="font-medium">
            {t('settings.maxStreams')}
            <span class="ml-1 text-xs text-muted-foreground">({t('settings.requiresRestart')})</span>
          </label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-max-streams"
              name="maxStreams"
              min="1"
              max="256"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.maxStreams}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <p class="text-xs text-muted-foreground mt-1">
              {t('settings.maxStreamsHelpBefore')} <strong>{t('settings.serviceRestart')}</strong>. {t('settings.maxStreamsHelpAfter')}
            </p>
          </div>
        </div>
        {/* Setup Wizard reset */}
        <div data-setting-label={t('settings.setupWizard')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label class="font-medium">{t('settings.setupWizard')}</label>
          <div class="col-span-2">
            <button
              type="button"
              class="px-4 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm"
              disabled={!canModifySettings}
              onClick={async () => {
                try {
                  const res = await fetch('/api/setup/status', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ complete: false }),
                  });
                  if (!res.ok) throw new Error(`HTTP ${res.status}`);
                  window.location.href = 'index.html';
                } catch (err) {
                  showStatusMessage(t('settings.resetWizardError', { message: err.message }), 'error');
                }
              }}
            >
              {t('settings.rerunSetupWizard')}
            </button>
            <p class="text-xs text-muted-foreground mt-1">
              {t('settings.setupWizardHelp')}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default GeneralTab;
