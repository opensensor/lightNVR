/**
 * LightNVR Web Interface StreamsView Component
 * Preact component for the streams page
 */

import { h } from '../../preact.min.js';
import { html } from '../../preact-app.js';
import { useState, useEffect, useRef } from '../../preact.hooks.module.js';
import { showStatusMessage } from './UI.js';

/**
 * StreamsView component
 * @returns {JSX.Element} StreamsView component
 */
export function StreamsView() {
  const [streams, setStreams] = useState([]);
  const [modalVisible, setModalVisible] = useState(false);
  const [onvifModalVisible, setOnvifModalVisible] = useState(false);
  const [discoveredDevices, setDiscoveredDevices] = useState([]);
  const [deviceProfiles, setDeviceProfiles] = useState([]);
  const [selectedDevice, setSelectedDevice] = useState(null);
  const [selectedProfile, setSelectedProfile] = useState(null);
  const [isDiscovering, setIsDiscovering] = useState(false);
  const [onvifCredentials, setOnvifCredentials] = useState({
    username: '',
    password: ''
  });
  const [currentStream, setCurrentStream] = useState({
    name: '',
    url: '',
    enabled: true,
    streamingEnabled: true,
    width: 1280,
    height: 720,
    fps: 15,
    codec: 'h264',
    protocol: '0',
    priority: '5',
    segment: 30,
    record: true,
    detectionEnabled: false,
    detectionModel: '',
    detectionThreshold: 50,
    detectionInterval: 10,
    preBuffer: 10,
    postBuffer: 30
  });
  const [detectionModels, setDetectionModels] = useState([]);
  const [isEditing, setIsEditing] = useState(false);
  
  // Load streams on mount
  useEffect(() => {
    loadStreams();
    loadDetectionModels();
  }, []);
  
  // Load streams from API
  const loadStreams = async () => {
    try {
      const response = await fetch('/api/streams');
      if (!response.ok) {
        throw new Error('Failed to load streams');
      }
      
      const data = await response.json();
      setStreams(data || []);
    } catch (error) {
      console.error('Error loading streams:', error);
      showStatusMessage('Error loading streams: ' + error.message);
    }
  };
  
  // Load detection models from API
  const loadDetectionModels = async () => {
    try {
      const response = await fetch('/api/detection/models');
      if (!response.ok) {
        throw new Error('Failed to load detection models');
      }
      
      const data = await response.json();
      setDetectionModels(data.models || []);
    } catch (error) {
      console.error('Error loading detection models:', error);
      // Don't show error message for this, just log it
    }
  };
  
  // Open add stream modal
  const openAddStreamModal = () => {
    setCurrentStream({
      name: '',
      url: '',
      enabled: true,
      streamingEnabled: true,
      width: 1280,
      height: 720,
      fps: 15,
      codec: 'h264',
      protocol: '0',
      priority: '5',
      segment: 30,
      record: true,
      detectionEnabled: false,
      detectionModel: '',
      detectionThreshold: 50,
      detectionInterval: 10,
      preBuffer: 10,
      postBuffer: 30
    });
    setIsEditing(false);
    setModalVisible(true);
  };
  
  // Open edit stream modal
  const openEditStreamModal = async (streamId) => {
    try {
      const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`);
      if (!response.ok) {
        throw new Error('Failed to load stream details');
      }
      
      const stream = await response.json();
      setCurrentStream({
        ...stream,
        // Convert numeric values to strings for form inputs
        width: stream.width || 1280,
        height: stream.height || 720,
        fps: stream.fps || 15,
        protocol: stream.protocol?.toString() || '0',
        priority: stream.priority?.toString() || '5',
        segment: stream.segment || 30,
        detectionThreshold: stream.detection_threshold || 50,
        detectionInterval: stream.detection_interval || 10,
        preBuffer: stream.pre_buffer || 10,
        postBuffer: stream.post_buffer || 30,
        // Map API fields to form fields
        detectionEnabled: stream.detection_based_recording || false,
        detectionModel: stream.detection_model || ''
      });
      setIsEditing(true);
      setModalVisible(true);
    } catch (error) {
      console.error('Error loading stream details:', error);
      showStatusMessage('Error loading stream details: ' + error.message);
    }
  };
  
  // Close modal
  const closeModal = () => {
    setModalVisible(false);
  };
  
  // Handle input change
  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    
    setCurrentStream(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };
  
  // Handle threshold slider change
  const handleThresholdChange = (e) => {
    const value = parseInt(e.target.value, 10);
    setCurrentStream(prev => ({
      ...prev,
      detectionThreshold: value
    }));
  };
  
  // Save stream
  const saveStream = async () => {
    try {
      // Prepare data for API
      const streamData = {
        ...currentStream,
        // Convert string values to numbers where needed
        width: parseInt(currentStream.width, 10),
        height: parseInt(currentStream.height, 10),
        fps: parseInt(currentStream.fps, 10),
        protocol: parseInt(currentStream.protocol, 10),
        priority: parseInt(currentStream.priority, 10),
        segment: parseInt(currentStream.segment, 10),
        // Map form fields to API fields
        detection_based_recording: currentStream.detectionEnabled,
        detection_model: currentStream.detectionModel,
        detection_threshold: currentStream.detectionThreshold,
        detection_interval: parseInt(currentStream.detectionInterval, 10),
        pre_buffer: parseInt(currentStream.preBuffer, 10),
        post_buffer: parseInt(currentStream.postBuffer, 10)
      };
      
      const url = isEditing 
        ? `/api/streams/${encodeURIComponent(currentStream.name)}`
        : '/api/streams';
      
      const method = isEditing ? 'PUT' : 'POST';
      
      const response = await fetch(url, {
        method,
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData)
      });
      
      if (!response.ok) {
        throw new Error(`Failed to ${isEditing ? 'update' : 'add'} stream`);
      }
      
      showStatusMessage(`Stream ${isEditing ? 'updated' : 'added'} successfully`);
      closeModal();
      loadStreams();
    } catch (error) {
      console.error(`Error ${isEditing ? 'updating' : 'adding'} stream:`, error);
      showStatusMessage(`Error ${isEditing ? 'updating' : 'adding'} stream: ${error.message}`);
    }
  };
  
  // Test stream connection
  const testStreamConnection = async () => {
    try {
      showStatusMessage('Testing stream connection...');
      
      const response = await fetch('/api/streams/test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          url: currentStream.url,
          protocol: parseInt(currentStream.protocol, 10)
        })
      });
      
      if (!response.ok) {
        throw new Error('Stream test failed');
      }
      
      const data = await response.json();
      
      if (data.success) {
        showStatusMessage('Stream connection successful!');
        
        // Update stream info if available
        if (data.info) {
          setCurrentStream(prev => ({
            ...prev,
            width: data.info.width || prev.width,
            height: data.info.height || prev.height,
            fps: data.info.fps || prev.fps,
            codec: data.info.codec || prev.codec
          }));
        }
      } else {
        showStatusMessage(`Stream test failed: ${data.message || 'Unknown error'}`, 3000, 'error');
      }
    } catch (error) {
      console.error('Error testing stream:', error);
      showStatusMessage('Error testing stream: ' + error.message, 3000, 'error');
    }
  };
  
  // Delete stream
  const deleteStream = async (streamId) => {
    if (!confirm(`Are you sure you want to delete stream "${streamId}"?`)) {
      return;
    }
    
    try {
      const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        throw new Error('Failed to delete stream');
      }
      
      showStatusMessage('Stream deleted successfully');
      loadStreams();
    } catch (error) {
      console.error('Error deleting stream:', error);
      showStatusMessage('Error deleting stream: ' + error.message);
    }
  };
  
  // Enable/disable stream
  const toggleStreamEnabled = async (streamId, enabled) => {
    try {
      const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}/enable`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ enabled: !enabled })
      });
      
      if (!response.ok) {
        throw new Error(`Failed to ${enabled ? 'disable' : 'enable'} stream`);
      }
      
      showStatusMessage(`Stream ${enabled ? 'disabled' : 'enabled'} successfully`);
      loadStreams();
    } catch (error) {
      console.error(`Error ${enabled ? 'disabling' : 'enabling'} stream:`, error);
      showStatusMessage(`Error ${enabled ? 'disabling' : 'enabling'} stream: ${error.message}`);
    }
  };
  
  // Handle ONVIF credential input change
  const handleCredentialChange = (e) => {
    const { name, value } = e.target;
    setOnvifCredentials(prev => ({
      ...prev,
      [name]: value
    }));
  };
  
  // Start ONVIF discovery
  const startOnvifDiscovery = async () => {
    try {
      setIsDiscovering(true);
      setDiscoveredDevices([]);
      showStatusMessage('Starting ONVIF discovery...');
      
      const response = await fetch('/api/onvif/discovery/discover', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
        })
      });
      
      if (!response.ok) {
        throw new Error('Failed to start ONVIF discovery');
      }
      
      const data = await response.json();
      setDiscoveredDevices(data.devices || []);
      
      if (data.devices && data.devices.length > 0) {
        showStatusMessage(`Discovered ${data.devices.length} ONVIF devices`);
      } else {
        showStatusMessage('No ONVIF devices found');
      }
    } catch (error) {
      console.error('Error discovering ONVIF devices:', error);
      showStatusMessage('Error discovering ONVIF devices: ' + error.message, 3000, 'error');
    } finally {
      setIsDiscovering(false);
    }
  };
  
  // State for profile loading
  const [isLoadingProfiles, setIsLoadingProfiles] = useState(false);

  // Get ONVIF device profiles
  const getDeviceProfiles = async (device) => {
    try {
      setSelectedDevice(device);
      setDeviceProfiles([]);
      setIsLoadingProfiles(true);
      showStatusMessage('Getting device profiles...');
      
      const response = await fetch(`/api/onvif/device/profiles`, {
        headers: {
          'X-Device-URL': device.device_service,
          'X-Username': onvifCredentials.username,
          'X-Password': onvifCredentials.password
        }
      });
      
      if (!response.ok) {
        throw new Error('Failed to get device profiles');
      }
      
      const data = await response.json();
      setDeviceProfiles(data.profiles || []);
      
      if (data.profiles && data.profiles.length > 0) {
        showStatusMessage(`Found ${data.profiles.length} profiles`);
      } else {
        showStatusMessage('No profiles found');
      }
    } catch (error) {
      console.error('Error getting device profiles:', error);
      showStatusMessage('Error getting device profiles: ' + error.message, 3000, 'error');
    } finally {
      setIsLoadingProfiles(false);
    }
  };
  
  // State for custom stream name
  const [customStreamName, setCustomStreamName] = useState('');
  const [showCustomNameInput, setShowCustomNameInput] = useState(false);

  // Add ONVIF device as stream
  const addOnvifDeviceAsStream = async (profile) => {
    try {
      setSelectedProfile(profile);
      
      // Generate a default stream name from device info
      const defaultStreamName = `ONVIF_${selectedDevice.ip_address.replace(/\./g, '_')}_${profile.name.replace(/\s+/g, '_')}`;
      
      // Show custom name input dialog
      setCustomStreamName(defaultStreamName);
      setShowCustomNameInput(true);
    } catch (error) {
      console.error('Error preparing to add ONVIF device:', error);
      showStatusMessage('Error preparing to add ONVIF device: ' + error.message, 3000, 'error');
    }
  };
  
  // State for adding stream
  const [isAddingStream, setIsAddingStream] = useState(false);

  // Submit ONVIF device with custom name
  const submitOnvifDevice = async () => {
    try {
      setIsAddingStream(true);
      showStatusMessage('Adding ONVIF device as stream...');
      
      const response = await fetch('/api/onvif/device/add', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          device_url: selectedDevice.device_service,
          profile_token: selectedProfile.token,
          stream_name: customStreamName,
          username: onvifCredentials.username,
          password: onvifCredentials.password
        })
      });
      
      if (!response.ok) {
        throw new Error('Failed to add ONVIF device as stream');
      }
      
      showStatusMessage('ONVIF device added as stream successfully');
      setOnvifModalVisible(false);
      setShowCustomNameInput(false);
      loadStreams();
    } catch (error) {
      console.error('Error adding ONVIF device as stream:', error);
      showStatusMessage('Error adding ONVIF device as stream: ' + error.message, 3000, 'error');
    } finally {
      setIsAddingStream(false);
    }
  };
  
  // Test ONVIF connection
  const testOnvifConnection = async (device) => {
    try {
      showStatusMessage('Testing ONVIF connection...');
      
      const response = await fetch('/api/onvif/device/test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          url: device.device_service,
          username: onvifCredentials.username,
          password: onvifCredentials.password
        })
      });
      
      if (!response.ok) {
        throw new Error('ONVIF connection test failed');
      }
      
      showStatusMessage('ONVIF connection successful!');
      
      // Get device profiles after successful connection test
      getDeviceProfiles(device);
    } catch (error) {
      console.error('Error testing ONVIF connection:', error);
      showStatusMessage('Error testing ONVIF connection: ' + error.message, 3000, 'error');
    }
  };
  
  return html`
    <section id="streams-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Streams</h2>
        <div class="controls flex space-x-2">
          <button 
            id="discover-onvif-btn" 
            class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${() => setOnvifModalVisible(true)}
          >
            Discover ONVIF Cameras
          </button>
          <button 
            id="add-stream-btn" 
            class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${openAddStreamModal}
          >
            Add Stream
          </button>
        </div>
      </div>
      
      <div class="streams-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
        <div class="overflow-x-auto">
          <table id="streams-table" class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead class="bg-gray-50 dark:bg-gray-700">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">URL</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Resolution</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">FPS</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Recording</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
              ${streams.length === 0 ? html`
                <tr>
                  <td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                    No streams configured
                  </td>
                </tr>
              ` : streams.map(stream => html`
                <tr key=${stream.name} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                      <span class=${`status-indicator w-2 h-2 rounded-full mr-2 ${stream.enabled ? 'bg-green-500' : 'bg-red-500'}`}></span>
                      ${stream.name}
                    </div>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">${stream.url}</td>
                  <td class="px-6 py-4 whitespace-nowrap">${stream.width}x${stream.height}</td>
                  <td class="px-6 py-4 whitespace-nowrap">${stream.fps}</td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    ${stream.record ? 'Enabled' : 'Disabled'}
                    ${stream.detection_based_recording ? ' (Detection)' : ''}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex space-x-2">
                      <button 
                        class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                        onClick=${() => openEditStreamModal(stream.name)}
                        title="Edit"
                      >
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                          <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                        </svg>
                      </button>
                      <button 
                        class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                        onClick=${() => deleteStream(stream.name)}
                        title="Delete"
                      >
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                          <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                        </svg>
                      </button>
                    </div>
                  </td>
                </tr>
              `)}
            </tbody>
          </table>
        </div>
      </div>
      
      ${modalVisible && html`
        <div id="stream-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">${isEditing ? 'Edit Stream' : 'Add Stream'}</h3>
              <span class="text-2xl cursor-pointer" onClick=${closeModal}>×</span>
            </div>
            <div class="p-4">
              <form id="stream-form" class="space-y-4">
                <div class="form-group">
                  <label for="stream-name" class="block text-sm font-medium mb-1">Name</label>
                  <input 
                    type="text" 
                    id="stream-name" 
                    name="name"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white ${isEditing ? 'bg-gray-100 dark:bg-gray-800' : ''}"
                    value=${currentStream.name}
                    onChange=${handleInputChange}
                    disabled=${isEditing}
                    required
                  />
                </div>
                <div class="form-group">
                  <label for="stream-url" class="block text-sm font-medium mb-1">URL</label>
                  <input 
                    type="text" 
                    id="stream-url" 
                    name="url"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                    placeholder="rtsp://example.com/stream" 
                    value=${currentStream.url}
                    onChange=${handleInputChange}
                    required
                  />
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-enabled" 
                    name="enabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${currentStream.enabled}
                    onChange=${handleInputChange}
                  />
                  <label for="stream-enabled" class="ml-2 block text-sm">Enabled</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable stream processing</span>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-streaming-enabled" 
                    name="streamingEnabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${currentStream.streamingEnabled}
                    onChange=${handleInputChange}
                  />
                  <label for="stream-streaming-enabled" class="ml-2 block text-sm">Streaming Enabled</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable live streaming</span>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="stream-width" class="block text-sm font-medium mb-1">Width</label>
                    <input 
                      type="number" 
                      id="stream-width" 
                      name="width"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="320" 
                      max="1920" 
                      value=${currentStream.width}
                      onChange=${handleInputChange}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-height" class="block text-sm font-medium mb-1">Height</label>
                    <input 
                      type="number" 
                      id="stream-height" 
                      name="height"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="240" 
                      max="1080" 
                      value=${currentStream.height}
                      onChange=${handleInputChange}
                    />
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div class="form-group">
                    <label for="stream-fps" class="block text-sm font-medium mb-1">FPS</label>
                    <input 
                      type="number" 
                      id="stream-fps" 
                      name="fps"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="1" 
                      max="30" 
                      value=${currentStream.fps}
                      onChange=${handleInputChange}
                    />
                  </div>
                  <div class="form-group">
                    <label for="stream-codec" class="block text-sm font-medium mb-1">Codec</label>
                    <select 
                      id="stream-codec" 
                      name="codec"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${currentStream.codec}
                      onChange=${handleInputChange}
                    >
                      <option value="h264">H.264</option>
                      <option value="h265">H.265</option>
                    </select>
                  </div>
                  <div class="form-group">
                    <label for="stream-protocol" class="block text-sm font-medium mb-1">Protocol</label>
                    <select 
                      id="stream-protocol" 
                      name="protocol"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${currentStream.protocol}
                      onChange=${handleInputChange}
                    >
                      <option value="0">TCP</option>
                      <option value="1">UDP</option>
                      <option value="2">ONVIF</option>
                    </select>
                    <span class="text-xs text-gray-500 dark:text-gray-400">Connection protocol</span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="stream-priority" class="block text-sm font-medium mb-1">Priority</label>
                    <select 
                      id="stream-priority" 
                      name="priority"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${currentStream.priority}
                      onChange=${handleInputChange}
                    >
                      <option value="1">Low (1)</option>
                      <option value="5">Medium (5)</option>
                      <option value="10">High (10)</option>
                    </select>
                  </div>
                  <div class="form-group">
                    <label for="stream-segment" class="block text-sm font-medium mb-1">Segment Duration (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-segment" 
                      name="segment"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="60" 
                      max="3600" 
                      value=${currentStream.segment}
                      onChange=${handleInputChange}
                    />
                  </div>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-record" 
                    name="record"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${currentStream.record}
                    onChange=${handleInputChange}
                  />
                  <label for="stream-record" class="ml-2 block text-sm">Record</label>
                </div>
                
                <!-- Detection-based recording options -->
                <div class="mt-6 mb-2 pb-1 border-b border-gray-200 dark:border-gray-700">
                  <h4 class="text-md font-medium">Detection-Based Recording</h4>
                </div>
                <div class="form-group flex items-center">
                  <input 
                    type="checkbox" 
                    id="stream-detection-enabled" 
                    name="detectionEnabled"
                    class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                    checked=${currentStream.detectionEnabled}
                    onChange=${handleInputChange}
                  />
                  <label for="stream-detection-enabled" class="ml-2 block text-sm">Enable Detection-Based Recording</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Only record when objects are detected</span>
                </div>
                <div class="form-group" style=${currentStream.detectionEnabled ? '' : 'display: none'}>
                  <label for="stream-detection-model" class="block text-sm font-medium mb-1">Detection Model</label>
                  <div class="flex space-x-2">
                    <select 
                      id="stream-detection-model" 
                      name="detectionModel"
                      class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      value=${currentStream.detectionModel}
                      onChange=${handleInputChange}
                    >
                      <option value="">Select a model</option>
                      ${detectionModels.map(model => html`
                        <option key=${model.id} value=${model.id}>${model.name}</option>
                      `)}
                    </select>
                    <button 
                      id="refresh-models-btn" 
                      class="p-2 rounded-md bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
                      title="Refresh Models"
                      onClick=${loadDetectionModels}
                      type="button"
                    >
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                <div class="form-group" style=${currentStream.detectionEnabled ? '' : 'display: none'}>
                  <label for="stream-detection-threshold" class="block text-sm font-medium mb-1">Detection Threshold</label>
                  <div class="flex items-center space-x-2">
                    <input 
                      type="range" 
                      id="stream-detection-threshold" 
                      name="detectionThreshold"
                      class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700"
                      min="0" 
                      max="100" 
                      step="1" 
                      value=${currentStream.detectionThreshold}
                      onChange=${handleThresholdChange}
                    />
                    <span id="stream-threshold-value" class="font-medium text-blue-600 dark:text-blue-400 min-w-[3rem] text-center">
                      ${currentStream.detectionThreshold}%
                    </span>
                  </div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4" style=${currentStream.detectionEnabled ? '' : 'display: none'}>
                  <div class="form-group">
                    <label for="stream-detection-interval" class="block text-sm font-medium mb-1">Detection Interval (frames)</label>
                    <input 
                      type="number" 
                      id="stream-detection-interval" 
                      name="detectionInterval"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="1" 
                      max="100" 
                      value=${currentStream.detectionInterval}
                      onChange=${handleInputChange}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Detect on every Nth frame</span>
                  </div>
                  <div class="form-group">
                    <label for="stream-pre-buffer" class="block text-sm font-medium mb-1">Pre-detection Buffer (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-pre-buffer" 
                      name="preBuffer"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="0" 
                      max="60" 
                      value=${currentStream.preBuffer}
                      onChange=${handleInputChange}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Seconds to keep before detection</span>
                  </div>
                  <div class="form-group">
                    <label for="stream-post-buffer" class="block text-sm font-medium mb-1">Post-detection Buffer (seconds)</label>
                    <input 
                      type="number" 
                      id="stream-post-buffer" 
                      name="postBuffer"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      min="0" 
                      max="300" 
                      value=${currentStream.postBuffer}
                      onChange=${handleInputChange}
                    />
                    <span class="text-xs text-gray-500 dark:text-gray-400">Seconds to keep after detection</span>
                  </div>
                </div>
              </form>
            </div>
            <div class="flex justify-between p-4 border-t border-gray-200 dark:border-gray-700">
              <button 
                id="stream-test-btn" 
                class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${testStreamConnection}
                type="button"
              >
                Test Connection
              </button>
              <div class="space-x-2">
                <button 
                  id="stream-save-btn" 
                  class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                  onClick=${saveStream}
                  type="button"
                >
                  Save
                </button>
                <button 
                  id="stream-cancel-btn" 
                  class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                  onClick=${closeModal}
                  type="button"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      `}
      
      ${onvifModalVisible && html`
        <div id="onvif-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">ONVIF Camera Discovery</h3>
              <span class="text-2xl cursor-pointer" onClick=${() => setOnvifModalVisible(false)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4">
                <h4 class="text-md font-medium mb-2">Authentication</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="form-group">
                    <label for="onvif-username" class="block text-sm font-medium mb-1">Username</label>
                    <input 
                      type="text" 
                      id="onvif-username" 
                      name="username"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      placeholder="admin" 
                      value=${onvifCredentials.username}
                      onChange=${handleCredentialChange}
                    />
                  </div>
                  <div class="form-group">
                    <label for="onvif-password" class="block text-sm font-medium mb-1">Password</label>
                    <input 
                      type="password" 
                      id="onvif-password" 
                      name="password"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                      placeholder="password" 
                      value=${onvifCredentials.password}
                      onChange=${handleCredentialChange}
                    />
                  </div>
                </div>
              </div>
              
              <div class="mb-4 flex justify-between items-center">
                <h4 class="text-md font-medium">Discovered Devices</h4>
                <button 
                  id="discover-btn" 
                  class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                  onClick=${startOnvifDiscovery}
                  disabled=${isDiscovering}
                  type="button"
                >
                  ${isDiscovering ? html`
                    <span class="flex items-center">
                      Discovering
                      <span class="ml-1 flex space-x-1">
                        <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                      </span>
                    </span>
                  ` : 'Start Discovery'}
                </button>
              </div>
              
              <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                  <thead class="bg-gray-50 dark:bg-gray-700">
                    <tr>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">IP Address</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Manufacturer</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Model</th>
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
                    </tr>
                  </thead>
                  <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                    ${discoveredDevices.length === 0 ? html`
                      <tr>
                        <td colspan="4" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                          ${isDiscovering ? html`
                            <div class="flex items-center justify-center">
                              <span>Discovering devices</span>
                              <span class="ml-1 flex space-x-1">
                                <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                                <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                                <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-gray-500 dark:bg-gray-400 rounded-full"></span>
                              </span>
                            </div>
                          ` : 'No devices discovered yet. Click "Start Discovery" to scan your network.'}
                        </td>
                      </tr>
                    ` : discoveredDevices.map(device => html`
                      <tr key=${device.ip_address} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td class="px-6 py-4 whitespace-nowrap">${device.ip_address}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${device.manufacturer || 'Unknown'}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${device.model || 'Unknown'}</td>
                        <td class="px-6 py-4 whitespace-nowrap">
                          <button 
                            class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors focus:outline-none"
                            onClick=${() => testOnvifConnection(device)}
                            disabled=${isLoadingProfiles && selectedDevice && selectedDevice.ip_address === device.ip_address}
                            type="button"
                          >
                            ${isLoadingProfiles && selectedDevice && selectedDevice.ip_address === device.ip_address ? html`
                              <span class="flex items-center">
                                Loading
                                <span class="ml-1 flex space-x-1">
                                  <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                                </span>
                              </span>
                            ` : 'Connect'}
                          </button>
                        </td>
                      </tr>
                    `)}
                  </tbody>
                </table>
              </div>
              
              ${selectedDevice && deviceProfiles.length > 0 && html`
                <div class="mt-6">
                  <h4 class="text-md font-medium mb-2">Available Profiles for ${selectedDevice.ip_address}</h4>
                  <div class="overflow-x-auto">
                    <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                      <thead class="bg-gray-50 dark:bg-gray-700">
                        <tr>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Name</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Resolution</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Encoding</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">FPS</th>
                          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">Actions</th>
                        </tr>
                      </thead>
                      <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                        ${deviceProfiles.map(profile => html`
                          <tr key=${profile.token} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                            <td class="px-6 py-4 whitespace-nowrap">${profile.name}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${profile.width}x${profile.height}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${profile.encoding}</td>
                            <td class="px-6 py-4 whitespace-nowrap">${profile.fps}</td>
                            <td class="px-6 py-4 whitespace-nowrap">
                              <button 
                                class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors focus:outline-none"
                                onClick=${() => addOnvifDeviceAsStream(profile)}
                                type="button"
                              >
                                Add as Stream
                              </button>
                            </td>
                          </tr>
                        `)}
                      </tbody>
                    </table>
                  </div>
                </div>
              `}
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700">
              <button 
                id="onvif-close-btn" 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                onClick=${() => setOnvifModalVisible(false)}
                type="button"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      `}
      
      ${showCustomNameInput && html`
        <div id="custom-name-modal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
            <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
              <h3 class="text-lg font-medium">Stream Name</h3>
              <span class="text-2xl cursor-pointer" onClick=${() => setShowCustomNameInput(false)}>×</span>
            </div>
            <div class="p-4">
              <div class="mb-4">
                <label for="custom-stream-name" class="block text-sm font-medium mb-1">Enter a name for this stream:</label>
                <input 
                  type="text" 
                  id="custom-stream-name" 
                  class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  value=${customStreamName}
                  onChange=${(e) => setCustomStreamName(e.target.value)}
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  This name will be used to identify the stream in the system.
                </p>
              </div>
            </div>
            <div class="flex justify-end p-4 border-t border-gray-200 dark:border-gray-700 space-x-2">
              <button 
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                onClick=${() => setShowCustomNameInput(false)}
                type="button"
              >
                Cancel
              </button>
              <button 
                class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                onClick=${submitOnvifDevice}
                type="button"
                disabled=${!customStreamName.trim() || isAddingStream}
              >
                ${isAddingStream ? html`
                  <span class="flex items-center">
                    Adding
                    <span class="ml-1 flex space-x-1">
                      <span class="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span class="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span class="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                    </span>
                  </span>
                ` : 'Add Stream'}
              </button>
            </div>
          </div>
        </div>
      `}
    </section>
  `;
}

/**
 * Load StreamsView component
 */
export function loadStreamsView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the StreamsView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${StreamsView} />`, mainContent);
  });
}
