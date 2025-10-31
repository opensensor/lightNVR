/**
 * LightNVR Web Interface MotionView Component
 * Component for managing motion recording settings
 */

import { useState } from 'react';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import {
  useQuery,
  useMutation,
  useQueryClient,
  fetchJSON
} from '../../query-client.js';

/**
 * MotionView component
 * @returns {JSX.Element} MotionView component
 */
export function MotionView() {
  const queryClient = useQueryClient();
  const [selectedStream, setSelectedStream] = useState(null);
  const [configModalVisible, setConfigModalVisible] = useState(false);
  const [currentConfig, setCurrentConfig] = useState({
    enabled: true,
    pre_buffer_seconds: 5,
    post_buffer_seconds: 10,
    max_file_duration: 300,
    codec: 'h264',
    quality: 'medium',
    retention_days: 7
  });

  // Fetch streams data
  const {
    data: streamsResponse = [],
    isLoading: streamsLoading
  } = useQuery(['streams'], '/api/streams', {
    timeout: 10000,
    retries: 2,
    retryDelay: 1000
  });

  // Fetch storage statistics
  const {
    data: storageStats,
    isLoading: storageLoading
  } = useQuery(['motionStorage'], '/api/motion/storage', {
    timeout: 5000,
    retries: 1,
    retryDelay: 1000
  });

  // Process streams response
  const streams = Array.isArray(streamsResponse) ? streamsResponse : (streamsResponse.streams || []);

  // Mutation for saving motion config
  const saveConfigMutation = useMutation({
    mutationFn: async (data) => {
      const { streamName, ...configData } = data;
      const url = `/api/motion/config/${encodeURIComponent(streamName)}`;
      return await fetchJSON(url, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(configData),
        timeout: 10000
      });
    },
    onSuccess: () => {
      showStatusMessage('Motion recording configuration saved');
      setConfigModalVisible(false);
      queryClient.invalidateQueries({ queryKey: ['motionConfig'] });
      queryClient.invalidateQueries({ queryKey: ['motionStats'] });
    },
    onError: (error) => {
      showStatusMessage(`Error saving configuration: ${error.message}`, 5000, 'error');
    }
  });

  // Mutation for deleting motion config
  const deleteConfigMutation = useMutation({
    mutationFn: async (streamName) => {
      const url = `/api/motion/config/${encodeURIComponent(streamName)}`;
      return await fetchJSON(url, {
        method: 'DELETE',
        timeout: 10000
      });
    },
    onSuccess: () => {
      showStatusMessage('Motion recording disabled');
      queryClient.invalidateQueries({ queryKey: ['motionConfig'] });
      queryClient.invalidateQueries({ queryKey: ['motionStats'] });
    },
    onError: (error) => {
      showStatusMessage(`Error disabling motion recording: ${error.message}`, 5000, 'error');
    }
  });

  // Open configuration modal
  const openConfigModal = async (stream) => {
    setSelectedStream(stream);
    
    // Try to load existing configuration
    try {
      const config = await fetchJSON(`/api/motion/config/${encodeURIComponent(stream.name)}`, {
        timeout: 5000
      });
      setCurrentConfig(config);
    } catch (error) {
      // No existing config, use defaults
      setCurrentConfig({
        enabled: true,
        pre_buffer_seconds: 5,
        post_buffer_seconds: 10,
        max_file_duration: 300,
        codec: 'h264',
        quality: 'medium',
        retention_days: 7
      });
    }
    
    setConfigModalVisible(true);
  };

  // Save configuration
  const handleSaveConfig = () => {
    if (!selectedStream) return;
    
    saveConfigMutation.mutate({
      streamName: selectedStream.name,
      ...currentConfig
    });
  };

  // Disable motion recording
  const handleDisableMotion = (streamName) => {
    if (confirm(`Disable motion recording for ${streamName}?`)) {
      deleteConfigMutation.mutate(streamName);
    }
  };

  // Format bytes to human readable
  const formatBytes = (bytes) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  };

  // Format timestamp
  const formatDate = (timestamp) => {
    if (!timestamp) return 'N/A';
    return new Date(timestamp * 1000).toLocaleString();
  };

  if (streamsLoading) {
    return <ContentLoader />;
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Motion Recording</h1>
        <p className="text-muted-foreground">
          Configure ONVIF motion detection recording for your cameras
        </p>
      </div>

      {/* Storage Statistics */}
      {storageStats && (
        <div className="bg-card text-card-foreground rounded-lg shadow-md p-6 mb-8">
          <h2 className="text-xl font-semibold mb-4">Storage Statistics</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="bg-muted p-4 rounded">
              <div className="text-sm text-muted-foreground">Total Recordings</div>
              <div className="text-2xl font-bold">{storageStats.total_recordings || 0}</div>
            </div>
            <div className="bg-muted p-4 rounded">
              <div className="text-sm text-muted-foreground">Total Size</div>
              <div className="text-2xl font-bold">{formatBytes(storageStats.total_size_bytes || 0)}</div>
            </div>
            <div className="bg-muted p-4 rounded">
              <div className="text-sm text-muted-foreground">Disk Usage</div>
              <div className="text-2xl font-bold">{(storageStats.disk_space_used_percent || 0).toFixed(1)}%</div>
            </div>
          </div>
        </div>
      )}

      {/* Streams List */}
      <div className="bg-card text-card-foreground rounded-lg shadow-md overflow-hidden">
        <div className="p-6 border-b border-border">
          <h2 className="text-xl font-semibold">Camera Motion Settings</h2>
        </div>
        
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-border">
            <thead className="bg-muted">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Camera
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Pre-Buffer
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Post-Buffer
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Retention
                </th>
                <th className="px-6 py-3 text-right text-xs font-medium text-muted-foreground uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="bg-card text-card-foreground divide-y divide-border">
              {streams.map((stream) => (
                <StreamRow
                  key={stream.name}
                  stream={stream}
                  onConfigure={openConfigModal}
                  onDisable={handleDisableMotion}
                />
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Configuration Modal */}
      {configModalVisible && selectedStream && (
        <ConfigModal
          stream={selectedStream}
          config={currentConfig}
          onConfigChange={setCurrentConfig}
          onSave={handleSaveConfig}
          onClose={() => setConfigModalVisible(false)}
          isSaving={saveConfigMutation.isPending}
        />
      )}
    </div>
  );
}

/**
 * StreamRow component for displaying individual stream motion settings
 */
function StreamRow({ stream, onConfigure, onDisable }) {
  const [stats, setStats] = useState(null);
  const [config, setConfig] = useState(null);

  // Fetch motion stats for this stream
  const { data: statsData } = useQuery(
    ['motionStats', stream.name],
    `/api/motion/stats/${encodeURIComponent(stream.name)}`,
    {
      timeout: 5000,
      retries: 1,
      retryDelay: 1000,
      onError: () => {
        // No stats available, motion recording not configured
      }
    }
  );

  // Fetch motion config for this stream
  const { data: configData } = useQuery(
    ['motionConfig', stream.name],
    `/api/motion/config/${encodeURIComponent(stream.name)}`,
    {
      timeout: 5000,
      retries: 1,
      retryDelay: 1000,
      onError: () => {
        // No config available
      }
    }
  );

  const isEnabled = configData?.enabled || false;

  return (
    <tr>
      <td className="px-6 py-4 whitespace-nowrap">
        <div className="text-sm font-medium text-foreground">{stream.name}</div>
      </td>
      <td className="px-6 py-4 whitespace-nowrap">
        <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
          isEnabled ? 'badge-success' : 'badge-muted'
        }`}>
          {isEnabled ? 'Enabled' : 'Disabled'}
        </span>
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">
        {configData?.pre_buffer_seconds || '-'}s
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">
        {configData?.post_buffer_seconds || '-'}s
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">
        {configData?.retention_days || '-'} days
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
        <button
          onClick={() => onConfigure(stream)}
          className="mr-4"
          style={{color: 'hsl(var(--primary))'}}
          onMouseOver={(e) => e.currentTarget.style.color = 'hsl(var(--primary) / 0.7)'}
          onMouseOut={(e) => e.currentTarget.style.color = 'hsl(var(--primary))'}
        >
          Configure
        </button>
        {isEnabled && (
          <button
            onClick={() => onDisable(stream.name)}
            style={{color: 'hsl(var(--danger))'}}
            onMouseOver={(e) => e.currentTarget.style.color = 'hsl(var(--danger) / 0.7)'}
            onMouseOut={(e) => e.currentTarget.style.color = 'hsl(var(--danger))'}
          >
            Disable
          </button>
        )}
      </td>
    </tr>
  );
}

/**
 * ConfigModal component for editing motion recording configuration
 */
function ConfigModal({ stream, config, onConfigChange, onSave, onClose, isSaving }) {
  const handleChange = (field, value) => {
    onConfigChange({
      ...config,
      [field]: value
    });
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto">
        <div className="p-6 border-b border-border">
          <h2 className="text-2xl font-bold">Motion Recording Configuration</h2>
          <p className="text-muted-foreground mt-1">Camera: {stream.name}</p>
        </div>

        <div className="p-6 space-y-6">
          {/* Enabled Toggle */}
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium text-foreground">
              Enable Motion Recording
            </label>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                checked={config.enabled}
                onChange={(e) => handleChange('enabled', e.target.checked)}
                className="sr-only peer"
                style={{accentColor: 'hsl(var(--primary))'}}
              />
              <div className="w-11 h-6 peer-focus:outline-none peer-focus:ring-4 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border after:rounded-full after:h-5 after:w-5 after:transition-all" style={{backgroundColor: config.enabled ? 'hsl(var(--primary))' : 'hsl(var(--muted))', borderColor: 'hsl(var(--border))'}}></div>
            </label>
          </div>

          {/* Pre-Buffer Seconds */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Pre-Event Buffer (seconds)
            </label>
            <input
              type="number"
              min="0"
              max="30"
              value={config.pre_buffer_seconds}
              onChange={(e) => handleChange('pre_buffer_seconds', parseInt(e.target.value))}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            />
            <p className="mt-1 text-sm text-muted-foreground">
              Seconds of video to include before motion event (0-30)
            </p>
          </div>

          {/* Post-Buffer Seconds */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Post-Event Buffer (seconds)
            </label>
            <input
              type="number"
              min="0"
              max="60"
              value={config.post_buffer_seconds}
              onChange={(e) => handleChange('post_buffer_seconds', parseInt(e.target.value))}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            />
            <p className="mt-1 text-sm text-muted-foreground">
              Seconds to continue recording after motion stops (0-60)
            </p>
          </div>

          {/* Max File Duration */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Maximum File Duration (seconds)
            </label>
            <input
              type="number"
              min="60"
              max="3600"
              step="60"
              value={config.max_file_duration}
              onChange={(e) => handleChange('max_file_duration', parseInt(e.target.value))}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            />
            <p className="mt-1 text-sm text-muted-foreground">
              Maximum duration for a single recording file (60-3600 seconds)
            </p>
          </div>

          {/* Codec */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Video Codec
            </label>
            <select
              value={config.codec}
              onChange={(e) => handleChange('codec', e.target.value)}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            >
              <option value="h264">H.264</option>
              <option value="h265">H.265</option>
            </select>
          </div>

          {/* Quality */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Recording Quality
            </label>
            <select
              value={config.quality}
              onChange={(e) => handleChange('quality', e.target.value)}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            >
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
            </select>
          </div>

          {/* Retention Days */}
          <div>
            <label className="block text-sm font-medium text-foreground mb-2">
              Retention Period (days)
            </label>
            <input
              type="number"
              min="1"
              max="365"
              value={config.retention_days}
              onChange={(e) => handleChange('retention_days', parseInt(e.target.value))}
              className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 bg-background text-foreground"
            />
            <p className="mt-1 text-sm text-muted-foreground">
              Automatically delete recordings older than this many days (1-365)
            </p>
          </div>
        </div>

        <div className="p-6 border-t border-border flex justify-end space-x-3">
          <button
            onClick={onClose}
            disabled={isSaving}
            className="px-4 py-2 border border-input rounded-md shadow-sm text-sm font-medium text-foreground bg-background hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            onClick={onSave}
            disabled={isSaving}
            className="btn-primary disabled:opacity-50"
          >
            {isSaving ? 'Saving...' : 'Save Configuration'}
          </button>
        </div>
      </div>
    </div>
  );
}

