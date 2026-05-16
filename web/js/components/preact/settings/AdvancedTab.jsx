/**
 * AdvancedTab — settings that don't belong in the first-hop tabs:
 * memory optimization (buffer size, swap file/size), WebRTC TURN relay,
 * ONVIF discovery.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

export function AdvancedTab({ settings, handleInputChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      {/* Memory optimization */}
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.memoryOptimization')}</h3>
        <div data-setting-label={t('settings.bufferSizeKb')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-buffer-size" class="font-medium">{t('settings.bufferSizeKb')}</label>
          <input
            type="number"
            id="setting-buffer-size"
            name="bufferSize"
            min="128"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.bufferSize}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.useSwapFile')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-use-swap" class="font-medium">{t('settings.useSwapFile')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-use-swap"
              name="useSwap"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.useSwap}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
        </div>
        <div data-setting-label={t('settings.swapSizeMb')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-swap-size" class="font-medium">{t('settings.swapSizeMb')}</label>
          <input
            type="number"
            id="setting-swap-size"
            name="swapSize"
            min="32"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.swapSize}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
      </div>

      {/* WebRTC TURN server */}
      <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.webrtcTurnServer')}</h3>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.webrtcTurnServerDescription')}
        </p>
        <div data-setting-label={t('settings.enableTurnRelay')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-turn-enabled" class="font-medium">{t('settings.enableTurnRelay')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-turn-enabled"
              name="turnEnabled"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.turnEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableTurnRelayHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.turnServerUrl')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-turn-server-url" class="font-medium">{t('settings.turnServerUrl')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-turn-server-url"
              name="turnServerUrl"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.turnServerUrl}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('settings.turnServerUrlPlaceholder')}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">
              {t('settings.turnServerUrlHelp')}
            </span>
          </div>
        </div>
        <div data-setting-label={t('auth.username')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-turn-username" class="font-medium">{t('auth.username')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-turn-username"
              name="turnUsername"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.turnUsername}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('settings.optionalPlaceholder')}
            />
          </div>
        </div>
        <div data-setting-label={t('auth.password')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-turn-password" class="font-medium">{t('auth.password')}</label>
          <div class="col-span-2">
            <input
              type="password"
              id="setting-turn-password"
              name="turnPassword"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.turnPassword}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('settings.optionalPlaceholder')}
            />
          </div>
        </div>
      </div>

      {/* ONVIF discovery */}
      <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.onvifDiscovery')}</h3>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.onvifDiscoveryDescription')}
        </p>
        <div data-setting-label={t('settings.enableOnvifDiscovery')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-onvif-discovery-enabled" class="font-medium">{t('settings.enableOnvifDiscovery')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-onvif-discovery-enabled"
              name="onvifDiscoveryEnabled"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.onvifDiscoveryEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableOnvifDiscoveryHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.discoveryIntervalSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-onvif-discovery-interval" class="font-medium">{t('settings.discoveryIntervalSeconds')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-onvif-discovery-interval"
              name="onvifDiscoveryInterval"
              min="30"
              max="3600"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.onvifDiscoveryInterval}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.discoveryIntervalSecondsRangeHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.discoveryNetwork')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-onvif-discovery-network" class="font-medium">{t('settings.discoveryNetwork')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-onvif-discovery-network"
              name="onvifDiscoveryNetwork"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.onvifDiscoveryNetwork}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('streams.auto')}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">
              {t('settings.discoveryNetworkExamples')}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AdvancedTab;
