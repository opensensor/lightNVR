import {getSystemSettings} from '../../api/system.ts';
import { evidenceJSON } from '../../api/evidence.ts';
import {deviceRequest} from '../../api/device.ts';
/**
 * LightNVR Web Interface StreamsView Component
 * React component for the streams page
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.tsx';
import { ContentLoader } from './LoadingIndicator.tsx';
import { StreamDeleteModal } from './StreamDeleteModal.tsx';
import { StreamBulkActionModal } from './StreamBulkActionModal.tsx';
import { StreamConfigModal } from './StreamConfigModal.tsx';
import { StreamCard } from './StreamCard.tsx';
import { HealthView } from './HealthView.tsx';
import { FleetView } from './FleetView.tsx';
import { EventRoutingView } from './EventRoutingView.tsx';
import { validateSession } from '../../utils/auth-utils.ts';
import {
  useQuery,
  useMutation,
  useQueryClient,
  usePostMutation,
  fetchJSON
} from '../../query-client.ts';
import { useI18n } from '../../i18n.ts';
import {
  eptzConfigFromForm,
  eptzFormFields,
  serializeEptzConfig,
} from '../../utils/eptz-config.ts';
import {
  getInitialStreamAvailability,
  hasStreamSummaryFilters,
} from '../../utils/stream-availability.ts';
import type { ClientConfig, StreamAvailabilityFilter } from '../../types/core.ts';
import type {
  DetectionModelsResponse,
  ExpandedSections,
  OnvifCredentials,
  OnvifDevice,
  OnvifDevicesResponse,
  OnvifProfile,
  OnvifProfilesResponse,
  OnvifTestRequest,
  OnvifTestResponse,
  OnvifStreamCreatePayload,
  StreamBulkAction,
  StreamFieldChange,
  StreamFieldTarget,
  StreamFormState,
  StreamFull,
  StreamFullResponse,
  StreamListItem,
  StreamListResponse,
  StreamRequestError,
  StreamSavePayload,
  StreamTestRequest,
  StreamTestResponse,
} from '../../types/streams.ts';
import { normalizePlaybackTransport } from '../../utils/playback-transport.ts';
import {
  DEFAULT_OBSERVATION_FORM_FIELDS,
  observationFormFields,
  observationPayloadFields,
  validateObservationForm,
} from '../../utils/observation-evidence.ts';
import { getStreamFull, listStreamSummaries, updateStream } from '../../api/streams.ts';
import { createStream, probeStream, deleteStream as requestDeleteStream } from '../../api/mediaLifecycle.ts';
import type { StreamSummaryQuery, StreamUpdateResult } from '../../api/streams.ts';
import { useEscapeKey } from '../../hooks/useEscapeKey.ts';

declare global {
  interface Window {
    /** Optional page-level overrides; set outside this codebase, if at all. */
    LIGHTNVR_CONFIG?: { disableOnvifHttpFallback?: unknown };
  }
}

/**
 * StreamsView component
 * @returns {JSX.Element} StreamsView component
 */
function getInitialWorkspaceTab() {
  if (typeof window === 'undefined') return 'streams';
  const view = new URLSearchParams(window.location.search).get('view');
  return view === 'inventory' || view === 'health' || view === 'events' ? view : 'streams';
}

const normalizeRecordingSchedule = (recordingSchedule: boolean[], recordOnSchedule: boolean): boolean[] => {
  if (!recordOnSchedule) {
    return Array(168).fill(true);
  }
  if (Array.isArray(recordingSchedule) && recordingSchedule.length === 168) {
    return recordingSchedule;
  }
  return Array(168).fill(true);
};

/** Build a typed wire DTO field by field; form/UI metadata never enters PUT. */
export function streamSavePayloadFromForm(currentStream: StreamFormState, isEditing: boolean): StreamSavePayload {
  const streamData: StreamSavePayload = {
    name: currentStream.name.trim(),
    url: currentStream.url,
    admin_url: currentStream.adminUrl || '',
    enabled: currentStream.enabled,
    streaming_enabled: currentStream.streamingEnabled,
    playback_transport: currentStream.playbackTransport || 'auto',
    // width/height/fps are read-only (auto-detected). codec is user-settable
    // as a hint (select Auto / H.264 / H.265) — the detection thread still
    // overwrites it with the real value from the source if they differ,
    // and triggers a go2rtc re-registration so WebRTC negotiation keeps
    // working after the correction (#374).
    codec: (currentStream.codec || '').toLowerCase(),
    protocol: parseInt(currentStream.protocol, 10),
    priority: parseInt(currentStream.priority, 10),
    segment_duration: parseInt(String(currentStream.segment), 10),
    record: currentStream.record,
    detection_based_recording: currentStream.detectionEnabled,
    detection_model: currentStream.detectionModel,
    // Persist ONVIF flag expected by backend (camelCase key)
    isOnvif: !!currentStream.isOnvif,
    // ONVIF credentials (for motion detection, PTZ, etc.)
    onvif_username: currentStream.onvifUsername || '',
    onvif_password: currentStream.onvifPassword || '',
    onvif_profile: currentStream.onvifProfile || '',
    onvif_port: parseInt(String(currentStream.onvifPort), 10) || 0,
    detection_threshold: parseInt(String(currentStream.detectionThreshold), 10),
    detection_interval: parseInt(String(currentStream.detectionInterval), 10),
    pre_detection_buffer: parseInt(String(currentStream.preBuffer), 10),
    post_detection_buffer: parseInt(String(currentStream.postBuffer), 10),
    record_audio: currentStream.recordAudio,
    audio_voice_enhancement: !!currentStream.audioVoiceEnhancement,
    backchannel_enabled: currentStream.backchannelEnabled,
    // PTZ control settings
    ptz_enabled: !!currentStream.ptzEnabled,
    ptz_max_x: parseInt(String(currentStream.ptzMaxX), 10) || 0,
    ptz_max_y: parseInt(String(currentStream.ptzMaxY), 10) || 0,
    ptz_max_z: parseInt(String(currentStream.ptzMaxZ), 10) || 0,
    ptz_has_home: !!currentStream.ptzHasHome,
    // Detection object filter settings
    detection_object_filter: currentStream.detectionObjectFilter || 'none',
    detection_object_filter_list: currentStream.detectionObjectFilterList || '',
    // Retention policy settings
    retention_days: Number.isFinite(parseInt(String(currentStream.retentionDays), 10))
      ? parseInt(String(currentStream.retentionDays), 10) : -1,
    detection_retention_days: Number.isFinite(parseInt(String(currentStream.detectionRetentionDays), 10))
      ? parseInt(String(currentStream.detectionRetentionDays), 10) : -1,
    max_storage_mb: parseInt(String(currentStream.maxStorageMb), 10) || 0,
    // Tiered retention weighting
    tier_critical_multiplier: Number.isFinite(parseFloat(String(currentStream.tierCriticalMultiplier))) ? parseFloat(String(currentStream.tierCriticalMultiplier)) : 3.0,
    tier_important_multiplier: Number.isFinite(parseFloat(String(currentStream.tierImportantMultiplier))) ? parseFloat(String(currentStream.tierImportantMultiplier)) : 2.0,
    tier_ephemeral_multiplier: Number.isFinite(parseFloat(String(currentStream.tierEphemeralMultiplier))) ? parseFloat(String(currentStream.tierEphemeralMultiplier)) : 0.25,
    storage_priority: Math.min(10, Math.max(1, parseInt(String(currentStream.storagePriority), 10) || 5)),
    // Recording schedule
    record_on_schedule: !!currentStream.recordOnSchedule,
    recording_schedule: normalizeRecordingSchedule(
      currentStream.recordingSchedule,
      currentStream.recordOnSchedule
    ),
    detection_record_on_schedule: !!currentStream.detectionRecordOnSchedule,
    detection_recording_schedule: normalizeRecordingSchedule(
      currentStream.detectionRecordingSchedule,
      currentStream.detectionRecordOnSchedule
    ),
    // Tags
    tags: currentStream.tags || '',
    // Cross-stream motion trigger source
    motion_trigger_source: currentStream.motionTriggerSource || '',
    // go2rtc source override
    go2rtc_source_override: currentStream.go2rtcSourceOverride?.trim()
      ? currentStream.go2rtcSourceOverride
      : '',
    publish_url: currentStream.publishUrl || '',
    // Sub-stream URL
    sub_stream_url: currentStream.subStreamUrl || '',
    // Secondary stream used only for detection (e.g. MJPEG sub-stream)
    detection_url: currentStream.detectionUrl || '',
    // Client-side fisheye calibration; recording remains the raw source.
    eptz_config: serializeEptzConfig(eptzConfigFromForm(currentStream)),
    // INT_01 per-camera observation evidence for integrations (off by default)
    ...observationPayloadFields(currentStream)
  };

  // When editing, set is_deleted to false to allow undeleting soft-deleted streams
  if (isEditing) {
    streamData.is_deleted = false;
  }

  return streamData;
}

/** Map validated details into UI fields without spreading unknown wire properties. */
export function streamFormFromFull(stream: StreamFull): StreamFormState {
  return {
    name: stream.name, url: stream.url, enabled: stream.enabled, record: stream.record,
    canControlPrivacy: stream.can_control_privacy,
    width: stream.width, height: stream.height, fps: stream.fps, codec: stream.codec,
    protocol: String(stream.protocol), priority: String(stream.priority),
    segment: stream.segment_duration,
    detectionThreshold: stream.detection_threshold,
    detectionInterval: stream.detection_interval,
    preBuffer: stream.pre_detection_buffer, postBuffer: stream.post_detection_buffer,
    // Zones have their own endpoint and are loaded by the modal.
    detectionZones: [],
    adminUrl: stream.admin_url,
    streamingEnabled: stream.streaming_enabled,
    playbackTransport: stream.playback_transport,
    isOnvif: stream.isOnvif,
    onvifUsername: stream.onvif_username, onvifPassword: stream.onvif_password,
    onvifProfile: stream.onvif_profile, onvifPort: stream.onvif_port,
    detectionEnabled: stream.detection_based_recording,
    detectionModel: stream.detection_model,
    recordAudio: stream.record_audio,
    audioVoiceEnhancement: stream.audio_voice_enhancement,
    backchannelEnabled: stream.backchannel_enabled,
    ptzEnabled: stream.ptz_enabled, ptzMaxX: stream.ptz_max_x,
    ptzMaxY: stream.ptz_max_y, ptzMaxZ: stream.ptz_max_z, ptzHasHome: stream.ptz_has_home,
    detectionObjectFilter: stream.detection_object_filter,
    detectionObjectFilterList: stream.detection_object_filter_list,
    retentionDays: stream.retention_days, detectionRetentionDays: stream.detection_retention_days,
    maxStorageMb: stream.max_storage_mb,
    tierCriticalMultiplier: stream.tier_critical_multiplier,
    tierImportantMultiplier: stream.tier_important_multiplier,
    tierEphemeralMultiplier: stream.tier_ephemeral_multiplier,
    storagePriority: stream.storage_priority,
    recordOnSchedule: stream.record_on_schedule,
    recordingSchedule: [...stream.recording_schedule],
    detectionRecordOnSchedule: stream.detection_record_on_schedule,
    detectionRecordingSchedule: [...stream.detection_recording_schedule],
    tags: stream.tags, motionTriggerSource: stream.motion_trigger_source,
    go2rtcSourceOverride: stream.go2rtc_source_override, publishUrl: stream.publish_url,
    subStreamUrl: stream.sub_stream_url, detectionUrl: stream.detection_url,
    ...eptzFormFields(stream.eptz_config),
    ...observationFormFields(stream),
  };
}

type StreamSaveVariables =
  | { streamData: StreamSavePayload; editing: true; editorGeneration?: number }
  | { streamData: StreamSavePayload | OnvifStreamCreatePayload; editing: false; editorGeneration?: number };
type StreamSaveResult =
  | { editing: true; result: StreamUpdateResult }
  | { editing: false; result: unknown };

export function StreamsView() {
  const { t } = useI18n();
  const queryClient = useQueryClient();

  // User role state for permission-based UI
  const [userRole, setUserRole] = useState<string | null>(null);
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
  // Keep legacy global/create/ONVIF controls separate from per-camera updates.
  // While loading, default to enabled so admin/user doesn't see hidden buttons
  const canModifyStreams = roleLoading || userRole === 'admin' || userRole === 'user';
  // Check if credentials should be hidden (demo mode or viewer role)
  const shouldHideCredentials = isDemoMode || userRole === 'viewer';

  // State for streams data
  const [activeTab, setActiveTab] = useState(getInitialWorkspaceTab);
  const [modalVisible, setModalVisible] = useState(false);
  const [onvifModalVisible, setOnvifModalVisible] = useState(false);
  const [showCustomNameInput, setShowCustomNameInput] = useState(false);
  useEscapeKey(closeOnvifModal, onvifModalVisible);
  useEscapeKey(() => setShowCustomNameInput(false), showCustomNameInput);
  const [isAddingStream, setIsAddingStream] = useState(false);
  const [discoveredDevices, setDiscoveredDevices] = useState<OnvifDevice[]>([]);
  const [deviceProfiles, setDeviceProfiles] = useState<OnvifProfile[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<OnvifDevice | null>(null);
  const [selectedProfile, setSelectedProfile] = useState<OnvifProfile | null>(null);
  const [customStreamName, setCustomStreamName] = useState('');
  const [onvifCredentials, setOnvifCredentials] = useState<OnvifCredentials>({ username: '', password: '' });
  const [isDiscovering, setIsDiscovering] = useState(false);
  const [isLoadingProfiles, setIsLoadingProfiles] = useState(false);
  const [onvifNetworkOverride, setOnvifNetworkOverride] = useState('auto');
  const editorGeneration = useRef(0);
  const onvifGeneration = useRef(0);
  const onvifSelection = useRef(0);
  const selectedDeviceRef = useRef<OnvifDevice | null>(null);

  function closeOnvifModal() {
    onvifGeneration.current++;
    onvifSelection.current++;
    selectedDeviceRef.current = null;
    setOnvifModalVisible(false);
    setShowCustomNameInput(false);
    setSelectedDevice(null);
    setSelectedProfile(null);
    setDeviceProfiles([]);
    setIsLoadingProfiles(false);
    setIsDiscovering(false);
    setIsAddingStream(false);
  }

  useEffect(() => () => {
    editorGeneration.current++;
    onvifGeneration.current++;
    onvifSelection.current++;
  }, []);

  const [streamPage, setStreamPage] = useState(1);
  const [streamPageSize, setStreamPageSize] = useState(50);
  const [streamSearch, setStreamSearch] = useState('');
  const [streamAvailability, setStreamAvailability] = useState(getInitialStreamAvailability);
  const [debouncedStreamSearch, setDebouncedStreamSearch] = useState('');
  const DEFAULT_SORT_COLUMN = 'name';
  const [sortColumn, setSortColumn] = useState<NonNullable<StreamSummaryQuery['sort_by']>>(DEFAULT_SORT_COLUMN);
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');

  useEffect(() => {
    if (typeof window === 'undefined') return;
    const url = new URL(window.location.href);
    if (activeTab === 'streams') url.searchParams.delete('view');
    else url.searchParams.set('view', activeTab);
    window.history.replaceState({}, '', url);
  }, [activeTab]);

  useEffect(() => {
    const timer = window.setTimeout(() => {
      setDebouncedStreamSearch(streamSearch.trim());
      setStreamPage(1);
    }, 250);
    return () => window.clearTimeout(timer);
  }, [streamSearch]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    const url = new URL(window.location.href);
    if (streamAvailability === 'all') url.searchParams.delete('availability');
    else url.searchParams.set('availability', streamAvailability);
    window.history.replaceState({}, '', url);
    setStreamPage(1);
  }, [streamAvailability]);

  // Credential-reveal toggle is now owned by StreamCard (per-card state).
  // StreamsView used to track a Set of revealed streams when the page was a
  // single flat table; after the T5 card refactor each card manages its own.

  // Bulk selection state
  const [selectionMode, setSelectionMode] = useState(false);
  const [selectedStreams, setSelectedStreams] = useState<Set<string>>(new Set());
  const [isBulkWorking, setIsBulkWorking] = useState(false);
  // null | 'disable' | 'delete' — which bulk confirmation modal is open
  const [bulkActionModal, setBulkActionModal] = useState<StreamBulkAction | null>(null);

  const exitSelectionMode = () => {
    setSelectionMode(false);
    setSelectedStreams(new Set());
  };

  const toggleSelect = (e: Event, name: string) => {
    e.stopPropagation();
    setSelectedStreams(prev => {
      const next = new Set(prev);
      next.has(name) ? next.delete(name) : next.add(name);
      return next;
    });
  };

  const toggleSelectAll = (e: Event) => {
    e.stopPropagation();
    if (selectedStreams.size > 0) {
      setSelectedStreams(new Set());
    } else {
      setSelectedStreams(new Set(streams.filter(stream => stream.can_configure).map(stream => stream.name)));
    }
  };

  // Accordion expanded state for StreamConfigModal sections
  const [expandedSections, setExpandedSections] = useState<ExpandedSections>({
    basic: true,
    recording: false,
    detection: false,
    zones: false,
    motion: false,
    eptz: false,
    ptz: false,
    advanced: false,
    observation: false,
    go2rtcOverride: false,
    restream: false
  });

  const toggleSection = (section: keyof ExpandedSections) => {
    setExpandedSections(prev => ({
      ...prev,
      [section]: !prev[section]
    }));
  };

  // The exact administration guard rejects malformed rows before rendering.
  const {
    data: streamsResponse,
    isLoading,
    error: streamsError,
    refetch: refetchStreams,
  } = useQuery<StreamListResponse>({
    queryKey: ['streams', 'summary', streamPage, streamPageSize, sortColumn,
      sortDirection, debouncedStreamSearch, streamAvailability],
    queryFn: ({ signal }) => listStreamSummaries({
      surface: 'admin', include_admin_url: 'true',
      page: streamPage, page_size: streamPageSize,
      sort_by: sortColumn, sort_order: sortDirection,
      availability: streamAvailability,
      ...(debouncedStreamSearch ? { search: debouncedStreamSearch } : {}),
    }, { signal, timeout: 10000, retries: 2, retryDelay: 1000 }),
    retry: false, // The transport already owns the bounded read retry policy.
  });

  // Fetch detection models
  const {
    data: detectionModelsData
  } = useQuery<DetectionModelsResponse>({queryKey:['detectionModels'], queryFn:({signal})=>evidenceJSON('/api/detection/models',{signal,timeout:5000,retries:1}),retry:false});

  const { data: clientConfig = {} } = useQuery<ClientConfig>(
    ['client-config'],
    '/api/client-config',
    { timeout: 5000, retries: 1, retryDelay: 1000 }
  );

  const streams = streamsResponse?.streams ?? [];
  const streamTotal = streamsResponse?.total ?? 0;
  const streamTotalPages = streamsResponse?.total_pages ?? 0;
  const streamResultsFiltered = hasStreamSummaryFilters(
    debouncedStreamSearch,
    streamAvailability
  );

  const handleSort = (column: NonNullable<StreamSummaryQuery['sort_by']>) => {
    if (sortColumn === column) {
      setSortDirection(prev => prev === 'asc' ? 'desc' : 'asc');
    } else {
      setSortColumn(column);
      setSortDirection('asc');
    }
    setStreamPage(1);
  };

  const sortedStreams = streams;

  useEffect(() => {
    if (streamTotalPages > 0 && streamPage > streamTotalPages) {
      setStreamPage(streamTotalPages);
    }
  }, [streamPage, streamTotalPages]);

  // Default stream state
  const [currentStream, setCurrentStream] = useState<StreamFormState>({
    name: '',
    url: '',
    adminUrl: '',
    enabled: true,
    streamingEnabled: true,
    playbackTransport: 'auto',
    width: 1280,
    height: 720,
    fps: 15,
    codec: 'h264',
    protocol: '0',
    priority: '5',
    segment: 30,
    record: true,
    recordAudio: true,
    audioVoiceEnhancement: false,
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
    // PTZ control settings
    ptzEnabled: false,
    ptzMaxX: 0,
    ptzMaxY: 0,
    ptzMaxZ: 0,
    ptzHasHome: false,
    // Retention policy settings
    retentionDays: -1,
    detectionRetentionDays: -1,
    maxStorageMb: 0,
    ...DEFAULT_OBSERVATION_FORM_FIELDS,
    // Recording schedule
    recordOnSchedule: false,
    recordingSchedule: Array(168).fill(true),
    detectionRecordOnSchedule: false,
    detectionRecordingSchedule: Array(168).fill(true),
    // Tags
    tags: '',
    // Cross-stream motion trigger: name of another stream whose ONVIF motion
    // events trigger recording on this stream (e.g., PTZ slaved to fixed wide lens)
    motionTriggerSource: '',
    go2rtcSourceOverride: '',
    publishUrl: '',
    subStreamUrl: '',
    detectionUrl: '',
    ...eptzFormFields('')
  });
  const [isEditing, setIsEditing] = useState(false);
  const [isCloning, setIsCloning] = useState(false);

  // State for delete modal
  const [deleteModalVisible, setDeleteModalVisible] = useState(false);
  const [streamToDelete, setStreamToDelete] = useState<StreamListItem | null>(null);

  // Compute hasData from streams
  const hasData = streams && streams.length > 0;

  // Extract detection models from query result
  const detectionModels = detectionModelsData?.models || [];

  // Capture intent with each submission; ONVIF creation never inherits editor state.
  const saveStreamMutation = useMutation<StreamSaveResult, StreamRequestError, StreamSaveVariables>({
    mutationFn: async ({ streamData, editing }) => {
      if (editing) {
        const { name, ...body } = streamData;
        return { editing: true, result: await updateStream(name, body, { timeout: 15000 }) };
      }
      // Create new stream via POST
      return { editing: false, result: await createStream(streamData, {timeout: 45000}) };
    },
    onSuccess: (data, variables) => {
      if (variables.editing) {
        showStatusMessage(
          data.editing && data.result.status === 202
            ? t('streams.streamUpdateQueued')
            : t('streams.streamUpdatedSuccessfully', { name: variables.streamData.name }),
          'success',
          3000
        );
      } else {
        showStatusMessage(t('streams.streamAddedSuccessfully'));
      }
      if (variables.editorGeneration === editorGeneration.current) closeModal();
      // Invalidate and refetch streams data
      queryClient.invalidateQueries({ queryKey: ['streams'] });
      queryClient.invalidateQueries({ queryKey: ['fleet-cameras'] });
    },
    onError: (error, variables) => {
      if (!variables.editing) {
        const isStreamLimitError = error?.status === 409 &&
          typeof error?.message === 'string' &&
          error.message.includes('Max streams limit reached');
        showStatusMessage(
          isStreamLimitError ? error.message : t('streams.errorAddingStream', { message: error.message }),
          'error',
          isStreamLimitError ? 9000 : 5000
        );
      } else {
        queryClient.invalidateQueries({ queryKey: ['streams'] });
        queryClient.invalidateQueries({ queryKey: ['stream-full', variables.streamData.name] });
        showStatusMessage(
          t('streams.errorUpdatingStream', { message: error.message }),
          'error',
          5000
        );
      }
    }
  });

  // Mutation for testing stream connection
  const testStreamMutation = useMutation<StreamTestResponse, Error, StreamTestRequest>({
      mutationFn: (body) => probeStream(body, {timeout: 20000}),
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
  });

  // Mutation for deleting stream
  const deleteStreamMutation = useMutation<unknown, Error, { streamId: string }>({
    mutationFn: async (params) => {
      const { streamId } = params;
      return await requestDeleteStream(streamId, true, {timeout: 15000});
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

  const disableStreamMutation = useMutation<unknown, Error, { streamId: string }>({
    mutationFn: async (params) => {
      const { streamId } = params;
      return await requestDeleteStream(streamId, false, {timeout: 15000});
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

  const enableStreamMutation = useMutation<unknown, Error, string>({
    mutationFn: async (streamId) => {
      return await updateStream(streamId, { enable_disabled: true, enabled: true }, { timeout: 15000 });
    },
    onSuccess: () => {
      showStatusMessage(t('streams.streamUpdateQueued'));
      queryClient.invalidateQueries({ queryKey: ['streams'] });
    },
    onError: (error) => {
      queryClient.invalidateQueries({ queryKey: ['streams'] });
      showStatusMessage(t('streams.errorEnablingStream', { message: error.message }) || `Error enabling stream: ${error.message}`, 'error', 5000);
    }
  });

  // Promise-returning variants used by <StreamCard> → <AsyncButton>.
  // AsyncButton handles the destructive-confirm UX itself (via its
  // `confirmText` prop), so these do NOT call `window.confirm`.
  const disableStreamFromCard = (streamName: string) =>
    disableStreamMutation.mutateAsync({ streamId: streamName });
  const enableStreamFromCard = (streamName: string) =>
    enableStreamMutation.mutateAsync(streamName);

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
        // Both flags are needed so the worker persists enabled=1.
        updateStream(name, { enable_disabled: true, enabled: true }, { timeout: 15000 })
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
      showStatusMessage(t('streams.streamUpdateQueued'), 'success');
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

  // Handle form submission.
  //
  // Returns the save promise so the <AsyncButton> in StreamConfigModal can
  // track pending state and suppress the rapid double-tap re-submit that
  // #399 / PRD UXD_01 §5.1 was chasing. The click event is a MouseEvent
  // from AsyncButton (not a form-submit event), so preventDefault is a
  // best-effort no-op in the AsyncButton path; we still call it so the
  // handler works if it's ever invoked from a form submit path.
  const handleSubmit = (e?: Event) => {
    if (e && typeof e.preventDefault === 'function') e.preventDefault();

    const observationError = validateObservationForm(currentStream);
    if (observationError) {
      showStatusMessage(t(observationError), 'error', 5000);
      return Promise.resolve(undefined);
    }

    const streamData = streamSavePayloadFromForm(currentStream, isEditing);

    // Use mutation to save stream. We return mutateAsync's promise so
    // AsyncButton in StreamConfigModal can observe pending/error state.
    const savedStreamName = streamData.name;
    return saveStreamMutation.mutateAsync({ streamData, editing: isEditing, editorGeneration: editorGeneration.current }).then(async (result) => {
      // Ensure both list and details are refreshed after save
      await queryClient.invalidateQueries({ queryKey: ['stream-full', savedStreamName] });
      queryClient.invalidateQueries({ queryKey: ['streams'] });
      return result;
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

  // Open delete modal
  const openDeleteModal = (stream: StreamListItem) => {
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
    editorGeneration.current++;
    closeOnvifModal();
    setCurrentStream({
      name: '',
      url: '',
      adminUrl: '',
      enabled: true,
      streamingEnabled: true,
      playbackTransport: normalizePlaybackTransport(clientConfig.default_playback_transport),
      width: 0,
      height: 0,
      fps: 0,
      codec: '',
      protocol: '0',
      priority: '5',
      segment: 30,
      record: true,
      recordAudio: !clientConfig.audio_disabled,
      audioVoiceEnhancement: false,
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
      ptzEnabled: false,
      ptzMaxX: 0,
      ptzMaxY: 0,
      ptzMaxZ: 0,
      ptzHasHome: false,
      retentionDays: -1,
      detectionRetentionDays: -1,
      maxStorageMb: 0,
      ...DEFAULT_OBSERVATION_FORM_FIELDS,
      // Recording schedule
      recordOnSchedule: false,
      recordingSchedule: Array(168).fill(true),
      detectionRecordOnSchedule: false,
      detectionRecordingSchedule: Array(168).fill(true),
      // Tags
      tags: '',
      motionTriggerSource: '',
      go2rtcSourceOverride: '',
      publishUrl: '',
      subStreamUrl: '',
      detectionUrl: '',
      ...eptzFormFields('')
    });
    setIsEditing(false);
    setIsCloning(false);
    setModalVisible(true);
  };

  // Open edit stream modal
  const openEditStreamModal = async (streamId: string) => {
    const generation = ++editorGeneration.current;
    closeOnvifModal();
    try {
      // Use queryClient to fetch stream details (force fresh)
      const data = await queryClient.fetchQuery<StreamFullResponse>({
        queryKey: ['stream-full', streamId],
        queryFn: ({ signal }) => getStreamFull(streamId, { signal }),
        retry: false,
        staleTime: 0 // Always fetch fresh when opening modal
      });
      if (generation !== editorGeneration.current) return;
      if (!data.stream.can_control_privacy) {
        showStatusMessage(t('streams.readOnlyInsufficientPermissions'), 'error', 5000);
        return;
      }
      setCurrentStream(streamFormFromFull(data.stream));
      setIsEditing(true);
      setModalVisible(true);
    } catch (error) {
      if (generation !== editorGeneration.current) return;
      if (error instanceof Error && error.name === 'AbortError') return;
      showStatusMessage(t('streams.errorLoadingStreamDetails', { message: error instanceof Error ? error.message : t('streams.error') }));
    }
  };

  // Open clone stream modal — pre-fills from an existing stream but treats as a new one
  const openCloneStreamModal = async (streamId: string) => {
    const generation = ++editorGeneration.current;
    closeOnvifModal();
    try {
      await queryClient.invalidateQueries({ queryKey: ['stream-full', streamId] });
      const data = await queryClient.fetchQuery<StreamFullResponse>({
        queryKey: ['stream-full', streamId],
        queryFn: ({ signal }) => getStreamFull(streamId, { signal }),
        retry: false,
        staleTime: 0
      });
      if (generation !== editorGeneration.current) return;
      if (!data.stream.can_control_privacy) {
        showStatusMessage(t('streams.readOnlyInsufficientPermissions'), 'error', 5000);
        return;
      }
      setCurrentStream({
        ...streamFormFromFull(data.stream),
        name: `${t('streams.copyOf')} ${data.stream.name}`,
        width: 0, height: 0, fps: 0, codec: '', detectionZones: [],
      });
      setIsEditing(false);
      setIsCloning(true);
      setModalVisible(true);
    } catch (error) {
      if (generation !== editorGeneration.current) return;
      if (error instanceof Error && error.name === 'AbortError') return;
      showStatusMessage(t('streams.errorLoadingStreamDetails', { message: error instanceof Error ? error.message : t('streams.error') }));
    }
  };

  // Close modal
  const closeModal = () => {
    editorGeneration.current++;
    setModalVisible(false);
    setIsEditing(false);
    setIsCloning(false);
  };

  // Open ONVIF discovery modal
  const openOnvifModal = async () => {
    closeModal();
    closeOnvifModal();
    const generation = ++onvifGeneration.current;
    setOnvifModalVisible(true);
    setDiscoveredDevices([]);
    setDeviceProfiles([]);
    setSelectedDevice(null);
    setSelectedProfile(null);
    setCustomStreamName('');
    // Reopen the durable staging inbox before the operator starts another scan.
    try {
      const inventory = await deviceRequest('getOnvifDevices', { timeout: 5000 });
      if (generation !== onvifGeneration.current) return;
      setDiscoveredDevices(inventory?.devices || []);
    } catch (e) {
      console.warn('Could not load persisted ONVIF discovery inventory', e);
    }
    if (generation !== onvifGeneration.current) return;
    // Fetch the configured default network from settings
    try {
      const settings = await getSystemSettings({timeout:5000});
      if (generation !== onvifGeneration.current) return;
      if (settings && settings.onvif_discovery_network) {
        setOnvifNetworkOverride(settings.onvif_discovery_network);
      } else {
        setOnvifNetworkOverride('auto');
      }
    } catch (e) {
      if (generation !== onvifGeneration.current) return;
      console.warn('Could not fetch ONVIF discovery network setting, using auto', e);
      setOnvifNetworkOverride('auto');
    }
  };

  // Handle form input change
  const handleInputChange = (e: Event | StreamFieldChange) => {
    const { name, value, type, checked } = e.target as StreamFieldTarget;

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
        detectionModel: value as string
      }));
      return;
    }

    setCurrentStream(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  // Disable stream (soft delete). Returns a promise so <AsyncButton>
  // inside the delete modal can show a pending spinner and guard
  // rapid-tap double-submits (PRD UXD_01 §5.1 / #399).
  const disableStream = (streamId: string) => {
    return disableStreamMutation.mutateAsync({
      streamId,
    });
  };

  // Delete stream (permanent). Returns a promise for the same reasons
  // as disableStream above.
  const deleteStream = (streamId: string) => {
    return deleteStreamMutation.mutateAsync({
      streamId,
    });
  };

  // Handle ONVIF credential input change
  const handleCredentialChange = (e: Event) => {
    const { name, value } = e.target as HTMLInputElement;
    setOnvifCredentials(prev => ({
      ...prev,
      [name]: value
    }));
  };

  // Handle threshold change
  const handleThresholdChange = (e: Event) => {
    const value = (e.target as HTMLInputElement).value;
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
  const onvifDiscoveryMutation = usePostMutation<OnvifDevicesResponse, { network: string }, Error, number>(
    '/api/onvif/discovery/discover',
    {
      timeout: 120000,
      retries: 0
    },
    {
      mutationFn: body => deviceRequest('discoverOnvifDevices', {body, timeout:120000}),
      onMutate: () => {
        setIsDiscovering(true);
        return onvifGeneration.current;
      },
      onSuccess: (data, _variables, generation) => {
        if (generation !== onvifGeneration.current) return;
        setDiscoveredDevices(data.devices || []);
        setIsDiscovering(false);
      },
      onError: (error, _variables, generation) => {
        if (generation !== onvifGeneration.current) return;
        showStatusMessage(t('streams.errorDiscoveringOnvifDevices', { message: error.message }), 'error', 5000);
        setIsDiscovering(false);
      }
    }
  );

  /**
   * Validate that an ONVIF device host address is a well-formed hostname, IPv4
   * address, or bracketed IPv6 address. Returns true when valid, false otherwise.
   */
  const isValidDeviceHost = (deviceHost: unknown) => {
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
  const getDeviceProfilesMutation = useMutation<
    OnvifProfilesResponse,
    Error,
    { device: OnvifDevice; credentials: OnvifCredentials; generation: number; selection: number }
  >({
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

      const buildDeviceServiceUrl = (scheme: string, host: string) => {
        try {
          const url = new URL(`${scheme}://${host}/onvif/device_service`);
          return url.toString();
        } catch (e) {
          throw new Error('Invalid device URL');
        }
      };

      const httpsFallbackUrl = buildDeviceServiceUrl('https', ipAddress);
      const httpFallbackUrl = buildDeviceServiceUrl('http', ipAddress);

      const attemptFetch = (deviceUrl: string): Promise<OnvifProfilesResponse> => {
        return deviceRequest('getOnvifProfiles', {headers:{'X-Device-URL':deviceUrl,'X-Username':credentials.username,'X-Password':credentials.password}});
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
    onSuccess: (data, variables) => {
      if (variables.generation !== onvifGeneration.current || variables.selection !== onvifSelection.current) return;
      setDeviceProfiles(data.profiles || []);
      setIsLoadingProfiles(false);
    },
    onError: (error, variables) => {
      if (variables.generation !== onvifGeneration.current || variables.selection !== onvifSelection.current) return;
      showStatusMessage(t('streams.errorLoadingDeviceProfiles', { message: error.message }), 'error', 5000);
      setIsLoadingProfiles(false);
    }
  });

  // Test ONVIF connection mutation
  const testOnvifConnectionMutation = usePostMutation<OnvifTestResponse, OnvifTestRequest, Error,
    { generation: number; selection: number; device: OnvifDevice | null }>(
    '/api/onvif/device/test',
    {
      timeout: 15000,
      retries: 0
    },
    {
      mutationFn: body => deviceRequest('testOnvifDevice', {body, timeout:15000}),
      onMutate: () => {
        setIsLoadingProfiles(true);
        return { generation: onvifGeneration.current, selection: onvifSelection.current, device: selectedDeviceRef.current };
      },
      onSuccess: (data, _variables, context) => {
        if (!context || context.generation !== onvifGeneration.current || context.selection !== onvifSelection.current) return;
        if (data.success) {
          showStatusMessage(t('streams.connectionSuccessful'), 'success', 3000);
          if (context.device) {
            getDeviceProfiles(context.device);
          }
        } else {
          showStatusMessage(t('streams.connectionFailed', { message: data.message }), 'error', 5000);
          setIsLoadingProfiles(false);
        }
      },
      onError: (error, _variables, context) => {
        if (!context || context.generation !== onvifGeneration.current || context.selection !== onvifSelection.current) return;
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
    const generation = onvifGeneration.current;
    const selection = onvifSelection.current;

    // Prepare stream data
    const streamData: OnvifStreamCreatePayload = {
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
      protocol: 0, // TCP
      priority: 5, // Medium
      segment_duration: 30,
      record: true,
      record_audio: !clientConfig.audio_disabled,
      audio_voice_enhancement: false,
      backchannel_enabled: false,
      // Backend expects camelCase key 'isOnvif'
      isOnvif: true,
      onvif_username: onvifCredentials.username || '',
      onvif_password: onvifCredentials.password || '',
      onvif_profile: selectedProfile.token || '',
      onvif_port: 0
    };

    // Use mutation to save stream
    saveStreamMutation.mutate({ streamData, editing: false }, {
      onSuccess: async () => {
        if (selectedDevice.inventory_uuid) {
          try {
            await deviceRequest('claimOnvifDevice', {
              body: {
                inventory_uuid: selectedDevice.inventory_uuid,
                stream_name: streamData.name
              },
              timeout: 5000
            });
            if (generation !== onvifGeneration.current) return;
            setDiscoveredDevices(previous => previous.map(device =>
              device.inventory_uuid === selectedDevice.inventory_uuid
                ? { ...device, claim_state: 'claimed' }
                : device
            ));
          } catch (error) {
            showStatusMessage(
              t('streams.onvifClaimWarning', { message: error instanceof Error ? error.message : t('streams.error') }),
              'warning',
              5000
            );
          }
        }
        if (generation !== onvifGeneration.current || selection !== onvifSelection.current) return;
        setIsAddingStream(false);
        setShowCustomNameInput(false);
        // Keep the discovery modal open with its device list intact so
        // additional cameras can be added without re-running discovery;
        // only reset the per-device selection.
        setSelectedProfile(null);
        setCustomStreamName('');
      },
      onError: () => {
        if (generation !== onvifGeneration.current || selection !== onvifSelection.current) return;
        setIsAddingStream(false);
      },
    });
  };

  // Start ONVIF discovery
  const startOnvifDiscovery = () => {
    onvifGeneration.current++;
    onvifSelection.current++;
    selectedDeviceRef.current = null;
    setSelectedDevice(null);
    setSelectedProfile(null);
    setDeviceProfiles([]);
    setShowCustomNameInput(false);
    onvifDiscoveryMutation.mutate({ network: onvifNetworkOverride || 'auto' });
  };

  // Get ONVIF device profiles
  const getDeviceProfiles = (device: OnvifDevice) => {
    const selection = ++onvifSelection.current;
    selectedDeviceRef.current = device;
    setSelectedDevice(device);
    setDeviceProfiles([]);
    getDeviceProfilesMutation.mutate({
      device,
      credentials: onvifCredentials,
      generation: onvifGeneration.current,
      selection
    });
  };

  // Add ONVIF device as stream with selected profile
  // Only reachable from the profile table, which renders when selectedDevice is set.
  const addOnvifDeviceAsStream = (profile: OnvifProfile) => {
    setSelectedProfile(profile);
    setCustomStreamName(`${selectedDevice!.name || t('streams.onvifDefaultName')}_${profile.name || t('streams.defaultStreamName')}`);
    setShowCustomNameInput(true);
  };

  // Check if a discovered ONVIF device is already added as a stream
  const isDeviceAlreadyAdded = (device: OnvifDevice) => {
    return streams.some(stream => {
      if (typeof stream.url !== 'string' || !stream.url || !device.ip_address) return false;
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
  const buildOnvifDeviceUrl = (device: OnvifDevice) => {
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
  const testOnvifConnection = (device: OnvifDevice) => {
    // Invalidate any older device request before starting this selection.
    onvifSelection.current++;
    selectedDeviceRef.current = device;
    setSelectedDevice(device);
    setDeviceProfiles([]);
    setSelectedProfile(null);
    setShowCustomNameInput(false);

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
      {activeTab === 'streams' && <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 className="text-xl font-bold">{t('nav.streams')}</h2>
        <div className="controls flex items-center space-x-2">
          {!canModifyStreams && userRole && !streams.some(stream => stream.can_configure) && (
            <span className="text-sm text-muted-foreground italic me-2">
              {t('streams.readOnlyInsufficientPermissions')}
            </span>
          )}
          {canModifyStreams && (
            <>
              {!selectionMode && (
                <button
                  className="flex items-center gap-1.5 text-sm px-2 py-1 rounded hover:bg-muted/70 transition-colors text-muted-foreground"
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
                  className="btn-secondary focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={openOnvifModal}
              >
                {t('streams.discoverOnvifCameras')}
              </button>
              <button
                  id="add-stream-btn"
                  className="btn-primary focus:ring-2 focus:ring-primary focus:ring-offset-2"
                  onClick={openAddStreamModal}
              >
                {t('streams.addStream')}
              </button>
            </>
          )}
        </div>
      </div>}

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
            {t('streams.configuration')}
          </button>
          <button
            type="button"
            id="inventory-tab"
            role="tab"
            aria-selected={activeTab === 'inventory'}
            aria-controls="inventory-panel"
            className={`rounded-t-lg px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'inventory'
                ? 'bg-card text-card-foreground border border-border border-b-0 -mb-px'
                : 'text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setActiveTab('inventory')}
          >
            {t('streams.inventory')}
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
          <button
            type="button"
            id="events-tab"
            role="tab"
            aria-selected={activeTab === 'events'}
            aria-controls="events-panel"
            className={`rounded-t-lg px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'events'
                ? 'bg-card text-card-foreground border border-border border-b-0 -mb-px'
                : 'text-muted-foreground hover:text-foreground'
            }`}
            onClick={() => setActiveTab('events')}
          >
            {t('events.tab')}
          </button>
        </div>
      </div>

      {activeTab === 'inventory' ? (
        <div role="tabpanel" id="inventory-panel" aria-labelledby="inventory-tab">
          <FleetView onEditCamera={openEditStreamModal} />
        </div>
      ) : activeTab === 'health' ? (
        <div role="tabpanel" id="health-panel" aria-labelledby="health-tab">
          <HealthView />
        </div>
      ) : activeTab === 'events' ? (
        <div role="tabpanel" id="events-panel" aria-labelledby="events-tab">
          <EventRoutingView />
        </div>
      ) : (
        <div role="tabpanel" id="streams-panel" aria-labelledby="streams-tab">
        <div className="streams-container">
          {/* Bulk action toolbar — visible in selection mode. Retained
              from the pre-T5 table layout so bulk enable/disable/delete
              still work the same way over the new card grid. */}
          {canModifyStreams && selectionMode && (
            <div className="flex flex-wrap items-center gap-3 px-3 py-2 mb-3 rounded-md bg-card border border-border">
              <label className="touch-target inline-flex cursor-pointer items-center gap-2 text-sm text-muted-foreground">
                <input
                  type="checkbox"
                  className="w-4 h-4 rounded cursor-pointer"
                  checked={selectedStreams.size > 0 && selectedStreams.size === (sortedStreams || []).length}
                  ref={el => { if (el) el.indeterminate = selectedStreams.size > 0 && selectedStreams.size < (sortedStreams || []).length; }}
                  onChange={toggleSelectAll}
                  title={t('streams.selectAll')}
                />
                <span>
                  {selectedStreams.size > 0
                    ? t('streams.selectedCount', { count: selectedStreams.size })
                    : t('streams.selectStreams')}
                </span>
              </label>
              <button
                className="px-3 py-1 text-sm rounded border transition-colors disabled:opacity-50"
                style={{borderColor: 'hsl(var(--success))', color: 'hsl(var(--success))', background: 'transparent'}}
                onMouseOver={e => e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'}
                onFocus={e => e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'}
                onMouseOut={e => e.currentTarget.style.backgroundColor = 'transparent'}
                onBlur={e => e.currentTarget.style.backgroundColor = 'transparent'}
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
                onFocus={e => e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'}
                onMouseOut={e => e.currentTarget.style.backgroundColor = 'transparent'}
                onBlur={e => e.currentTarget.style.backgroundColor = 'transparent'}
                disabled={isBulkWorking || selectedStreams.size === 0}
                onClick={handleBulkDelete}
              >
                {isBulkWorking ? t('streams.working') : t('streams.deleteSelected')}
              </button>
              <button
                className="ms-auto text-sm text-muted-foreground hover:text-foreground transition-colors"
                onClick={exitSelectionMode}
              >
                {t('common.cancel')}
              </button>
            </div>
          )}

          {/* Sort controls — kept as a compact toolbar above the grid so
              the pre-T5 column-header sort affordances still work. */}
          <div className="flex flex-wrap items-center gap-2 mb-3 text-sm text-muted-foreground">
            <label className="sr-only" htmlFor="streams-search">{t('streams.search')}</label>
            <input
              id="streams-search"
              type="search"
              className="min-w-56 rounded-md border border-input bg-background px-3 py-1.5 text-sm text-foreground"
              placeholder={t('streams.searchPlaceholder')}
              value={streamSearch}
              onInput={(event) => setStreamSearch(event.currentTarget.value)}
            />
            <label className="sr-only" htmlFor="streams-availability">{t('availability.label')}</label>
            <select
              id="streams-availability"
              className="rounded-md border border-input bg-background px-3 py-1.5 text-sm text-foreground"
              value={streamAvailability}
              onChange={(event) => setStreamAvailability(event.currentTarget.value as StreamAvailabilityFilter)}
            >
              <option value="live">{t('availability.live')}</option>
              <option value="offline">{t('availability.offline')}</option>
              <option value="never_connected">{t('availability.neverConnected')}</option>
              <option value="disabled">{t('availability.disabled')}</option>
              <option value="all">{t('availability.all')}</option>
            </select>
            <span className="font-medium">{t('streams.sortBy')}:</span>
            {([
              { key: 'name',       label: t('common.name') },
              { key: 'status',     label: t('common.status') },
              { key: 'resolution', label: t('streams.resolution') },
              { key: 'fps',        label: t('streams.fps') },
              { key: 'recording',  label: t('streams.recording') },
            ] as const).map(({ key, label }) => (
              <button
                key={key}
                type="button"
                className={`px-2 py-1 text-xs rounded border transition-colors ${
                  sortColumn === key
                    ? 'bg-primary/10 border-primary text-primary'
                    : 'border-input bg-background hover:bg-accent'
                }`}
                onClick={() => handleSort(key)}
                aria-pressed={sortColumn === key}
              >
                <span className="inline-flex items-center gap-1">
                  {label}
                  {sortColumn === key && (
                    <span aria-hidden="true">{sortDirection === 'asc' ? '▲' : '▼'}</span>
                  )}
                </span>
              </button>
            ))}
          </div>

          {streamsError ? (
            <div role="alert" className="rounded-md border border-destructive p-4 text-destructive">
              <p>{t('live.errorLoadingStreams', { message: streamsError.message })}</p>
              <button type="button" className="btn-secondary mt-3" onClick={() => refetchStreams()}>
                {t('common.retry')}
              </button>
            </div>
          ) : <ContentLoader
              isLoading={isLoading}
              hasData={hasData}
              loadingMessage={t('streams.loadingStreams')}
              emptyMessage={streamResultsFiltered
                ? t('streams.noStreamsMatchFilters')
                : canModifyStreams
                  ? t('streams.noStreamsConfiguredYetAdd')
                  : t('streams.noStreamsConfiguredYet')}
          >
          <>
          {/* Responsive card grid — explicit breakpoints so the grid
              caps at 4 columns on ultra-wide (≥1536 px) displays while
              still reflowing down to 1 column at ≤640 px (PRD §5.5 /
              #399). */}
          <div
            id="streams-grid"
            className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-4"
            role="list"
            aria-label={t('nav.streams')}
          >
            {sortedStreams.map(stream => (
              <div role="listitem" key={stream.name}>
                <StreamCard
                  stream={stream}
                  canModifyStreams={canModifyStreams && stream.can_configure}
                  shouldHideCredentials={isDemoMode || !stream.can_configure}
                  selectionMode={selectionMode}
                  isSelected={selectedStreams.has(stream.name)}
                  onToggleSelect={toggleSelect}
                  onEdit={openEditStreamModal}
                  onClone={openCloneStreamModal}
                  onOpenDelete={openDeleteModal}
                  onEnable={enableStreamFromCard}
                  onDisable={disableStreamFromCard}
                  t={t}
                />
              </div>
            ))}
          </div>
          <div className="mt-4 flex flex-col gap-3 rounded-md border border-border bg-card px-4 py-3 sm:flex-row sm:items-center sm:justify-between">
            <span className="text-sm text-muted-foreground">
              {t('fleet.paginationSummary', {
                first: streamTotal === 0 ? 0 : ((streamPage - 1) * streamPageSize) + 1,
                last: Math.min(streamPage * streamPageSize, streamTotal),
                total: streamTotal,
              })}
            </span>
            <div className="flex flex-wrap items-center gap-2">
              <label className="text-sm text-muted-foreground" htmlFor="streams-page-size">
                {t('fleet.perPage')}
              </label>
              <select
                id="streams-page-size"
                className="rounded-md border border-input bg-background px-2 py-1.5 text-sm"
                value={streamPageSize}
                onChange={(event) => {
                  setStreamPageSize(Number(event.currentTarget.value));
                  setStreamPage(1);
                }}
              >
                {[25, 50, 100].map((size) => <option key={size} value={size}>{size}</option>)}
              </select>
              <button
                type="button"
                className="btn-secondary px-3 py-1.5"
                disabled={streamPage <= 1}
                onClick={() => setStreamPage((value) => Math.max(1, value - 1))}
              >
                {t('common.previous')}
              </button>
              <span className="min-w-20 text-center text-sm tabular-nums">
                {t('fleet.pageOf', {
                  page: streamTotalPages === 0 ? 0 : streamPage,
                  pages: streamTotalPages,
                })}
              </span>
              <button
                type="button"
                className="btn-secondary px-3 py-1.5"
                disabled={streamTotalPages === 0 || streamPage >= streamTotalPages}
                onClick={() => setStreamPage((value) => value + 1)}
              >
                {t('common.next')}
              </button>
            </div>
          </div>
          </>
          </ContentLoader>}
        </div>
        </div>
      )}

      {deleteModalVisible && streamToDelete && (
        <StreamDeleteModal
            streamId={streamToDelete.name}
            streamName={streamToDelete.name}
            onClose={closeDeleteModal}
            onDisable={disableStream}
            onDelete={deleteStream}
            isDeleting={deleteStreamMutation.isLoading}
            isDisabling={disableStreamMutation.isLoading}
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
          onSave={handleSubmit}
          onClose={closeModal}
          onRefreshModels={loadDetectionModels}
          audioDisabled={!!clientConfig.audio_disabled}
          hideCredentials={isEditing
            ? currentStream.canControlPrivacy !== true
            : shouldHideCredentials}
          transportOfferings={{
            webrtc: !clientConfig.webrtc_disabled,
            mse: !clientConfig.mse_disabled,
            hls: !clientConfig.hls_disabled,
          }}
        />
      )}

      {onvifModalVisible && (
        <div id="onvif-modal" className="app-modal-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">{t('streams.onvifCameraDiscovery')}</h3>
              <button type="button" aria-label={t('common.close')} className="text-2xl cursor-pointer" onClick={closeOnvifModal}>×</button>
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
                    onChange={(e) => setOnvifNetworkOverride((e.target as HTMLInputElement).value)}
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
                    className="btn-primary focus:ring-2 focus:ring-primary"
                    onClick={startOnvifDiscovery}
                    disabled={isDiscovering}
                    type="button"
                >
                  {isDiscovering ? (
                    <span className="flex items-center">
                      {t('streams.discovering')}
                      <span className="ms-1 flex space-x-1">
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
                    <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.ipAddress')}</th>
                    <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.manufacturer')}</th>
                    <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.model')}</th>
                    <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.status')}</th>
                    <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.actions')}</th>
                  </tr>
                  </thead>
                  <tbody className="bg-card divide-y divide-border">
                  {discoveredDevices.length === 0 ? (
                    <tr>
                      <td colSpan={5} className="px-6 py-4 text-center text-muted-foreground">
                        {isDiscovering ? (
                          <div className="flex items-center justify-center">
                            <span>{t('streams.discoveringDevices')}</span>
                            <span className="ms-1 flex space-x-1">
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
                    const rowClassName = alreadyAdded || !device.online ? `${baseRowClass} opacity-60` : baseRowClass;
                    return (
                      <tr key={device.inventory_uuid || device.endpoint || device.ip_address} className={rowClassName}>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div>{device.ip_address}</div>
                          {device.last_seen_at > 0 && (
                            <div className="text-xs text-muted-foreground mt-1">
                              {t('streams.lastSeen', { time: new Date(device.last_seen_at * 1000).toLocaleString() })}
                            </div>
                          )}
                          {alreadyAdded && (
                            <span className="ms-2 inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-full bg-muted text-muted-foreground border border-border">
                              {t('streams.alreadyAdded')}
                            </span>
                          )}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">{device.manufacturer || t('common.unknown')}</td>
                        <td className="px-6 py-4 whitespace-nowrap">{device.model || t('common.unknown')}</td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <div className="flex flex-wrap gap-1">
                            <span className={`inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-full border ${device.online ? 'border-green-500/40 text-green-600' : 'border-border text-muted-foreground'}`}>
                              {device.online ? t('common.online') : t('common.offline')}
                            </span>
                            {device.claim_state === 'claimed' && (
                              <span className="inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-full border border-primary/40 text-primary">
                                {t('streams.claimed')}
                              </span>
                            )}
                            {device.duplicate_suspected && (
                              <span className="inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-full border border-yellow-500/40 text-yellow-600" title={t('streams.duplicateSuspectedHelp')}>
                                {t('streams.duplicateSuspected')}
                              </span>
                            )}
                          </div>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <button
                              className={alreadyAdded ? 'btn-secondary' : 'btn-primary'}
                              onClick={() => testOnvifConnection(device)}
                              disabled={isConnecting as boolean}
                              type="button"
                              title={alreadyAdded ? t('streams.deviceAlreadyInUseTitle') : undefined}
                          >
                            {isConnecting ? (
                              <span className="flex items-center">
                                  {t('common.loading')}
                                  <span className="ms-1 flex space-x-1">
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
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm bg-background text-foreground"
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
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm bg-background text-foreground"
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
                        <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.name')}</th>
                        <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.resolution')}</th>
                        <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.encoding')}</th>
                        <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('streams.fps')}</th>
                        <th className="px-6 py-3 text-start text-xs font-medium text-muted-foreground uppercase tracking-wider">{t('common.actions')}</th>
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
                                className="btn-primary focus:ring-2 focus:ring-primary"
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
                  onClick={closeOnvifModal}
                  type="button"
              >
                {t('common.close')}
              </button>
            </div>
          </div>
        </div>
      )}

      {showCustomNameInput && (
        <div id="custom-name-modal" className="app-modal-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center transition-opacity duration-300">
          <div className="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">
            <div className="flex justify-between items-center p-4 border-b border-border">
              <h3 className="text-lg font-medium">{t('streams.streamName')}</h3>
              <button type="button" aria-label={t('common.close')} className="text-2xl cursor-pointer" onClick={() => setShowCustomNameInput(false)}>×</button>
            </div>
            <div className="p-4">
              <div className="mb-4">
                <label htmlFor="custom-stream-name" className="block text-sm font-medium mb-1">{t('streams.enterNameForStream')}</label>
                <input
                    type="text"
                    id="custom-stream-name"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm bg-background text-foreground"
                    value={customStreamName}
                    onChange={(e) => setCustomStreamName((e.target as HTMLInputElement).value)}
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
                  className="btn-primary focus:ring-2 focus:ring-primary"
                  onClick={submitOnvifDevice}
                  type="button"
                  disabled={!customStreamName.trim() || isAddingStream}
              >
                {isAddingStream ? (
                  <span className="flex items-center">
                    {t('common.adding')}
                    <span className="ms-1 flex space-x-1">
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
