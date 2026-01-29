/**
 * LightNVR Web Interface StreamsView Component
 * React component for the streams page
 */

import { useState, useEffect } from 'react';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { StreamDeleteModal } from './StreamDeleteModal.jsx';
import { StreamConfigModal } from './StreamConfigModal.jsx';
import { validateSession } from '../../utils/auth-utils.js';
import {
  useQuery,
  useMutation,
  useQueryClient,
  usePostMutation,
  fetchJSON
} from '../../query-client.js';

/**
 * StreamsView component
 * @returns {JSX.Element} StreamsView component
 */
export function StreamsView() {
  const queryClient = useQueryClient();

  // User role state for permission-based UI
  const [userRole, setUserRole] = useState(null);

  // Fetch user role on mount
  useEffect(() => {
    const fetchUserRole = async () => {
      try {
        const result = await validateSession();
        if (result.valid && result.role) {
          setUserRole(result.role);
        } else {
          // Session invalid - set to empty string to indicate fetch completed
          setUserRole('');
          console.log('Session validation failed, no role');
        }
      } catch (error) {
        console.error('Error fetching user role:', error);
        setUserRole('');
      }
    };
    fetchUserRole();
  }, []);

  // Role is still loading if null
  const roleLoading = userRole === null;
  // Check if user can modify streams (admin or user role, not viewer)
  // While loading, default to enabled so admin/user doesn't see hidden buttons
  const canModifyStreams = roleLoading || userRole === 'admin' || userRole === 'user';

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

  // Accordion expanded state for StreamConfigModal sections
  const [expandedSections, setExpandedSections] = useState({
    basic: true,
    recording: false,
    detection: false,
    zones: false,
    motion: false,
    ptz: false,
    advanced: false
  });

  const toggleSection = (section) => {
    setExpandedSections(prev => ({
      ...prev,
      [section]: !prev[section]
    }));
  };

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
    backchannelEnabled: false,
    // ONVIF capability flag
    isOnvif: false,
    // ONVIF credentials (for cameras that require authentication)
    onvifUsername: '',
    onvifPassword: '',
    onvifProfile: '',
    // AI Detection recording
    detectionEnabled: false,
    detectionModel: '',
    detectionThreshold: 50,
    detectionInterval: 10,
    preBuffer: 10,
    postBuffer: 30,
    detectionZones: [],
    // Motion (ONVIF) recording defaults
    motionRecordingEnabled: false,
    motionPreBuffer: 5,
    motionPostBuffer: 10,
    motionMaxDuration: 300,
    motionRetentionDays: 7,
    motionCodec: 'h264',
    motionQuality: 'medium',
    // PTZ control settings
    ptzEnabled: false,
    ptzMaxX: 0,
    ptzMaxY: 0,
    ptzMaxZ: 0,
    ptzHasHome: false,
    // Retention policy settings
    retentionDays: 0,
    detectionRetentionDays: 0,
    maxStorageMb: 0
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
  const createStreamMutation = usePostMutation(
    '/api/streams',

    {
      timeout: 15000,
      retries: 1,
      retryDelay: 1000
    },
    {
      onSuccess: () => {
        showStatusMessage('Stream added successfully');
        closeModal();
        // Invalidate and refetch streams data
        queryClient.invalidateQueries({ queryKey: ['streams'] });
      },
      onError: (error) => {
        showStatusMessage(`Error adding stream: ${error.message}`, 'error', 5000);
      }
    }
  );

  const updateStreamMutation = useMutation({
    mutationFn: async (data) => {
      const { streamName, ...streamData } = data;
      const url = `/api/streams/${encodeURIComponent(streamName)}`;
      return await fetchJSON(url, {
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
    onSuccess: (_data, variables) => {
      showStatusMessage('Stream updated successfully');
      // Evict per-stream cache so next open fetches fresh data
      if (variables?.streamName) {
        queryClient.invalidateQueries({ queryKey: ['stream-full', variables.streamName] });
        // Also remove to avoid returning cached data before refetch completes
        if (typeof queryClient.removeQueries === 'function') {
          queryClient.removeQueries({ queryKey: ['stream-full', variables.streamName], exact: true });
        }
      }
      closeModal();
      // Invalidate and refetch streams list
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(`Error updating stream: ${error.message}`, 'error', 5000);
    }
  });

  // Helper function to use the appropriate mutation based on editing state
  const saveStreamMutation = {
    mutate: (streamData, options) => {
      if (isEditing) {
        // For PUT requests, we need to include the streamName parameter
        updateStreamMutation.mutate({
          ...streamData,
          streamName: streamData.name
        }, options);
      } else {
        createStreamMutation.mutate(streamData, options);
      }
    }
  };

  // Mutation for testing stream connection
  const testStreamMutation = usePostMutation(
    '/api/streams/test',
    {
      timeout: 20000,
      retries: 1,
      retryDelay: 2000
    },
    {
      onMutate: () => {
        showStatusMessage('Testing stream connection...');
      },
      onSuccess: (data) => {
        if (data.success) {
          showStatusMessage('Stream connection successful!', 'success', 3000);
        } else {
          showStatusMessage(`Stream connection failed: ${data.message}`, 'error', 5000);
        }
      },
      onError: (error) => {
        showStatusMessage(`Error testing stream: ${error.message}`, 'error', 5000);
      }
    }
  );

  // Mutation for deleting stream
  const deleteStreamMutation = useMutation({
    mutationFn: async (params) => {
      const { streamId } = params;
      const url = `/api/streams/${encodeURIComponent(streamId)}?permanent=true`;
      return await fetchJSON(url, {
        method: 'DELETE',
        timeout: 15000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream successfully deleted.');
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] })
        .then(() => {
          // Close the modal after the query invalidation is complete
          closeDeleteModal();
        });
    },
    onError: (error) => {
      showStatusMessage(`Error deleting stream: ${error.message}`, 'error', 5000);
    }
  });

  const disableStreamMutation = useMutation({
    mutationFn: async (params) => {
      const { streamId } = params;
      const url = `/api/streams/${encodeURIComponent(streamId)}`;
      return await fetchJSON(url, {
        method: 'DELETE',
        timeout: 15000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage('Stream successfully disabled.');
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] })
        .then(() => {
          // Close the modal after the query invalidation is complete
          closeDeleteModal();
        });
    },
    onError: (error) => {
      showStatusMessage(`Error disabling stream: ${error.message}`, 'error', 5000);
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
      // Persist ONVIF flag expected by backend (camelCase key)
      isOnvif: !!currentStream.isOnvif,
      // ONVIF credentials (for motion detection, PTZ, etc.)
      onvif_username: currentStream.onvifUsername || '',
      onvif_password: currentStream.onvifPassword || '',
      onvif_profile: currentStream.onvifProfile || '',
      detection_threshold: parseInt(currentStream.detectionThreshold, 10),
      detection_interval: parseInt(currentStream.detectionInterval, 10),
      pre_detection_buffer: parseInt(currentStream.preBuffer, 10),
      post_detection_buffer: parseInt(currentStream.postBuffer, 10),
      record_audio: currentStream.recordAudio,
      backchannel_enabled: currentStream.backchannelEnabled,
      // PTZ control settings
      ptz_enabled: !!currentStream.ptzEnabled,
      ptz_max_x: parseInt(currentStream.ptzMaxX, 10) || 0,
      ptz_max_y: parseInt(currentStream.ptzMaxY, 10) || 0,
      ptz_max_z: parseInt(currentStream.ptzMaxZ, 10) || 0,
      ptz_has_home: !!currentStream.ptzHasHome,
      // Retention policy settings
      retention_days: parseInt(currentStream.retentionDays, 10) || 0,
      detection_retention_days: parseInt(currentStream.detectionRetentionDays, 10) || 0,
      max_storage_mb: parseInt(currentStream.maxStorageMb, 10) || 0
    };

    // When editing, set is_deleted to false to allow undeleting soft-deleted streams
    if (isEditing) {
      streamData.is_deleted = false;
    }

    // Use mutation to save stream and then handle motion config if applicable
    saveStreamMutation.mutate(streamData, {
      onSuccess: async () => {
        try {
          if (currentStream.isOnvif) {
            const motionUrl = `/api/motion/config/${encodeURIComponent(currentStream.name)}`;
            if (currentStream.motionRecordingEnabled) {
              const body = {
                enabled: true,
                pre_buffer_seconds: parseInt(currentStream.motionPreBuffer, 10),
                post_buffer_seconds: parseInt(currentStream.motionPostBuffer, 10),
                max_file_duration: parseInt(currentStream.motionMaxDuration, 10),
                codec: currentStream.motionCodec,
                quality: currentStream.motionQuality,
                retention_days: parseInt(currentStream.motionRetentionDays, 10)
              };
              await fetchJSON(motionUrl, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(body),
                timeout: 15000
              });
            } else {
              // Delete motion config if disabling
              await fetchJSON(motionUrl, {
                method: 'DELETE',
                timeout: 15000
              });
            }
          }
        } catch (err) {
          showStatusMessage(`Motion config save failed: ${err.message}`, 'error', 5000);
        }
        // Ensure both list and details are refreshed after save
        await queryClient.invalidateQueries({ queryKey: ['stream-full', currentStream.name] });
        queryClient.invalidateQueries({ queryKey: ['streams'] });
      }
    });
  };

  // Save stream (wrapper for handleSubmit to use in onClick)
  const saveStream = (e) => {
    handleSubmit(e);
  };

  // Test stream connection
  const testStreamConnection = () => {
    testStreamMutation.mutate({
      url: currentStream.url,
      protocol: parseInt(currentStream.protocol, 10)
    });
  };

  // Trigger a simulated ONVIF motion event for the current stream
  const triggerTestMotionEvent = async () => {
    if (!currentStream?.name) {
      showStatusMessage('Please set a stream name and save before testing motion.', 'error', 5000);
      return;
    }
    if (!currentStream?.isOnvif) {
      showStatusMessage('Enable "ONVIF Camera" in Basic Settings first.', 'error', 5000);
      return;
    }
    try {
      // Ensure the server has motion recording enabled with current UI settings
      if (currentStream.motionRecordingEnabled) {
        const motionUrl = `/api/motion/config/${encodeURIComponent(currentStream.name)}`;
        const body = {
          enabled: true,
          pre_buffer_seconds: parseInt(currentStream.motionPreBuffer, 10),
          post_buffer_seconds: parseInt(currentStream.motionPostBuffer, 10),
          max_file_duration: parseInt(currentStream.motionMaxDuration, 10),
          codec: currentStream.motionCodec,
          quality: currentStream.motionQuality,
          retention_days: parseInt(currentStream.motionRetentionDays, 10)
        };
        await fetchJSON(motionUrl, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
          timeout: 10000
        });
      }

      // Now fire the test motion event
      const data = await fetchJSON(`/api/motion/test/${encodeURIComponent(currentStream.name)}`, {
        method: 'POST',
        timeout: 15000
      });
      if (data?.success) {
        showStatusMessage('Test motion event triggered successfully.', 'success', 3000);
      } else {
        showStatusMessage(`Test motion event failed: ${data?.message || 'Unknown error'}`, 'error', 5000);
      }
    } catch (err) {
      showStatusMessage(`Error triggering test motion: ${err.message}`, 'error', 5000);
    }
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
      backchannelEnabled: false,
      isOnvif: false,
      onvifUsername: '',
      onvifPassword: '',
      onvifProfile: '',
      detectionEnabled: false,
      detectionModel: '',
      detectionThreshold: 50,
      detectionInterval: 10,
      preBuffer: 10,
      postBuffer: 30,
      motionRecordingEnabled: false,
      motionPreBuffer: 5,
      motionPostBuffer: 10,
      motionMaxDuration: 300,
      motionRetentionDays: 7,
      motionCodec: 'h264',
      motionQuality: 'medium',
      ptzEnabled: false,
      ptzMaxX: 0,
      ptzMaxY: 0,
      ptzMaxZ: 0,
      ptzHasHome: false,
      retentionDays: 0,
      detectionRetentionDays: 0,
      maxStorageMb: 0
    });
    setIsEditing(false);
    setModalVisible(true);
  };

  // Open edit stream modal
  const openEditStreamModal = async (streamId) => {
    try {
      // Ensure any cached details are marked stale before fetching
      await queryClient.invalidateQueries({ queryKey: ['stream-full', streamId] });
      // Use queryClient to fetch stream details (force fresh)
      const data = await queryClient.fetchQuery({
        queryKey: ['stream-full', streamId],
        queryFn: async () => {
          const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}/full`);
          if (!response.ok) {
            throw new Error(`HTTP error ${response.status}`);
          }
          return response.json();
        },
        staleTime: 0 // Always fetch fresh when opening modal
      });
      const stream = data.stream || {};
      const motion = data.motion_config || null;

      setCurrentStream({
        ...stream,
        // Convert numeric values to strings for form inputs
        width: stream.width || 1280,
        height: stream.height || 720,
        fps: stream.fps || 15,
        protocol: (stream.protocol != null ? stream.protocol : 0).toString(),
        priority: (stream.priority != null ? stream.priority : 5).toString(),
        segment: stream.segment_duration || 30,
        detectionThreshold: stream.detection_threshold || 50,
        detectionInterval: stream.detection_interval || 10,
        preBuffer: stream.pre_detection_buffer || 10,
        detectionZones: stream.detection_zones || [],
        postBuffer: stream.post_detection_buffer || 30,
        // Map API fields to form fields
        streamingEnabled: stream.streaming_enabled !== undefined ? stream.streaming_enabled : true,
        isOnvif: stream.isOnvif !== undefined ? stream.isOnvif : false,
        // ONVIF credentials
        onvifUsername: stream.onvif_username || '',
        onvifPassword: stream.onvif_password || '',
        onvifProfile: stream.onvif_profile || '',
        detectionEnabled: stream.detection_based_recording || false,
        detectionModel: stream.detection_model || '',
        recordAudio: stream.record_audio !== undefined ? stream.record_audio : true,
        backchannelEnabled: stream.backchannel_enabled !== undefined ? stream.backchannel_enabled : false,
        // Motion config mapping
        motionRecordingEnabled: motion ? !!motion.enabled : false,
        motionPreBuffer: motion ? (motion.pre_buffer_seconds || 5) : 5,
        motionPostBuffer: motion ? (motion.post_buffer_seconds || 10) : 10,
        motionMaxDuration: motion ? (motion.max_file_duration || 300) : 300,
        motionRetentionDays: motion ? (motion.retention_days || 7) : 7,
        motionCodec: motion ? (motion.codec || 'h264') : 'h264',
        motionQuality: motion ? (motion.quality || 'medium') : 'medium',
        // PTZ control settings
        ptzEnabled: stream.ptz_enabled !== undefined ? stream.ptz_enabled : false,
        ptzMaxX: stream.ptz_max_x || 0,
        ptzMaxY: stream.ptz_max_y || 0,
        ptzMaxZ: stream.ptz_max_z || 0,
        ptzHasHome: stream.ptz_has_home !== undefined ? stream.ptz_has_home : false,
        // Retention policy settings
        retentionDays: stream.retention_days || 0,
        detectionRetentionDays: stream.detection_retention_days || 0,
        maxStorageMb: stream.max_storage_mb || 0
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

    // Special handling for detection model changes
    if (name === 'detectionModel') {
      if (value === 'custom-api') {
        // When switching to custom API, initialize with empty URL
        setCurrentStream(prev => ({
          ...prev,
          detectionModel: ''
        }));
        return;
      }
    }

    // Special handling for custom API URL input
    if (name === 'customApiUrl') {
      setCurrentStream(prev => ({
        ...prev,
        detectionModel: value
      }));
      return;
    }

    setCurrentStream(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  // Disable stream (soft delete)
  const disableStream = (streamId) => {
    disableStreamMutation.mutate({
      streamId,
    });
  };

  // Delete stream (permanent)
  const deleteStream = (streamId) => {
    deleteStreamMutation.mutate({
      streamId,
    });
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
  const onvifDiscoveryMutation = usePostMutation(
    '/api/onvif/discovery/discover',
    {
      timeout: 120000,
      retries: 0
    },
    {
      onMutate: () => {
        setIsDiscovering(true);
      },
      onSuccess: (data) => {
        setDiscoveredDevices(data.devices || []);
        setIsDiscovering(false);
      },
      onError: (error) => {
        showStatusMessage(`Error discovering ONVIF devices: ${error.message}`, 'error', 5000);
        setIsDiscovering(false);
      }
    }
  );

  // Get device profiles mutation
  const getDeviceProfilesMutation = useMutation({
    mutationFn: ({ device, credentials }) => {
      setIsLoadingProfiles(true);

      // Use device_service URL which includes the correct port from discovery
      // Fall back to constructing URL from ip_address if device_service is not available
      const deviceUrl = device.device_service || `http://${device.ip_address}/onvif/device_service`;

      // Make a simple GET request
      return fetch('/api/onvif/device/profiles', {
        method: 'GET',
        headers: {
          'X-Device-URL': deviceUrl,
          'X-Username': credentials.username,
          'X-Password': credentials.password
        }
      }).then(response => {
        if (!response.ok) {
          throw new Error(`HTTP error ${response.status}`);
        }
        return response.json();
      });
    },
    onSuccess: (data) => {
      setDeviceProfiles(data.profiles || []);
      setIsLoadingProfiles(false);
    },
    onError: (error) => {
      showStatusMessage(`Error loading device profiles: ${error.message}`, 'error', 5000);
      setIsLoadingProfiles(false);
    }
  });

  // Test ONVIF connection mutation
  const testOnvifConnectionMutation = usePostMutation(
    '/api/onvif/device/test',
    {
      timeout: 15000,
      retries: 0
    },
    {
      onMutate: () => {
        setIsLoadingProfiles(true);
      },
      onSuccess: (data, variables) => {
        if (data.success) {
          showStatusMessage('Connection successful!', 'success', 3000);
          // The device object is no longer passed directly in variables
          // We need to use the selectedDevice state instead
          if (selectedDevice) {
            getDeviceProfiles(selectedDevice);
          }
        } else {
          showStatusMessage(`Connection failed: ${data.message}`, 'error', 5000);
          setIsLoadingProfiles(false);
        }
      },
      onError: (error) => {
        showStatusMessage(`Error testing connection: ${error.message}`, 'error', 5000);
        setIsLoadingProfiles(false);
      }
    }
  );



  // Submit ONVIF device
  const submitOnvifDevice = () => {
    if (!selectedDevice || !selectedProfile || !customStreamName.trim()) {
      showStatusMessage('Missing required information', 'error', 5000);
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
      backchannel_enabled: false,
      // Backend expects camelCase key 'isOnvif'
      isOnvif: true
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
    onvifDiscoveryMutation.mutate({});
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
    // Store the selected device first
    setSelectedDevice(device);

    // Use device_service URL which includes the correct port from discovery
    // Fall back to constructing URL from ip_address if device_service is not available
    const deviceUrl = device.device_service || `http://${device.ip_address}/onvif/device_service`;

    // Then make the API call
    testOnvifConnectionMutation.mutate({
      url: deviceUrl,
      username: onvifCredentials.username,
      password: onvifCredentials.password
    });
  };

  return (
    <section id="streams-page" className="page">
      <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 className="text-xl font-bold">Streams</h2>
        <div className="controls flex items-center space-x-2">
          {!canModifyStreams && userRole && (
            <span className="text-sm text-muted-foreground italic mr-2">
              Read-only (insufficient permissions to modify)
            </span>
          )}
          {canModifyStreams && (
            <>
              <button
                  id="discover-onvif-btn"
                  className="btn-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={() => setOnvifModalVisible(true)}
              >
                Discover ONVIF Cameras
              </button>
              <button
                  id="add-stream-btn"
                  className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={openAddStreamModal}
              >
                Add Stream
              </button>
            </>
          )}
        </div>
      </div>

      <ContentLoader
          isLoading={isLoading}
          hasData={hasData}
          loadingMessage="Loading streams..."
          emptyMessage={canModifyStreams ? "No streams configured yet. Click 'Add Stream' to create one." : "No streams configured yet."}
      >
        <div className="streams-container bg-card text-card-foreground rounded-lg shadow overflow-hidden">
          <div className="overflow-x-auto">
            <table id="streams-table" className="min-w-full divide-y divide-border">
              <thead className="bg-muted">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">URL</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Resolution</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">FPS</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Recording</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
              </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border">
              {streams.map(stream => (
                <tr key={stream.name} className="hover:bg-muted/50">
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex items-center">
                      <span className={`status-indicator w-2 h-2 rounded-full mr-2 ${stream.enabled ? 'status-ok' : 'status-danger'}`} style={{backgroundColor: stream.enabled ? 'hsl(var(--success))' : 'hsl(var(--danger))'}}></span>
                      {stream.name}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">{stream.url}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{stream.width}x{stream.height}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{stream.fps}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {stream.record ? 'Enabled' : 'Disabled'}
                    {stream.detection_based_recording ? ' (Detection)' : ''}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="flex space-x-2">
                      {/* Edit button - only show if user can modify streams */}
                      {canModifyStreams && (
                        <button
                            className="p-1 rounded-full focus:outline-none"
                            style={{color: 'hsl(var(--primary))'}}
                            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'}
                            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                            onClick={() => openEditStreamModal(stream.name)}
                            title="Edit"
                        >
                          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                          </svg>
                        </button>
                      )}
                      {/* Delete button - only show if user can modify streams */}
                      {canModifyStreams && (
                        <button
                            className="p-1 rounded-full focus:outline-none"
                            style={{color: 'hsl(var(--danger))'}}
                            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'}
                            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                            onClick={() => openDeleteModal(stream)}
                            title="Delete"
                        >
                          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path fillRule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clipRule="evenodd"></path>
                          </svg>
                        </button>
                      )}
                      {/* Show dash when no actions available */}
                      {!canModifyStreams && (
                        <span className="text-muted-foreground">—</span>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
              </tbody>
            </table>
          </div>
        </div>
      </ContentLoader>

      {deleteModalVisible && streamToDelete && (
        <StreamDeleteModal
            streamId={streamToDelete.name}
            streamName={streamToDelete.name}
            onClose={closeDeleteModal}
            onDisable={disableStream}
            onDelete={deleteStream}
        />
      )}

      {modalVisible && (
        <StreamConfigModal
          isEditing={isEditing}
          currentStream={currentStream}
          detectionModels={detectionModels}
          expandedSections={expandedSections}
          onToggleSection={toggleSection}
          onInputChange={handleInputChange}
          onThresholdChange={handleThresholdChange}
          onTestConnection={testStreamConnection}
          onTestMotion={triggerTestMotionEvent}
          onSave={handleSubmit}
          onClose={closeModal}
          onRefreshModels={loadDetectionModels}
        />
      )}

      {onvifModalVisible && (
        <div id="onvif-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">{isEditing ? 'Edit Stream' : 'Add Stream'}</h3>
              <span className="text-2xl cursor-pointer" onClick={closeModal}>×</span>
            </div>
            <div className="p-4">
              <form id="stream-form" className="space-y-4">
                <div className="form-group">
                  <label for="stream-name" className="block text-sm font-medium mb-1">Name</label>
                  <input
                      type="text"
                      id="stream-name"
                      name="name"
                      className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground ${isEditing ? 'bg-muted/30' : ''}`}
                      value={currentStream.name}
                      onChange={handleInputChange}
                      disabled={isEditing}
                      required
                  />
                </div>
                <div className="form-group">
                  <label for="stream-url" className="block text-sm font-medium mb-1">URL</label>
                  <input
                      type="text"
                      id="stream-url"
                      name="url"
                      className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                      placeholder="rtsp://example.com/stream"
                      value={currentStream.url}
                      onChange={handleInputChange}
                      required
                  />
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-enabled"
                      name="enabled"
                      className="h-4 w-4 border-gray-300 rounded"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.enabled}
                      onChange={handleInputChange}
                  />
                  <label for="stream-enabled" className="ml-2 block text-sm">Stream Active</label>
                  <span className="ml-2 text-xs text-muted-foreground">Enable/disable all stream processing</span>
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-streaming-enabled"
                      name="streamingEnabled"
                      className="h-4 w-4 border-gray-300 rounded" style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.streamingEnabled}
                      onChange={handleInputChange}
                  />
                  <label for="stream-streaming-enabled" className="ml-2 block text-sm">Live View Enabled</label>
                  <span className="ml-2 text-xs text-muted-foreground">Enable/disable live viewing in browser</span>
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-is-onvif"
                      name="isOnvif"
                      className="h-4 w-4 border-gray-300 rounded" style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.isOnvif}
                      onChange={handleInputChange}
                  />
                  <label for="stream-is-onvif" className="ml-2 block text-sm">ONVIF Camera</label>
                  <span className="ml-2 text-xs text-muted-foreground">Mark this stream as an ONVIF camera for special handling</span>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="form-group">
                    <label for="stream-width" className="block text-sm font-medium mb-1">Width</label>
                    <input
                        type="number"
                        id="stream-width"
                        name="width"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="320"
                        max="1920"
                        value={currentStream.width}
                        onChange={handleInputChange}
                    />
                  </div>
                  <div className="form-group">
                    <label for="stream-height" className="block text-sm font-medium mb-1">Height</label>
                    <input
                        type="number"
                        id="stream-height"
                        name="height"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="240"
                        max="1080"
                        value={currentStream.height}
                        onChange={handleInputChange}
                    />
                  </div>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div className="form-group">
                    <label for="stream-fps" className="block text-sm font-medium mb-1">FPS</label>
                    <input
                        type="number"
                        id="stream-fps"
                        name="fps"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="1"
                        max="30"
                        value={currentStream.fps}
                        onChange={handleInputChange}
                    />
                  </div>
                  <div className="form-group">
                    <label for="stream-codec" className="block text-sm font-medium mb-1">Codec</label>
                    <select
                        id="stream-codec"
                        name="codec"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        value={currentStream.codec}
                        onChange={handleInputChange}
                    >
                      <option value="h264">H.264</option>
                      <option value="h265">H.265</option>
                    </select>
                  </div>
                  <div className="form-group">
                    <label for="stream-protocol" className="block text-sm font-medium mb-1">Protocol</label>
                    <select
                        id="stream-protocol"
                        name="protocol"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        value={currentStream.protocol}
                        onChange={handleInputChange}
                    >
                      <option value="0">TCP</option>
                      <option value="1">UDP</option>
                    </select>
                    <span className="text-xs text-muted-foreground">Connection protocol (ONVIF cameras use either TCP or UDP)</span>
                  </div>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="form-group">
                    <label for="stream-priority" className="block text-sm font-medium mb-1">Priority</label>
                    <select
                        id="stream-priority"
                        name="priority"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        value={currentStream.priority}
                        onChange={handleInputChange}
                    >
                      <option value="1">Low (1)</option>
                      <option value="5">Medium (5)</option>
                      <option value="10">High (10)</option>
                    </select>
                  </div>
                  <div className="form-group">
                    <label for="stream-segment" className="block text-sm font-medium mb-1">Segment Duration (seconds)</label>
                    <input
                        type="number"
                        id="stream-segment"
                        name="segment"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="60"
                        max="3600"
                        value={currentStream.segment}
                        onChange={handleInputChange}
                    />
                  </div>
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-record"
                      name="record"
                      className="h-4 w-4 border-gray-300 rounded" style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.record}
                      onChange={handleInputChange}
                  />
                  <label for="stream-record" className="ml-2 block text-sm">Record</label>
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-record-audio"
                      name="recordAudio"
                      className="h-4 w-4 border-gray-300 rounded" style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.recordAudio}
                      onChange={handleInputChange}
                  />
                  <label for="stream-record-audio" className="ml-2 block text-sm">Record Audio</label>
                  <span className="ml-2 text-xs text-muted-foreground">Include audio in recordings if available in the stream</span>
                </div>

                {/* Detection-based recording options */}
                <div className="mt-6 mb-2 pb-1 border-b border-border">
                  <h4 className="text-md font-medium">Detection-Based Recording</h4>
                </div>
                <div className="form-group flex items-center">
                  <input
                      type="checkbox"
                      id="stream-detection-enabled"
                      name="detectionEnabled"
                      className="h-4 w-4 border-gray-300 rounded" style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.detectionEnabled}
                      onChange={handleInputChange}
                  />
                  <label for="stream-detection-enabled" className="ml-2 block text-sm">Enable Detection-Based Recording</label>
                  <span className="ml-2 text-xs text-muted-foreground">Only record when objects are detected</span>
                </div>
                <div className="form-group" style={currentStream.detectionEnabled ? '' : 'display: none'}>
                  <label for="stream-detection-model" className="block text-sm font-medium mb-1">Detection Model</label>
                  <div className="flex space-x-2">
                    <select
                        id="stream-detection-model"
                        name="detectionModel"
                        className="flex-1 px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        value={currentStream.detectionModel}
                        onChange={handleInputChange}
                    >
                      <option value="">Select a model</option>
                      {detectionModels.map(model => (
                        <option key={model.id} value={model.id}>{model.name}</option>
                      ))}
                    </select>
                    <button
                        id="refresh-models-btn"
                        className="p-2 rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none"
                        title="Refresh Models"
                        onClick={loadDetectionModels}
                        type="button"
                    >
                      <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                <div className="form-group" style={currentStream.detectionEnabled ? '' : 'display: none'}>
                  <label for="stream-detection-threshold" className="block text-sm font-medium mb-1">Detection Threshold</label>
                  <div className="flex items-center space-x-2">
                    <input
                        type="range"
                        id="stream-detection-threshold"
                        name="detectionThreshold"
                        className="flex-1 h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                        min="0"
                        max="100"
                        step="1"
                        value={currentStream.detectionThreshold}
                        onInput={handleThresholdChange}
                    />
                    <span id="stream-threshold-value" className="font-medium min-w-[3rem] text-center" style={{color: 'hsl(var(--primary))'}}>
                      {currentStream.detectionThreshold}%
                    </span>
                  </div>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4" style={currentStream.detectionEnabled ? '' : 'display: none'}>
                  <div className="form-group">
                    <label for="stream-detection-interval" className="block text-sm font-medium mb-1">Detection Interval (frames)</label>
                    <input
                        type="number"
                        id="stream-detection-interval"
                        name="detectionInterval"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="1"
                        max="100"
                        value={currentStream.detectionInterval}
                        onChange={handleInputChange}
                    />
                    <span className="text-xs text-muted-foreground">Detect on every Nth frame</span>
                  </div>
                  <div className="form-group">
                    <label for="stream-pre-buffer" className="block text-sm font-medium mb-1">Pre-detection Buffer (seconds)</label>
                    <input
                        type="number"
                        id="stream-pre-buffer"
                        name="preBuffer"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="0"
                        max="60"
                        value={currentStream.preBuffer}
                        onChange={handleInputChange}
                    />
                    <span className="text-xs text-muted-foreground">Seconds to keep before detection</span>
                  </div>
                  <div className="form-group">
                    <label for="stream-post-buffer" className="block text-sm font-medium mb-1">Post-detection Buffer (seconds)</label>
                    <input
                        type="number"
                        id="stream-post-buffer"
                        name="postBuffer"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        min="0"
                        max="300"
                        value={currentStream.postBuffer}
                        onChange={handleInputChange}
                    />
                    <span className="text-xs text-muted-foreground">Seconds to keep after detection</span>
                  </div>
                </div>
              </form>
            </div>
            <div className="flex justify-between p-4 border-t border-border">
              <button
                  id="stream-test-btn"
                  className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                  onClick={testStreamConnection}
                  type="button"
              >
                Test Connection
              </button>
              <div className="space-x-2">
                <button
                    id="stream-save-btn"
                    className="btn-primary"
                    onClick={saveStream}
                    type="button"
                >
                  Save
                </button>
                <button
                    id="stream-cancel-btn"
                    className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                    onClick={closeModal}
                    type="button"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {onvifModalVisible && (
        <div id="onvif-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">ONVIF Camera Discovery</h3>
              <span className="text-2xl cursor-pointer" onClick={() => setOnvifModalVisible(false)}>×</span>
            </div>
            <div className="p-4">
              <div className="mb-4 flex justify-between items-center">
                <h4 className="text-md font-medium">Discovered Devices</h4>
                <button
                    id="discover-btn"
                    className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary"
                    onClick={startOnvifDiscovery}
                    disabled={isDiscovering}
                    type="button"
                >
                  {isDiscovering ? (
                    <span className="flex items-center">
                      Discovering
                      <span className="ml-1 flex space-x-1">
                        <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                      </span>
                    </span>
                  ) : 'Start Discovery'}
                </button>
              </div>

              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-border">
                  <thead className="bg-muted">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">IP Address</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Manufacturer</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Model</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
                  </tr>
                  </thead>
                  <tbody className="bg-card divide-y divide-border">
                  {discoveredDevices.length === 0 ? (
                    <tr>
                      <td colSpan="4" className="px-6 py-4 text-center text-muted-foreground">
                        {isDiscovering ? (
                          <div className="flex items-center justify-center">
                            <span>Discovering devices</span>
                            <span className="ml-1 flex space-x-1">
                                <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                                <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                                <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                              </span>
                          </div>
                        ) : 'No devices discovered yet. Click "Start Discovery" to scan your network.'}
                      </td>
                    </tr>
                  ) : discoveredDevices.map(device => (
                    <tr key={device.ip_address} className="hover:bg-muted/50">
                      <td className="px-6 py-4 whitespace-nowrap">{device.ip_address}</td>
                      <td className="px-6 py-4 whitespace-nowrap">{device.manufacturer || 'Unknown'}</td>
                      <td className="px-6 py-4 whitespace-nowrap">{device.model || 'Unknown'}</td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <button
                            className="btn-primary focus:outline-none"
                            onClick={() => testOnvifConnection(device)}
                            disabled={isLoadingProfiles && selectedDevice && selectedDevice.ip_address === device.ip_address}
                            type="button"
                        >
                          {isLoadingProfiles && selectedDevice && selectedDevice.ip_address === device.ip_address ? (
                            <span className="flex items-center">
                                Loading
                                <span className="ml-1 flex space-x-1">
                                  <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                                  <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                                </span>
                              </span>
                          ) : 'Connect'}
                        </button>
                      </td>
                    </tr>
                  ))}
                  </tbody>
                </table>
              </div>

              <div className="mt-6 mb-4">
                <h4 className="text-md font-medium mb-2">Authentication</h4>
                <p className="text-sm text-muted-foreground mb-3">
                  Enter credentials to connect to the selected ONVIF device. Credentials are not needed for discovery, only for connecting to devices.
                </p>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="form-group">
                    <label for="onvif-username" className="block text-sm font-medium mb-1">Username</label>
                    <input
                        type="text"
                        id="onvif-username"
                        name="username"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        placeholder="admin"
                        value={onvifCredentials.username}
                        onChange={handleCredentialChange}
                    />
                  </div>
                  <div className="form-group">
                    <label for="onvif-password" className="block text-sm font-medium mb-1">Password</label>
                    <input
                        type="password"
                        id="onvif-password"
                        name="password"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                        placeholder="password"
                        value={onvifCredentials.password}
                        onChange={handleCredentialChange}
                    />
                  </div>
                </div>
              </div>

              {selectedDevice && deviceProfiles.length > 0 && (
                <div className="mt-6">
                  <h4 className="text-md font-medium mb-2">Available Profiles for {selectedDevice.ip_address}</h4>
                  <div className="overflow-x-auto">
                    <table className="min-w-full divide-y divide-border">
                      <thead className="bg-muted">
                      <tr>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Resolution</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Encoding</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">FPS</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
                      </tr>
                      </thead>
                      <tbody className="bg-card divide-y divide-border">
                      {deviceProfiles.map(profile => (
                        <tr key={profile.token} className="hover:bg-muted/50">
                          <td className="px-6 py-4 whitespace-nowrap">{profile.name}</td>
                          <td className="px-6 py-4 whitespace-nowrap">{profile.width}x{profile.height}</td>
                          <td className="px-6 py-4 whitespace-nowrap">{profile.encoding}</td>
                          <td className="px-6 py-4 whitespace-nowrap">{profile.fps}</td>
                          <td className="px-6 py-4 whitespace-nowrap">
                            <button
                                className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary"
                                onClick={() => addOnvifDeviceAsStream(profile)}
                                type="button"
                            >
                              Add as Stream
                            </button>
                          </td>
                        </tr>
                      ))}
                      </tbody>
                    </table>
                  </div>
                </div>
            )}
            </div>
            <div className="flex justify-end p-4 border-t border-border">
              <button
                  id="onvif-close-btn"
                  className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                  onClick={() => setOnvifModalVisible(false)}
                  type="button"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}

      {showCustomNameInput && (
        <div id="custom-name-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">Stream Name</h3>
              <span className="text-2xl cursor-pointer" onClick={() => setShowCustomNameInput(false)}>×</span>
            </div>
            <div className="p-4">
              <div className="mb-4">
                <label for="custom-stream-name" className="block text-sm font-medium mb-1">Enter a name for this stream:</label>
                <input
                    type="text"
                    id="custom-stream-name"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                    value={customStreamName}
                    onChange={(e) => setCustomStreamName(e.target.value)}
                />
                <p className="mt-1 text-sm text-muted-foreground">
                  This name will be used to identify the stream in the system.
                </p>
              </div>
            </div>
            <div className="flex justify-end p-4 border-t border-border space-x-2">
              <button
                  className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                  onClick={() => setShowCustomNameInput(false)}
                  type="button"
              >
                Cancel
              </button>
              <button
                  className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary"
                  onClick={submitOnvifDevice}
                  type="button"
                  disabled={!customStreamName.trim() || isAddingStream}
              >
                {isAddingStream ? (
                  <span className="flex items-center">
                    Adding
                    <span className="ml-1 flex space-x-1">
                      <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                    </span>
                  </span>
                ) : 'Add Stream'}
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
