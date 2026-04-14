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
import { formatLocalDateTime } from '../../utils/date-utils.js';
import { useI18n } from '../../i18n.js';

/**
 * SettingsView component
 * @returns {JSX.Element} SettingsView component
 */
export function SettingsView() {
  const { t } = useI18n();
  const [userRole, setUserRole] = useState(null);
  const [restartNotice, setRestartNotice] = useState(null);
  const [settings, setSettings] = useState({
    logLevel: '2',
    syslogEnabled: false,
    syslogIdent: 'lightnvr',
    syslogFacility: 'LOG_USER',
    storagePath: '/var/lib/lightnvr/recordings',
    storagePathHls: '', // Optional HLS storage path; when empty, HLS segments use storagePath
    maxStorage: '0',
    retention: '30',
    autoDelete: true,
    generateThumbnails: true,
    dbPath: '/var/lib/lightnvr/lightnvr.db',
    dbBackupIntervalMinutes: '60',
    dbBackupRetentionCount: '24',
    dbPostBackupScript: '',
    webPort: '8080',
    webBindIp: '0.0.0.0',
    webThreadPoolSize: '',   // populated from API; blank = use server default (2x cores)
    maxStreams: '32',
    authEnabled: true,
    demoMode: false, // Demo mode: allows unauthenticated viewer access
    webrtcDisabled: false, // Whether WebRTC is disabled (use HLS only)
    authTimeoutHours: '24',
    authAbsoluteTimeoutHours: '168',
    trustedDeviceDays: '30',
    trustedProxyCidrs: '',
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
    go2rtcConfigOverride: '',
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
    onSuccess: (response) => {
      if (response?.restart_required) {
        setRestartNotice(t('settings.restartNotice'));
        showStatusMessage(t('settings.savedRestart'), 'warning', 7000);
      } else {
        showStatusMessage(t('settings.saved'));
      }
      refetch(); // Refresh settings after saving
    },
    onError: (error) => {
      console.error('Error saving settings:', error);
      showStatusMessage(t('settings.saveError', { message: error.message }));
    }
  });

  const {
    data: authSessionsData,
    refetch: refetchAuthSessions,
    isLoading: authSessionsLoading,
    isError: authSessionsIsError,
    error: authSessionsError
  } = useQuery(['auth-sessions'], '/api/auth/sessions', {
    timeout: 10000,
    retries: 1,
    retryDelay: 1000
  });

  const {
    data: trustedDevicesData,
    refetch: refetchTrustedDevices,
    isLoading: trustedDevicesLoading,
    isError: trustedDevicesIsError,
    error: trustedDevicesError
  } = useQuery(['trusted-devices'], '/api/auth/trusted-devices', {
    timeout: 10000,
    retries: 1,
    retryDelay: 1000
  });

  const revokeSessionMutation = useMutation({
    mutationKey: ['revokeSession'],
    mutationFn: async (sessionId) => fetchJSON(`/api/auth/sessions/${sessionId}`, { method: 'DELETE' }),
    onSuccess: () => {
      showStatusMessage(t('settings.sessionRevoked'));
      refetchAuthSessions();
    },
    onError: (error) => {
      showStatusMessage(t('settings.revokeSessionError', { message: error.message }));
    }
  });

  const revokeTrustedDeviceMutation = useMutation({
    mutationKey: ['revokeTrustedDevice'],
    mutationFn: async (deviceId) => fetchJSON(`/api/auth/trusted-devices/${deviceId}`, { method: 'DELETE' }),
    onSuccess: () => {
      showStatusMessage(t('settings.trustedDeviceRevoked'));
      refetchTrustedDevices();
    },
    onError: (error) => {
      showStatusMessage(t('settings.revokeTrustedDeviceError', { message: error.message }));
    }
  });

  const formatTimestamp = (timestamp) => {
    if (!timestamp) return t('settings.notAvailable');
    return formatLocalDateTime(timestamp);
  };

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
        dbBackupIntervalMinutes: settingsData.db_backup_interval_minutes?.toString() || '60',
        dbBackupRetentionCount: settingsData.db_backup_retention_count?.toString() || '24',
        dbPostBackupScript: settingsData.db_post_backup_script || '',
        webPort: settingsData.web_port?.toString() || '',
        webBindIp: settingsData.web_bind_ip?.toString() || '0.0.0.0',
        webThreadPoolSize: settingsData.web_thread_pool_size?.toString() || '',
        maxStreams: settingsData.max_streams?.toString() || '32',
        authEnabled: settingsData.web_auth_enabled || false,
        demoMode: settingsData.demo_mode || false,
        webrtcDisabled: settingsData.webrtc_disabled || false,
        authTimeoutHours: settingsData.auth_timeout_hours?.toString() || '24',
        authAbsoluteTimeoutHours: settingsData.auth_absolute_timeout_hours?.toString() || '168',
        trustedDeviceDays: settingsData.trusted_device_days?.toString() || '30',
        trustedProxyCidrs: settingsData.trusted_proxy_cidrs || '',
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
        go2rtcConfigOverride: settingsData.go2rtc_config_override || '',
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
    const webThreadPoolSize = parseInt(settings.webThreadPoolSize, 10);
    const parsedMaxStreams = parseInt(settings.maxStreams, 10);
    const parsedDbBackupIntervalMinutes = parseInt(settings.dbBackupIntervalMinutes, 10);
    const parsedDbBackupRetentionCount = parseInt(settings.dbBackupRetentionCount, 10);
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
      db_backup_interval_minutes: Number.isNaN(parsedDbBackupIntervalMinutes) ? 0 : parsedDbBackupIntervalMinutes,
      db_backup_retention_count: Number.isNaN(parsedDbBackupRetentionCount) ? 0 : parsedDbBackupRetentionCount,
      db_post_backup_script: settings.dbPostBackupScript,
      web_port: parseInt(settings.webPort, 10),
      web_bind_ip: settings.webBindIp,
      web_thread_pool_size: Number.isNaN(webThreadPoolSize) ? undefined : webThreadPoolSize,
      max_streams: Number.isNaN(parsedMaxStreams) ? 32 : parsedMaxStreams,
      web_auth_enabled: settings.authEnabled,
      demo_mode: settings.demoMode,
      webrtc_disabled: settings.webrtcDisabled,
      auth_timeout_hours: parseInt(settings.authTimeoutHours, 10),
      auth_absolute_timeout_hours: parseInt(settings.authAbsoluteTimeoutHours, 10),
      trusted_device_days: parseInt(settings.trustedDeviceDays, 10),
      trusted_proxy_cidrs: settings.trustedProxyCidrs,
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
      go2rtc_config_override: settings.go2rtcConfigOverride,
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
          <h2 class="text-xl font-bold">{t('settings.title')}</h2>
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
              <h3 class="text-lg font-semibold">{t('settings.appearance')}</h3>
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
              {t('settings.adminOnly')}
            </p>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section id="settings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 class="text-xl font-bold">{t('settings.title')}</h2>
        <div class="controls flex items-center gap-4">
          {!canModifySettings && userRole && (
            <span class="text-sm text-muted-foreground italic">
              {t('settings.readOnly')}
            </span>
          )}
        </div>
      </div>

      <ContentLoader
        isLoading={isLoading}
        hasData={!!settingsData}
        loadingMessage={t('settings.loading')}
        emptyMessage={t('settings.empty')}
      >
        <div class="settings-container space-y-6">
          {restartNotice && (
            <div class="rounded-lg border border-yellow-400 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-900 dark:text-yellow-100 shadow p-4">
              <div class="flex items-start gap-3">
                <div class="text-lg leading-none">⚠️</div>
                <div>
                  <div class="font-semibold">{t('settings.restartRequired')}</div>
                  <p class="text-sm mt-1">
                    {restartNotice}
                  </p>
                  <p class="text-sm mt-2 opacity-90">
                    {t('settings.restartExplanation')}
                  </p>
                </div>
              </div>
            </div>
          )}

          {/* Appearance Settings - available to all users */}
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <button
              onClick={toggleAppearance}
              class="w-full flex items-center justify-between pb-2 border-b border-border mb-4 group"
              aria-expanded={showAppearance}
              aria-controls="appearance-settings-content"
            >
              <h3 class="text-lg font-semibold">{t('settings.appearance')}</h3>
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
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.general')}</h3>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-syslog-enabled" class="font-medium">{t('settings.enableSyslog')}</label>
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
              <span class="hint ml-2 text-sm text-muted-foreground">{t('settings.enableSyslogHelp')}</span>
            </div>
          </div>
          {settings.syslogEnabled && (
            <>
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
            <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.storage')}</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path" class="font-medium">{t('settings.storagePath')}</label>
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
            <label for="setting-storage-path-hls" class="font-medium">{t('settings.hlsStoragePath')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.hlsStoragePathHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-max-storage" class="font-medium">{t('settings.maxStorageGb')}</label>
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
              <span class="hint ml-2 text-sm text-muted-foreground">{t('settings.zeroUnlimited')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-retention" class="font-medium">{t('settings.retentionDays')}</label>
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
            <label for="setting-auto-delete" class="font-medium">{t('settings.autoDeleteOldest')}</label>
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
            <label for="setting-generate-thumbnails" class="font-medium">{t('settings.enableGridViewThumbnails')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableGridViewThumbnailsHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-path" class="font-medium">{t('settings.databasePath')}</label>
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-backup-interval" class="font-medium">{t('settings.databaseBackupIntervalMinutes')}</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-db-backup-interval"
                name="dbBackupIntervalMinutes"
                min="0"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.dbBackupIntervalMinutes}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupIntervalHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-backup-retention" class="font-medium">{t('settings.databaseBackupRetentionCopies')}</label>
            <div class="col-span-2">
              <input
                type="number"
                id="setting-db-backup-retention"
                name="dbBackupRetentionCount"
                min="0"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.dbBackupRetentionCount}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupRetentionHelpBefore')} <code>.bak</code> {t('settings.databaseBackupRetentionHelpAfter')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
            <label for="setting-db-post-backup-script" class="font-medium">{t('settings.postBackupScript')}</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-db-post-backup-script"
                name="dbPostBackupScript"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-2xl disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.dbPostBackupScript}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder="/usr/local/bin/lightnvr-post-backup"
              />
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.postBackupScriptHelp')}</span>
            </div>
          </div>
          </div>
          
          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.webInterface')}</h3>
          {restartNotice && (
            <div class="mb-4 rounded border border-yellow-300 bg-yellow-50 dark:bg-yellow-900/20 px-4 py-3 text-sm text-yellow-900 dark:text-yellow-100">
              <strong>{t('settings.pendingRestartLabel')}</strong> {restartNotice}
            </div>
          )}
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-enabled" class="font-medium">{t('settings.enableAuthentication')}</label>
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
            <label for="setting-demo-mode" class="font-medium">{t('auth.demoMode')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.demoModeHelp')}</span>
            </div>
          </div>

          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-webrtc-disabled" class="font-medium">{t('settings.disableWebrtcUseHlsOnly')}</label>
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
              <span class="hint ml-2 text-sm text-muted-foreground">{t('settings.disableWebrtcHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
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
          {/* Setup Wizard reset */}
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
                    <button
                      type="button"
                      class="px-3 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm self-start"
                      disabled={revokeSessionMutation.isPending}
                      onClick={() => revokeSessionMutation.mutate(session.id)}
                    >
                      {t('settings.revoke')}
                    </button>
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
                    <button
                      type="button"
                      class="px-3 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-sm self-start"
                      disabled={revokeTrustedDeviceMutation.isPending}
                      onClick={() => revokeTrustedDeviceMutation.mutate(device.id)}
                    >
                      {t('settings.revoke')}
                    </button>
                  </div>
                ))
              ))}
            </div>
          </div>
          </div>

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.loginSecurity')}</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-force-mfa" class="font-medium">{t('settings.forceMfaOnLogin')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.forceMfaOnLoginHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-rate-limit-enabled" class="font-medium">{t('settings.loginRateLimiting')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.loginRateLimitingHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.memoryOptimization')}</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-use-swap" class="font-medium">{t('settings.useSwapFile')}</label>
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

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.detectionBasedRecording')}</h3>
          <div class="setting mb-4">
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              {t('settings.detectionBasedRecordingDescription')}
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>{t('settings.motionDetectionLabel')}</strong> {t('settings.motionDetectionDescription')}
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>{t('settings.optimizedMotionDetectionLabel')}</strong> {t('settings.optimizedMotionDetectionDescription')}
            </p>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-detection-models-path" class="font-medium">{t('settings.detectionModelsPath')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.detectionModelsPathHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-api-detection-url" class="font-medium">{t('settings.apiDetectionUrl')}</label>
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
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.apiDetectionUrlHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-api-detection-backend" class="font-medium">{t('settings.apiDetectionBackend')}</label>
            <div class="col-span-2">
              <select
                id="setting-api-detection-backend"
                name="apiDetectionBackend"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.apiDetectionBackend}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="onnx">{t('settings.apiDetectionBackendOnnx')}</option>
                <option value="tflite">{t('settings.apiDetectionBackendTflite')}</option>
                <option value="opencv">{t('settings.apiDetectionBackendOpencv')}</option>
              </select>
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.apiDetectionBackendHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-detection-threshold" class="font-medium">{t('settings.defaultDetectionThreshold')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.defaultDetectionThresholdHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-pre-buffer" class="font-medium">{t('settings.defaultPreDetectionBufferSeconds')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.defaultPreDetectionBufferHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-default-post-buffer" class="font-medium">{t('settings.defaultPostDetectionBufferSeconds')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.defaultPostDetectionBufferHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-buffer-strategy" class="font-medium">{t('settings.defaultBufferStrategy')}</label>
            <div class="col-span-2">
              <select
                id="setting-buffer-strategy"
                name="bufferStrategy"
                class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.bufferStrategy}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              >
                <option value="auto">{t('settings.bufferStrategyAuto')}</option>
                <option value="go2rtc">{t('settings.bufferStrategyGo2rtc')}</option>
                <option value="hls_segment">{t('settings.bufferStrategyHlsSegment')}</option>
                <option value="memory_packet">{t('settings.bufferStrategyMemoryPacket')}</option>
                <option value="mmap_hybrid">{t('settings.bufferStrategyMmapHybrid')}</option>
              </select>
              <p class="hint text-sm text-muted-foreground mt-1">
                {t('settings.defaultBufferStrategyHelp')}
              </p>
            </div>
          </div>
          </div>

          {/* go2rtc Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.go2rtcIntegration')}</h3>
            <p class="text-sm text-muted-foreground mb-4">
              {t('settings.go2rtcIntegrationDescription')}
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-enabled" class="font-medium">{t('settings.enableGo2rtc')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableGo2rtcHelp')}</span>
            </div>
          </div>
          {settings.go2rtcEnabled && (
          <>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-force-native-hls" class="font-medium">{t('settings.forceNativeHls')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.forceNativeHlsHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-binary-path" class="font-medium">{t('settings.binaryPath')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.go2rtcBinaryPathHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-config-dir" class="font-medium">{t('settings.configDirectory')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.go2rtcConfigDirectoryHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-api-port" class="font-medium">{t('settings.apiPort')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.go2rtcApiPortHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-rtsp-port" class="font-medium">{t('settings.rtspPort')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.go2rtcRtspPortHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-webrtc-enabled" class="font-medium">{t('settings.enableWebrtc')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableWebrtcHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-webrtc-listen-port" class="font-medium">{t('settings.webrtcListenPort')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.webrtcListenPortHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-stun-enabled" class="font-medium">{t('settings.enableStun')}</label>
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableStunHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-stun-server" class="font-medium">{t('settings.stunServer')}</label>
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
              <span class="hint text-sm text-muted-foreground">{t('settings.stunServerHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-external-ip" class="font-medium">{t('settings.externalIp')}</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-external-ip"
                name="go2rtcExternalIp"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcExternalIp}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder={t('settings.autoDetect')}
              />
              <span class="hint text-sm text-muted-foreground">{t('settings.externalIpHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-go2rtc-ice-servers" class="font-medium">{t('settings.iceServers')}</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-go2rtc-ice-servers"
                name="go2rtcIceServers"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.go2rtcIceServers}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder={t('settings.iceServersPlaceholder')}
              />
              <span class="hint text-sm text-muted-foreground">{t('settings.iceServersHelp')}</span>
            </div>
          </div>

          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
            <label for="setting-go2rtc-config-override" class="font-medium pt-2">
              {t('settings.go2rtcConfigOverride')}
            </label>
            <div class="col-span-2">
              <textarea
                id="setting-go2rtc-config-override"
                name="go2rtcConfigOverride"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md font-mono text-sm disabled:opacity-60 disabled:cursor-not-allowed"
                rows="6"
                value={settings.go2rtcConfigOverride}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder={"ffmpeg:\n  h264_hw: \"-codec:v h264_v4l2m2m\"\n\nlog:\n  level: trace"}
              />
              <span class="hint text-sm text-muted-foreground">{t('settings.go2rtcConfigOverrideHelp')}</span>
            </div>
          </div>
          </>
          )}
          </div>

          {/* MQTT Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.mqttEventStreaming')}</h3>
            <p class="text-sm text-muted-foreground mb-4">
              {t('settings.mqttEventStreamingDescription')}
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableMqttHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.brokerPortHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-username" class="font-medium">{t('auth.username')}</label>
            <div class="col-span-2">
              <input
                type="text"
                id="setting-mqtt-username"
                name="mqttUsername"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.mqttUsername}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder={t('settings.optionalPlaceholder')}
              />
              <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.leaveEmptyForAnonymousAccess')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-mqtt-password" class="font-medium">{t('auth.password')}</label>
            <div class="col-span-2">
              <input
                type="password"
                id="setting-mqtt-password"
                name="mqttPassword"
                class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.mqttPassword}
                onChange={handleInputChange}
                disabled={!canModifySettings}
                placeholder={t('settings.optionalPlaceholder')}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableTlsHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.keepaliveSecondsHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.retainMessagesHelp')}</span>
            </div>
          </div>

          {/* Home Assistant Auto-Discovery sub-section */}
          <h4 class="text-md font-semibold mt-6 mb-3 pb-1 border-b border-border">{t('settings.homeAssistantAutoDiscovery')}</h4>
          <p class="text-sm text-muted-foreground mb-4">
            {t('settings.homeAssistantAutoDiscoveryDescription')}
          </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableHaDiscoveryHelp')}</span>
            </div>
          </div>
          {settings.mqttHaDiscovery && (
          <>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.snapshotIntervalSecondsHelp')}</span>
            </div>
          </div>
          </>
          )}
          </div>

          {/* TURN Server Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.webrtcTurnServer')}</h3>
            <p class="text-sm text-muted-foreground mb-4">
              {t('settings.webrtcTurnServerDescription')}
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
              <span class="hint text-sm text-muted-foreground ml-2">{t('settings.enableTurnRelayHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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

          {/* ONVIF Discovery Settings */}
          <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.onvifDiscovery')}</h3>
            <p class="text-sm text-muted-foreground mb-4">
              {t('settings.onvifDiscoveryDescription')}
            </p>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-onvif-discovery-enabled" class="font-medium">{t('settings.enableOnvifDiscovery')}</label>
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
              <span class="hint ml-2 text-sm text-muted-foreground">{t('settings.enableOnvifDiscoveryHelp')}</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
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

          {/* Save Settings Button - at bottom of form */}
          {canModifySettings && (
            <div class="flex justify-end mt-6">
              <button
                id="save-settings-btn"
                class="px-6 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                onClick={saveSettings}
              >
                {t('settings.saveSettings')}
              </button>
            </div>
          )}
        </div>
      </ContentLoader>
    </section>
  );
}
