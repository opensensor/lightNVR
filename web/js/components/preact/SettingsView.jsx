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
    storagePath: '/var/lib/lightnvr/recordings',
    storagePathHls: '', // New field for HLS storage path
    maxStorage: '0',
    retention: '30',
    autoDelete: true,
    dbPath: '/var/lib/lightnvr/lightnvr.db',
    webPort: '8080',
    authEnabled: true,
    username: 'admin',
    password: 'admin',
    webrtcDisabled: false, // Whether WebRTC is disabled (use HLS only)
    bufferSize: '1024',
    useSwap: true,
    swapSize: '128',
    detectionModelsPath: '',
    defaultDetectionThreshold: 50,
    defaultPreBuffer: 5,
    defaultPostBuffer: 10,
    bufferStrategy: 'auto',
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
    mqttRetain: false
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
    error,
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
        storagePath: settingsData.storage_path || '',
        storagePathHls: settingsData.storage_path_hls || '', // Map the HLS storage path
        maxStorage: settingsData.max_storage_size?.toString() || '',
        retention: settingsData.retention_days?.toString() || '',
        autoDelete: settingsData.auto_delete_oldest || false,
        dbPath: settingsData.db_path || '',
        webPort: settingsData.web_port?.toString() || '',
        authEnabled: settingsData.web_auth_enabled || false,
        username: settingsData.web_username || '',
        password: settingsData.web_password || '',
        webrtcDisabled: settingsData.webrtc_disabled || false,
        bufferSize: settingsData.buffer_size?.toString() || '',
        useSwap: settingsData.use_swap || false,
        swapSize: settingsData.swap_size?.toString() || '',
        detectionModelsPath: settingsData.models_path || '',
        defaultDetectionThreshold: settingsData.default_detection_threshold || 50,
        defaultPreBuffer: settingsData.pre_detection_buffer?.toString() || '5',
        defaultPostBuffer: settingsData.post_detection_buffer?.toString() || '10',
        bufferStrategy: settingsData.buffer_strategy || 'auto',
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
        mqttRetain: settingsData.mqtt_retain || false
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
      storage_path: settings.storagePath,
      storage_path_hls: settings.storagePathHls, // Include the HLS storage path
      max_storage_size: parseInt(settings.maxStorage, 10),
      retention_days: parseInt(settings.retention, 10),
      auto_delete_oldest: settings.autoDelete,
      db_path: settings.dbPath,
      web_port: parseInt(settings.webPort, 10),
      web_auth_enabled: settings.authEnabled,
      web_username: settings.username,
      web_password: settings.password,
      webrtc_disabled: settings.webrtcDisabled,
      buffer_size: parseInt(settings.bufferSize, 10),
      use_swap: settings.useSwap,
      swap_size: parseInt(settings.swapSize, 10),
      models_path: settings.detectionModelsPath,
      default_detection_threshold: settings.defaultDetectionThreshold,
      pre_detection_buffer: parseInt(settings.defaultPreBuffer, 10),
      post_detection_buffer: parseInt(settings.defaultPostBuffer, 10),
      buffer_strategy: settings.bufferStrategy,
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
      mqtt_retain: settings.mqttRetain
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
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Appearance</h3>
            <ThemeCustomizer />
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
            <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">Appearance</h3>
            <ThemeCustomizer />
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
            <label for="setting-username" class="font-medium">Username</label>
            <input
              type="text"
              id="setting-username"
              name="username"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.username}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-password" class="font-medium">Password</label>
            <input
              type="password"
              id="setting-password"
              name="password"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.password}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
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
              Select "motion" as the detection model in stream settings to use this feature.
            </p>
            <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
              <strong>Optimized Motion Detection:</strong> Memory and CPU optimized motion detection for embedded devices.
              Select "motion" as the detection model in stream settings to use this feature.
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
