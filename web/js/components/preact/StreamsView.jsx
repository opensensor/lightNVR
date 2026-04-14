/**
 * LightNVR Web Interface StreamsView Component
 * React component for the streams page
 */

import { Fragment } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { ContentLoader } from './LoadingIndicator.jsx';
import { StreamDeleteModal } from './StreamDeleteModal.jsx';
import { StreamBulkActionModal } from './StreamBulkActionModal.jsx';
import { StreamConfigModal } from './StreamConfigModal.jsx';
import { HealthView } from './HealthView.jsx';
import { validateSession } from '../../utils/auth-utils.js';
import { obfuscateUrlCredentials, urlHasCredentials } from '../../utils/url-utils.js';
import {
  useQuery,
  useMutation,
  useQueryClient,
  usePostMutation,
  fetchJSON
} from '../../query-client.js';
import { useI18n } from '../../i18n.js';

/**
 * StreamsView component
 * @returns {JSX.Element} StreamsView component
 */
export function StreamsView() {
  const { t } = useI18n();
  const queryClient = useQueryClient();

  // User role state for permission-based UI
  const [userRole, setUserRole] = useState(null);
  // Demo mode state - credentials should be hidden in demo mode
  const [isDemoMode, setIsDemoMode] = useState(false);

  // Fetch user role on mount
  useEffect(() => {
    const fetchUserRole = async () => {
      try {
        const result = await validateSession();
        if (result.valid && result.role) {
          setUserRole(result.role);
          setIsDemoMode(result.demo_mode === true);
        } else {
          // Session invalid - set to empty string to indicate fetch completed
          setUserRole('');
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
  // Check if credentials should be hidden (demo mode or viewer role)
  const shouldHideCredentials = isDemoMode || userRole === 'viewer';

  // State for streams data
  const [activeTab, setActiveTab] = useState('streams');
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
  const [onvifNetworkOverride, setOnvifNetworkOverride] = useState('auto');

  // Track which stream rows are expanded to show health/status details
  const [expandedStreams, setExpandedStreams] = useState({});

  const toggleStreamExpand = (streamName) => {
    setExpandedStreams(prev => ({
      ...prev,
      [streamName]: !prev[streamName]
    }));
  };

  // Track which stream URL credentials are currently revealed (by stream name).
  // By default every URL with credentials is masked, even for admins.
  const [revealedUrls, setRevealedUrls] = useState(new Set());

  const toggleUrlReveal = (streamName, e) => {
    e.stopPropagation();
    setRevealedUrls(prev => {
      const next = new Set(prev);
      next.has(streamName) ? next.delete(streamName) : next.add(streamName);
      return next;
    });
  };

  // Bulk selection state
  const [selectionMode, setSelectionMode] = useState(false);
  const [selectedStreams, setSelectedStreams] = useState(new Set());
  const [isBulkWorking, setIsBulkWorking] = useState(false);
  // null | 'disable' | 'delete' — which bulk confirmation modal is open
  const [bulkActionModal, setBulkActionModal] = useState(null);

  const exitSelectionMode = () => {
    setSelectionMode(false);
    setSelectedStreams(new Set());
  };

  const toggleSelect = (e, name) => {
    e.stopPropagation();
    setSelectedStreams(prev => {
      const next = new Set(prev);
      next.has(name) ? next.delete(name) : next.add(name);
      return next;
    });
  };

  const toggleSelectAll = (e) => {
    e.stopPropagation();
    if (selectedStreams.size > 0) {
      setSelectedStreams(new Set());
    } else {
      setSelectedStreams(new Set((streams || []).map(s => s.name)));
    }
  };

  // Accordion expanded state for StreamConfigModal sections
  const [expandedSections, setExpandedSections] = useState({
    basic: true,
    recording: false,
    detection: false,
    zones: false,
    motion: false,
    ptz: false,
    advanced: false,
    go2rtcOverride: false
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
    isLoading
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

  // Sorting state for the streams table
  const DEFAULT_SORT_COLUMN = null;
  const [sortColumn, setSortColumn] = useState(DEFAULT_SORT_COLUMN);
  const [sortDirection, setSortDirection] = useState('asc');

  const handleSort = (column) => {
    if (sortColumn === column) {
      setSortDirection(prev => prev === 'asc' ? 'desc' : 'asc');
    } else {
      setSortColumn(column);
      setSortDirection('asc');
    }
  };

  const sortedStreams = (() => {
    if (sortColumn === DEFAULT_SORT_COLUMN) return streams;
    return [...streams].sort((a, b) => {
      let aVal, bVal;
      if (sortColumn === 'name') {
        aVal = (a.name || '').toLowerCase();
        bVal = (b.name || '').toLowerCase();
      } else if (sortColumn === 'status') {
        aVal = (a.status || '').toLowerCase();
        bVal = (b.status || '').toLowerCase();
      } else if (sortColumn === 'url') {
        aVal = (a.url || '').toLowerCase();
        bVal = (b.url || '').toLowerCase();
      } else if (sortColumn === 'resolution') {
        aVal = (a.width || 0) * (a.height || 0);
        bVal = (b.width || 0) * (b.height || 0);
      } else if (sortColumn === 'fps') {
        aVal = a.fps || 0;
        bVal = b.fps || 0;
      } else if (sortColumn === 'recording') {
        aVal = a.record ? 1 : 0;
        bVal = b.record ? 1 : 0;
      } else {
        return 0;
      }
      if (aVal < bVal) return sortDirection === 'asc' ? -1 : 1;
      if (aVal > bVal) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  })();

  // Default stream state
  const [currentStream, setCurrentStream] = useState({
    name: '',
    url: '',
    adminUrl: '',
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
    onvifPort: 0,
    // AI Detection recording
    detectionEnabled: false,
    detectionModel: '',
    detectionThreshold: 50,
    detectionInterval: 10,
    preBuffer: 10,
    postBuffer: 30,
    detectionZones: [],
    // Detection object filter defaults
    detectionObjectFilter: 'none',
    detectionObjectFilterList: '',
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
    maxStorageMb: 0,
    // Recording schedule
    recordOnSchedule: false,
    recordingSchedule: Array(168).fill(true),
    // Tags
    tags: '',
    // Cross-stream motion trigger: name of another stream whose ONVIF motion
    // events trigger recording on this stream (e.g., PTZ slaved to fixed wide lens)
    motionTriggerSource: '',
    go2rtcSourceOverride: '',
    subStreamUrl: ''
  });
  const [isEditing, setIsEditing] = useState(false);
  const [isCloning, setIsCloning] = useState(false);

  // State for delete modal
  const [deleteModalVisible, setDeleteModalVisible] = useState(false);
  const [streamToDelete, setStreamToDelete] = useState(null);

  // Compute hasData from streams
  const hasData = streams && streams.length > 0;

  // Extract detection models from query result
  const detectionModels = detectionModelsData?.models || [];

  // Unified mutation that internally decides between create and update
  const saveStreamMutation = useMutation({
    mutationFn: async (streamData) => {
      if (isEditing) {
        // Update existing stream via PUT
        const url = `/api/streams/${encodeURIComponent(streamData.name)}`;
        // `name` is used as the stream identifier in the URL path and must not be included in the request body
        const { name, ...rest } = streamData;
        return await fetchJSON(url, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(rest),
          timeout: 15000,
          retries: 1,
          retryDelay: 1000
        });
      }
      // Create new stream via POST
      return await fetchJSON('/api/streams', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(streamData),
        timeout: 45000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: (data, variables) => {
      if (isEditing) {
        showStatusMessage(
          t('streams.streamUpdatedSuccessfully', { name: variables?.name }),
          'success',
          3000
        );
      } else {
        showStatusMessage(t('streams.streamAddedSuccessfully'));
      }
      closeModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error, variables) => {
      if (!isEditing) {
        const isStreamLimitError = error?.status === 409 &&
          typeof error?.message === 'string' &&
          error.message.includes('Max streams limit reached');
        showStatusMessage(
          isStreamLimitError ? error.message : t('streams.errorAddingStream', { message: error.message }),
          'error',
          isStreamLimitError ? 9000 : 5000
        );
      } else {
        showStatusMessage(
          t('streams.errorUpdatingStream', { message: error.message }),
          'error',
          5000
        );
      }
    }
  });

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
        showStatusMessage(t('streams.testingStreamConnection'));
      },
      onSuccess: (data) => {
        if (data.success) {
          showStatusMessage(t('streams.streamConnectionSuccessful'), 'success', 3000);
        } else {
          showStatusMessage(t('streams.streamConnectionFailed', { message: data.message }), 'error', 5000);
        }
      },
      onError: (error) => {
        showStatusMessage(t('streams.errorTestingStream', { message: error.message }), 'error', 5000);
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
      showStatusMessage(t('streams.streamSuccessfullyDeleted'));
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] })
        .then(() => {
          // Close the modal after the query invalidation is complete
          closeDeleteModal();
        });
    },
    onError: (error) => {
      showStatusMessage(t('streams.errorDeletingStream', { message: error.message }), 'error', 5000);
      closeDeleteModal();
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
      showStatusMessage(t('streams.streamSuccessfullyDisabled'));
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] })
        .then(() => {
          // Close the modal after the query invalidation is complete
          closeDeleteModal();
        });
    },
    onError: (error) => {
      showStatusMessage(t('streams.errorDisablingStream', { message: error.message }), 'error', 5000);
      closeDeleteModal();
    }
  });

  const enableStreamMutation = useMutation({
    mutationFn: async (streamId) => {
      return await fetchJSON(`/api/streams/${encodeURIComponent(streamId)}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enable_disabled: true, enabled: true }),
        timeout: 15000,
        retries: 1,
        retryDelay: 1000
      });
    },
    onSuccess: () => {
      showStatusMessage(t('streams.streamSuccessfullyEnabled') || 'Stream enabled.');
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      showStatusMessage(t('streams.errorEnablingStream', { message: error.message }) || `Error enabling stream: ${error.message}`, 'error', 5000);
    }
  });

  const handleToggleStreamEnabled = (stream) => {
    if (stream.enabled) {
      if (window.confirm(t('live.disableStreamConfirm'))) {
        disableStreamMutation.mutate({ streamId: stream.name });
      }
    } else {
      enableStreamMutation.mutate(stream.name);
    }
  };

  // Bulk action handlers — open the confirmation modal instead of using alert()
  const handleBulkEnable = () => {
    if (selectedStreams.size === 0 || isBulkWorking) return;
    setBulkActionModal('enable');
  };

  const handleBulkDisable = () => {
    if (selectedStreams.size === 0 || isBulkWorking) return;
    setBulkActionModal('disable');
  };

  const handleBulkDelete = () => {
    if (selectedStreams.size === 0 || isBulkWorking) return;
    setBulkActionModal('delete');
  };

  // Execute bulk enable after modal confirmation
  const executeBulkEnable = async () => {
    const names = [...selectedStreams];
    setIsBulkWorking(true);
    const results = await Promise.allSettled(
      names.map(name =>
        fetchJSON(`/api/streams/${encodeURIComponent(name)}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          // Both enable_disabled:true and enabled:true are required so the worker
          // thread writes enabled=1 to the DB; without enabled:true the worker
          // overwrites the direct SQL enable with the stale enabled=0 from the
          // config struct.
          body: JSON.stringify({ enable_disabled: true, enabled: true }),
          timeout: 15000,
        })
      )
    );
    setIsBulkWorking(false);
    setBulkActionModal(null);
    setSelectedStreams(new Set());
    queryClient.invalidateQueries({ queryKey: ['streams'] });
    const failed = results.filter(r => r.status === 'rejected').length;
    if (failed > 0) {
      showStatusMessage(t('streams.enabledSomeStreams', { success: names.length - failed, failed }), 'warning', 5000);
    } else {
      showStatusMessage(t('streams.enabledStreams', { count: names.length }), 'success');
    }
  };

  // Execute bulk disable after modal confirmation
  const executeBulkDisable = async () => {
    const names = [...selectedStreams];
    setIsBulkWorking(true);
    const results = await Promise.allSettled(
      names.map(name =>
        fetchJSON(`/api/streams/${encodeURIComponent(name)}`, { method: 'DELETE', timeout: 15000 })
      )
    );
    setIsBulkWorking(false);
    setBulkActionModal(null);
    setSelectedStreams(new Set());
    queryClient.invalidateQueries({ queryKey: ['streams'] });
    const failed = results.filter(r => r.status === 'rejected').length;
    if (failed > 0) {
      showStatusMessage(t('streams.disabledSomeStreams', { success: names.length - failed, failed }), 'warning', 5000);
    } else {
      showStatusMessage(t('streams.disabledStreams', { count: names.length }), 'success');
    }
  };

  // Execute bulk delete after modal confirmation
  const executeBulkDelete = async () => {
    const names = [...selectedStreams];
    setIsBulkWorking(true);
    const results = await Promise.allSettled(
      names.map(name =>
        fetchJSON(`/api/streams/${encodeURIComponent(name)}?permanent=true`, { method: 'DELETE', timeout: 15000 })
      )
    );
    setIsBulkWorking(false);
    setBulkActionModal(null);
    setSelectedStreams(new Set());
    queryClient.invalidateQueries({ queryKey: ['streams'] });
    const failed = results.filter(r => r.status === 'rejected').length;
    if (failed > 0) {
      showStatusMessage(t('streams.deletedSomeStreams', { success: names.length - failed, failed }), 'warning', 5000);
    } else {
      showStatusMessage(t('streams.deletedStreams', { count: names.length }), 'success');
    }
  };

  const normalizeRecordingSchedule = (recordingSchedule, recordOnSchedule) => {
    if (!recordOnSchedule) {
      return Array(168).fill(true);
    }
    if (Array.isArray(recordingSchedule) && recordingSchedule.length === 168) {
      return recordingSchedule;
    }
    return Array(168).fill(true);
  };

  // Handle form submission
  const handleSubmit = (e) => {
    e.preventDefault();

    // Prepare stream data for API
    const streamData = {
      name: currentStream.name.trim(),
      url: currentStream.url,
      admin_url: currentStream.adminUrl || '',
      enabled: currentStream.enabled,
      streaming_enabled: currentStream.streamingEnabled,
      // Note: width, height, fps, and codec are auto-detected and not sent from the frontend
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
      onvif_port: parseInt(currentStream.onvifPort, 10) || 0,
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
      // Detection object filter settings
      detection_object_filter: currentStream.detectionObjectFilter || 'none',
      detection_object_filter_list: currentStream.detectionObjectFilterList || '',
      // Retention policy settings
      retention_days: parseInt(currentStream.retentionDays, 10) || 0,
      detection_retention_days: parseInt(currentStream.detectionRetentionDays, 10) || 0,
      max_storage_mb: parseInt(currentStream.maxStorageMb, 10) || 0,
      // Recording schedule
      record_on_schedule: !!currentStream.recordOnSchedule,
      recording_schedule: normalizeRecordingSchedule(
        currentStream.recordingSchedule,
        currentStream.recordOnSchedule
      ),
      // Tags
      tags: currentStream.tags || '',
      // Cross-stream motion trigger source
      motion_trigger_source: currentStream.motionTriggerSource || '',
      // go2rtc source override
      go2rtc_source_override: currentStream.go2rtcSourceOverride || '',
      // Sub-stream URL
      sub_stream_url: currentStream.subStreamUrl || ''
    };

    // When editing, set is_deleted to false to allow undeleting soft-deleted streams
    if (isEditing) {
      streamData.is_deleted = false;
    }

    // Use mutation to save stream and then handle motion config if applicable
    const savedStreamName = streamData.name;
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
          showStatusMessage(t('streams.motionConfigSaveFailed', { message: err.message }), 'error', 5000);
        }
        // Ensure both list and details are refreshed after save
        await queryClient.invalidateQueries({ queryKey: ['stream-full', savedStreamName] });
        queryClient.invalidateQueries({ queryKey: ['streams'] });
        // Close the modal after successful save
        closeModal();
      }
    });
  };

  // Test stream connection
  const testStreamConnection = () => {
    testStreamMutation.mutate({
      url: currentStream.url,
      protocol: parseInt(currentStream.protocol, 10),
      onvif_username: currentStream.isOnvif ? currentStream.onvifUsername : undefined,
      onvif_password: currentStream.isOnvif ? currentStream.onvifPassword : undefined,
    });
  };

  // Trigger a simulated ONVIF motion event for the current stream
  const triggerTestMotionEvent = async () => {
    if (!currentStream?.name) {
      showStatusMessage(t('streams.setStreamNameAndSaveBeforeTestingMotion'), 'error', 5000);
      return;
    }
    if (!currentStream?.isOnvif) {
      showStatusMessage(t('streams.enableOnvifCameraFirst'), 'error', 5000);
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
        showStatusMessage(t('streams.testMotionEventTriggeredSuccessfully'), 'success', 3000);
      } else {
        showStatusMessage(t('streams.testMotionEventFailed', { message: data?.message || t('common.unknown') }), 'error', 5000);
      }
    } catch (err) {
      showStatusMessage(t('streams.errorTriggeringTestMotion', { message: err.message }), 'error', 5000);
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
      adminUrl: '',
      enabled: true,
      streamingEnabled: true,
      width: 0,
      height: 0,
      fps: 0,
      codec: '',
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
      onvifPort: 0,
      detectionEnabled: false,
      detectionModel: '',
      detectionThreshold: 50,
      detectionInterval: 10,
      preBuffer: 10,
      postBuffer: 30,
      detectionObjectFilter: 'none',
      detectionObjectFilterList: '',
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
      maxStorageMb: 0,
      // Recording schedule
      recordOnSchedule: false,
      recordingSchedule: Array(168).fill(true),
      // Tags
      tags: ''
    });
    setIsEditing(false);
    setIsCloning(false);
    setModalVisible(true);
  };

  // Open edit stream modal
  const openEditStreamModal = async (streamId) => {
    try {
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
        // These are read-only auto-detected values (0/empty means not yet detected)
        width: stream.width || 0,
        height: stream.height || 0,
        fps: stream.fps || 0,
        codec: stream.codec || '',
        protocol: (stream.protocol != null ? stream.protocol : 0).toString(),
        priority: (stream.priority != null ? stream.priority : 5).toString(),
        segment: stream.segment_duration || 30,
        detectionThreshold: stream.detection_threshold || 50,
        detectionInterval: stream.detection_interval || 10,
        preBuffer: stream.pre_detection_buffer || 10,
        detectionZones: stream.detection_zones || [],
        postBuffer: stream.post_detection_buffer || 30,
        // Map API fields to form fields
        adminUrl: stream.admin_url || '',
        streamingEnabled: stream.streaming_enabled !== undefined ? stream.streaming_enabled : true,
        isOnvif: stream.isOnvif !== undefined ? stream.isOnvif : false,
        // ONVIF credentials
        onvifUsername: stream.onvif_username || '',
        onvifPassword: stream.onvif_password || '',
        onvifProfile: stream.onvif_profile || '',
        onvifPort: stream.onvif_port || 0,
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
        // Detection object filter settings
        detectionObjectFilter: stream.detection_object_filter || 'none',
        detectionObjectFilterList: stream.detection_object_filter_list || '',
        // Retention policy settings
        retentionDays: stream.retention_days || 0,
        detectionRetentionDays: stream.detection_retention_days || 0,
        maxStorageMb: stream.max_storage_mb || 0,
        // Recording schedule
        recordOnSchedule: stream.record_on_schedule || false,
        recordingSchedule: (Array.isArray(stream.recording_schedule) && stream.recording_schedule.length === 168)
          ? stream.recording_schedule
          : Array(168).fill(true),
        // Tags
        tags: stream.tags || '',
        // Cross-stream motion trigger source
        motionTriggerSource: stream.motion_trigger_source || '',
        go2rtcSourceOverride: stream.go2rtc_source_override || '',
        subStreamUrl: stream.sub_stream_url || ''
      });
      setIsEditing(true);
      setModalVisible(true);
    } catch (error) {
      console.error('Error loading stream details:', error);
      showStatusMessage(t('streams.errorLoadingStreamDetails', { message: error.message }));
    }
  };

  // Open clone stream modal — pre-fills from an existing stream but treats as a new one
  const openCloneStreamModal = async (streamId) => {
    try {
      await queryClient.invalidateQueries({ queryKey: ['stream-full', streamId] });
      const data = await queryClient.fetchQuery({
        queryKey: ['stream-full', streamId],
        queryFn: async () => {
          const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}/full`);
          if (!response.ok) {
            throw new Error(`HTTP error ${response.status}`);
          }
          return response.json();
        },
        staleTime: 0
      });
      const stream = data.stream || {};
      const motion = data.motion_config || null;

      setCurrentStream({
        ...stream,
        // Clear the name so the user must provide a unique one
        name: `${t('streams.copyOf')} ${stream.name || streamId}`,
        // Reset auto-detected fields — the cloned stream may be a different camera
        width: 0,
        height: 0,
        fps: 0,
        codec: '',
        protocol: (stream.protocol != null ? stream.protocol : 0).toString(),
        priority: (stream.priority != null ? stream.priority : 5).toString(),
        segment: stream.segment_duration || 30,
        detectionThreshold: stream.detection_threshold || 50,
        detectionInterval: stream.detection_interval || 10,
        preBuffer: stream.pre_detection_buffer || 10,
        detectionZones: [],
        postBuffer: stream.post_detection_buffer || 30,
        adminUrl: stream.admin_url || '',
        streamingEnabled: stream.streaming_enabled !== undefined ? stream.streaming_enabled : true,
        isOnvif: stream.isOnvif !== undefined ? stream.isOnvif : false,
        onvifUsername: stream.onvif_username || '',
        onvifPassword: stream.onvif_password || '',
        onvifProfile: stream.onvif_profile || '',
        onvifPort: stream.onvif_port || 0,
        detectionEnabled: stream.detection_based_recording || false,
        detectionModel: stream.detection_model || '',
        recordAudio: stream.record_audio !== undefined ? stream.record_audio : true,
        backchannelEnabled: stream.backchannel_enabled !== undefined ? stream.backchannel_enabled : false,
        motionRecordingEnabled: motion ? !!motion.enabled : false,
        motionPreBuffer: motion ? (motion.pre_buffer_seconds || 5) : 5,
        motionPostBuffer: motion ? (motion.post_buffer_seconds || 10) : 10,
        motionMaxDuration: motion ? (motion.max_file_duration || 300) : 300,
        motionRetentionDays: motion ? (motion.retention_days || 7) : 7,
        motionCodec: motion ? (motion.codec || 'h264') : 'h264',
        motionQuality: motion ? (motion.quality || 'medium') : 'medium',
        ptzEnabled: stream.ptz_enabled !== undefined ? stream.ptz_enabled : false,
        ptzMaxX: stream.ptz_max_x || 0,
        ptzMaxY: stream.ptz_max_y || 0,
        ptzMaxZ: stream.ptz_max_z || 0,
        ptzHasHome: stream.ptz_has_home !== undefined ? stream.ptz_has_home : false,
        detectionObjectFilter: stream.detection_object_filter || 'none',
        detectionObjectFilterList: stream.detection_object_filter_list || '',
        retentionDays: stream.retention_days || 0,
        detectionRetentionDays: stream.detection_retention_days || 0,
        maxStorageMb: stream.max_storage_mb || 0,
        recordOnSchedule: stream.record_on_schedule || false,
        recordingSchedule: (Array.isArray(stream.recording_schedule) && stream.recording_schedule.length === 168)
          ? stream.recording_schedule
          : Array(168).fill(true),
        tags: stream.tags || '',
        // Cross-stream motion trigger source
        motionTriggerSource: stream.motion_trigger_source || '',
        go2rtcSourceOverride: stream.go2rtc_source_override || '',
        subStreamUrl: stream.sub_stream_url || ''
      });
      setIsEditing(false);
      setIsCloning(true);
      setModalVisible(true);
    } catch (error) {
      console.error('Error loading stream details for clone:', error);
      showStatusMessage(t('streams.errorLoadingStreamDetails', { message: error.message }));
    }
  };

  // Close modal
  const closeModal = () => {
    setModalVisible(false);
    setIsCloning(false);
  };

  // Open ONVIF discovery modal
  const openOnvifModal = async () => {
    setDiscoveredDevices([]);
    setDeviceProfiles([]);
    setSelectedDevice(null);
    setSelectedProfile(null);
    setCustomStreamName('');
    // Fetch the configured default network from settings
    try {
      const settings = await fetchJSON('/api/settings', { timeout: 5000 });
      if (settings && settings.onvif_discovery_network) {
        setOnvifNetworkOverride(settings.onvif_discovery_network);
      } else {
        setOnvifNetworkOverride('auto');
      }
    } catch (e) {
      console.warn('Could not fetch ONVIF discovery network setting, using auto', e);
      setOnvifNetworkOverride('auto');
    }
    setOnvifModalVisible(true);
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
    '/api/onvif/discover',
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
        showStatusMessage(t('streams.errorDiscoveringOnvifDevices', { message: error.message }), 'error', 5000);
        setIsDiscovering(false);
      }
    }
  );

  /**
   * Validate that an ONVIF device host address is a well-formed hostname, IPv4
   * address, or bracketed IPv6 address. Returns true when valid, false otherwise.
   */
  const isValidDeviceHost = (deviceHost) => {
    if (typeof deviceHost !== 'string') {
      return false;
    }
    // Validate the discovered IP/host before constructing fallback URLs to avoid malformed URLs.
    // Allow typical hostnames / IPv4-like hosts and optional IPv6 literal in brackets, but reject
    // malformed inputs such as consecutive dots or invalid IPv6 sequences.
    const hostnamePattern =
      /^(?=.{1,253}$)(?:[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?)(?:\.(?:[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?))*$/;
    // IPv4 addresses (e.g. 192.168.1.100) — hostnamePattern allows alphanumeric labels but
    // the look-ahead length check can reject short dotted-decimal addresses, so validate them
    // explicitly with a dedicated pattern.
    const ipv4Pattern =
      /^(25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)){3}$/;
    // IPv6 literal in brackets. This pattern supports full and compressed IPv6 forms,
    // e.g. "[2001:db8::1]", "[::1]", and "[::]".
    const ipv6BracketPattern =
      /^\[((?:(?:[0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})|(?:(?:[0-9A-Fa-f]{1,4}:){1,7}:)|(?:::(?:[0-9A-Fa-f]{1,4}:){0,6}[0-9A-Fa-f]{1,4})|(?:(?:[0-9A-Fa-f]{1,4}:){1,6}:[0-9A-Fa-f]{1,4})|(?:::[0-9A-Fa-f]{1,4})|(?:[0-9A-Fa-f]{1,4}::))\]$/;
    return (
      hostnamePattern.test(deviceHost) ||
      ipv4Pattern.test(deviceHost) ||
      ipv6BracketPattern.test(deviceHost)
    );
  };

  // Get device profiles mutation
  const getDeviceProfilesMutation = useMutation({
    mutationFn: ({ device, credentials }) => {
      setIsLoadingProfiles(true);

      // Use device_service URL which includes the correct port and scheme from discovery
      // Fall back to constructing URL from ip_address if device_service is not available.
      // When constructing the URL, prefer HTTPS and only fall back to HTTP if necessary.
      const discoveredUrl = device.device_service;
      const ipAddress = device.ip_address;

      const isValidHost = isValidDeviceHost(ipAddress);
      if (!isValidHost) {
        throw new Error('Invalid device IP address');
      }

      const buildDeviceServiceUrl = (scheme, host) => {
        try {
          const url = new URL(`${scheme}://${host}/onvif/device_service`);
          return url.toString();
        } catch (e) {
          throw new Error('Invalid device URL');
        }
      };

      const httpsFallbackUrl = buildDeviceServiceUrl('https', ipAddress);
      const httpFallbackUrl = buildDeviceServiceUrl('http', ipAddress);

      const attemptFetch = (deviceUrl) => {
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
      };

      if (discoveredUrl) {
        // If discovery provided a full URL (including scheme), use it as-is.
        return attemptFetch(discoveredUrl);
      }
      // In security-sensitive environments, set this to true (or wire it to configuration)
      // to disable falling back from HTTPS to HTTP for device profile discovery.
      const DISABLE_HTTP_FALLBACK =
        typeof window !== 'undefined' &&
        window.LIGHTNVR_CONFIG &&
        typeof window.LIGHTNVR_CONFIG.disableOnvifHttpFallback === 'boolean'
          ? window.LIGHTNVR_CONFIG.disableOnvifHttpFallback
          : false;
      // Try HTTPS first; if that fails, optionally fall back to HTTP.
      return attemptFetch(httpsFallbackUrl).catch((error) => {
        if (DISABLE_HTTP_FALLBACK) {
          // In strict mode, propagate the original error without attempting insecure HTTP.
          throw error;
        }
        showStatusMessage(
          t('streams.warningFallingBackToHttp', 'HTTPS connection failed; retrying over HTTP. Credentials may be sent in plain text.'),
          'warning',
          5000
        );
        return attemptFetch(httpFallbackUrl);
      });
    },
    onSuccess: (data) => {
      setDeviceProfiles(data.profiles || []);
      setIsLoadingProfiles(false);
    },
    onError: (error) => {
      showStatusMessage(t('streams.errorLoadingDeviceProfiles', { message: error.message }), 'error', 5000);
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
      onSuccess: (data) => {
        if (data.success) {
          showStatusMessage(t('streams.connectionSuccessful'), 'success', 3000);
          if (selectedDevice) {
            getDeviceProfiles(selectedDevice);
          }
        } else {
          showStatusMessage(t('streams.connectionFailed', { message: data.message }), 'error', 5000);
          setIsLoadingProfiles(false);
        }
      },
      onError: (error) => {
        showStatusMessage(t('streams.errorTestingConnection', { message: error.message }), 'error', 5000);
        setIsLoadingProfiles(false);
      }
    }
  );



  // Submit ONVIF device
  const submitOnvifDevice = () => {
    if (!selectedDevice || !selectedProfile || !customStreamName.trim()) {
      showStatusMessage(t('streams.missingRequiredInformation'), 'error', 5000);
      return;
    }

    setIsAddingStream(true);

    // Prepare stream data
    const streamData = {
      name: customStreamName.trim(),
      // ONVIF profiles expose the media endpoint as `stream_uri`, while our
      // backend `Stream` model expects this value under the generic `url` field.
      // This mapping intentionally translates the ONVIF profile shape to the
      // stream configuration shape expected by the API.
      url: selectedProfile.stream_uri,
      enabled: true,
      streaming_enabled: true,
      // Note: width, height, fps, and codec are auto-detected from the stream
      // (ONVIF discovery on the backend will also populate these from the camera profile)
      protocol: '0', // TCP
      priority: '5', // Medium
      segment_duration: 30,
      record: true,
      record_audio: true,
      backchannel_enabled: false,
      // Backend expects camelCase key 'isOnvif'
      isOnvif: true
    };

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
    });
  };

  // Start ONVIF discovery
  const startOnvifDiscovery = () => {
    onvifDiscoveryMutation.mutate({ network: onvifNetworkOverride || 'auto' });
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

  // Add ONVIF device as stream with selected profile
  const addOnvifDeviceAsStream = (profile) => {
    setSelectedProfile(profile);
    setCustomStreamName(`${selectedDevice.name || t('streams.onvifDefaultName')}_${profile.name || t('streams.defaultStreamName')}`);
    setShowCustomNameInput(true);
  };

  // Check if a discovered ONVIF device is already added as a stream
  const isDeviceAlreadyAdded = (device) => {
    return streams.some(stream => {
      if (!stream.url || !device.ip_address) return false;
      try {
        const url = new URL(stream.url);
        return url.hostname === device.ip_address;
      } catch {
        return stream.url.includes(device.ip_address);
      }
    });
  };

  /**
   * Build the ONVIF device_service URL for a discovered device.
   * Returns a string URL on success, or null if the address is invalid
   * (and shows an appropriate status message).
   */
  const buildOnvifDeviceUrl = (device) => {
    let deviceUrl = device && device.device_service;
    if (!deviceUrl) {
      if (!device || !device.ip_address) {
        showStatusMessage(
          t('streams.onvifInvalidAddress') || 'ONVIF device IP address is missing or invalid.',
          'error'
        );
        return null;
      }
      try {
        // Construct a well-formed URL using the browser URL parser
        const urlObj = new URL(
          '/onvif/device_service',
          'http://' + String(device.ip_address).trim()
        );
        deviceUrl = urlObj.toString();
      } catch (e) {
        showStatusMessage(
          t('streams.onvifInvalidAddress') || 'ONVIF device IP address is missing or invalid.',
          'error'
        );
        return null;
      }
    }
    return deviceUrl;
  };

  // Test ONVIF connection
  const testOnvifConnection = (device) => {
    // Store the selected device first
    setSelectedDevice(device);

    // Build the device_service URL (using discovery data or ip_address fallback)
    const deviceUrl = buildOnvifDeviceUrl(device);
    if (!deviceUrl) {
      return;
    }
    const isInsecure = typeof deviceUrl === 'string' && deviceUrl.startsWith('http://');

    // Then make the API call
    testOnvifConnectionMutation.mutate({
      url: deviceUrl,
      username: onvifCredentials.username,
      password: onvifCredentials.password,
      insecure: isInsecure
    });
  };

  return (
    <section id="streams-page" className="page">
      <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 className="text-xl font-bold">{t('nav.streams')}</h2>
        <div className="controls flex items-center space-x-2">
          {!canModifyStreams && userRole && (
            <span className="text-sm text-muted-foreground italic mr-2">
              {t('streams.readOnlyInsufficientPermissions')}
            </span>
          )}
          {canModifyStreams && (
            <>
              {!selectionMode && (
                <button
                  className="flex items-center gap-1.5 text-sm px-2 py-1 rounded hover:bg-muted/70 transition-colors text-muted-foreground focus:outline-none"
                  onClick={() => setSelectionMode(true)}
                  title={t('streams.enterSelectionMode')}
                >
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                      d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
                  </svg>
                  {t('streams.select')}
                </button>
              )}
              <button
                  id="discover-onvif-btn"
                  className="btn-secondary focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={openOnvifModal}
              >
                {t('streams.discoverOnvifCameras')}
              </button>
              <button
                  id="add-stream-btn"
                  className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={openAddStreamModal}
              >
                {t('streams.addStream')}
              </button>
            </>
          )}
        </div>
      </div>

      <div className="mb-4 border-b border-border" role="tablist" aria-label={t('nav.streams')}>
        <div className="flex gap-2">
          <button
            type="button"
            id="streams-tab"
            role="tab"
            aria-selected={activeTab === 'streams'}
            aria-controls="streams-panel"
            className={`rounded-t-lg px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'streams'
                ? 'bg-card text-card-foreground border border-border border-b-0 -mb-px'
                : 'text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setActiveTab('streams')}
          >
            {t('nav.streams')}
          </button>
          <button
            type="button"
            id="health-tab"
            role="tab"
            aria-selected={activeTab === 'health'}
            aria-controls="health-panel"
            className={`rounded-t-lg px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'health'
                ? 'bg-card text-card-foreground border border-border border-b-0 -mb-px'
                : 'text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setActiveTab('health')}
          >
            {t('nav.health')}
          </button>
        </div>
      </div>

      {activeTab === 'health' ? (
        <div role="tabpanel" id="health-panel" aria-labelledby="health-tab">
          <HealthView />
        </div>
      ) : (
        <div role="tabpanel" id="streams-panel" aria-labelledby="streams-tab">
          <ContentLoader
              isLoading={isLoading}
              hasData={hasData}
              loadingMessage={t('streams.loadingStreams')}
              emptyMessage={canModifyStreams ? t('streams.noStreamsConfiguredYetAdd') : t('streams.noStreamsConfiguredYet')}
          >
        <div className="streams-container bg-card text-card-foreground rounded-lg shadow overflow-hidden">
          {/* Bulk action toolbar — visible in selection mode */}
          {canModifyStreams && selectionMode && (
            <div className="flex items-center gap-3 px-4 py-2 border-b border-border">
              <span className="text-sm text-muted-foreground">
                {selectedStreams.size > 0
                  ? t('streams.selectedCount', { count: selectedStreams.size })
                  : t('streams.selectStreams')}
              </span>
              <button
                className="px-3 py-1 text-sm rounded border transition-colors disabled:opacity-50"
                style={{borderColor: 'hsl(var(--success))', color: 'hsl(var(--success))', background: 'transparent'}}
                onMouseOver={e => e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'}
                onMouseOut={e => e.currentTarget.style.backgroundColor = 'transparent'}
                disabled={isBulkWorking || selectedStreams.size === 0}
                onClick={handleBulkEnable}
              >
                {isBulkWorking ? t('streams.working') : t('streams.enableSelected')}
              </button>
              <button
                className="px-3 py-1 text-sm rounded border border-input bg-background hover:bg-accent transition-colors disabled:opacity-50"
                disabled={isBulkWorking || selectedStreams.size === 0}
                onClick={handleBulkDisable}
              >
                {isBulkWorking ? t('streams.working') : t('streams.disableSelected')}
              </button>
              <button
                className="px-3 py-1 text-sm rounded border transition-colors disabled:opacity-50"
                style={{borderColor: 'hsl(var(--danger))', color: 'hsl(var(--danger))', background: 'transparent'}}
                onMouseOver={e => e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'}
                onMouseOut={e => e.currentTarget.style.backgroundColor = 'transparent'}
                disabled={isBulkWorking || selectedStreams.size === 0}
                onClick={handleBulkDelete}
              >
                {isBulkWorking ? t('streams.working') : t('streams.deleteSelected')}
              </button>
              <button
                className="ml-auto text-sm text-muted-foreground hover:text-foreground transition-colors"
                onClick={exitSelectionMode}
              >
                {t('common.cancel')}
              </button>
            </div>
          )}
          <div className="overflow-x-auto">
            <table id="streams-table" className="min-w-full divide-y divide-border">
              <thead className="bg-muted">
              <tr>
                {canModifyStreams && selectionMode && (
                  <th className="w-8 px-3 py-3">
                    <input
                      type="checkbox"
                      className="w-4 h-4 rounded cursor-pointer"
                      checked={selectedStreams.size > 0 && selectedStreams.size === (sortedStreams || []).length}
                      ref={el => { if (el) el.indeterminate = selectedStreams.size > 0 && selectedStreams.size < (sortedStreams || []).length; }}
                      onChange={toggleSelectAll}
                      title={t('streams.selectAll')}
                    />
                  </th>
                )}
                <th className="w-8 px-2 py-3"></th>
                {[
                  { key: 'name',       label: t('common.name') },
                  { key: 'status',     label: t('common.status') },
                  { key: 'url',        label: t('common.url') },
                  { key: 'resolution', label: t('streams.resolution') },
                  { key: 'fps',        label: t('streams.fps') },
                  { key: 'recording',  label: t('streams.recording') },
                ].map(({ key, label }) => (
                  <th
                    key={key}
                    className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider cursor-pointer select-none hover:text-foreground"
                    onClick={() => handleSort(key)}
                  >
                    <span className="inline-flex items-center gap-1">
                      {label}
                      <span className="inline-flex flex-col leading-none" style={{fontSize: '0.6rem', lineHeight: 1}}>
                        <span style={{opacity: sortColumn === key && sortDirection === 'asc' ? 1 : 0.3}}>▲</span>
                        <span style={{opacity: sortColumn === key && sortDirection === 'desc' ? 1 : 0.3}}>▼</span>
                      </span>
                    </span>
                  </th>
                ))}
                <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.actions')}</th>
              </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border">
              {sortedStreams.map(stream => {
                const isExpanded = expandedStreams[stream.name];
                const statusColor =
                  stream.status === 'Running'      ? 'hsl(var(--success))' :
                  stream.status === 'Starting'     ? 'hsl(var(--warning, 45 93% 47%))' :
                  stream.status === 'Reconnecting' ? 'hsl(var(--warning, 45 93% 47%))' :
                  stream.status === 'Error'        ? 'hsl(var(--danger))' :
                  stream.status === 'Stopping'     ? 'hsl(var(--warning, 45 93% 47%))' :
                  stream.status === 'Stopped'      ? 'hsl(var(--danger))' :
                  'hsl(var(--muted-foreground))';
                const statusLabel =
                  stream.status === 'Running'      ? t('streams.running')      :
                  stream.status === 'Starting'     ? t('streams.starting')     :
                  stream.status === 'Reconnecting' ? t('streams.reconnecting') :
                  stream.status === 'Error'        ? t('streams.error')        :
                  stream.status === 'Stopping'     ? t('streams.stopping')     :
                  stream.status === 'Stopped'      ? t('streams.stopped')      :
                  (stream.status || t('common.unknown'));
                const hasAdminLauncher = !shouldHideCredentials && /^https?:\/\//i.test(stream.admin_url || '');

                return (
                  <Fragment key={stream.name}>
                    <tr className="hover:bg-muted/50 cursor-pointer" onClick={() => toggleStreamExpand(stream.name)}>
                      {/* Bulk-select checkbox — only in selection mode */}
                      {canModifyStreams && selectionMode && (
                        <td className="w-8 px-3 py-4" onClick={e => e.stopPropagation()}>
                          <input
                            type="checkbox"
                            className="w-4 h-4 rounded cursor-pointer"
                            checked={selectedStreams.has(stream.name)}
                            onChange={e => toggleSelect(e, stream.name)}
                          />
                        </td>
                      )}
                      {/* Expand/collapse chevron */}
                      <td className="w-8 px-2 py-4 text-center text-muted-foreground">
                        <svg
                          className={`w-4 h-4 inline-block transition-transform duration-200 ${isExpanded ? 'rotate-90' : ''}`}
                          fill="none" stroke="currentColor" viewBox="0 0 24 24"
                        >
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7" />
                        </svg>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <div className="flex items-center">
                          <span className="w-2 h-2 rounded-full mr-2 flex-shrink-0" style={{backgroundColor: stream.enabled ? 'hsl(var(--success))' : 'hsl(var(--danger))'}} title={stream.enabled ? t('common.enabled') : t('common.disabled')}></span>
                          {stream.name}
                        </div>
                      </td>
                      {/* Status badge */}
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span
                          className="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium"
                          style={{
                            backgroundColor: statusColor,
                            color: 'white',
                            opacity: stream.enabled ? 1 : 0.6
                          }}
                        >
                          {statusLabel}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap" onClick={(e) => e.stopPropagation()}>
                        <div className="flex items-center gap-1.5">
                          <span className="font-mono text-xs truncate max-w-xs" title={revealedUrls.has(stream.name) ? stream.url : obfuscateUrlCredentials(stream.url)}>
                            {revealedUrls.has(stream.name) ? stream.url : obfuscateUrlCredentials(stream.url)}
                          </span>
                          {urlHasCredentials(stream.url) && !shouldHideCredentials && (
                            <button
                              type="button"
                              className="flex-shrink-0 p-0.5 rounded text-muted-foreground hover:text-foreground transition-colors focus:outline-none"
                              onClick={(e) => toggleUrlReveal(stream.name, e)}
                              title={revealedUrls.has(stream.name) ? t('streams.hideCredentials') : t('streams.showCredentials')}
                            >
                              {revealedUrls.has(stream.name) ? (
                                /* eye-off */
                                <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 4.411m0 0L21 21" />
                                </svg>
                              ) : (
                                /* eye */
                                <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                </svg>
                              )}
                            </button>
                          )}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">{stream.width}x{stream.height}</td>
                      <td className="px-6 py-4 whitespace-nowrap">{stream.fps}</td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        {stream.record ? t('common.enabled') : t('common.disabled')}
                        {stream.record && stream.record_on_schedule ? ` (${t('streams.schedule')})` : ''}
                        {stream.detection_based_recording ? ` (${t('streams.detection')})` : ''}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap" onClick={(e) => e.stopPropagation()}>
                        <div className="flex space-x-2">
                          {hasAdminLauncher && (
                            <a
                              className="p-1 rounded-full focus:outline-none"
                              style={{color: 'hsl(var(--primary))'}}
                              onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'}
                              onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                              href={stream.admin_url}
                              target="_blank"
                              rel="noopener noreferrer"
                              title={t('streams.openCameraAdminPage')}
                              aria-label={t('streams.openAdminPageFor', { name: stream.name })}
                            >
                              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M14 3h7m0 0v7m0-7L10 14"></path>
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 5h5M5 5v14h14v-5"></path>
                              </svg>
                            </a>
                          )}
                          {/* Edit button - only show if user can modify streams */}
                          {canModifyStreams && (
                            <button
                                className="p-1 rounded-full focus:outline-none"
                                style={{color: 'hsl(var(--primary))'}}
                                onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'}
                                onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                                onClick={() => openEditStreamModal(stream.name)}
                                title={t('common.edit')}
                            >
                              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                              </svg>
                            </button>
                          )}
                          {/* Clone button - only show if user can modify streams */}
                          {canModifyStreams && (
                            <button
                                className="p-1 rounded-full focus:outline-none"
                                style={{color: 'hsl(var(--success))'}}
                                onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'}
                                onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                                onClick={(e) => { e.stopPropagation(); openCloneStreamModal(stream.name); }}
                                title={t('streams.cloneStream')}
                            >
                              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                              </svg>
                            </button>
                          )}
                          {/* Enable/Disable toggle button - only show if user can modify streams */}
                          {canModifyStreams && (
                            <button
                                className="p-1 rounded-full focus:outline-none"
                                style={{color: stream.enabled ? 'hsl(var(--success))' : 'hsl(var(--muted-foreground))'}}
                                onMouseOver={(e) => e.currentTarget.style.backgroundColor = stream.enabled ? 'hsl(var(--success) / 0.1)' : 'hsl(var(--muted-foreground) / 0.1)'}
                                onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                                onClick={(e) => { e.stopPropagation(); handleToggleStreamEnabled(stream); }}
                                title={stream.enabled ? t('streams.toggleDisable') : t('streams.toggleEnable')}
                            >
                              {/* Power icon */}
                              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
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
                                title={t('common.delete')}
                            >
                              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                                <path fillRule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clipRule="evenodd"></path>
                              </svg>
                            </button>
                          )}
                          {/* Show dash when no actions available */}
                          {!canModifyStreams && !hasAdminLauncher && (
                            <span className="text-muted-foreground">—</span>
                          )}
                        </div>
                      </td>
                    </tr>
                    {/* Expandable detail row */}
                    {isExpanded && (
                      <tr className="bg-muted/30">
                        <td colSpan={canModifyStreams && selectionMode ? 9 : 8} className="px-6 py-3">
                          <div className="flex flex-wrap gap-4 text-sm">
                            {/* Enabled */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('common.enabled')}:</span>
                              <span style={{color: stream.enabled ? 'hsl(var(--success))' : 'hsl(var(--danger))'}}>
                                {stream.enabled ? t('common.yes') : t('common.no')}
                              </span>
                            </div>
                            {/* Streaming */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('streams.streaming')}:</span>
                              <span style={{color: stream.streaming_enabled ? 'hsl(var(--success))' : 'hsl(var(--muted-foreground))'}}>
                                {stream.streaming_enabled ? t('common.active') : t('streams.off')}
                              </span>
                            </div>
                            {/* Recording */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('streams.recording')}:</span>
                              <span style={{color: stream.record ? 'hsl(var(--success))' : 'hsl(var(--muted-foreground))'}}>
                                {stream.record
                                  ? (stream.record_on_schedule ? t('streams.scheduled') : t('streams.continuous'))
                                  : t('streams.off')}
                              </span>
                            </div>
                            {/* Detection */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('streams.detection')}:</span>
                              <span style={{color: stream.detection_based_recording ? 'hsl(var(--success))' : 'hsl(var(--muted-foreground))'}}>
                                {stream.detection_based_recording
                                  ? (stream.detection_model ? stream.detection_model.split('/').pop() : t('common.enabled'))
                                  : t('streams.off')}
                              </span>
                            </div>
                            {/* Codec */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('streams.codec')}:</span>
                              <span>{stream.codec || t('streams.notAvailable')}</span>
                            </div>
                            {/* Priority */}
                            <div className="flex items-center gap-1.5">
                              <span className="font-medium text-muted-foreground">{t('streams.priority')}:</span>
                              <span>{stream.priority || t('streams.notAvailable')}</span>
                            </div>
                            {/* Segment duration */}
                            {stream.record && (
                              <div className="flex items-center gap-1.5">
                                <span className="font-medium text-muted-foreground">{t('streams.segment')}:</span>
                                <span>{stream.segment_duration
                                  ? (stream.segment_duration >= 60
                                    ? `${Math.round(stream.segment_duration / 60)}min`
                                    : `${stream.segment_duration}s`)
                                  : t('streams.notAvailable')}</span>
                              </div>
                            )}
                          </div>
                        </td>
                      </tr>
                    )}
                  </Fragment>
                );
              })}
              </tbody>
            </table>
          </div>
        </div>
      </ContentLoader>
        </div>
      )}

      {deleteModalVisible && streamToDelete && (
        <StreamDeleteModal
            streamId={streamToDelete.name}
            streamName={streamToDelete.name}
            onClose={closeDeleteModal}
            onDisable={disableStream}
            onDelete={deleteStream}
            isDeleting={deleteStreamMutation.isPending}
            isDisabling={disableStreamMutation.isPending}
        />
      )}

      {bulkActionModal && (
        <StreamBulkActionModal
            action={bulkActionModal}
            streamNames={[...selectedStreams]}
            onClose={() => setBulkActionModal(null)}
            onConfirm={
              bulkActionModal === 'delete'  ? executeBulkDelete  :
              bulkActionModal === 'enable'  ? executeBulkEnable  :
                                              executeBulkDisable
            }
            isWorking={isBulkWorking}
        />
      )}

      {modalVisible && (
        <StreamConfigModal
          isEditing={isEditing}
          isCloning={isCloning}
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
          hideCredentials={shouldHideCredentials}
        />
      )}

      {onvifModalVisible && (
        <div id="onvif-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">{t('streams.onvifCameraDiscovery')}</h3>
              <span className="text-2xl cursor-pointer" onClick={() => setOnvifModalVisible(false)}>×</span>
            </div>
            <div className="p-4">
              <div className="mb-4 p-3 bg-muted/50 rounded-lg">
                <div className="flex items-center gap-3">
                  <label htmlFor="onvif-network-override" className="text-sm font-medium whitespace-nowrap">
                    {t('streams.discoveryNetwork')}
                  </label>
                  <input
                    type="text"
                    id="onvif-network-override"
                    className="p-2 border border-input rounded bg-background text-foreground w-full max-w-xs text-sm"
                    value={onvifNetworkOverride}
                    onChange={(e) => setOnvifNetworkOverride(e.target.value)}
                    disabled={isDiscovering}
                    placeholder={t('streams.auto')}
                  />
                  <span className="text-xs text-muted-foreground whitespace-nowrap">
                    {t('streams.discoveryNetworkHelp')}
                  </span>
                </div>
              </div>
              <div className="mb-4 flex justify-between items-center">
                <h4 className="text-md font-medium">{t('streams.discoveredDevices')}</h4>
                <button
                    id="discover-btn"
                    className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary"
                    onClick={startOnvifDiscovery}
                    disabled={isDiscovering}
                    type="button"
                >
                  {isDiscovering ? (
                    <span className="flex items-center">
                      {t('streams.discovering')}
                      <span className="ml-1 flex space-x-1">
                        <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                        <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                      </span>
                    </span>
                  ) : t('streams.startDiscovery')}
                </button>
              </div>

              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-border">
                  <thead className="bg-muted">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.ipAddress')}</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.manufacturer')}</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.model')}</th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.actions')}</th>
                  </tr>
                  </thead>
                  <tbody className="bg-card divide-y divide-border">
                  {discoveredDevices.length === 0 ? (
                    <tr>
                      <td colSpan="4" className="px-6 py-4 text-center text-muted-foreground">
                        {isDiscovering ? (
                          <div className="flex items-center justify-center">
                            <span>{t('streams.discoveringDevices')}</span>
                            <span className="ml-1 flex space-x-1">
                                <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                                <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                                <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-muted-foreground rounded-full"></span>
                              </span>
                          </div>
                        ) : t('streams.noDevicesDiscoveredYet')}
                      </td>
                    </tr>
                  ) : discoveredDevices.map(device => {
                    const alreadyAdded = isDeviceAlreadyAdded(device);
                    const isConnecting = isLoadingProfiles && selectedDevice && selectedDevice.ip_address === device.ip_address;
                    const baseRowClass = 'hover:bg-muted/50 transition-opacity';
                    const rowClassName = alreadyAdded ? `${baseRowClass} opacity-60` : baseRowClass;
                    return (
                      <tr key={device.ip_address} className={rowClassName}>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <span>{device.ip_address}</span>
                          {alreadyAdded && (
                            <span className="ml-2 inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-full bg-muted text-muted-foreground border border-border">
                              {t('streams.alreadyAdded')}
                            </span>
                          )}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">{device.manufacturer || t('common.unknown')}</td>
                        <td className="px-6 py-4 whitespace-nowrap">{device.model || t('common.unknown')}</td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <button
                              className={alreadyAdded ? 'btn-secondary focus:outline-none' : 'btn-primary focus:outline-none'}
                              onClick={() => testOnvifConnection(device)}
                              disabled={isConnecting}
                              type="button"
                              title={alreadyAdded ? t('streams.deviceAlreadyInUseTitle') : undefined}
                          >
                            {isConnecting ? (
                              <span className="flex items-center">
                                  {t('common.loading')}
                                  <span className="ml-1 flex space-x-1">
                                    <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-current rounded-full"></span>
                                    <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-current rounded-full"></span>
                                    <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-current rounded-full"></span>
                                  </span>
                                </span>
                            ) : alreadyAdded ? t('streams.connectAnyway') : t('streams.connect')}
                          </button>
                        </td>
                      </tr>
                    );
                  })}
                  </tbody>
                </table>
              </div>

              <div className="mt-6 mb-4">
                <h4 className="text-md font-medium mb-2">{t('streams.authentication')}</h4>
                <p className="text-sm text-muted-foreground mb-3">
                  {t('streams.onvifAuthenticationHelp')}
                </p>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="form-group">
                    <label htmlFor="onvif-username" className="block text-sm font-medium mb-1">{t('auth.username')}</label>
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
                    <label htmlFor="onvif-password" className="block text-sm font-medium mb-1">{t('auth.password')}</label>
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
                  <h4 className="text-md font-medium mb-2">{t('streams.availableProfilesFor', { ip: selectedDevice.ip_address })}</h4>
                  <div className="overflow-x-auto">
                    <table className="min-w-full divide-y divide-border">
                      <thead className="bg-muted">
                      <tr>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.name')}</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.resolution')}</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.encoding')}</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.fps')}</th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.actions')}</th>
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
                              {t('streams.addAsStream')}
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
                {t('common.close')}
              </button>
            </div>
          </div>
        </div>
      )}

      {showCustomNameInput && (
        <div id="custom-name-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">{t('streams.streamName')}</h3>
              <span className="text-2xl cursor-pointer" onClick={() => setShowCustomNameInput(false)}>×</span>
            </div>
            <div className="p-4">
              <div className="mb-4">
                <label htmlFor="custom-stream-name" className="block text-sm font-medium mb-1">{t('streams.enterNameForStream')}</label>
                <input
                    type="text"
                    id="custom-stream-name"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                    value={customStreamName}
                    onChange={(e) => setCustomStreamName(e.target.value)}
                />
                <p className="mt-1 text-sm text-muted-foreground">
                  {t('streams.streamNameHelp')}
                </p>
              </div>
            </div>
            <div className="flex justify-end p-4 border-t border-border space-x-2">
              <button
                  className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                  onClick={() => setShowCustomNameInput(false)}
                  type="button"
              >
                {t('common.cancel')}
              </button>
              <button
                  className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary"
                  onClick={submitOnvifDevice}
                  type="button"
                  disabled={!customStreamName.trim() || isAddingStream}
              >
                {isAddingStream ? (
                  <span className="flex items-center">
                    {t('common.adding')}
                    <span className="ml-1 flex space-x-1">
                      <span className="animate-pulse delay-0 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span className="animate-pulse delay-150 h-1.5 w-1.5 bg-white rounded-full"></span>
                      <span className="animate-pulse delay-300 h-1.5 w-1.5 bg-white rounded-full"></span>
                    </span>
                  </span>
                ) : t('streams.addStream')}
              </button>
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
