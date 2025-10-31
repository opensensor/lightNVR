/**
 * LightNVR Web Interface SettingsView Component
 * Preact component for the settings page
 */

import { useState, useEffect } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { useQuery, useMutation, fetchJSON } from '../../query-client.js';
import { ThemeCustomizer } from './ThemeCustomizer.jsx';

/**
 * SettingsView component
 * @returns {JSX.Element} SettingsView component
 */
export function SettingsView() {
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
    defaultPostBuffer: 10
  });
  
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
        defaultPostBuffer: settingsData.post_detection_buffer?.toString() || '10'
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
      post_detection_buffer: parseInt(settings.defaultPostBuffer, 10)
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

  return (
    <section id="settings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 class="text-xl font-bold">Settings</h2>
        <div class="controls">
          <button 
            id="save-settings-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick={saveSettings}
          >
            Save Settings
          </button>
        </div>
      </div>
      
      <ContentLoader
        isLoading={isLoading}
        hasData={!!settingsData}
        loadingMessage="Loading settings..."
        emptyMessage="No settings available. Please try again later."
      >
        <div class="settings-container space-y-6">
          {/* Appearance Settings */}
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
                class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
                value={settings.logLevel}
                onChange={handleInputChange}
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
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.storagePath}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path-hls" class="font-medium">HLS Storage Path</label>
            <div class="col-span-2">
              <input 
                type="text" 
                id="setting-storage-path-hls" 
                name="storagePathHls"
                class="w-full p-2 border border-input rounded bg-background text-foreground"
                value={settings.storagePathHls}
                onChange={handleInputChange}
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
                class="p-2 border border-input rounded bg-background text-foreground"
                value={settings.maxStorage}
                onChange={handleInputChange}
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
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.retention}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auto-delete" class="font-medium">Auto Delete Oldest</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-auto-delete"
                name="autoDelete"
                class="w-4 h-4 rounded focus:ring-2"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.autoDelete}
                onChange={handleInputChange}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-db-path" class="font-medium">Database Path</label>
            <input 
              type="text" 
              id="setting-db-path" 
              name="dbPath"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.dbPath}
              onChange={handleInputChange}
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
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.webPort}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-enabled" class="font-medium">Enable Authentication</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-auth-enabled"
                name="authEnabled"
                class="w-4 h-4 rounded focus:ring-2"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.authEnabled}
                onChange={handleInputChange}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-username" class="font-medium">Username</label>
            <input 
              type="text" 
              id="setting-username" 
              name="username"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.username}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-password" class="font-medium">Password</label>
            <input 
              type="password" 
              id="setting-password" 
              name="password"
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.password}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-webrtc-disabled" class="font-medium">Disable WebRTC (Use HLS Only)</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-webrtc-disabled"
                name="webrtcDisabled"
                class="w-4 h-4 rounded focus:ring-2"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.webrtcDisabled}
                onChange={handleInputChange}
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
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.bufferSize}
              onChange={handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-use-swap" class="font-medium">Use Swap File</label>
            <div class="col-span-2">
              <input
                type="checkbox"
                id="setting-use-swap"
                name="useSwap"
                class="w-4 h-4 rounded focus:ring-2"
                style={{accentColor: 'hsl(var(--primary))'}}
                checked={settings.useSwap}
                onChange={handleInputChange}
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
              class="col-span-2 p-2 border border-input rounded bg-background text-foreground"
              value={settings.swapSize}
              onChange={handleInputChange}
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
                class="w-full p-2 border border-input rounded bg-background text-foreground"
                value={settings.detectionModelsPath}
                onChange={handleInputChange}
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
                  class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                  value={settings.defaultDetectionThreshold}
                  onChange={handleThresholdChange}
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
                class="p-2 border border-input rounded bg-background text-foreground"
                value={settings.defaultPreBuffer}
                onChange={handleInputChange}
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
                class="p-2 border border-input rounded bg-background text-foreground"
                value={settings.defaultPostBuffer}
                onChange={handleInputChange}
              />
              <span class="hint text-sm text-muted-foreground">Seconds of video to keep after detection</span>
            </div>
          </div>
          </div>
        </div>
      </ContentLoader>
    </section>
  );
}
