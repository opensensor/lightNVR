/**
 * LightNVR Web Interface StreamsView Component
 * Preact component for the streams page
 */

import { h } from 'preact';
import { html } from '../../html-helper.js';
import { useState, useEffect, useRef } from 'preact/hooks';
import { showStatusMessage } from './UI.js';
import { ContentLoader } from './LoadingIndicator.js';
import { StreamDeleteModal } from './StreamDeleteModal.js';
import {
  useQuery,
  useMutation,
  useQueryClient,
  fetchJSON
} from '../../query-client.js';

/**
 * StreamsView component
 * @returns {JSX.Element} StreamsView component
 */
export function StreamsView() {
  const queryClient = useQueryClient();

  // State for streams data
  const [modalVisible, setModalVisible] = useState(false);
  const [onvifModalVisible, setOnvifModalVisible] = useState(false);
  const [showCustomNameInput, setShowCustomNameInput] = useState(false);
  const [isAddingStream, setIsAddingStream] = useState(false);
  const [discoveredDevices, setDiscoveredDevices] = useState([]);
  const [deviceProfiles, setDeviceProfiles] = useState([]);
  const [selectedDevice, setSelectedDevice] = useState(null);
  const [selectedProfile, setSelectedProfile] = useState(null);
  const [customStreamName, setCustomStreamName] = useState('');
  const [onvifCredentials, setOnvifCredentials] = useState({ username: '', password: '' });
  const [isDiscovering, setIsDiscovering] = useState(false);
  const [isLoadingProfiles, setIsLoadingProfiles] = useState(false);

  // Fetch streams data
  const {
    data: streamsResponse = [],
    isLoading,
    error
  } = useQuery(['streams'], '/api/streams', {
    timeout: 10000,
    retries: 2,
    retryDelay: 1000
  });

  // Fetch detection models
  const {
    data: detectionModelsData
  } = useQuery(['detectionModels'], '/api/detection/models', {
    timeout: 5000,
    retries: 1,
    retryDelay: 1000
  });

  // Process the response to handle both array and object formats
  const streams = Array.isArray(streamsResponse) ? streamsResponse : (streamsResponse.streams || []);

  // Log streams data for debugging
  console.log('API Response:', streamsResponse);

  // Log streams data for debugging
  console.log('Streams data:', streams);
  console.log('hasData:', streams && streams.length > 0);

  // Default stream state
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
    recordAudio: true,
    detectionEnabled: false,
    detectionModel: '',
    detectionThreshold: 50,
    detectionInterval: 10,
    preBuffer: 10,
    postBuffer: 30
  });
  const [isEditing, setIsEditing] = useState(false);

  // State for delete modal
  const [deleteModalVisible, setDeleteModalVisible] = useState(false);
  const [streamToDelete, setStreamToDelete] = useState(null);

  // Compute hasData from streams
  const hasData = streams && streams.length > 0;

  // Extract detection models from query result
  const detectionModels = detectionModelsData?.models || [];

  // Mutations for saving stream (create or update)
  const createStreamMutation = useMutation({
    mutationFn: async (streamData) => {
      return await fetchJSON('/api/streams', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData),
        timeout: 15000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream added successfully');
      closeModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error adding stream: ${error.message}`, 5000, 'error');
    }
  });

  const updateStreamMutation = useMutation({
    mutationFn: async (streamData) => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamData.name)}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData),
        timeout: 15000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream updated successfully');
      closeModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error updating stream: ${error.message}`, 5000, 'error');
    }
  });

  // Helper function to use the appropriate mutation based on editing state
  const saveStreamMutation = {
    mutate: (streamData, options) => {
      if (isEditing) {
        updateStreamMutation.mutate(streamData, options);
      } else {
        createStreamMutation.mutate(streamData, options);
      }
    }
  };

  // Mutation for testing stream connection
  const testStreamMutation = useMutation({
    mutationFn: async (streamData) => {
      showStatusMessage('Testing stream connection...');

      return await fetchJSON('/api/streams/test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          url: streamData.url,
          protocol: parseInt(streamData.protocol, 10)
        }),
        timeout: 20000,
        retries: 1,
        retryDelay: 2000
      });
    },
    onSuccess: (data) => {
      if (data.success) {
        showStatusMessage('Stream connection successful!', 3000, 'success');
      } else {
        showStatusMessage(`Stream connection failed: ${data.message}`, 5000, 'error');
      }
    },
    onError: (error) => {
      showStatusMessage(`Error testing stream: ${error.message}`, 5000, 'error');
    }
  });

  // Mutation for deleting stream
  const deleteStreamMutation = useMutation({
    mutationFn: async (streamId) => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'DELETE',
        timeout: 10000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream deleted successfully');
      closeDeleteModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error deleting stream: ${error.message}`, 5000, 'error');
    }
  });

  // Mutation for toggling stream enabled state
  const toggleStreamEnabledMutation = useMutation({
    mutationFn: async ({ streamId, enabled }) => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}/enable`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ enabled: !enabled }),
        timeout: 10000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: (_, variables) => {
      showStatusMessage(`Stream ${variables.enabled ? 'disabled' : 'enabled'} successfully`);
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error toggling stream state: ${error.message}`, 5000, 'error');
    }
  });

  // Handle form submission
  const handleSubmit = (e) => {
    e.preventDefault();

    // Prepare stream data for API
    const streamData = {
      name: currentStream.name,
      url: currentStream.url,
      enabled: currentStream.enabled,
      streaming_enabled: currentStream.streamingEnabled,
      width: parseInt(currentStream.width, 10),
      height: parseInt(currentStream.height, 10),
      fps: parseInt(currentStream.fps, 10),
      codec: currentStream.codec,
      protocol: parseInt(currentStream.protocol, 10),
      priority: parseInt(currentStream.priority, 10),
      segment_duration: parseInt(currentStream.segment, 10),
      record: currentStream.record,
      detection_based_recording: currentStream.detectionEnabled,
      detection_model: currentStream.detectionModel,
      detection_threshold: parseInt(currentStream.detectionThreshold, 10),
      detection_interval: parseInt(currentStream.detectionInterval, 10),
      pre_detection_buffer: parseInt(currentStream.preBuffer, 10),
      post_detection_buffer: parseInt(currentStream.postBuffer, 10),
      record_audio: currentStream.recordAudio
    };

    // When editing, set is_deleted to false to allow undeleting soft-deleted streams
    if (isEditing) {
      streamData.is_deleted = false;
    }

    // Use mutation to save stream
    saveStreamMutation.mutate(streamData);
  };

  // Save stream (wrapper for handleSubmit to use in onClick)
  const saveStream = (e) => {
    handleSubmit(e);
  };

  // Test stream connection
  const testStreamConnection = () => {
    testStreamMutation.mutate(currentStream);
  };

  // Open delete modal
  const openDeleteModal = (stream) => {
    setStreamToDelete(stream);
    setDeleteModalVisible(true);
  };

  // Close delete modal
  const closeDeleteModal = () => {
    setDeleteModalVisible(false);
    setStreamToDelete(null);
  };

  // Handle delete confirmation
  const confirmDelete = () => {
    if (streamToDelete) {
      deleteStreamMutation.mutate(streamToDelete.name);
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
      recordAudio: true,
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
      // Use queryClient to fetch stream details
      const stream = await queryClient.fetchQuery({
        queryKey: ['stream', streamId],
        queryFn: async () => {
          const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`);
          if (!response.ok) {
            throw new Error(`HTTP error ${response.status}`);
          }
          return response.json();
        },
        staleTime: 10000 // 10 seconds
      });

      setCurrentStream({
        ...stream,
        // Convert numeric values to strings for form inputs
        width: stream.width || 1280,
        height: stream.height || 720,
        fps: stream.fps || 15,
        protocol: stream.protocol?.toString() || '0',
        priority: stream.priority?.toString() || '5',
        segment: stream.segment_duration || 30,
        detectionThreshold: stream.detection_threshold || 50,
        detectionInterval: stream.detection_interval || 10,
        preBuffer: stream.pre_detection_buffer || 10,
        postBuffer: stream.post_detection_buffer || 30,
        // Map API fields to form fields
        streamingEnabled: stream.streaming_enabled !== undefined ? stream.streaming_enabled : true,
        isOnvif: stream.is_onvif !== undefined ? stream.is_onvif : false,
        detectionEnabled: stream.detection_based_recording || false,
        detectionModel: stream.detection_model || '',
        recordAudio: stream.record_audio !== undefined ? stream.record_audio : true
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

  // Open ONVIF discovery modal
  const openOnvifModal = () => {
    setDiscoveredDevices([]);
    setDeviceProfiles([]);
    setSelectedDevice(null);
    setSelectedProfile(null);
    setCustomStreamName('');
    setOnvifModalVisible(true);
  };

  // Close ONVIF modal
  const closeOnvifModal = () => {
    setOnvifModalVisible(false);
  };

  // Handle form input change
  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    setCurrentStream(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  // Disable stream (soft delete)
  const disableStream = (streamId) => {
    disableStreamMutation.mutate(streamId);
  };

  // Delete stream (permanent)
  const deleteStream = (streamId) => {
    deleteStreamMutation.mutate(streamId);
  };

  // Enable/disable stream
  const toggleStreamEnabled = (streamId, enabled) => {
    toggleStreamEnabledMutation.mutate({ streamId, enabled });
  };

  // Handle ONVIF credential input change
  const handleCredentialChange = (e) => {
    const { name, value } = e.target;
    setOnvifCredentials(prev => ({
      ...prev,
      [name]: value
    }));
  };

  // Handle threshold change
  const handleThresholdChange = (e) => {
    const value = e.target.value;
    setCurrentStream(prev => ({
      ...prev,
      detectionThreshold: value
    }));
  };

  // Load detection models
  const loadDetectionModels = () => {
    queryClient.invalidateQueries({ queryKey: ['detectionModels'] });
  };

  // ONVIF discovery mutation
  const onvifDiscoveryMutation = useMutation({
    mutationFn: async () => {
      setIsDiscovering(true);
      return await fetchJSON('/api/onvif/discovery/discover', {
        method: 'POST',
        timeout: 120000,
        retries: 0,
        body: JSON.stringify({})
      });
    },
    onSuccess: (data) => {
      setDiscoveredDevices(data.devices || []);
      setIsDiscovering(false);
    },
    onError: (error) => {
      showStatusMessage(`Error discovering ONVIF devices: ${error.message}`, 5000, 'error');
      setIsDiscovering(false);
    }
  });

  // Get device profiles mutation
  const getDeviceProfilesMutation = useMutation({
    mutationFn: async ({ device, credentials }) => {
      setIsLoadingProfiles(true);
      return await fetchJSON('/api/onvif/device/profiles', {
        method: 'GET',
        headers: {
          'X-Device-URL': `http://${device.ip_address}/onvif/device_service`,
          'X-Username': credentials.username,
          'X-Password': credentials.password
        },
        timeout: 20000,
        retries: 0
      });
    },
    onSuccess: (data) => {
      setDeviceProfiles(data.profiles || []);
      setIsLoadingProfiles(false);
    },
    onError: (error) => {
      showStatusMessage(`Error loading device profiles: ${error.message}`, 5000, 'error');
      setIsLoadingProfiles(false);
    }
  });

  // Test ONVIF connection mutation
  const testOnvifConnectionMutation = useMutation({
    mutationFn: async (device) => {
      setIsLoadingProfiles(true);
      return await fetchJSON('/api/onvif/device/test', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          url: `http://${device.ip_address}/onvif/device_service`,
          username: onvifCredentials.username,
          password: onvifCredentials.password
        }),
        timeout: 15000,
        retries: 0
      });
    },
    onSuccess: (data, variables) => {
      if (data.success) {
        showStatusMessage('Connection successful!', 3000, 'success');
        getDeviceProfiles(variables);
      } else {
        showStatusMessage(`Connection failed: ${data.message}`, 5000, 'error');
        setIsLoadingProfiles(false);
      }
    },
    onError: (error) => {
      showStatusMessage(`Error testing connection: ${error.message}`, 5000, 'error');
      setIsLoadingProfiles(false);
    }
  });

  // Disable stream mutation
  const disableStreamMutation = useMutation({
    mutationFn: async (streamId) => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}/disable`, {
        method: 'POST',
        timeout: 10000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream disabled successfully');
      closeDeleteModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error disabling stream: ${error.message}`, 5000, 'error');
    }
  });

  // Submit ONVIF device
  const submitOnvifDevice = () => {
    if (!selectedDevice || !selectedProfile || !customStreamName.trim()) {
      showStatusMessage('Missing required information', 5000, 'error');
      return;
    }

    setIsAddingStream(true);

    // Prepare stream data
    const streamData = {
      name: customStreamName.trim(),
      url: selectedProfile.stream_uri, // Use stream_uri instead of url
      enabled: true,
      streaming_enabled: true,
      width: selectedProfile.width,
      height: selectedProfile.height,
      fps: selectedProfile.fps || 15,
      codec: selectedProfile.encoding === 'H264' ? 'h264' : 'h265',
      protocol: '0', // TCP
      priority: '5', // Medium
      segment_duration: 30,
      record: true,
      record_audio: true,
      is_onvif: true
    };

    // Log the stream data for debugging
    console.log('Adding ONVIF stream with data:', streamData);

    // Use mutation to save stream
    saveStreamMutation.mutate(streamData, {
      onSuccess: () => {
        setIsAddingStream(false);
        setShowCustomNameInput(false);
        setOnvifModalVisible(false);
      },
      onError: () => {
        setIsAddingStream(false);
      },
      isEditing: false,
    });
  };

  // Start ONVIF discovery
  const startOnvifDiscovery = () => {
    onvifDiscoveryMutation.mutate();
  };

  // Get ONVIF device profiles
  const getDeviceProfiles = (device) => {
    setSelectedDevice(device);
    setDeviceProfiles([]);
    getDeviceProfilesMutation.mutate({
      device,
      credentials: onvifCredentials
    });
  };

  // Handle custom stream name change
  const handleStreamNameChange = (e) => {
    setCustomStreamName(e.target.value);
  };

  // Add ONVIF device as stream with selected profile
  const addOnvifDeviceAsStream = (profile) => {
    setSelectedProfile(profile);
    setCustomStreamName(`${selectedDevice.name || 'ONVIF'}_${profile.name || 'Stream'}`);
    setShowCustomNameInput(true);
  };

  // Test ONVIF connection
  const testOnvifConnection = (device) => {
    testOnvifConnectionMutation.mutate(device);
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

      <${ContentLoader}
          isLoading=${isLoading}
          hasData=${hasData}
          loadingMessage="Loading streams..."
          emptyMessage="No streams configured yet. Click 'Add Stream' to create one."
      >
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
              ${streams.map(stream => html`
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
                          onClick=${() => openDeleteModal(stream)}
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
      <//>

      ${deleteModalVisible && streamToDelete && html`
        <${StreamDeleteModal}
            streamId=${streamToDelete.name}
            streamName=${streamToDelete.name}
            onClose=${closeDeleteModal}
            onDisable=${disableStream}
            onDelete=${deleteStream}
        />
      `}

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
                  <label for="stream-enabled" class="ml-2 block text-sm">Stream Active</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable all stream processing</span>
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
                  <label for="stream-streaming-enabled" class="ml-2 block text-sm">Live View Enabled</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Enable/disable live viewing in browser</span>
                </div>
                <div class="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-is-onvif"
                      name="isOnvif"
                      class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                      checked=${currentStream.isOnvif}
                      onChange=${handleInputChange}
                  />
                  <label for="stream-is-onvif" class="ml-2 block text-sm">ONVIF Camera</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Mark this stream as an ONVIF camera for special handling</span>
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
                    </select>
                    <span class="text-xs text-gray-500 dark:text-gray-400">Connection protocol (ONVIF cameras use either TCP or UDP)</span>
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
                <div class="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-record-audio"
                      name="recordAudio"
                      class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                      checked=${currentStream.recordAudio}
                      onChange=${handleInputChange}
                  />
                  <label for="stream-record-audio" class="ml-2 block text-sm">Record Audio</label>
                  <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">Include audio in recordings if available in the stream</span>
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
                        onInput=${handleThresholdChange}
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

              <div class="mt-6 mb-4">
                <h4 class="text-md font-medium mb-2">Authentication</h4>
                <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
                  Enter credentials to connect to the selected ONVIF device. Credentials are not needed for discovery, only for connecting to devices.
                </p>
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
  import('preact').then(({ render }) => {
    import('../../query-client.js').then(({ QueryClientProvider, queryClient }) => {
      render(
        html`<${QueryClientProvider} client=${queryClient}><${StreamsView} /></${QueryClientProvider}>`, 
        mainContent
      );
    });
  });
}