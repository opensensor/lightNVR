/**
 * MqttTab — MQTT broker settings plus Home Assistant MQTT auto-discovery.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

export function MqttTab({ settings, handleInputChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.mqttEventStreaming')}</h3>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.mqttEventStreamingDescription')}
        </p>
        <div data-setting-label={t('settings.enableMqtt')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-enabled" class="font-medium">{t('settings.enableMqtt')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-mqtt-enabled"
              name="mqttEnabled"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.mqttEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableMqttHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.brokerHost')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-broker-host" class="font-medium">{t('settings.brokerHost')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-mqtt-broker-host"
              name="mqttBrokerHost"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttBrokerHost}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="localhost"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.brokerHostHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.brokerPort')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-broker-port" class="font-medium">{t('settings.brokerPort')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-mqtt-broker-port"
              name="mqttBrokerPort"
              min="1"
              max="65535"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttBrokerPort}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.brokerPortHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('auth.username')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-username" class="font-medium">{t('auth.username')}</label>
          <div class="col-span-2">
            {/*
              Firefox autofill suppression (#399): the password manager was
              repopulating these fields with a saved site-login (or blank) on
              every Settings page load. `autocomplete="off"` is honored for
              text fields, but for password fields Firefox ignores `off` and
              only respects `new-password` — see the password input below.
              The visible username field is kept as `name="mqttUsername"` so
              it stays wired to the shared `handleInputChange` reducer in
              SettingsView, which keys off `e.target.name`.
            */}
            <input
              type="text"
              id="setting-mqtt-username"
              name="mqttUsername"
              autocomplete="off"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttUsername}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('settings.optionalPlaceholder')}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.leaveEmptyForAnonymousAccess')}</span>
          </div>
        </div>
        <div data-setting-label={t('auth.password')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-password" class="font-medium">{t('auth.password')}</label>
          <div class="col-span-2">
            <input
              type="password"
              id="setting-mqtt-password"
              name="mqttPassword"
              autocomplete="new-password"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttPassword}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder={t('settings.optionalPlaceholder')}
            />
          </div>
        </div>
        <div data-setting-label={t('settings.clientId')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-client-id" class="font-medium">{t('settings.clientId')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-mqtt-client-id"
              name="mqttClientId"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttClientId}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="lightnvr"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.clientIdHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.topicPrefix')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-topic-prefix" class="font-medium">{t('settings.topicPrefix')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-mqtt-topic-prefix"
              name="mqttTopicPrefix"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttTopicPrefix}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="lightnvr"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.topicPrefixPublishedTo', { prefix: settings.mqttTopicPrefix })}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.enableTls')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-tls-enabled" class="font-medium">{t('settings.enableTls')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-mqtt-tls-enabled"
              name="mqttTlsEnabled"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.mqttTlsEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableTlsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.keepaliveSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-keepalive" class="font-medium">{t('settings.keepaliveSeconds')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-mqtt-keepalive"
              name="mqttKeepalive"
              min="10"
              max="3600"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttKeepalive}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.keepaliveSecondsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.qosLevel')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-qos" class="font-medium">{t('settings.qosLevel')}</label>
          <div class="col-span-2">
            <select
              id="setting-mqtt-qos"
              name="mqttQos"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.mqttQos}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            >
              <option value="0">{t('settings.qosAtMostOnce')}</option>
              <option value="1">{t('settings.qosAtLeastOnce')}</option>
              <option value="2">{t('settings.qosExactlyOnce')}</option>
            </select>
          </div>
        </div>
        <div data-setting-label={t('settings.retainMessages')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-retain" class="font-medium">{t('settings.retainMessages')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-mqtt-retain"
              name="mqttRetain"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.mqttRetain}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.retainMessagesHelp')}</span>
          </div>
        </div>

        {/* Home Assistant Auto-Discovery sub-section */}
        <h4 class="text-md font-semibold mt-6 mb-3 pb-1 border-b border-border">{t('settings.homeAssistantAutoDiscovery')}</h4>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.homeAssistantAutoDiscoveryDescription')}
        </p>
        <div data-setting-label={t('settings.enableHaDiscovery')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-mqtt-ha-discovery" class="font-medium">{t('settings.enableHaDiscovery')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-mqtt-ha-discovery"
              name="mqttHaDiscovery"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.mqttHaDiscovery}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableHaDiscoveryHelp')}</span>
          </div>
        </div>
        {settings.mqttHaDiscovery && (
          <>
            <div data-setting-label={t('settings.discoveryPrefix')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-mqtt-ha-discovery-prefix" class="font-medium">{t('settings.discoveryPrefix')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-mqtt-ha-discovery-prefix"
                  name="mqttHaDiscoveryPrefix"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.mqttHaDiscoveryPrefix}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder="homeassistant"
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.discoveryPrefixHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.snapshotIntervalSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-mqtt-ha-snapshot-interval" class="font-medium">{t('settings.snapshotIntervalSeconds')}</label>
              <div class="col-span-2">
                <input
                  type="number"
                  id="setting-mqtt-ha-snapshot-interval"
                  name="mqttHaSnapshotInterval"
                  min="0"
                  max="300"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.mqttHaSnapshotInterval}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.snapshotIntervalSecondsHelp')}</span>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default MqttTab;
