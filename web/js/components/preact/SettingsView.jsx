/**
 * LightNVR Web Interface SettingsView Component
 *
 * PRD UXD_01 §5.2 / T2 (#399): tabbed layout, URL-hash persistence, live
 * label search, and a sticky Save button that uses T1's <AsyncButton>.
 * The per-tab form JSX lives in `web/js/components/preact/settings/*.jsx`;
 * this file owns the shared `settings` state, the save wiring, and the
 * chrome that wraps the tabs.
 */

import { useState, useEffect, useMemo, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { useQuery, useMutation, fetchJSON } from '../../query-client.js';
import { AsyncButton } from './AsyncButton.jsx';
import { validateSession } from '../../utils/auth-utils.js';
import { formatLocalDateTime } from '../../utils/date-utils.js';
import { useI18n } from '../../i18n.js';
import {
  getReduceMotionPref,
  setReduceMotionPref,
  applyReduceMotion,
} from '../../utils/reduceMotion.js';

import { GeneralTab } from './settings/GeneralTab.jsx';
import { StorageTab } from './settings/StorageTab.jsx';
import { StreamsDefaultsTab } from './settings/StreamsDefaultsTab.jsx';
import { DetectionTab } from './settings/DetectionTab.jsx';
import { Go2rtcTab } from './settings/Go2rtcTab.jsx';
import { MqttTab } from './settings/MqttTab.jsx';
import { AuthTab } from './settings/AuthTab.jsx';
import { AppearanceTab } from './settings/AppearanceTab.jsx';
import { AdvancedTab } from './settings/AdvancedTab.jsx';

/**
 * Tab definitions. `id` is the URL hash fragment ("#general", "#go2rtc", …).
 * `label` resolves via i18n; we fall back to the English display label if the
 * key is missing so existing locale files keep rendering.
 */
const TAB_DEFS = [
  { id: 'general',   labelKey: 'settings.tab.general',        labelFallback: 'General' },
  { id: 'storage',   labelKey: 'settings.tab.storage',        labelFallback: 'Storage' },
  { id: 'streams',   labelKey: 'settings.tab.streamsDefaults', labelFallback: 'Streams Defaults' },
  { id: 'detection', labelKey: 'settings.tab.detection',      labelFallback: 'Detection' },
  { id: 'go2rtc',    labelKey: 'settings.tab.go2rtc',         labelFallback: 'go2rtc' },
  { id: 'mqtt',      labelKey: 'settings.tab.mqtt',           labelFallback: 'MQTT' },
  { id: 'auth',      labelKey: 'settings.tab.auth',           labelFallback: 'Auth / Security' },
  { id: 'appearance', labelKey: 'settings.tab.appearance',    labelFallback: 'Appearance' },
  { id: 'advanced',  labelKey: 'settings.tab.advanced',       labelFallback: 'Advanced' },
];

const VALID_TAB_IDS = new Set(TAB_DEFS.map(t => t.id));

/** Read the initial tab from URL hash, defaulting to "general". */
function readTabFromHash() {
  try {
    if (typeof window === 'undefined' || !window.location) return 'general';
    const raw = (window.location.hash || '').replace(/^#/, '').trim();
    return VALID_TAB_IDS.has(raw) ? raw : 'general';
  } catch (_err) {
    return 'general';
  }
}

/**
 * SettingsView component
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
    storagePathHls: '',
    maxStorage: '0',
    retention: '30',
    autoDelete: true,
    generateThumbnails: true,
    thumbnailsPerRecording: 3,
    dbPath: '/var/lib/lightnvr/lightnvr.db',
    dbBackupIntervalMinutes: '60',
    dbBackupRetentionCount: '24',
    dbPostBackupScript: '',
    webPort: '8080',
    webBindIp: '0.0.0.0',
    webThreadPoolSize: '',
    maxStreams: '32',
    authEnabled: true,
    demoMode: false,
    webrtcDisabled: false,
    hlsDisabled: false,
    mseDisabled: false,
    authTimeoutHours: '24',
    authAbsoluteTimeoutHours: '168',
    trustedDeviceDays: '30',
    trustedProxyCidrs: '',
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
    detectionGracePeriod: 2,
    // In-process LiteRT detection engine
    detectionEngineEnabled: false,
    detectionEngineThreads: 1,
    detectionEngineDelegate: 'xnnpack',
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
    mqttHaDiscovery: false,
    mqttHaDiscoveryPrefix: 'homeassistant',
    mqttHaSnapshotInterval: '30',
    turnEnabled: false,
    turnServerUrl: '',
    turnUsername: '',
    turnPassword: '',
    onvifDiscoveryEnabled: false,
    onvifDiscoveryInterval: '300',
    onvifDiscoveryNetwork: 'auto'
  });

  // Baseline snapshot of the last-loaded/saved settings, used for dirty detection.
  // The save button is disabled until `settings` differs from this reference.
  const baselineRef = useRef(null);
  const [isDirty, setIsDirty] = useState(false);

  // Active tab (synced to window.location.hash) and search query.
  const [activeTab, setActiveTab] = useState(readTabFromHash);
  const [searchQuery, setSearchQuery] = useState('');
  const [debouncedQuery, setDebouncedQuery] = useState('');
  const containerRef = useRef(null);
  const firstMatchScrolledRef = useRef(false);

  // Fetch user role on mount
  useEffect(() => {
    async function fetchUserRole() {
      const session = await validateSession();
      if (session.valid) {
        setUserRole(session.role);
      } else {
        setUserRole('');
      }
    }
    fetchUserRole();
  }, []);

  // UXD T4 — Reduce Motion preference.
  const [reduceMotionPref, setReduceMotionPrefState] = useState(() => getReduceMotionPref());
  const handleReduceMotionChange = (value) => {
    const normalized = setReduceMotionPref(value);
    setReduceMotionPrefState(normalized);
    applyReduceMotion();
  };

  // Role loading / permissions
  const roleLoading = userRole === null;
  const canModifySettings = roleLoading || userRole === 'admin';
  const isViewer = userRole === 'viewer';

  // Fetch settings
  const {
    data: settingsData,
    isLoading,
    refetch
  } = useQuery(
    ['settings'],
    '/api/settings',
    {
      timeout: 15000,
      retries: 2,
      retryDelay: 1000
    }
  );

  // Save settings mutation
  const saveSettingsMutation = useMutation({
    mutationKey: ['saveSettings'],
    mutationFn: async (mappedSettings) => {
      return await fetchJSON('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(mappedSettings),
        timeout: 20000,
        retries: 1,
        retryDelay: 2000
      });
    },
    onSuccess: (response) => {
      if (response?.restart_required) {
        setRestartNotice(t('settings.restartNotice'));
        showStatusMessage(t('settings.savedRestart'), 'warning', 7000);
      } else {
        showStatusMessage(t('settings.saved'));
      }
      // Re-baseline: the server accepted our payload, so the current in-memory
      // `settings` is the new ground truth until the user edits again.
      baselineRef.current = settings;
      setIsDirty(false);
      refetch();
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

  // Map server response → local `settings` shape and stash baseline for dirty tracking.
  useEffect(() => {
    if (!settingsData) return;
    const mappedData = {
      logLevel: settingsData.log_level?.toString() || '',
      syslogEnabled: settingsData.syslog_enabled || false,
      syslogIdent: settingsData.syslog_ident || 'lightnvr',
      syslogFacility: settingsData.syslog_facility || 'LOG_USER',
      storagePath: settingsData.storage_path || '',
      storagePathHls: settingsData.storage_path_hls || '',
      // max_storage_size is stored in bytes; the UI field is in GB. Convert on
      // load (0/unset => empty = unlimited).
      maxStorage: settingsData.max_storage_size
        ? String(Math.round(settingsData.max_storage_size / (1024 * 1024 * 1024)))
        : '',
      retention: settingsData.retention_days?.toString() || '',
      autoDelete: settingsData.auto_delete_oldest || false,
      minFreePct: (settingsData.storage_min_free_pct ?? 10).toString(),
      pressureWarningPct: (settingsData.storage_pressure_warning_pct ?? 20).toString(),
      pressureCriticalPct: (settingsData.storage_pressure_critical_pct ?? 10).toString(),
      pressureEmergencyPct: (settingsData.storage_pressure_emergency_pct ?? 5).toString(),
      generateThumbnails: settingsData.generate_thumbnails !== false,
      thumbnailsPerRecording: settingsData.thumbnails_per_recording === 1 ? 1 : 3,
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
      hlsDisabled:    settingsData.hls_disabled    || false,
      mseDisabled:    settingsData.mse_disabled    || false,
      authTimeoutHours: settingsData.auth_timeout_hours?.toString() || '24',
      authAbsoluteTimeoutHours: settingsData.auth_absolute_timeout_hours?.toString() || '168',
      trustedDeviceDays: settingsData.trusted_device_days?.toString() || '30',
      trustedProxyCidrs: settingsData.trusted_proxy_cidrs || '',
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
      detectionGracePeriod: settingsData.detection_grace_period ?? 2,
      detectionEngineEnabled: settingsData.detection_engine_enabled || false,
      detectionEngineThreads: settingsData.detection_engine_threads ?? 1,
      detectionEngineDelegate: settingsData.detection_engine_delegate || 'xnnpack',
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
      // T4b/T14 quarantine surface — read-only
      go2rtcOverrideDisabledReason: settingsData.go2rtc_config_override_disabled_reason || '',
      go2rtcOverrideQuarantined: settingsData.go2rtc_config_override_quarantined || '',
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
      mqttHaDiscovery: settingsData.mqtt_ha_discovery || false,
      mqttHaDiscoveryPrefix: settingsData.mqtt_ha_discovery_prefix || 'homeassistant',
      mqttHaSnapshotInterval: settingsData.mqtt_ha_snapshot_interval?.toString() || '30',
      turnEnabled: settingsData.turn_enabled || false,
      turnServerUrl: settingsData.turn_server_url || '',
      turnUsername: settingsData.turn_username || '',
      turnPassword: settingsData.turn_password || '',
      onvifDiscoveryEnabled: settingsData.onvif_discovery_enabled || false,
      onvifDiscoveryInterval: settingsData.onvif_discovery_interval?.toString() || '300',
      onvifDiscoveryNetwork: settingsData.onvif_discovery_network || 'auto'
    };
    setSettings(prev => {
      const merged = { ...prev, ...mappedData };
      baselineRef.current = merged;
      return merged;
    });
    setIsDirty(false);
  }, [settingsData]);

  // Shallow-diff settings vs baseline to drive the sticky-save enabled state.
  useEffect(() => {
    const base = baselineRef.current;
    if (!base) {
      setIsDirty(false);
      return;
    }
    let dirty = false;
    for (const key of Object.keys(settings)) {
      if (settings[key] !== base[key]) {
        dirty = true;
        break;
      }
    }
    setIsDirty(dirty);
  }, [settings]);

  // Save.
  // Returns a promise so <AsyncButton> can track pending state and guard
  // against rapid-tap double-submits (#399 / PRD UXD_01 §5.1).
  const saveSettings = () => {
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
      storage_path_hls: settings.storagePathHls,
      // UI field is GB; API expects bytes. Empty/0 => unlimited.
      max_storage_size: (parseInt(settings.maxStorage, 10) || 0) * 1024 * 1024 * 1024,
      retention_days: parseInt(settings.retention, 10),
      auto_delete_oldest: settings.autoDelete,
      storage_min_free_pct: Number.isNaN(parseInt(settings.minFreePct, 10)) ? 10 : parseInt(settings.minFreePct, 10),
      storage_pressure_warning_pct: parseFloat(settings.pressureWarningPct),
      storage_pressure_critical_pct: parseFloat(settings.pressureCriticalPct),
      storage_pressure_emergency_pct: parseFloat(settings.pressureEmergencyPct),
      generate_thumbnails: settings.generateThumbnails,
      thumbnails_per_recording: parseInt(settings.thumbnailsPerRecording, 10) === 1 ? 1 : 3,
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
      hls_disabled:    settings.hlsDisabled,
      mse_disabled:    settings.mseDisabled,
      auth_timeout_hours: parseInt(settings.authTimeoutHours, 10),
      auth_absolute_timeout_hours: parseInt(settings.authAbsoluteTimeoutHours, 10),
      trusted_device_days: parseInt(settings.trustedDeviceDays, 10),
      trusted_proxy_cidrs: settings.trustedProxyCidrs,
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
      detection_grace_period: parseInt(settings.detectionGracePeriod, 10),
      detection_engine_enabled: !!settings.detectionEngineEnabled,
      detection_engine_threads: parseInt(settings.detectionEngineThreads, 10) || 1,
      detection_engine_delegate: settings.detectionEngineDelegate || 'xnnpack',
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
      mqtt_ha_discovery: settings.mqttHaDiscovery,
      mqtt_ha_discovery_prefix: settings.mqttHaDiscoveryPrefix,
      mqtt_ha_snapshot_interval: parseInt(settings.mqttHaSnapshotInterval, 10),
      turn_enabled: settings.turnEnabled,
      turn_server_url: settings.turnServerUrl,
      turn_username: settings.turnUsername,
      turn_password: settings.turnPassword,
      onvif_discovery_enabled: settings.onvifDiscoveryEnabled,
      onvif_discovery_interval: parseInt(settings.onvifDiscoveryInterval, 10),
      onvif_discovery_network: settings.onvifDiscoveryNetwork
    };
    return saveSettingsMutation.mutateAsync(mappedSettings);
  };

  // Shared input handlers (passed to every tab via props).
  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    setSettings(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  const handleThresholdChange = (e) => {
    const value = parseInt(e.target.value, 10);
    setSettings(prev => ({ ...prev, defaultDetectionThreshold: value }));
  };

  // Tab → hash sync.  Keep `history.replaceState` calls so reloads + shared
  // links preserve position without stacking history entries.
  const selectTab = (id) => {
    if (!VALID_TAB_IDS.has(id)) return;
    setActiveTab(id);
    try {
      if (typeof window !== 'undefined' && window.location && window.history && typeof window.history.replaceState === 'function') {
        window.history.replaceState(null, '', `#${id}`);
      }
    } catch (_err) {
      /* no-op */
    }
  };

  // React to external hash changes (back/forward buttons, user pasting URL).
  useEffect(() => {
    const onHashChange = () => {
      const next = readTabFromHash();
      setActiveTab(prev => (prev === next ? prev : next));
    };
    if (typeof window !== 'undefined') {
      window.addEventListener('hashchange', onHashChange);
      return () => window.removeEventListener('hashchange', onHashChange);
    }
    return undefined;
  }, []);

  // Debounce search input (150 ms per PRD).
  useEffect(() => {
    const handle = setTimeout(() => setDebouncedQuery(searchQuery.trim()), 150);
    return () => clearTimeout(handle);
  }, [searchQuery]);

  // On query change, walk [data-setting-label] across ALL tabs (they're
  // rendered concurrently via display:none toggling — see render below),
  // mark matches, optionally auto-switch tab, scroll first match into view.
  useEffect(() => {
    if (!containerRef.current) return;
    const root = containerRef.current;
    const query = debouncedQuery.toLowerCase();
    const nodes = root.querySelectorAll('[data-setting-label]');

    if (!query) {
      nodes.forEach((node) => {
        node.classList.remove('ring-2', 'ring-primary', 'bg-primary/5', 'rounded-md', 'opacity-40');
      });
      firstMatchScrolledRef.current = false;
      return;
    }

    let firstMatch = null;
    nodes.forEach((node) => {
      const label = (node.getAttribute('data-setting-label') || '').toLowerCase();
      const matches = label.includes(query);
      if (matches) {
        node.classList.add('ring-2', 'ring-primary', 'bg-primary/5', 'rounded-md');
        node.classList.remove('opacity-40');
        if (!firstMatch) firstMatch = node;
      } else {
        node.classList.remove('ring-2', 'ring-primary', 'bg-primary/5', 'rounded-md');
        node.classList.add('opacity-40');
      }
    });

    if (firstMatch && !firstMatchScrolledRef.current) {
      // Auto-switch to the tab containing the first match.
      const panel = firstMatch.closest('[data-tab-panel]');
      if (panel) {
        const nextTab = panel.getAttribute('data-tab-panel');
        if (nextTab && VALID_TAB_IDS.has(nextTab) && nextTab !== activeTab) {
          selectTab(nextTab);
        }
      }
      // Scroll into view after a tick so tab switch has landed.
      setTimeout(() => {
        try {
          firstMatch.scrollIntoView({ behavior: 'smooth', block: 'center' });
        } catch (_err) {
          /* no-op for non-DOM env */
        }
      }, 0);
      firstMatchScrolledRef.current = true;
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [debouncedQuery]);

  // When the user clears the search, reset the auto-scroll guard so the next
  // query re-scrolls.
  useEffect(() => {
    if (!debouncedQuery) firstMatchScrolledRef.current = false;
  }, [debouncedQuery]);

  // Memoize tab metadata with live-resolved labels.
  const tabs = useMemo(() => TAB_DEFS.map((tab) => {
    const label = t(tab.labelKey);
    return {
      ...tab,
      // `t()` returns the key back when missing; fall back to English label.
      label: (label && label !== tab.labelKey) ? label : tab.labelFallback,
    };
  }), [t]);

  // ---- Viewer short-circuit: Appearance only, no tabs ----
  if (isViewer) {
    return (
      <section id="settings-page" class="page">
        <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
          <h2 class="text-xl font-bold">{t('settings.title')}</h2>
        </div>

        <div class="settings-container space-y-6">
          <AppearanceTab
            reduceMotionPref={reduceMotionPref}
            handleReduceMotionChange={handleReduceMotionChange}
            t={t}
          />

          <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
            <p class="text-muted-foreground">
              {t('settings.adminOnly')}
            </p>
          </div>
        </div>
      </section>
    );
  }

  // ---- Admin render: tabs + search + sticky save ----
  const renderTabPanel = (tab) => {
    const hidden = tab.id !== activeTab;
    // We keep every panel mounted so the search scan can walk labels across
    // all tabs; CSS-hide the inactive ones.  `data-tab-panel` lets the search
    // handler auto-switch to whichever tab owns the first match.
    const style = hidden ? { display: 'none' } : undefined;
    return (
      <div
        key={tab.id}
        data-tab-panel={tab.id}
        role="tabpanel"
        id={`settings-panel-${tab.id}`}
        aria-labelledby={`settings-tab-${tab.id}`}
        style={style}
      >
        {tab.id === 'general' && (
          <GeneralTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            restartNotice={restartNotice}
            t={t}
          />
        )}
        {tab.id === 'storage' && (
          <StorageTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
        {tab.id === 'streams' && (
          <StreamsDefaultsTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
        {tab.id === 'detection' && (
          <DetectionTab
            settings={settings}
            handleInputChange={handleInputChange}
            handleThresholdChange={handleThresholdChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
        {tab.id === 'go2rtc' && (
          <Go2rtcTab
            settings={settings}
            setSettings={setSettings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
        {tab.id === 'mqtt' && (
          <MqttTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
        {tab.id === 'auth' && (
          <AuthTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
            authSessionsData={authSessionsData}
            authSessionsLoading={authSessionsLoading}
            authSessionsIsError={authSessionsIsError}
            authSessionsError={authSessionsError}
            revokeSessionMutation={revokeSessionMutation}
            trustedDevicesData={trustedDevicesData}
            trustedDevicesLoading={trustedDevicesLoading}
            trustedDevicesIsError={trustedDevicesIsError}
            trustedDevicesError={trustedDevicesError}
            revokeTrustedDeviceMutation={revokeTrustedDeviceMutation}
            formatTimestamp={formatTimestamp}
          />
        )}
        {tab.id === 'appearance' && (
          <AppearanceTab
            reduceMotionPref={reduceMotionPref}
            handleReduceMotionChange={handleReduceMotionChange}
            t={t}
          />
        )}
        {tab.id === 'advanced' && (
          <AdvancedTab
            settings={settings}
            handleInputChange={handleInputChange}
            canModifySettings={canModifySettings}
            t={t}
          />
        )}
      </div>
    );
  };

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
        {restartNotice && (
          <div class="rounded-lg border border-yellow-400 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-900 dark:text-yellow-100 shadow p-4 mb-4">
            <div class="flex items-start gap-3">
              <div class="text-lg leading-none">⚠️</div>
              <div>
                <div class="font-semibold">{t('settings.restartRequired')}</div>
                <p class="text-sm mt-1">{restartNotice}</p>
                <p class="text-sm mt-2 opacity-90">{t('settings.restartExplanation')}</p>
              </div>
            </div>
          </div>
        )}

        {/* Layout: vertical sidebar on desktop, horizontal scrollable chip
            strip on mobile — mirrors the pattern described in PRD §5.2 and
            the in-page navigation already familiar from SystemView's tabs. */}
        <div ref={containerRef} class="settings-container grid grid-cols-1 sm:grid-cols-[220px_minmax(0,1fr)] gap-4 pb-28 sm:pb-32">
          {/* Sidebar / mobile tab strip + search */}
          <div class="sm:sticky sm:top-4 self-start">
            <div class="mb-3">
              <div class="relative">
                <input
                  type="search"
                  value={searchQuery}
                  onInput={(e) => setSearchQuery(e.target.value)}
                  placeholder={t('settings.searchPlaceholder') !== 'settings.searchPlaceholder' ? t('settings.searchPlaceholder') : 'Search settings…'}
                  class="w-full p-2 pr-8 border border-input rounded bg-background text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-primary min-h-11"
                  aria-label="Search settings"
                />
                {searchQuery ? (
                  <button
                    type="button"
                    onClick={() => setSearchQuery('')}
                    aria-label="Clear search"
                    class="absolute right-1 top-1/2 -translate-y-1/2 h-8 w-8 flex items-center justify-center text-muted-foreground hover:text-foreground"
                  >
                    ×
                  </button>
                ) : null}
              </div>
            </div>

            {/* Mobile: horizontal scrollable chip strip.
                Desktop: vertical pill list. */}
            <div
              role="tablist"
              aria-label={t('settings.tabsLabel') !== 'settings.tabsLabel' ? t('settings.tabsLabel') : 'Settings sections'}
              class="flex overflow-x-auto snap-x snap-mandatory gap-2 border-b border-border pb-2 sm:flex-col sm:border-b-0 sm:overflow-x-visible sm:snap-none sm:gap-1"
            >
              {tabs.map((tab) => {
                const isActive = tab.id === activeTab;
                const baseCls = 'px-4 py-2 text-sm font-medium transition-colors snap-start shrink-0 min-h-11 whitespace-nowrap';
                const mobileCls = isActive
                  ? 'border-b-2 border-primary text-primary -mb-px'
                  : 'text-muted-foreground hover:text-foreground border-b-2 border-transparent';
                const desktopCls = isActive
                  ? 'sm:bg-primary sm:text-primary-foreground sm:rounded-md sm:border-b-0 sm:whitespace-normal sm:text-left sm:shadow-sm'
                  : 'sm:hover:bg-muted sm:rounded-md sm:border-b-0 sm:whitespace-normal sm:text-left';
                return (
                  <button
                    key={tab.id}
                    type="button"
                    role="tab"
                    id={`settings-tab-${tab.id}`}
                    aria-selected={isActive}
                    aria-controls={`settings-panel-${tab.id}`}
                    class={`${baseCls} ${mobileCls} ${desktopCls}`}
                    onClick={() => selectTab(tab.id)}
                  >
                    {tab.label}
                  </button>
                );
              })}
            </div>
          </div>

          {/* Panel area */}
          <div class="min-w-0">
            {tabs.map(renderTabPanel)}
          </div>
        </div>

        {/* Sticky Save bar.
            Preserves T1's <AsyncButton> wiring verbatim — only the chrome
            (sticky container + disabled-when-clean state) is new. */}
        {canModifySettings && (
          <div
            class="sticky bottom-0 left-0 right-0 z-30 backdrop-blur supports-[backdrop-filter]:bg-[hsl(var(--card)/0.85)] bg-card border-t border-border px-4 py-3"
            style={{ paddingBottom: 'calc(0.75rem + env(safe-area-inset-bottom))' }}
          >
            <div class="flex items-center justify-end gap-3">
              {isDirty ? (
                <span class="text-xs text-muted-foreground">{t('settings.unsavedChanges') !== 'settings.unsavedChanges' ? t('settings.unsavedChanges') : 'Unsaved changes'}</span>
              ) : null}
              <AsyncButton
                id="save-settings-btn"
                class="px-6 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 min-h-11 disabled:opacity-60 disabled:cursor-not-allowed"
                onClick={saveSettings}
                disabled={!isDirty}
              >
                {t('settings.saveSettings')}
              </AsyncButton>
            </div>
          </div>
        )}
      </ContentLoader>
    </section>
  );
}
