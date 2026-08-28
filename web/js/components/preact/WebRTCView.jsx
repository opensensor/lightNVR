/**
 * LightNVR Web Interface WebRTCView Component
 * Preact component for the WebRTC view page
 */

import { useState, useEffect, useCallback, useMemo } from 'preact/hooks';
// Note: useCallback is still used by getStreamsToShow
import { showStatusMessage } from './ToastContainer.jsx';
import { useFullscreenManager, FullscreenManager, useFullscreenGridNav, useFullscreenCellStream, getNativeFullscreenElement, requestNativeFullscreen, exitNativeFullscreen } from './FullscreenManager.jsx';
import { useQuery } from '../../query-client.js';
import { PlaybackTransportCell } from './PlaybackTransportCell.jsx';
import { SnapshotManager, useSnapshotManager } from './SnapshotManager.jsx';
import { isGo2rtcEnabled } from '../../utils/settings-utils.js';
import { useCameraOrder } from './useCameraOrder.js';
import { GridPicker, computeOptimalGrid, MAX_GRID_CELLS } from './GridPicker.jsx';
import { useI18n } from '../../i18n.js';
import { buildLiveViewHref, resolveForcedLiveTransport } from '../../utils/live-view-url.js';
import { useCollectionMembership } from './fleet/collectionMembership.js';
import { AlwaysFullscreenToggle } from './AlwaysFullscreenToggle.jsx';
import { useAlwaysFullscreenOnTap } from './useAlwaysFullscreenOnTap.js';
import { usePullToRefresh } from './usePullToRefresh.js';
import { shouldShowGestureTip } from './mobileLiveGestures.js';
import { fetchAllStreamSummaries } from '../../utils/stream-summaries.js';
import { LiveOperatorNavigator } from './live/LiveOperatorNavigator.jsx';

/**
 * Convert the old single-string layout value to cols/rows for backward compat.
 * Defined as a function declaration so it hoists safely in the bundle.
 */
function legacyLayoutToColsRowsWebRTC(layout) {
  switch (layout) {
    case '1':  return [1, 1];
    case '2':  return [2, 1];
    case '6':  return [3, 2];
    case '9':  return [3, 3];
    case '16': return [4, 4];
    default:   return [2, 2];
  }
}

/**
 * WebRTCView component
 * @returns {JSX.Element} WebRTCView component
 */
export function WebRTCView({ isAutoDisabled = false, isWebRTCDisabled, isHlsDisabled, isMseDisabled }) {
  const { t } = useI18n();
  const forcedTransport = resolveForcedLiveTransport(
    window.location.pathname,
    window.location.search,
    {
      autoDisabled: isAutoDisabled,
      offerings: {
        webrtc: !isWebRTCDisabled,
        mse: !isMseDisabled,
        hls: !isHlsDisabled,
      },
    }
  );
  const [alwaysFullscreenOnTap, setAlwaysFullscreenOnTap] = useAlwaysFullscreenOnTap();

  // Use the snapshot manager hook
  useSnapshotManager();

  // Use the fullscreen manager hook
  const { isFullscreen, setIsFullscreen, toggleFullscreen } = useFullscreenManager();

  // Name of the video cell currently in per-cell native fullscreen (or null).
  // When a cell enters fullscreen we flip that cell off the sub-stream so the
  // full-resolution main stream is shown (#366).
  const fullscreenCellStream = useFullscreenCellStream();

  // State for streams and layout
  const [streams, setStreams] = useState([]);
  const [refreshGeneration, setRefreshGeneration] = useState(0);
  const [operatorFilter, setOperatorFilter] = useState(null);

  // Tag filter: '' means "All tags", or a single tag value to filter by
  const [tagFilter, setTagFilter] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('tag') || localStorage.getItem('lightnvr-webrtc-tag-filter') || '';
  });
  const [collectionFilter, setCollectionFilter] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('collection') || '';
  });
  const {
    collections,
    cameraUuids: collectionCameraUuids,
    isLoading: isCollectionLoading,
    error: collectionError,
  } = useCollectionMembership(collectionFilter);

  // State for toggling stream labels and controls visibility
  const [showLabels, setShowLabels] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    const u = p.get('labels');
    if (u !== null) return u !== '0';
    const stored = localStorage.getItem('lightnvr-show-labels');
    return stored !== null ? stored === 'true' : true;
  });
  const [showControls, setShowControls] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    const u = p.get('controls');
    if (u !== null) return u !== '0';
    const stored = localStorage.getItem('lightnvr-show-controls');
    return stored !== null ? stored === 'true' : true;
  });
  // Global detection overlay toggle (persisted to localStorage)
  const [showDetections, setShowDetections] = useState(() => {
    const stored = localStorage.getItem('lightnvr-show-detections');
    return stored !== null ? stored === 'true' : true;
  });
  const [isLoading, setIsLoading] = useState(true);

  // State for go2rtc availability (to show MSE View button)
  const [go2rtcAvailable, setGo2rtcAvailable] = useState(false);

  // Initialize cols/rows from URL params, shared localStorage key, or legacy per-view keys.
  // All live views (WebRTC / HLS / MSE) share 'lightnvr-live-cols' / 'lightnvr-live-rows'
  // so a layout change on one page carries over to the others.
  // autoGrid stays true when no preference exists — streams-load effect will
  // auto-size the grid to fit the available camera count.
  const [autoGrid, setAutoGrid] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('cols') === null
      && localStorage.getItem('lightnvr-live-cols') === null
      && localStorage.getItem('lightnvr-webrtc-cols') === null
      && localStorage.getItem('lightnvr-webrtc-layout') === null;
  });
  const [cols, setCols] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const cp = urlParams.get('cols');
    if (cp) return Math.max(1, Math.min(9, parseInt(cp, 10) || 2));
    const shared = localStorage.getItem('lightnvr-live-cols');
    if (shared) return Math.max(1, Math.min(9, parseInt(shared, 10) || 2));
    const legacy = localStorage.getItem('lightnvr-webrtc-cols');
    if (legacy) return Math.max(1, Math.min(9, parseInt(legacy, 10) || 2));
    const oldLayout = localStorage.getItem('lightnvr-webrtc-layout');
    if (oldLayout) return legacyLayoutToColsRowsWebRTC(oldLayout)[0];
    return 2; // placeholder until autoGrid resolves
  });
  const [rows, setRows] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const rp = urlParams.get('rows');
    if (rp) return Math.max(1, Math.min(9, parseInt(rp, 10) || 2));
    const shared = localStorage.getItem('lightnvr-live-rows');
    if (shared) return Math.max(1, Math.min(9, parseInt(shared, 10) || 2));
    const legacy = localStorage.getItem('lightnvr-webrtc-rows');
    if (legacy) return Math.max(1, Math.min(9, parseInt(legacy, 10) || 2));
    const oldLayout = localStorage.getItem('lightnvr-webrtc-layout');
    if (oldLayout) return legacyLayoutToColsRowsWebRTC(oldLayout)[1];
    return 2; // placeholder until autoGrid resolves
  });

  // Total streams per page is simply cols × rows — derived immediately so every
  // subsequent useEffect that needs it can reference it without TDZ issues.
  const maxStreams = cols * rows;

  // Clamp cols×rows to MAX_GRID_CELLS — guards against stale URL params or
  // localStorage values written before the 36-stream cap was enforced.
  useEffect(() => {
    if (cols * rows > MAX_GRID_CELLS) {
      setRows(Math.max(1, Math.floor(MAX_GRID_CELLS / cols)));
    }
  }, [cols, rows]);

  // True when we're in single-stream mode
  const isSingleStream = maxStreams === 1;

  // Let desktop CSS size the live grid against the viewport instead of the
  // static fallback height. Mobile/tablet keep their natural page scroll.
  useEffect(() => {
    if (isFullscreen) return;
    document.body.classList.add('live-view-page');
    return () => document.body.classList.remove('live-view-page');
  }, [isFullscreen]);

  // Initialize selectedStream from URL or sessionStorage if available
  const [selectedStream, setSelectedStream] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const streamParam = urlParams.get('stream');
    if (streamParam) {
      return streamParam;
    }
    // Check sessionStorage as a backup
    const storedStream = sessionStorage.getItem('webrtc_selected_stream');
    return storedStream || '';
  });

  // Initialize currentPage from URL or sessionStorage if available (URL uses 1-based indexing, internal state uses 0-based)
  const [currentPage, setCurrentPage] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const pageParam = urlParams.get('page');
    if (pageParam) {
      // Convert from 1-based (URL) to 0-based (internal)
      return Math.max(0, parseInt(pageParam, 10) - 1);
    }
    // Check sessionStorage as a backup
    const storedPage = sessionStorage.getItem('webrtc_current_page');
    if (storedPage) {
      // Convert from 1-based (stored) to 0-based (internal)
      return Math.max(0, parseInt(storedPage, 10) - 1);
    }
    return 0;
  });

  // Check if go2rtc is enabled (for showing MSE View button)
  useEffect(() => {
    const checkGo2rtc = async () => {
      try {
        const enabled = await isGo2rtcEnabled();
        setGo2rtcAvailable(enabled);
      } catch (error) {
        console.error('[WebRTCView] Error checking go2rtc status:', error);
      }
    };
    checkGo2rtc();
  }, []);

  // Fetch streams using preact-query, and periodically refresh so stream
  // status (Running / Reconnecting / Stopped etc.) stays up-to-date — the
  // video cells use status transitions to auto-retry when a camera that was
  // offline comes back.
  const {
    data: streamsData,
    isLoading: isLoadingStreams,
    error: streamsError,
    refetch: refetchStreams,
  } = useQuery({
    queryKey: ['streams', 'webrtc-summary'],
    queryFn: ({ signal }) => fetchAllStreamSummaries({
      surface: 'live', signal,
    }),
    staleTime: 15000,
    refetchInterval: 30000,
  });

  const refreshLiveGrid = useCallback(async () => {
    try {
      const result = await refetchStreams();
      if (result?.error) throw result.error;
      setRefreshGeneration((generation) => generation + 1);
      showStatusMessage(t('live.streamsRefreshed'), 'success', 2000);
    } catch (error) {
      showStatusMessage(t('live.refreshStreamsFailed', { message: error.message }), 'error', 5000);
    }
  }, [refetchStreams, t]);
  const pullToRefresh = usePullToRefresh(refreshLiveGrid, { disabled: isFullscreen });

  // Update loading state based on streams query status
  useEffect(() => {
    setIsLoading(isLoadingStreams);
  }, [isLoadingStreams]);

  // Process streams data when it's loaded.
  useEffect(() => {
    let cancelled = false;
    if (streamsData && Array.isArray(streamsData)) {
      if (collectionFilter && isCollectionLoading) return;
      // Process the streams data
      const processStreams = async () => {
        try {
          // Filter and process the streams
          const candidateStreams = collectionFilter
            ? streamsData.filter((stream) => collectionCameraUuids.has(stream.camera_uuid))
            : streamsData;
          const filteredStreams = await filterStreamsForWebRTC(candidateStreams);
          if (cancelled) return;

          if (filteredStreams.length > 0) {
            setStreams(filteredStreams);

            // Auto-size the grid to fit the stream count when no preference is stored
            if (autoGrid) {
              const [optCols, optRows] = computeOptimalGrid(filteredStreams.length);
              setCols(optCols);
              setRows(optRows);
              setAutoGrid(false);
            }

            // Set selectedStream based on URL parameter if it exists and is valid
            const urlParams = new URLSearchParams(window.location.search);
            const streamParam = urlParams.get('stream');

            if (streamParam && filteredStreams.some(stream => stream.name === streamParam)) {
              // If the stream from URL exists in the loaded streams, use it
              setSelectedStream(streamParam);
            } else if (!selectedStream || !filteredStreams.some(stream => stream.name === selectedStream)) {
              // Otherwise use the first stream if selectedStream is not set or invalid
              setSelectedStream(filteredStreams[0].name);
            }
          } else {
            console.warn('No streams available for WebRTC view after filtering');
            setStreams([]);
            setSelectedStream('');
          }
        } catch (error) {
          if (cancelled) return;
          console.error('Error processing streams:', error);
          showStatusMessage(t('live.errorProcessingStreams', { message: error.message }));
        }
      };

      processStreams();
    }
    return () => { cancelled = true; };
    // Note: selectedStream is read to preserve the current selection when valid,
    // but we still need to populate streams even when a selection already exists.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [streamsData, autoGrid, collectionFilter, collectionCameraUuids, isCollectionLoading]);

  // Sync layout/page/stream to URL — only meaningful once streams are loaded.
  useEffect(() => {
    if (streams.length === 0) return;

    const url = new URL(window.location);

    // Page (1-based in URL, omit on first page)
    if (currentPage === 0) url.searchParams.delete('page');
    else url.searchParams.set('page', currentPage + 1);

    // cols/rows — omit from URL when at the default (2×2)
    if (cols !== 2 || rows !== 2) {
      url.searchParams.set('cols', cols);
      url.searchParams.set('rows', rows);
    } else {
      url.searchParams.delete('cols');
      url.searchParams.delete('rows');
    }
    url.searchParams.delete('layout'); // remove legacy param

    // Stream selection (single-stream mode only)
    if (isSingleStream && selectedStream) url.searchParams.set('stream', selectedStream);
    else url.searchParams.delete('stream');

    window.history.replaceState({}, '', url);

    // Persist to storage
    if (currentPage > 0) sessionStorage.setItem('webrtc_current_page', (currentPage + 1).toString());
    else sessionStorage.removeItem('webrtc_current_page');
    localStorage.setItem('lightnvr-live-cols', String(cols));
    localStorage.setItem('lightnvr-live-rows', String(rows));
    // Clean up old per-view keys so reads don't fall back to stale values
    localStorage.removeItem('lightnvr-webrtc-cols');
    localStorage.removeItem('lightnvr-webrtc-rows');
    localStorage.removeItem('lightnvr-webrtc-layout');
    if (isSingleStream && selectedStream) sessionStorage.setItem('webrtc_selected_stream', selectedStream);
    else sessionStorage.removeItem('webrtc_selected_stream');
  }, [currentPage, cols, rows, isSingleStream, selectedStream, streams.length]);

  // Sync UI preference controls (group, tag, labels, controls) to URL and localStorage.
  // Runs independently of streams-loaded state so the URL is always accurate.
  useEffect(() => {
    const url = new URL(window.location);

    if (tagFilter) url.searchParams.set('tag', tagFilter);
    else url.searchParams.delete('tag');

    if (collectionFilter) url.searchParams.set('collection', collectionFilter);
    else url.searchParams.delete('collection');

    // Omit params when at their defaults (true) to keep URL clean
    if (!showLabels) url.searchParams.set('labels', '0');
    else url.searchParams.delete('labels');

    if (!showControls) url.searchParams.set('controls', '0');
    else url.searchParams.delete('controls');

    window.history.replaceState({}, '', url);

    // Persist to localStorage for sessions without URL
    if (tagFilter) localStorage.setItem('lightnvr-webrtc-tag-filter', tagFilter);
    else localStorage.removeItem('lightnvr-webrtc-tag-filter');
    localStorage.setItem('lightnvr-show-labels', String(showLabels));
    localStorage.setItem('lightnvr-show-controls', String(showControls));
    localStorage.setItem('lightnvr-show-detections', String(showDetections));
  }, [tagFilter, collectionFilter, showLabels, showControls, showDetections]);

  useEffect(() => {
    if (collectionError) {
      showStatusMessage(t('collections.loadMembersError', { message: collectionError.message }));
    }
  }, [collectionError, t]);

  /**
   * Filter streams for WebRTC view
   * @param {Array} streams - Array of streams
   * @returns {Promise<Array>} Promise resolving to filtered array of streams
   */
  const filterStreamsForWebRTC = async (streams) => {
    try {
      if (!streams || !Array.isArray(streams)) {
        console.warn('No streams data provided to filter');
        return [];
      }

      // Filter out streams that are soft deleted, administratively disabled, or not configured for streaming.
      // Streams in privacy mode (privacy_mode=true) are kept visible with a privacy overlay.
      const filteredStreams = streams.filter(stream => {
        // Filter out soft deleted streams
        if (stream.is_deleted) {
          console.log(`Stream ${stream.name} is soft deleted, filtering out`);
          return false;
        }

        // Filter out administratively disabled streams (privacy_mode streams remain visible)
        if (!stream.enabled) {
          console.log(`Stream ${stream.name} is administratively disabled, filtering out`);
          return false;
        }

        // Filter out streams not configured for streaming
        if (!stream.streaming_enabled) {
          console.log(`Stream ${stream.name} is not configured for streaming, filtering out`);
          return false;
        }

        return true;
      });

      console.log('Filtered streams for WebRTC view:', filteredStreams);

      return filteredStreams || [];
    } catch (error) {
      console.error('Error filtering streams for WebRTC view:', error);
      showStatusMessage(t('live.errorProcessingStreams', { message: error.message }));
      return [];
    }
  };

  // Derive unique tags from all streams for the filter dropdown
  const availableTags = useMemo(() => {
    const tags = new Set();
    streams.forEach(s => {
      if (s.tags) {
        s.tags.split(',').forEach(t => { const trimmed = t.trim(); if (trimmed) tags.add(trimmed); });
      }
    });
    return Array.from(tags).sort();
  }, [streams]);

  // Apply reusable collection and ad-hoc tag filters before camera ordering.
  const tagFilteredStreams = useMemo(() => {
    return streams.filter((stream) => {
      if (collectionFilter && !collectionCameraUuids.has(stream.camera_uuid)) return false;
      if (operatorFilter && !operatorFilter.cameraUuids.has(stream.camera_uuid)) return false;
      if (operatorFilter && operatorFilter.availability !== 'all' &&
          stream.availability !== operatorFilter.availability) return false;
      return !tagFilter || (stream.tags && stream.tags.split(',').some(tag => tag.trim() === tagFilter));
    });
  }, [streams, tagFilter, collectionFilter, collectionCameraUuids, operatorFilter]);

  // Camera ordering hook (operates on group-filtered streams)
  const {
    orderedStreams,
    reorderMode,
    toggleReorderMode,
    enterReorderMode,
    resetOrder,
    setCameraOrder,
    placeCameraAtIndex,
    handleDragStart,
    handleDragOver,
    handleDrop,
    handleDragEnd,
    handleReorderPointerDown,
    handleReorderPointerMove,
    handleReorderPointerUp,
    handleReorderPointerCancel,
  } = useCameraOrder(tagFilteredStreams, 'webrtc');

  const applyOperatorLayout = useCallback((layout) => {
    const nextCols = Math.max(1, Math.min(9, Number(layout.columns) || cols));
    const nextRows = Math.max(1, Math.min(9, Number(layout.rows) || rows));
    setCols(nextCols);
    setRows(Math.min(nextRows, Math.floor(MAX_GRID_CELLS / nextCols)));
    setCurrentPage(0);
    setAutoGrid(false);
    const names = (layout.cameraUuids || []).map((cameraUuid) =>
      streams.find((stream) => stream.camera_uuid === cameraUuid)?.name).filter(Boolean);
    setCameraOrder(names);
    if (nextCols * nextRows === 1 && names[0]) setSelectedStream(names[0]);
  }, [cols, rows, streams, setCameraOrder]);

  // Ensure current page is valid when orderedStreams or maxStreams changes
  useEffect(() => {
    if (orderedStreams.length === 0) return;
    const totalPages = Math.ceil(orderedStreams.length / maxStreams);
    if (currentPage >= totalPages) setCurrentPage(Math.max(0, totalPages - 1));
  }, [orderedStreams.length, maxStreams, currentPage]);

  /**
   * Get streams to show based on layout, selected stream, and pagination
   * @returns {Array} Streams to show
   */
  const getStreamsToShow = useCallback(() => {
    if (isSingleStream && selectedStream) {
      return orderedStreams.filter(stream => stream.name === selectedStream);
    }
    const totalPages = Math.ceil(orderedStreams.length / maxStreams);
    if (currentPage >= totalPages && totalPages > 0) return [];
    const startIdx = currentPage * maxStreams;
    const endIdx = Math.min(startIdx + maxStreams, orderedStreams.length);
    return orderedStreams.slice(startIdx, endIdx);
  }, [orderedStreams, isSingleStream, selectedStream, currentPage, maxStreams]);

  /**
   * Toggle fullscreen mode for a specific stream
   * @param {string} streamName - Stream name
   * @param {Event} event - Click event
   * @param {HTMLElement} cellElement - The video cell element
   */
  const toggleStreamFullscreen = (streamName, event, cellElement) => {
    if (event?.currentTarget?.classList?.contains('fullscreen-btn')
        && shouldShowGestureTip('double-tap-fullscreen', 3)) {
      showStatusMessage(t('live.tipDoubleTapFullscreen'), 'info', 5000);
    }
    // Prevent default button behavior
    if (event) {
      event.preventDefault();
      event.stopPropagation();
    }

    if (!streamName) {
      console.error('Stream name not provided for fullscreen toggle');
      return;
    }

    console.log(`Toggling fullscreen for stream: ${streamName}`);

    if (!cellElement) {
      console.error('Video cell element not provided for fullscreen toggle');
      return;
    }

    if (!getNativeFullscreenElement()) {
      console.log('Entering fullscreen mode for video cell');
      requestNativeFullscreen(cellElement).catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
        showStatusMessage(`Could not enable fullscreen mode: ${err.message}`);
      });
    } else {
      console.log('Exiting fullscreen mode');
      exitNativeFullscreen().catch(err => {
        console.error(`Error attempting to exit fullscreen: ${err.message}`);
      });
    }

    // Prevent event propagation
    if (event) {
      event.preventDefault();
      event.stopPropagation();
    }
  };

  // Memoize the streams to show to prevent unnecessary re-renders
  // Note: getMaxStreamsForLayout is omitted from dependencies because it's memoized from `layout`.
  const streamsToShow = useMemo(() => getStreamsToShow(), [getStreamsToShow]);

  // Arrow-key navigation between streams while one is in native fullscreen.
  useFullscreenGridNav(streamsToShow, cols, rows);

  return (
    <section
      id="live-page"
      data-testid="webrtc-view"
      className={`page ${isFullscreen ? 'fullscreen-mode' : ''} ${isSingleStream && !isFullscreen ? 'single-stream' : ''}`}
    >
      {/* Include the SnapshotManager component */}
      <SnapshotManager />

      {/* Include the FullscreenManager component */}
      <FullscreenManager
        isFullscreen={isFullscreen}
        setIsFullscreen={setIsFullscreen}
        targetId="live-page"
      />

      <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow" style={{ position: 'relative', zIndex: 10, pointerEvents: 'auto' }}>
        <div className="flex flex-wrap items-center gap-3">
          <h2 className="text-xl font-bold whitespace-nowrap">{t('live.liveView')}</h2>
          {/* Auto honors per-stream precedence; explicit modes force every tile. */}
          <div className="inline-flex items-center bg-muted rounded-lg p-1 gap-1" style={{ position: 'relative', zIndex: 50 }}>
            {!isAutoDisabled && (
              forcedTransport === null ? (
                <span className="px-3 py-1.5 rounded text-sm font-medium bg-primary text-primary-foreground select-none">
                  Auto
                </span>
              ) : (
                <a
                  href={buildLiveViewHref('/index.html', window.location.search)}
                  className="px-3 py-1.5 rounded text-sm font-medium transition-colors no-underline text-muted-foreground hover:bg-background hover:text-foreground focus:outline-none"
                >
                  Auto
                </a>
              )
            )}

            {forcedTransport === 'webrtc' ? (
              <span className="px-3 py-1.5 rounded text-sm font-medium bg-primary text-primary-foreground select-none">
                WebRTC
              </span>
            ) : (
              <a
                href={buildLiveViewHref('/index.html', window.location.search, 'webrtc')}
                className="px-3 py-1.5 rounded text-sm font-medium transition-colors no-underline text-muted-foreground hover:bg-background hover:text-foreground focus:outline-none"
              >
                WebRTC
              </a>
            )}

            {/* Tab HLS */}
            {!isHlsDisabled && (
              forcedTransport === 'hls' ? (
                <span className="px-3 py-1.5 rounded text-sm font-medium bg-primary text-primary-foreground select-none">
                  {t('live.hlsShort')}
                </span>
              ) : (
                <a
                  href={buildLiveViewHref('/hls.html', window.location.search)}
                  className="px-3 py-1.5 rounded text-sm font-medium transition-colors no-underline text-muted-foreground hover:bg-background hover:text-foreground focus:outline-none"
                >
                  {t('live.hlsShort')}
                </a>
              )
            )}

            {/* Tab MSE */}
            {go2rtcAvailable && !isMseDisabled && (
              forcedTransport === 'mse' ? (
                <span className="px-3 py-1.5 rounded text-sm font-medium bg-primary text-primary-foreground select-none">
                  {t('live.mseShort')}
                </span>
              ) : (
                <a
                  href={buildLiveViewHref('/hls.html', window.location.search, 'mse')}
                  className="px-3 py-1.5 rounded text-sm font-medium transition-colors no-underline text-muted-foreground hover:bg-background hover:text-foreground focus:outline-none"
                >
                  {t('live.mseShort')}
                </a>
              )
            )}
          </div>
        </div>
        
        <div className="controls flex items-center space-x-2">
          {collections.length > 0 && (
            <div className="flex items-center gap-1.5">
              <label htmlFor="webrtc-collection-filter" className="text-sm whitespace-nowrap">{t('collections.filter')}:</label>
              <select
                id="webrtc-collection-filter"
                className="px-3 py-2 border border-border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                value={collectionFilter}
                disabled={isCollectionLoading}
                onChange={(event) => { setCollectionFilter(event.currentTarget.value); setCurrentPage(0); }}
              >
                <option value="">{t('collections.all')}</option>
                {collections.map((collection) => (
                  <option key={collection.uuid} value={collection.uuid}>{collection.name} ({collection.effective_count})</option>
                ))}
              </select>
            </div>
          )}
          {availableTags.length > 0 && (
            <div className="flex items-center gap-1.5 flex-wrap">
              <span className="text-sm whitespace-nowrap">{t('live.tags')}:</span>
              <button
                className={`px-2 py-1 rounded text-xs font-medium border transition-colors ${!tagFilter ? 'bg-primary text-primary-foreground border-primary' : 'bg-background text-foreground border-border hover:border-primary'}`}
                onClick={() => { setTagFilter(''); setCurrentPage(0); }}
              >
                {t('live.allTags')}
              </button>
              {availableTags.map(tag => (
                <button
                  key={tag}
                  className={`px-2 py-1 rounded text-xs font-medium border transition-colors ${tagFilter === tag ? 'bg-primary text-primary-foreground border-primary' : 'bg-background text-foreground border-border hover:border-primary'}`}
                  onClick={() => { setTagFilter(tagFilter === tag ? '' : tag); setCurrentPage(0); }}
                  title={t('live.filterByTag', { tag })}
                >
                  #{tag}
                </button>
              ))}
            </div>
          )}

          {/* Grid layout picker */}
          <div className="flex items-center gap-1.5">
            <label className="text-sm whitespace-nowrap">{t('live.layout')}:</label>
            <GridPicker
              cols={cols}
              rows={rows}
              onSelect={(c, r) => { setCols(c); setRows(r); setCurrentPage(0); setAutoGrid(false); }}
              maxCells={orderedStreams.length}
            />
          </div>

          {isSingleStream && (
            <div className="flex items-center gap-1.5">
              <label htmlFor="stream-selector" className="text-sm whitespace-nowrap">{t('live.stream')}:</label>
              <select
                id="stream-selector"
                className="px-3 py-2 border border-border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                value={selectedStream}
                onChange={(e) => setSelectedStream(e.target.value)}
              >
                {orderedStreams.map(stream => (
                  <option key={stream.name} value={stream.name}>{stream.name}</option>
                ))}
              </select>
            </div>
          )}

          <button
            className={`p-2 rounded-full focus:outline-none focus:ring-2 focus:ring-primary ${showLabels ? 'bg-secondary hover:bg-secondary/80 text-secondary-foreground' : 'bg-primary/20 hover:bg-primary/30 text-primary'}`}
            onClick={() => setShowLabels(v => !v)}
            title={showLabels ? t('live.hideStreamLabels') : t('live.showStreamLabels')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"></path>
              <line x1="7" y1="7" x2="7.01" y2="7"></line>
              {!showLabels && <line x1="2" y1="22" x2="22" y2="2" stroke="currentColor" strokeWidth="2"></line>}
            </svg>
          </button>

          <button
            className={`p-2 rounded-full focus:outline-none focus:ring-2 focus:ring-primary ${showControls ? 'bg-secondary hover:bg-secondary/80 text-secondary-foreground' : 'bg-primary/20 hover:bg-primary/30 text-primary'}`}
            onClick={() => setShowControls(v => !v)}
            title={showControls ? t('live.hideStreamControls') : t('live.showStreamControls')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <line x1="4" y1="21" x2="4" y2="14"></line>
              <line x1="4" y1="10" x2="4" y2="3"></line>
              <line x1="12" y1="21" x2="12" y2="12"></line>
              <line x1="12" y1="8" x2="12" y2="3"></line>
              <line x1="20" y1="21" x2="20" y2="16"></line>
              <line x1="20" y1="12" x2="20" y2="3"></line>
              <line x1="1" y1="14" x2="7" y2="14"></line>
              <line x1="9" y1="8" x2="15" y2="8"></line>
              <line x1="17" y1="16" x2="23" y2="16"></line>
              {!showControls && <line x1="2" y1="22" x2="22" y2="2" stroke="currentColor" strokeWidth="2"></line>}
            </svg>
          </button>

          <button
            className={`p-2 rounded-full focus:outline-none focus:ring-2 focus:ring-primary ${showDetections ? 'bg-secondary hover:bg-secondary/80 text-secondary-foreground' : 'bg-primary/20 hover:bg-primary/30 text-primary'}`}
            onClick={() => setShowDetections(v => !v)}
            title={showDetections ? t('live.hideAllDetectionOverlays') : t('live.showAllDetectionOverlays')}
          >
            {/* Eye icon for detection overlay toggle */}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
              <circle cx="12" cy="12" r="3"/>
              {!showDetections && <line x1="2" y1="22" x2="22" y2="2" stroke="currentColor" strokeWidth="2"></line>}
            </svg>
          </button>

          <AlwaysFullscreenToggle
            enabled={alwaysFullscreenOnTap}
            onChange={setAlwaysFullscreenOnTap}
          />

          {orderedStreams.length > 1 && (
            <button
              className={`p-2 rounded-full focus:outline-none focus:ring-2 focus:ring-primary ${reorderMode ? 'bg-primary text-primary-foreground hover:bg-primary/90' : 'bg-secondary hover:bg-secondary/80 text-secondary-foreground'}`}
              onClick={toggleReorderMode}
              title={reorderMode ? t('live.exitReorderMode') : t('live.dragToReorderCameras')}
            >
              {/* Drag-handle dots icon */}
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"
                   fill="currentColor" stroke="none">
                <circle cx="9"  cy="5"  r="1.6"/><circle cx="15" cy="5"  r="1.6"/>
                <circle cx="9"  cy="12" r="1.6"/><circle cx="15" cy="12" r="1.6"/>
                <circle cx="9"  cy="19" r="1.6"/><circle cx="15" cy="19" r="1.6"/>
              </svg>
            </button>
          )}

          {reorderMode && (
            <button
              className="p-2 rounded-full bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none focus:ring-2 focus:ring-primary"
              onClick={resetOrder}
              title={t('live.resetCameraOrder')}
            >
              {/* Reset / circular-arrow icon */}
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"
                   fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <polyline points="1 4 1 10 7 10"/>
                <path d="M3.51 15a9 9 0 1 0 .49-4.95"/>
              </svg>
            </button>
          )}

          <button
            id="fullscreen-btn"
            className="p-2 rounded-full bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none focus:ring-2 focus:ring-primary"
            onClick={() => toggleFullscreen()}
            title={t('live.toggleFullscreen')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path
                d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
            </svg>
          </button>
        </div>
      </div>

      <div className="live-workspace-shell">
        <LiveOperatorNavigator
          streams={streams}
          orderedStreams={orderedStreams}
          columns={cols}
          rows={rows}
          onFilterChange={setOperatorFilter}
          onApplyLayout={applyOperatorLayout}
        />
        <div className="live-operator-main">
        <div
          className="live-grid-frame flex flex-col space-y-4 h-full"
          {...pullToRefresh.bind}
        >
        {(pullToRefresh.distance > 0 || pullToRefresh.refreshing) && (
          <div
            className={`mobile-pull-refresh ${pullToRefresh.ready ? 'ready' : ''}`}
            style={{ '--pull-distance': `${pullToRefresh.distance}px` }}
            role="status"
          >
            {pullToRefresh.refreshing
              ? t('live.refreshingStreams')
              : pullToRefresh.ready
                ? t('live.releaseToRefresh')
                : t('live.pullToRefresh')}
          </div>
        )}
        <div
          id="video-grid"
          className="video-container"
          style={{ '--grid-cols': cols, '--grid-rows': rows }}
        >
          {isLoadingStreams ? (
              <div className="flex justify-center items-center col-span-full row-span-full h-64 w-full" style={{ pointerEvents: 'none', zIndex: 1 }}>
                <div className="flex flex-col items-center justify-center py-8">
                <div
                  className="inline-block animate-spin rounded-full border-4 border-secondary border-t-primary w-16 h-16"></div>
                <p className="mt-4 text-muted-foreground">{t('live.loadingStreams')}</p>
              </div>
            </div>
          ) : isLoading ? (
            <div
                className="flex justify-center items-center col-span-full row-span-full h-64 w-full"
                style={{
                  pointerEvents: 'none',
                  position: 'relative',
                  zIndex: 1
                }}
            >
              <div className="flex flex-col items-center justify-center py-8">
                <div
                  className="inline-block animate-spin rounded-full border-4 border-secondary border-t-primary w-16 h-16"></div>
                <p className="mt-4 text-muted-foreground">{t('live.loadingStreams')}</p>
              </div>
            </div>
          ) : (streamsError) ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-card text-card-foreground rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-muted-foreground text-lg">{t('live.errorLoadingStreams', { message: streamsError.message })}</p>
              <button
                onClick={() => window.location.reload()}
                className="btn-primary"
              >
                {t('common.retry')}
              </button>
            </div>
          ) : streams.length === 0 ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-card text-card-foreground rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-muted-foreground text-lg">{t('live.noStreamsConfigured')}</p>
              <a href="streams.html" className="btn-primary">{t('live.configureStreams')}</a>
            </div>
          ) : (
            // Render video cells. Connection concurrency is bounded by the
            // shared stream connection gate (see stream-connection-gate.js),
            // which replaced the old fixed per-index stagger: healthy cameras
            // connect as fast as slots free up, and offline cameras fail fast
            // instead of pinning browser connections.
            streamsToShow.map((stream, index) => {
              const globalIndex = currentPage * maxStreams + index;
              return (
                <div
                  key={`${stream.name}:${refreshGeneration}`}
                  data-camera-order-index={globalIndex}
                  style={{ position: 'relative', touchAction: reorderMode ? 'none' : undefined }}
                  draggable={reorderMode}
                  onDragStart={reorderMode ? () => handleDragStart(globalIndex) : undefined}
                  onDragOver={(event) => {
                    if (reorderMode) handleDragOver(event, globalIndex);
                    else if (event.dataTransfer?.types?.includes('application/x-lightnvr-camera')) event.preventDefault();
                  }}
                  onDrop={(event) => {
                    if (reorderMode) handleDrop(event);
                    else {
                      const cameraUuid = event.dataTransfer?.getData('application/x-lightnvr-camera');
                      if (cameraUuid) { event.preventDefault(); placeCameraAtIndex(cameraUuid, globalIndex); }
                    }
                  }}
                  onDragEnd={reorderMode ? handleDragEnd : undefined}
                  onPointerDown={reorderMode ? (event) => handleReorderPointerDown(event, globalIndex) : undefined}
                  onPointerMove={reorderMode ? handleReorderPointerMove : undefined}
                  onPointerUp={reorderMode ? handleReorderPointerUp : undefined}
                  onPointerCancel={reorderMode ? handleReorderPointerCancel : undefined}
                >
                  {reorderMode && (
                    <div
                      style={{
                        position: 'absolute', top: 0, left: 0, right: 0, zIndex: 20,
                        background: 'rgba(0,0,0,0.55)', color: '#fff',
                        display: 'flex', alignItems: 'center', justifyContent: 'center',
                        padding: '6px 8px', cursor: 'grab', fontSize: '13px', gap: '6px',
                        userSelect: 'none',
                      }}
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                           fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                        <line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/>
                      </svg>
                      {t('live.dragToReorder')}
                    </div>
                  )}
                  <PlaybackTransportCell
                    stream={stream}
                    offerings={{
                      webrtc: !isWebRTCDisabled,
                      mse: !isMseDisabled,
                      hls: !isHlsDisabled,
                    }}
                    defaultTransport="webrtc"
                    forcedTransport={forcedTransport}
                    useSubStream={!isSingleStream && fullscreenCellStream !== stream.name && (stream.has_sub_stream || !!stream.sub_stream_url)}
                    fullscreenUpgraded={!isSingleStream && fullscreenCellStream === stream.name && (stream.has_sub_stream || !!stream.sub_stream_url)}
                    onToggleFullscreen={toggleStreamFullscreen}
                    streamId={stream.name}
                    showLabels={showLabels}
                    showControls={showControls}
                    globalShowDetections={showDetections}
                    alwaysFullscreenOnTap={alwaysFullscreenOnTap && !reorderMode}
                    onRequestReorder={orderedStreams.length > 1 ? enterReorderMode : undefined}
                    mobileGesturesDisabled={reorderMode}
                  />
                </div>
              );
            })
          )}
        </div>

        {!isSingleStream && orderedStreams.length > maxStreams ? (
          <div className="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button
              className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => {
                console.log('Changing to previous page');
                setCurrentPage(Math.max(0, currentPage - 1));
              }}
              disabled={currentPage === 0}
            >
              {t('common.previous')}
            </button>

            <span className="text-foreground">
              {t('live.pageOf', { current: currentPage + 1, total: Math.ceil(orderedStreams.length / maxStreams) })}
            </span>

            <button
              className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => {
                console.log('Changing to next page');
                const totalPages = Math.ceil(orderedStreams.length / maxStreams);
                setCurrentPage(Math.min(totalPages - 1, currentPage + 1));
              }}
              disabled={currentPage >= Math.ceil(orderedStreams.length / maxStreams) - 1}
            >
              {t('common.next')}
            </button>
          </div>
        ) : null}
        </div>
        </div>
      </div>
    </section>
  );
}
