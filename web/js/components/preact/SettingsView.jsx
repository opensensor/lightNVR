/**
 * LightNVR Web Interface SettingsView Component
 * Preact component for the settings page
 */

import { useState, useEffect } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { useQuery, useMutation, fetchJSON } from '../../query-client.js';
import { ThemeCustomizer } from './ThemeCustomizer.jsx';
import { validateSession } from '../../utils/auth-utils.js';

/**
 * SettingsView component
 * @returns {JSX.Element} SettingsView component
 */
export function SettingsView() {
  const [userRole, setUserRole] = useState(null);
  const [settings, setSettings] = useState({
    logLevel: '2',
    syslogEnabled: false,
    syslogIdent: 'lightnvr',
    syslogFacility: 'LOG_USER',
    storagePath: '/var/lib/lightnvr/recordings',
    storagePathHls: '', // New field for HLS storage path
    maxStorage: '0',
    retention: '30',
    autoDelete: true,
    generateThumbnails: true,
    dbPath: '/var/lib/lightnvr/lightnvr.db',
    webPort: '8080',
    webThreadPoolSize: '',   // populated from API; blank = use server default (2x cores)
    maxStreams: '32',
    authEnabled: true,
    demoMode: false, // Demo mode: allows unauthenticated viewer access
    webrtcDisabled: false, // Whether WebRTC is disabled (use HLS only)
    authTimeoutHours: '24',
    // Security settings
    forceMfaOnLogin: false,
    loginRateLimitEnabled: true,
    loginRateLimitMaxAttempts: '5',
    loginRateLimitWindowSeconds: '300',
    bufferSize: '1024',
    useSwap: true,
    swapSize: '128',
    detectionModelsPath: '',
    apiDetectionUrl: 'http://localhost:8000/detect',
    apiDetectionBackend: 'onnx',
    defaultDetectionThreshold: 50,
    defaultPreBuffer: 5,
    defaultPostBuffer: 10,
    bufferStrategy: 'auto',
    // go2rtc settings
    go2rtcEnabled: true,
    go2rtcBinaryPath: '/usr/local/bin/go2rtc',
    go2rtcConfigDir: '/etc/lightnvr/go2rtc',
    go2rtcApiPort: '1984',
    go2rtcRtspPort: '8554',
    go2rtcWebrtcEnabled: true,
    go2rtcWebrtcListenPort: '8555',
    go2rtcStunEnabled: true,
    go2rtcStunServer: 'stun.l.google.com:19302',
    go2rtcExternalIp: '',
    go2rtcIceServers: '',
    go2rtcForceNativeHls: false,
    // MQTT settings
    mqttEnabled: false,
    mqttBrokerHost: 'localhost',
    mqttBrokerPort: '1883',
    mqttUsername: '',
    mqttPassword: '',
    mqttClientId: 'lightnvr',
    mqttTopicPrefix: 'lightnvr',
    mqttTlsEnabled: false,
    mqttKeepalive: '60',
    mqttQos: '1',
    mqttRetain: false,
    // Home Assistant MQTT auto-discovery
    mqttHaDiscovery: false,
    mqttHaDiscoveryPrefix: 'homeassistant',
    mqttHaSnapshotInterval: '30',
    // TURN server settings for WebRTC relay
    turnEnabled: false,
    turnServerUrl: '',
    turnUsername: '',
    turnPassword: '',
    // ONVIF discovery settings
    onvifDiscoveryEnabled: false,
    onvifDiscoveryInterval: '300',
    onvifDiscoveryNetwork: 'auto'
  });

  // Fetch user role on mount
  useEffect(() => {
    async function fetchUserRole() {
      const session = await validateSession();
      if (session.valid) {
        setUserRole(session.role);
        console.log('User role:', session.role);
      } else {
        // Session invalid - set to empty string to indicate fetch completed
        setUserRole('');
        console.log('Session validation failed, no role');
      }
    }
    fetchUserRole();
  }, []);

  // Session-level toggle for Appearance section (persisted to sessionStorage)
  const [showAppearance, setShowAppearance] = useState(() => {
    const stored = sessionStorage.getItem('lightnvr-show-appearance');
    return stored === null ? true : stored !== 'false';
  });

  const toggleAppearance = () => {
    setShowAppearance(prev => {
      const next = !prev;
      sessionStorage.setItem('lightnvr-show-appearance', next.toString());
      return next;
    });
  };

  // Role is still loading if null
  const roleLoading = userRole === null;
  // Check if user can modify system settings (admin only)
  // While loading, default to enabled so admin doesn't see disabled inputs
  const canModifySettings = roleLoading || userRole === 'admin';
  // Viewers can only see/change theme (only applies when role is confirmed)
  const isViewer = userRole === 'viewer';
  
  // Fetch settings using useQuery
  const {
    data: settingsData,
    isLoading,
    refetch
  } = useQuery(
    ['settings'], 
    '/api/settings',
    {
      timeout: 15000, // 15 second timeout
      retries: 2,     // Retry twice
      retryDelay: 1000 // 1 second between retries
    }
  );

  // Save settings mutation
  const saveSettingsMutation = useMutation({
    mutationKey: ['saveSettings'],
    mutationFn: async (mappedSettings) => {
      return await fetchJSON('/api/settings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(mappedSettings),
        timeout: 20000, // 20 second timeout for saving settings
        retries: 1,     // Retry once
        retryDelay: 2000 // 2 seconds between retries
      });
    },
    onSuccess: () => {
      showStatusMessage('Settings saved successfully');
      refetch(); // Refresh settings after saving
    },
    onError: (error) => {
      console.error('Error saving settings:', error);
      showStatusMessage(`Error saving settings: ${error.message}`);
    }
  });

  // Update settings state when data is loaded
  useEffect(() => {
    if (settingsData) {
      console.log('Settings loaded:', settingsData);
      
      // Map backend property names to frontend property names
      const mappedData = {
        logLevel: settingsData.log_level?.toString() || '',
        syslogEnabled: settingsData.syslog_enabled || false,
        syslogIdent: settingsData.syslog_ident || 'lightnvr',
        syslogFacility: settingsData.syslog_facility || 'LOG_USER',
        storagePath: settingsData.storage_path || '',
        storagePathHls: settingsData.storage_path_hls || '', // Map the HLS storage path
        maxStorage: settingsData.max_storage_size?.toString() || '',
        retention: settingsData.retention_days?.toString() || '',
        autoDelete: settingsData.auto_delete_oldest || false,
        generateThumbnails: settingsData.generate_thumbnails !== false,
        dbPath: settingsData.db_path || '',
        webPort: settingsData.web_port?.toString() || '',
        webThreadPoolSize: settingsData.web_thread_pool_size?.toString() || '',
        maxStreams: settingsData.max_streams?.toString() || '32',
        authEnabled: settingsData.web_auth_enabled || false,
        demoMode: settingsData.demo_mode || false,
        webrtcDisabled: settingsData.webrtc_disabled || false,
        authTimeoutHours: settingsData.auth_timeout_hours?.toString() || '24',
        // Security settings
        forceMfaOnLogin: settingsData.force_mfa_on_login || false,
        loginRateLimitEnabled: settingsData.login_rate_limit_enabled !== undefined ? settingsData.login_rate_limit_enabled : true,
        loginRateLimitMaxAttempts: settingsData.login_rate_limit_max_attempts?.toString() || '5',
        loginRateLimitWindowSeconds: settingsData.login_rate_limit_window_seconds?.toString() || '300',
        bufferSize: settingsData.buffer_size?.toString() || '',
        useSwap: settingsData.use_swap || false,
        swapSize: settingsData.swap_size?.toString() || '',
        detectionModelsPath: settingsData.models_path || '',
        apiDetectionUrl: settingsData.api_detection_url || 'http://localhost:8000/detect',
        apiDetectionBackend: settingsData.api_detection_backend || 'onnx',
        defaultDetectionThreshold: settingsData.default_detection_threshold || 50,
        defaultPreBuffer: settingsData.pre_detection_buffer ?? 5,
        defaultPostBuffer: settingsData.post_detection_buffer ?? 10,
        bufferStrategy: settingsData.buffer_strategy || 'auto',
        // go2rtc settings
        go2rtcEnabled: settingsData.go2rtc_enabled !== undefined ? settingsData.go2rtc_enabled : true,
        go2rtcBinaryPath: settingsData.go2rtc_binary_path || '/usr/local/bin/go2rtc',
        go2rtcConfigDir: settingsData.go2rtc_config_dir || '/etc/lightnvr/go2rtc',
        go2rtcApiPort: settingsData.go2rtc_api_port?.toString() || '1984',
        go2rtcRtspPort: settingsData.go2rtc_rtsp_port?.toString() || '8554',
        go2rtcWebrtcEnabled: settingsData.go2rtc_webrtc_enabled !== undefined ? settingsData.go2rtc_webrtc_enabled : true,
        go2rtcWebrtcListenPort: settingsData.go2rtc_webrtc_listen_port?.toString() || '8555',
        go2rtcStunEnabled: settingsData.go2rtc_stun_enabled !== undefined ? settingsData.go2rtc_stun_enabled : true,
        go2rtcStunServer: settingsData.go2rtc_stun_server || 'stun.l.google.com:19302',
        go2rtcExternalIp: settingsData.go2rtc_external_ip || '',
        go2rtcIceServers: settingsData.go2rtc_ice_servers || '',
        go2rtcForceNativeHls: settingsData.go2rtc_force_native_hls || false,
        // MQTT settings
        mqttEnabled: settingsData.mqtt_enabled || false,
        mqttBrokerHost: settingsData.mqtt_broker_host || 'localhost',
        mqttBrokerPort: settingsData.mqtt_broker_port?.toString() || '1883',
        mqttUsername: settingsData.mqtt_username || '',
        mqttPassword: settingsData.mqtt_password || '',
        mqttClientId: settingsData.mqtt_client_id || 'lightnvr',
        mqttTopicPrefix: settingsData.mqtt_topic_prefix || 'lightnvr',
        mqttTlsEnabled: settingsData.mqtt_tls_enabled || false,
        mqttKeepalive: settingsData.mqtt_keepalive?.toString() || '60',
        mqttQos: settingsData.mqtt_qos?.toString() || '1',
        mqttRetain: settingsData.mqtt_retain || false,
        // Home Assistant MQTT auto-discovery
        mqttHaDiscovery: settingsData.mqtt_ha_discovery || false,
        mqttHaDiscoveryPrefix: settingsData.mqtt_ha_discovery_prefix || 'homeassistant',
        mqttHaSnapshotInterval: settingsData.mqtt_ha_snapshot_interval?.toString() || '30',
        // TURN server settings for WebRTC relay
        turnEnabled: settingsData.turn_enabled || false,
        turnServerUrl: settingsData.turn_server_url || '',
        turnUsername: settingsData.turn_username || '',
        turnPassword: settingsData.turn_password || '',
        // ONVIF discovery settings
        onvifDiscoveryEnabled: settingsData.onvif_discovery_enabled || false,
        onvifDiscoveryInterval: settingsData.onvif_discovery_interval?.toString() || '300',
        onvifDiscoveryNetwork: settingsData.onvif_discovery_network || 'auto'
      };
      
      // Update state with loaded settings
      setSettings(prev => ({
        ...prev,
        ...mappedData
      }));
    }
  }, [settingsData]);
  
  // Save settings
  const saveSettings = () => {
    // Map frontend property names to backend property names
    const mappedSettings = {
      log_level: parseInt(settings.logLevel, 10),
      syslog_enabled: settings.syslogEnabled,
      syslog_ident: settings.syslogIdent,
      syslog_facility: settings.syslogFacility,
      storage_path: settings.storagePath,
      storage_path_hls: settings.storagePathHls, // Include the HLS storage path
      max_storage_size: parseInt(settings.maxStorage, 10),
      retention_days: parseInt(settings.retention, 10),
      auto_delete_oldest: settings.autoDelete,
      generate_thumbnails: settings.generateThumbnails,
      db_path: settings.dbPath,
      web_port: parseInt(settings.webPort, 10),
      web_thread_pool_size: parseInt(settings.webThreadPoolSize, 10) || undefined,
      max_streams: parseInt(settings.maxStreams, 10) || 32,
      web_auth_enabled: settings.authEnabled,
      demo_mode: settings.demoMode,
      webrtc_disabled: settings.webrtcDisabled,
      auth_timeout_hours: parseInt(settings.authTimeoutHours, 10),
      // Security settings
      force_mfa_on_login: settings.forceMfaOnLogin,
      login_rate_limit_enabled: settings.loginRateLimitEnabled,
      login_rate_limit_max_attempts: parseInt(settings.loginRateLimitMaxAttempts, 10),
      login_rate_limit_window_seconds: parseInt(settings.loginRateLimitWindowSeconds, 10),
      buffer_size: parseInt(settings.bufferSize, 10),
      use_swap: settings.useSwap,
      swap_size: parseInt(settings.swapSize, 10),
      models_path: settings.detectionModelsPath,
      api_detection_url: settings.apiDetectionUrl,
      api_detection_backend: settings.apiDetectionBackend,
      default_detection_threshold: settings.defaultDetectionThreshold,
      pre_detection_buffer: parseInt(settings.defaultPreBuffer, 10),
      post_detection_buffer: parseInt(settings.defaultPostBuffer, 10),
      buffer_strategy: settings.bufferStrategy,
      // go2rtc settings
      go2rtc_enabled: settings.go2rtcEnabled,
      go2rtc_binary_path: settings.go2rtcBinaryPath,
      go2rtc_config_dir: settings.go2rtcConfigDir,
      go2rtc_api_port: parseInt(settings.go2rtcApiPort, 10),
      go2rtc_rtsp_port: parseInt(settings.go2rtcRtspPort, 10),
      go2rtc_webrtc_enabled: settings.go2rtcWebrtcEnabled,
      go2rtc_webrtc_listen_port: parseInt(settings.go2rtcWebrtcListenPort, 10),
      go2rtc_stun_enabled: settings.go2rtcStunEnabled,
      go2rtc_stun_server: settings.go2rtcStunServer,
      go2rtc_external_ip: settings.go2rtcExternalIp,
      go2rtc_ice_servers: settings.go2rtcIceServers,
      go2rtc_force_native_hls: settings.go2rtcForceNativeHls,
      // MQTT settings
      mqtt_enabled: settings.mqttEnabled,
      mqtt_broker_host: settings.mqttBrokerHost,
      mqtt_broker_port: parseInt(settings.mqttBrokerPort, 10),
      mqtt_username: settings.mqttUsername,
      mqtt_password: settings.mqttPassword,
      mqtt_client_id: settings.mqttClientId,
      mqtt_topic_prefix: settings.mqttTopicPrefix,
      mqtt_tls_enabled: settings.mqttTlsEnabled,
      mqtt_keepalive: parseInt(settings.mqttKeepalive, 10),
      mqtt_qos: parseInt(settings.mqttQos, 10),
      mqtt_retain: settings.mqttRetain,
      // Home Assistant MQTT auto-discovery
      mqtt_ha_discovery: settings.mqttHaDiscovery,
      mqtt_ha_discovery_prefix: settings.mqttHaDiscoveryPrefix,
      mqtt_ha_snapshot_interval: parseInt(settings.mqttHaSnapshotInterval, 10),
      // TURN server settings for WebRTC relay
      turn_enabled: settings.turnEnabled,
      turn_server_url: settings.turnServerUrl,
      turn_username: settings.turnUsername,
      turn_password: settings.turnPassword,
      // ONVIF discovery settings
      onvif_discovery_enabled: settings.onvifDiscoveryEnabled,
      onvif_discovery_interval: parseInt(settings.onvifDiscoveryInterval, 10),
      onvif_discovery_network: settings.onvifDiscoveryNetwork
    };
    
    // Use mutation to save settings
    saveSettingsMutation.mutate(mappedSettings);
  };
  
  // Handle input change
  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    
    setSettings(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };
  
  // Handle threshold slider change
  const handleThresholdChange = (e) => {
    const value = parseInt(e.target.value, 10);
    setSettings(prev => ({
      ...prev,
      defaultDetectionThreshold: value
    }));
  };

  // For viewers, show only appearance settings
  if (isViewer) {
    return (
      <section id="settings-page" class="page">
        <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
          <h2 class="text-xl font-bold">Settings</h2>
        </div>

        <div class="settings-container space-y-6">
          {/* Appearance Settings - available to all users */}
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <button
              onClick={toggleAppearance}
              class="w-full flex items-center justify-between pb-2 border-b border-border mb-4 group"
              aria-expanded={showAppearance}
              aria-controls="appearance-settings-content"
            >
              <h3 class="text-lg font-semibold">Appearance</h3>
              <span class={`text-muted-foreground transition-transform duration-200 ${showAppearance ? 'rotate-0' : '-rotate-90'}`}>
                ▾
              </span>
            </button>
            {showAppearance && (
              <div id="appearance-settings-content">
                <ThemeCustomizer />
              </div>
            )}
          </div>

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <p class="text-muted-foreground">
              System settings are only available to administrators.
            </p>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section id="settings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 class="text-xl font-bold">Settings</h2>
        <div class="controls flex items-center gap-4">
          {!canModifySettings && userRole && (
            <span class="text-sm text-muted-foreground italic">
              Read-only (admin privileges required to modify)
            </span>
          )}
        </div>
      </div>

      <ContentLoader
        isLoading={isLoading}
        hasData={!!settingsData}
        loadingMessage="Loading settings..."
        emptyMessage="No settings available. Please try again later."
      >
        <div class="settings-container space-y-6">
          {/* Appearance Settings - available to all users */}
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <button
              onClick={toggleAppearance}
              class="w-full flex items-center justify-between pb-2 border-b border-border mb-4 group"
              aria-expanded={showAppearance}
              aria-controls="appearance-settings-content"
            >
              <h3 class="text-lg font-semibold">Appearance</h3>
              <span class={`text-muted-foreground transition-transform duration-200 ${showAppearance ? 'rotate-0' : '-rotate-90'}`}>
                ▾
              </span>
            </button>
            {showAppearance && (
              <div id="appearance-settings-content">
                <ThemeCustomizer />
              </div>
            )}
          </div>

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">General Settings</h3>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-log-level" class="font-medium">Log Level</label>
              <select
                id="setting-log-level"
                name="logLevel"
                class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.logLevel}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="0">Error</option>
                <option value="1">Warning</option>
                <option value="2">Info</option>
                <option value="3">Debug</option>
              </select>
            </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-syslog-enabled" class="font-medium">Enable Syslog</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-syslog-enabled"
                name="syslogEnabled"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.syslogEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint ml-2 text-sm text-muted-foreground">Send log messages to syslog</span>
            </div>
          </div>
          {settings.syslogEnabled && (
            <>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-syslog-ident" class="font-medium">Syslog Ident</label>
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
                <span class="hint text-sm text-muted-foreground block mt-1">Identifier prepended to syslog messages</span>
              </div>
            </div>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-syslog-facility" class="font-medium">Syslog Facility</label>
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
                <span class="hint text-sm text-muted-foreground block mt-1">Syslog facility for message routing</span>
              </div>
            </div>
            </>
          )}
          </div>
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Storage Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path" class="font-medium">Storage Path</label>
            <input
              type="text"
              id="setting-storage-path"
              name="storagePath"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.storagePath}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path-hls" class="font-medium">HLS Storage Path</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-storage-path-hls"
                name="storagePathHls"
                class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.storagePathHls}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground">Optional path for HLS segments. If not specified, Storage Path will be used.</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-max-storage" class="font-medium">Maximum Storage Size (GB)</label>
            <div class="col-span-2 flex items-center">
              <input
                type="number"
                id="setting-max-storage"
                name="maxStorage"
                min="0"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.maxStorage}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint ml-2 text-sm text-muted-foreground">0 = unlimited</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-retention" class="font-medium">Retention Period (days)</label>
            <input
              type="number"
              id="setting-retention"
              name="retention"
              min="1"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.retention}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auto-delete" class="font-medium">Auto Delete Oldest</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-auto-delete"
                name="autoDelete"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.autoDelete}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-generate-thumbnails" class="font-medium">Enable Grid View (Thumbnails)</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-generate-thumbnails"
                name="generateThumbnails"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.generateThumbnails}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Allow grid view with thumbnail previews on recordings page</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-path" class="font-medium">Database Path</label>
            <input
              type="text"
              id="setting-db-path"
              name="dbPath"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbPath}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
          </div>
          
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Web Interface Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-web-port" class="font-medium">Web Port</label>
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-thread-pool" class="font-medium">
              Thread Pool Size
              <span class="ml-1 text-xs text-muted-foreground">(requires restart)</span>
            </label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-thread-pool"
                name="webThreadPoolSize"
                min="2"
                max="128"
                placeholder="Default: 2× CPU cores"
                class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.webThreadPoolSize}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <p class="text-xs text-muted-foreground mt-1">
                Number of libuv worker threads. Default is 2× the number of CPU cores (clamped 2–128).
              </p>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-max-streams" class="font-medium">
              Max Streams
              <span class="ml-1 text-xs text-muted-foreground">(requires restart)</span>
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
                Maximum concurrent stream slots (default 32, ceiling 256). Restart required.
              </p>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-enabled" class="font-medium">Enable Authentication</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-auth-enabled"
                name="authEnabled"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.authEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-demo-mode" class="font-medium">Demo Mode</label>
            <div class="col-span-2 flex items-center">
              <input
                type="checkbox"
                id="setting-demo-mode"
                name="demoMode"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.demoMode}
                onChange={handleInputChange}
                disabled={!canModifySettings || !settings.authEnabled}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Allow unauthenticated users to view streams (viewer access only)</span>
            </div>
          </div>

          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-webrtc-disabled" class="font-medium">Disable WebRTC (Use HLS Only)</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-webrtc-disabled"
                name="webrtcDisabled"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.webrtcDisabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint ml-2 text-sm text-muted-foreground">When enabled, all streams will use HLS instead of WebRTC</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-timeout" class="font-medium">Auth Session Timeout (hours)</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">How long authentication sessions remain valid (minimum 1 hour)</span>
            </div>
          </div>
          </div>

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Login Security</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-force-mfa" class="font-medium">Force MFA on Login</label>
            <div class="col-span-2 flex items-center">
              <input
                type="checkbox"
                id="setting-force-mfa"
                name="forceMfaOnLogin"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.forceMfaOnLogin}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Require TOTP code alongside password at login (prevents password-only brute force). Users must have MFA configured.</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-rate-limit-enabled" class="font-medium">Login Rate Limiting</label>
            <div class="col-span-2 flex items-center">
              <input
                type="checkbox"
                id="setting-rate-limit-enabled"
                name="loginRateLimitEnabled"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.loginRateLimitEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Limit login attempts to prevent brute force attacks</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-rate-limit-max" class="font-medium">Max Login Attempts</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">Maximum failed login attempts before lockout (minimum 1)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-rate-limit-window" class="font-medium">Rate Limit Window (seconds)</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">Time window in seconds for rate limiting (minimum 10 seconds, default 300 = 5 minutes)</span>
            </div>
          </div>
          </div>

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Memory Optimization</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-buffer-size" class="font-medium">Buffer Size (KB)</label>
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-use-swap" class="font-medium">Use Swap File</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-use-swap"
                name="useSwap"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.useSwap}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-swap-size" class="font-medium">Swap Size (MB)</label>
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

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Detection-Based Recording</h3>
          <div class="setting mb-4">
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              Configure detection-based recording for streams. When enabled, recordings will only be saved when objects are detected.
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>Motion Detection:</strong> Built-in motion detection is available without requiring any external models.
              Select "motion" as the detection model in stream settings to use this feature. You can configure the detection model for each stream in the Stream Settings page.
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>Optimized Motion Detection:</strong> A memory- and CPU-optimized variant of motion detection designed for embedded and low-power devices.
              When "motion" is selected as the detection model, this optimized implementation is used automatically on supported devices; no additional configuration is required.
            </p>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-detection-models-path" class="font-medium">Detection Models Path</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-detection-models-path"
                name="detectionModelsPath"
                placeholder="/var/lib/lightnvr/models"
                class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.detectionModelsPath}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground">Directory where detection models are stored</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-api-detection-url" class="font-medium">API Detection URL</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-api-detection-url"
                name="apiDetectionUrl"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.apiDetectionUrl}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="http://localhost:8000/detect"
              />
              <span class="hint text-sm text-muted-foreground block mt-1">URL of the external object detection API endpoint</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-api-detection-backend" class="font-medium">API Detection Backend</label>
            <div class="col-span-2">
              <select
                id="setting-api-detection-backend"
                name="apiDetectionBackend"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.apiDetectionBackend}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="onnx">ONNX Runtime</option>
                <option value="tflite">TensorFlow Lite</option>
                <option value="opencv">OpenCV DNN</option>
              </select>
              <span class="hint text-sm text-muted-foreground block mt-1">Inference backend used by the detection API</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-detection-threshold" class="font-medium">Default Detection Threshold</label>
            <div class="col-span-2">
              <div class="flex items-center">
                <input
                  type="range"
                  id="setting-default-detection-threshold"
                  name="defaultDetectionThreshold"
                  min="0"
                  max="100"
                  step="1"
                  class="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.defaultDetectionThreshold}
                  onChange={handleThresholdChange}
                  disabled={!canModifySettings}
                />
                <span id="threshold-value" class="ml-2 min-w-[3rem] text-center">{settings.defaultDetectionThreshold}%</span>
              </div>
              <span class="hint text-sm text-muted-foreground">Confidence threshold for detection (0-100%)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-pre-buffer" class="font-medium">Default Pre-detection Buffer (seconds)</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-default-pre-buffer"
                name="defaultPreBuffer"
                min="0"
                max="60"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.defaultPreBuffer}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground">Seconds of video to keep before detection</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-post-buffer" class="font-medium">Default Post-detection Buffer (seconds)</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-default-post-buffer"
                name="defaultPostBuffer"
                min="0"
                max="300"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.defaultPostBuffer}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground">Seconds of video to keep after detection</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-buffer-strategy" class="font-medium">Default Buffer Strategy</label>
            <div class="col-span-2">
              <select
                id="setting-buffer-strategy"
                name="bufferStrategy"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.bufferStrategy}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="auto">Auto (recommended)</option>
                <option value="go2rtc">go2rtc Native</option>
                <option value="hls_segment">HLS Segment Tracking</option>
                <option value="memory_packet">Memory Packet Buffer</option>
                <option value="mmap_hybrid">Memory-Mapped Hybrid</option>
              </select>
              <p class="hint text-sm text-muted-foreground mt-1">
                How pre-detection video is buffered. "Auto" selects the best strategy based on your setup.
              </p>
            </div>
          </div>
          </div>

          {/* go2rtc Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">go2rtc Integration</h3>
            <p class="text-sm text-muted-foreground mb-4">
              go2rtc provides WebRTC and HLS streaming. Changes require a restart to take effect.
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-enabled" class="font-medium">Enable go2rtc</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-go2rtc-enabled"
                name="go2rtcEnabled"
                class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                checked={settings.go2rtcEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">When disabled, go2rtc will not be started and HLS will connect directly to camera streams. This saves bandwidth for WiFi cameras but disables WebRTC live view.</span>
            </div>
          </div>
          {settings.go2rtcEnabled && (
          <>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-force-native-hls" class="font-medium">Force Native HLS</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-go2rtc-force-native-hls"
                name="go2rtcForceNativeHls"
                class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                checked={settings.go2rtcForceNativeHls}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Use lightNVR's native FFmpeg-based HLS instead of go2rtc HLS for live view. Useful if go2rtc HLS has compatibility issues. WebRTC is not affected.</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-binary-path" class="font-medium">Binary Path</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-binary-path"
                name="go2rtcBinaryPath"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcBinaryPath}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="/usr/local/bin/go2rtc"
              />
              <span class="hint text-sm text-muted-foreground">Path to the go2rtc binary</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-config-dir" class="font-medium">Config Directory</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-config-dir"
                name="go2rtcConfigDir"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcConfigDir}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="/etc/lightnvr/go2rtc"
              />
              <span class="hint text-sm text-muted-foreground">Directory for go2rtc configuration files</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-api-port" class="font-medium">API Port</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-go2rtc-api-port"
                name="go2rtcApiPort"
                min="1"
                max="65535"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcApiPort}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">go2rtc API port (default: 1984)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-rtsp-port" class="font-medium">RTSP Port</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-go2rtc-rtsp-port"
                name="go2rtcRtspPort"
                min="1"
                max="65535"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcRtspPort}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">RTSP listen port (default: 8554)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-webrtc-enabled" class="font-medium">Enable WebRTC</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-go2rtc-webrtc-enabled"
                name="go2rtcWebrtcEnabled"
                class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                checked={settings.go2rtcWebrtcEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Enable WebRTC streaming via go2rtc</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-webrtc-listen-port" class="font-medium">WebRTC Listen Port</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-go2rtc-webrtc-listen-port"
                name="go2rtcWebrtcListenPort"
                min="1"
                max="65535"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcWebrtcListenPort}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">WebRTC listen port (default: 8555)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-stun-enabled" class="font-medium">Enable STUN</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-go2rtc-stun-enabled"
                name="go2rtcStunEnabled"
                class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                checked={settings.go2rtcStunEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground ml-2">Enable STUN for NAT traversal (needed for remote WebRTC)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-stun-server" class="font-medium">STUN Server</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-stun-server"
                name="go2rtcStunServer"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcStunServer}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="stun.l.google.com:19302"
              />
              <span class="hint text-sm text-muted-foreground">STUN server address for NAT traversal</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-external-ip" class="font-medium">External IP</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-external-ip"
                name="go2rtcExternalIp"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcExternalIp}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="Auto-detect"
              />
              <span class="hint text-sm text-muted-foreground">Optional: External IP for NAT (leave empty for auto-detect)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-ice-servers" class="font-medium">ICE Servers</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-ice-servers"
                name="go2rtcIceServers"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcIceServers}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="Optional comma-separated ICE servers"
              />
              <span class="hint text-sm text-muted-foreground">Optional: Custom ICE servers (comma-separated)</span>
            </div>
          </div>
          </>
          )}
          </div>

          {/* MQTT Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">MQTT Event Streaming</h3>
            <p class="text-sm text-muted-foreground mb-4">
              Publish detection events to an MQTT broker for integration with Home Assistant or other systems.
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-enabled" class="font-medium">Enable MQTT</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Stream detection events to MQTT broker</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-broker-host" class="font-medium">Broker Host</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">MQTT broker hostname or IP address</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-broker-port" class="font-medium">Broker Port</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Default: 1883 (8883 for TLS)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-username" class="font-medium">Username</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-mqtt-username"
                name="mqttUsername"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.mqttUsername}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="(optional)"
              />
              <span class="hint text-sm text-muted-foreground block mt-1">Leave empty for anonymous access</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-password" class="font-medium">Password</label>
            <div class="col-span-2">
              <input
                type="password"
                id="setting-mqtt-password"
                name="mqttPassword"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.mqttPassword}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="(optional)"
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-client-id" class="font-medium">Client ID</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">Unique identifier for this client</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-topic-prefix" class="font-medium">Topic Prefix</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">Events published to: {settings.mqttTopicPrefix}/detections/&lt;stream_name&gt;</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-tls-enabled" class="font-medium">Enable TLS</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Use encrypted connection (typically port 8883)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-keepalive" class="font-medium">Keepalive (seconds)</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Interval to check connection health</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-qos" class="font-medium">QoS Level</label>
            <div class="col-span-2">
              <select
                id="setting-mqtt-qos"
                name="mqttQos"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.mqttQos}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="0">0 - At most once (fire and forget)</option>
                <option value="1">1 - At least once (recommended)</option>
                <option value="2">2 - Exactly once (highest overhead)</option>
              </select>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-retain" class="font-medium">Retain Messages</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Broker stores last message for new subscribers</span>
            </div>
          </div>

          {/* Home Assistant Auto-Discovery sub-section */}
          <h4 class="text-md font-semibold mt-6 mb-3 pb-1 border-b border-border">Home Assistant Auto-Discovery</h4>
          <p class="text-sm text-muted-foreground mb-4">
            Automatically register cameras and sensors in Home Assistant via MQTT discovery. Requires the same MQTT broker used by Home Assistant.
          </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-ha-discovery" class="font-medium">Enable HA Discovery</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Cameras and motion sensors appear automatically in Home Assistant</span>
            </div>
          </div>
          {settings.mqttHaDiscovery && (
          <>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-ha-discovery-prefix" class="font-medium">Discovery Prefix</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">Must match Home Assistant's MQTT discovery prefix (default: homeassistant)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-ha-snapshot-interval" class="font-medium">Snapshot Interval (seconds)</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">How often to publish camera snapshots (0 = disabled, max 300)</span>
            </div>
          </div>
          </>
          )}
          </div>

          {/* TURN Server Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">WebRTC TURN Server</h3>
            <p class="text-sm text-muted-foreground mb-4">
              Configure a TURN relay server for WebRTC when direct peer-to-peer connections fail (e.g., behind restrictive NAT/firewall).
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-turn-enabled" class="font-medium">Enable TURN Relay</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">Use TURN server for WebRTC relay when direct connection fails</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-turn-server-url" class="font-medium">TURN Server URL</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-turn-server-url"
                name="turnServerUrl"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.turnServerUrl}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="turn:turn.example.com:3478"
              />
              <span class="hint text-sm text-muted-foreground block mt-1">
                Format: turn:hostname:port or turn:hostname:port?transport=tcp
              </span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-turn-username" class="font-medium">Username</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-turn-username"
                name="turnUsername"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.turnUsername}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="(optional)"
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-turn-password" class="font-medium">Password</label>
            <div class="col-span-2">
              <input
                type="password"
                id="setting-turn-password"
                name="turnPassword"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.turnPassword}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="(optional)"
              />
            </div>
          </div>
          </div>

          {/* ONVIF Discovery Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">ONVIF Discovery</h3>
            <p class="text-sm text-muted-foreground mb-4">
              Configure ONVIF camera discovery settings. Use CIDR notation (e.g., 192.168.1.0/24) or "auto" for automatic network detection.
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-onvif-discovery-enabled" class="font-medium">Enable ONVIF Discovery</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-onvif-discovery-enabled"
                name="onvifDiscoveryEnabled"
                class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.onvifDiscoveryEnabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint ml-2 text-sm text-muted-foreground">Automatically discover ONVIF cameras on the network</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-onvif-discovery-interval" class="font-medium">Discovery Interval (seconds)</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">How often to scan for cameras (30–3600 seconds)</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-onvif-discovery-network" class="font-medium">Discovery Network</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-onvif-discovery-network"
                name="onvifDiscoveryNetwork"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.onvifDiscoveryNetwork}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="auto"
              />
              <span class="hint text-sm text-muted-foreground block mt-1">
                Examples: 192.168.1.0/24, 10.0.0.0/16, auto
              </span>
            </div>
          </div>
          </div>

          {/* Save Settings Button - at bottom of form */}
          {canModifySettings && (
            <div class="flex justify-end mt-6">
              <button
                id="save-settings-btn"
                class="px-6 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                onClick={saveSettings}
              >
                Save Settings
              </button>
            </div>
          )}
        </div>
      </ContentLoader>
    </section>
  );
}
