/**
 * LightNVR Web Interface SettingsView Component
 * Preact component for the settings page
 */

import { h } from '../../preact.min.js';
import { html } from '../../preact-app.js';
import { useState, useEffect, useRef } from '../../preact.hooks.module.js';
import { showStatusMessage } from './UI.js';

/**
 * SettingsView component
 * @returns {JSX.Element} SettingsView component
 */
export function SettingsView() {
  const [settings, setSettings] = useState({
    logLevel: '2',
    storagePath: '/var/lib/lightnvr/recordings',
    maxStorage: '0',
    retention: '30',
    autoDelete: true,
    webPort: '8080',
    authEnabled: true,
    username: 'admin',
    password: 'admin',
    bufferSize: '1024',
    useSwap: true,
    swapSize: '128',
    detectionModelsPath: '',
    defaultDetectionThreshold: 50,
    defaultPreBuffer: 5,
    defaultPostBuffer: 10
  });
  
  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, []);
  
  // Load settings from API
  const loadSettings = async () => {
    try {
      const response = await fetch('/api/settings');
      if (!response.ok) {
        throw new Error('Failed to load settings');
      }
      
      const data = await response.json();
      console.log('Settings loaded:', data);
      
      // Update state with loaded settings
      setSettings(prev => ({
        ...prev,
        ...data,
        // Convert numeric values to strings for form inputs
        logLevel: data.logLevel?.toString() || prev.logLevel,
        maxStorage: data.maxStorage?.toString() || prev.maxStorage,
        retention: data.retention?.toString() || prev.retention,
        webPort: data.webPort?.toString() || prev.webPort,
        bufferSize: data.bufferSize?.toString() || prev.bufferSize,
        swapSize: data.swapSize?.toString() || prev.swapSize,
        defaultDetectionThreshold: data.defaultDetectionThreshold || prev.defaultDetectionThreshold,
        defaultPreBuffer: data.defaultPreBuffer?.toString() || prev.defaultPreBuffer,
        defaultPostBuffer: data.defaultPostBuffer?.toString() || prev.defaultPostBuffer
      }));
    } catch (error) {
      console.error('Error loading settings:', error);
      showStatusMessage('Error loading settings: ' + error.message);
    }
  };
  
  // Save settings
  const saveSettings = async () => {
    try {
      const response = await fetch('/api/settings', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          ...settings,
          // Convert string values to numbers where needed
          logLevel: parseInt(settings.logLevel, 10),
          maxStorage: parseInt(settings.maxStorage, 10),
          retention: parseInt(settings.retention, 10),
          webPort: parseInt(settings.webPort, 10),
          bufferSize: parseInt(settings.bufferSize, 10),
          swapSize: parseInt(settings.swapSize, 10),
          defaultPreBuffer: parseInt(settings.defaultPreBuffer, 10),
          defaultPostBuffer: parseInt(settings.defaultPostBuffer, 10)
        })
      });
      
      if (!response.ok) {
        throw new Error('Failed to save settings');
      }
      
      showStatusMessage('Settings saved successfully');
    } catch (error) {
      console.error('Error saving settings:', error);
      showStatusMessage('Error saving settings: ' + error.message);
    }
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
  
  return html`
    <section id="settings-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Settings</h2>
        <div class="controls">
          <button 
            id="save-settings-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${saveSettings}
          >
            Save Settings
          </button>
        </div>
      </div>
      
      <div class="settings-container space-y-6">
        <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">General Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-log-level" class="font-medium">Log Level</label>
            <select 
              id="setting-log-level" 
              name="logLevel"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.logLevel}
              onChange=${handleInputChange}
            >
              <option value="0">Error</option>
              <option value="1">Warning</option>
              <option value="2">Info</option>
              <option value="3">Debug</option>
            </select>
          </div>
        </div>
        
        <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Storage Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-storage-path" class="font-medium">Storage Path</label>
            <input 
              type="text" 
              id="setting-storage-path" 
              name="storagePath"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.storagePath}
              onChange=${handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-max-storage" class="font-medium">Maximum Storage Size (GB)</label>
            <div class="col-span-2 flex items-center">
              <input 
                type="number" 
                id="setting-max-storage" 
                name="maxStorage"
                min="0" 
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${settings.maxStorage}
                onChange=${handleInputChange}
              />
              <span class="hint ml-2 text-sm text-gray-500 dark:text-gray-400">0 = unlimited</span>
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-retention" class="font-medium">Retention Period (days)</label>
            <input 
              type="number" 
              id="setting-retention" 
              name="retention"
              min="1" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.retention}
              onChange=${handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auto-delete" class="font-medium">Auto Delete Oldest</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-auto-delete" 
                name="autoDelete"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${settings.autoDelete}
                onChange=${handleInputChange}
              />
            </div>
          </div>
        </div>
        
        <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Web Interface Settings</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-web-port" class="font-medium">Web Port</label>
            <input 
              type="number" 
              id="setting-web-port" 
              name="webPort"
              min="1" 
              max="65535" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.webPort}
              onChange=${handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-auth-enabled" class="font-medium">Enable Authentication</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-auth-enabled" 
                name="authEnabled"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${settings.authEnabled}
                onChange=${handleInputChange}
              />
            </div>
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-username" class="font-medium">Username</label>
            <input 
              type="text" 
              id="setting-username" 
              name="username"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.username}
              onChange=${handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-password" class="font-medium">Password</label>
            <input 
              type="password" 
              id="setting-password" 
              name="password"
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.password}
              onChange=${handleInputChange}
            />
          </div>
        </div>
        
        <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory Optimization</h3>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-buffer-size" class="font-medium">Buffer Size (KB)</label>
            <input 
              type="number" 
              id="setting-buffer-size" 
              name="bufferSize"
              min="128" 
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.bufferSize}
              onChange=${handleInputChange}
            />
          </div>
          <div class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
            <label for="setting-use-swap" class="font-medium">Use Swap File</label>
            <div class="col-span-2">
              <input 
                type="checkbox" 
                id="setting-use-swap" 
                name="useSwap"
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600"
                checked=${settings.useSwap}
                onChange=${handleInputChange}
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
              class="col-span-2 p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
              value=${settings.swapSize}
              onChange=${handleInputChange}
            />
          </div>
        </div>
        
        <div class="settings-group bg-white dark:bg-gray-800 rounded-lg shadow p-4">
          <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Detection-Based Recording</h3>
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
                class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${settings.detectionModelsPath}
                onChange=${handleInputChange}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Directory where detection models are stored</span>
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
                  value=${settings.defaultDetectionThreshold}
                  onChange=${handleThresholdChange}
                />
                <span id="threshold-value" class="ml-2 min-w-[3rem] text-center">${settings.defaultDetectionThreshold}%</span>
              </div>
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Confidence threshold for detection (0-100%)</span>
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
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${settings.defaultPreBuffer}
                onChange=${handleInputChange}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep before detection</span>
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
                class="p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                value=${settings.defaultPostBuffer}
                onChange=${handleInputChange}
              />
              <span class="hint text-sm text-gray-500 dark:text-gray-400">Seconds of video to keep after detection</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  `;
}

/**
 * Load SettingsView component
 */
export function loadSettingsView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the SettingsView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${SettingsView} />`, mainContent);
  });
}
