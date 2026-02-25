/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the HLS live view page
 */

import { useState, useEffect, useMemo } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { useFullscreenManager, FullscreenManager } from './FullscreenManager.jsx';
import { useQuery, useQueryClient } from '../../query-client.js';
import { SnapshotManager, useSnapshotManager } from './SnapshotManager.jsx';
import { HLSVideoCell } from './HLSVideoCell.jsx';
import { MSEVideoCell } from './MSEVideoCell.jsx';
import { isGo2rtcEnabled } from '../../utils/settings-utils.js';
import { useCameraOrder } from './useCameraOrder.js';
import { GridPicker, computeOptimalGrid } from './GridPicker.jsx';

/**
 * Convert the old single-string layout value to cols/rows for backward compat.
 * Defined as a function declaration so it hoists safely in the bundle.
 */
function legacyLayoutToColsRowsHLS(layout) {
  switch (layout) {
    case '1':  return [1, 1];
    case '2':  return [2, 1];
    case '6':  return [3, 2];
    case '9':  return [3, 3];
    case '16': return [4, 4];
    default:   return [2, 2]; // '4' and anything else → 2×2
  }
}

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView({isWebRTCDisabled}) {
  // Use the snapshot manager hook
  useSnapshotManager();

  // Use the fullscreen manager hook
  const { isFullscreen, setIsFullscreen, toggleFullscreen } = useFullscreenManager();

  // State for streams and layout
  const [streams, setStreams] = useState([]);

  // Group filter: '' means "All groups"
  const [groupFilter, setGroupFilter] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('group') || localStorage.getItem('lightnvr-hls-group-filter') || '';
  });

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
  const [isLoading, setIsLoading] = useState(true);

  // State for go2rtc availability
  const [go2rtcAvailable, setGo2rtcAvailable] = useState(false);

  // State for go2rtc mode - determines whether to use MSE or HLS
  // Initialize from URL param if present
  const [useMSE, setUseMSE] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get('mode') === 'mse';
  });

  // Initialize cols/rows from URL params, localStorage, or legacy layout key.
  // autoGrid stays true when no preference exists — streams-load effect will
  // auto-size the grid to fit the available camera count.
  const [autoGrid, setAutoGrid] = useState(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get('cols') === null && localStorage.getItem('lightnvr-hls-cols') === null
      && localStorage.getItem('lightnvr-hls-layout') === null;
  });
  const [cols, setCols] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const cp = urlParams.get('cols');
    if (cp) return Math.max(1, Math.min(8, parseInt(cp, 10) || 2));
    const stored = localStorage.getItem('lightnvr-hls-cols');
    if (stored) return Math.max(1, Math.min(8, parseInt(stored, 10) || 2));
    const oldLayout = localStorage.getItem('lightnvr-hls-layout');
    if (oldLayout) return legacyLayoutToColsRowsHLS(oldLayout)[0];
    return 2; // placeholder until autoGrid resolves
  });
  const [rows, setRows] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const rp = urlParams.get('rows');
    if (rp) return Math.max(1, Math.min(8, parseInt(rp, 10) || 2));
    const stored = localStorage.getItem('lightnvr-hls-rows');
    if (stored) return Math.max(1, Math.min(8, parseInt(stored, 10) || 2));
    const oldLayout = localStorage.getItem('lightnvr-hls-layout');
    if (oldLayout) return legacyLayoutToColsRowsHLS(oldLayout)[1];
    return 2; // placeholder until autoGrid resolves
  });

  // Total streams per page — derived immediately so URL-sync useEffect can reference
  // isSingleStream in its dependency array without TDZ issues.
  const maxStreams = cols * rows;

  // True when we're in single-stream mode
  const isSingleStream = maxStreams === 1;

  // Initialize selectedStream from URL or sessionStorage if available
  const [selectedStream, setSelectedStream] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const streamParam = urlParams.get('stream');
    if (streamParam) {
      return streamParam;
    }
    // Check sessionStorage as a backup
    const storedStream = sessionStorage.getItem('hls_selected_stream');
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
    const storedPage = sessionStorage.getItem('hls_current_page');
    if (storedPage) {
      // Convert from 1-based (stored) to 0-based (internal)
      return Math.max(0, parseInt(storedPage, 10) - 1);
    }
    return 0;
  });

  // Get query client for fetching and invalidating queries
  const queryClient = useQueryClient();

  // Check if go2rtc is enabled (for showing mode toggle)
  useEffect(() => {
    const checkGo2rtcMode = async () => {
      try {
        const go2rtcEnabled = await isGo2rtcEnabled();
        console.log(`[LiveView] go2rtc enabled: ${go2rtcEnabled}`);
        setGo2rtcAvailable(go2rtcEnabled);
        // If user requested MSE via URL but go2rtc is not enabled, fall back to HLS
        if (useMSE && !go2rtcEnabled) {
          console.log('[LiveView] MSE requested but go2rtc not enabled, falling back to HLS');
          setUseMSE(false);
        }
      } catch (error) {
        console.error('[LiveView] Error checking go2rtc status:', error);
        setGo2rtcAvailable(false);
        setUseMSE(false);
      }
    };
    checkGo2rtcMode();
  }, []);

  // Fetch streams using preact-query
  const {
    data: streamsData,
    isLoading: isLoadingStreams,
    error: streamsError
  } = useQuery(
    'streams',
    '/api/streams',
    {
      timeout: 15000, // 15 second timeout
      retries: 2,     // Retry twice
      retryDelay: 1000 // 1 second between retries
    }
  );

  // Update loading state based on streams query status
  useEffect(() => {
    setIsLoading(isLoadingStreams);
  }, [isLoadingStreams]);

  // Process streams data when it's loaded
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData)) {
      // Process the streams data
      const processStreams = async () => {
        try {
          // Filter and process the streams
          // Note: filterStreamsForHLS is defined below but called here via closure
          // We use a local function to fetch stream details to avoid hoisting issues
          const fetchStreamDetails = async (stream) => {
            try {
              const streamId = stream.id || stream.name;
              const streamDetails = await queryClient.fetchQuery({
                queryKey: ['stream-details', streamId],
                queryFn: async () => {
                  const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`);
                  if (!response.ok) {
                    throw new Error(`Failed to load details for stream ${stream.name}`);
                  }
                  return response.json();
                },
                staleTime: 30000 // 30 seconds
              });
              return streamDetails;
            } catch (error) {
              console.error(`Error loading details for stream ${stream.name}:`, error);
              return stream;
            }
          };

          // Fetch stream details with bounded concurrency (max 3 at a time) to avoid
          // overwhelming the backend with simultaneous /api/streams/{id} requests.
          const BATCH_SIZE = 3;
          const detailedStreams = [];
          for (let i = 0; i < streamsData.length; i += BATCH_SIZE) {
            const batch = streamsData.slice(i, i + BATCH_SIZE);
            const batchResults = await Promise.all(batch.map(fetchStreamDetails));
            detailedStreams.push(...batchResults);
          }
          console.log('Loaded detailed streams for HLS view:', detailedStreams);

          // Filter out streams that are soft deleted, inactive, or not configured for streaming
          const filteredStreams = detailedStreams.filter(stream => {
            if (stream.is_deleted) {
              console.log(`Stream ${stream.name} is soft deleted, filtering out`);
              return false;
            }
            if (!stream.enabled) {
              console.log(`Stream ${stream.name} is inactive, filtering out`);
              return false;
            }
            if (!stream.streaming_enabled) {
              console.log(`Stream ${stream.name} is not configured for streaming, filtering out`);
              return false;
            }
            return true;
          });

          console.log('Filtered streams for HLS view:', filteredStreams);

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
            console.warn('No streams available for HLS view after filtering');
          }
        } catch (error) {
          console.error('Error processing streams:', error);
          showStatusMessage('Error processing streams: ' + error.message);
        }
      };

      processStreams();
    }
    // Note: We intentionally only re-run when streamsData changes
    // selectedStream is read but we don't want to trigger refetch when it changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [streamsData, queryClient]);

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
    if (currentPage > 0) sessionStorage.setItem('hls_current_page', (currentPage + 1).toString());
    else sessionStorage.removeItem('hls_current_page');
    localStorage.setItem('lightnvr-hls-cols', String(cols));
    localStorage.setItem('lightnvr-hls-rows', String(rows));
    localStorage.removeItem('lightnvr-hls-layout');
    if (isSingleStream && selectedStream) sessionStorage.setItem('hls_selected_stream', selectedStream);
    else sessionStorage.removeItem('hls_selected_stream');
  }, [currentPage, cols, rows, isSingleStream, selectedStream, streams.length]);

  // Sync UI preference controls (group, labels, controls) to URL and localStorage.
  // Runs independently of streams-loaded state so the URL is always accurate.
  useEffect(() => {
    const url = new URL(window.location);

    if (groupFilter) url.searchParams.set('group', groupFilter);
    else url.searchParams.delete('group');

    // Omit params when at their defaults (true) to keep URL clean
    if (!showLabels) url.searchParams.set('labels', '0');
    else url.searchParams.delete('labels');

    if (!showControls) url.searchParams.set('controls', '0');
    else url.searchParams.delete('controls');

    window.history.replaceState({}, '', url);

    // Persist to localStorage for sessions without URL
    if (groupFilter) localStorage.setItem('lightnvr-hls-group-filter', groupFilter);
    else localStorage.removeItem('lightnvr-hls-group-filter');
    localStorage.setItem('lightnvr-show-labels', String(showLabels));
    localStorage.setItem('lightnvr-show-controls', String(showControls));
  }, [groupFilter, showLabels, showControls]);

  // Derive unique group names from all streams for the filter dropdown
  const availableGroups = useMemo(() => {
    const groups = new Set();
    streams.forEach(s => { if (s.group_name) groups.add(s.group_name); });
    return Array.from(groups).sort();
  }, [streams]);

  // Apply group filter before passing to the order hook
  const groupFilteredStreams = useMemo(() => {
    if (!groupFilter) return streams;
    return streams.filter(s => s.group_name === groupFilter);
  }, [streams, groupFilter]);

  // Camera ordering hook (operates on group-filtered streams)
  const {
    orderedStreams,
    reorderMode,
    toggleReorderMode,
    resetOrder,
    handleDragStart,
    handleDragOver,
    handleDrop,
    handleDragEnd,
  } = useCameraOrder(groupFilteredStreams, 'hls');

  // Ensure current page is valid when orderedStreams or layout changes
  useEffect(() => {
    if (orderedStreams.length === 0) return;

    const totalPages = Math.ceil(orderedStreams.length / maxStreams);

    if (currentPage >= totalPages) {
      setCurrentPage(Math.max(0, totalPages - 1));
    }
  }, [orderedStreams.length, maxStreams, currentPage]);

  /**
   * Toggle fullscreen mode for a specific stream
   * @param {string} streamName - Stream name
   * @param {Event} event - Click event
   * @param {HTMLElement} cellElement - The video cell element
   */
  const toggleStreamFullscreen = (streamName, event, cellElement) => {
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

    if (!document.fullscreenElement) {
      console.log('Entering fullscreen mode for video cell');
      cellElement.requestFullscreen().catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
        showStatusMessage(`Could not enable fullscreen mode: ${err.message}`);
      });
    } else {
      console.log('Exiting fullscreen mode');
      document.exitFullscreen();
    }

    // Prevent event propagation
    if (event) {
      event.preventDefault();
      event.stopPropagation();
    }
  };

  // Memoize the streams to show to prevent unnecessary re-renders
  const streamsToShow = useMemo(() => {
    // Filter streams based on layout and selected stream
    let result;
    if (isSingleStream && selectedStream) {
      result = orderedStreams.filter(stream => stream.name === selectedStream);
    } else {
      // Apply pagination
      const totalPages = Math.ceil(orderedStreams.length / maxStreams);

      // Ensure current page is valid
      if (currentPage >= totalPages && totalPages > 0) {
        result = []; // Will be handled by the effect that watches currentPage
      } else {
        // Get streams for current page
        const startIdx = currentPage * maxStreams;
        const endIdx = Math.min(startIdx + maxStreams, orderedStreams.length);
        result = orderedStreams.slice(startIdx, endIdx);
      }
    }

    console.log(`[LiveView] streamsToShow computed: ${result.length} streams`, result.map(s => s.name));
    console.log(`[LiveView] cols=${cols}, rows=${rows}, currentPage=${currentPage}, totalStreams=${orderedStreams.length}`);
    return result;
  }, [orderedStreams, isSingleStream, selectedStream, currentPage, maxStreams, cols, rows]);

  return (
    <section
      id="live-page"
      className={`page ${isFullscreen ? 'fullscreen-mode' : ''}`}
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
        <div className="flex items-center space-x-2">
          <h2 className="text-xl font-bold mr-4">Live View ({useMSE ? 'MSE' : 'HLS'})</h2>
          <div className="flex space-x-2">
            {!isWebRTCDisabled && (
            <a
              href="/index.html"
              className="btn-secondary focus:outline-none focus:ring-2 focus:ring-primary inline-block text-center"
              style={{ position: 'relative', zIndex: 50 }}
            >
              WebRTC View
            </a>
                )}
            {go2rtcAvailable && (
            <button
              className="btn-secondary focus:outline-none focus:ring-2 focus:ring-primary inline-block text-center"
              style={{ position: 'relative', zIndex: 50 }}
              onClick={() => {
                setUseMSE(!useMSE);
                // Update URL to reflect mode
                const url = new URL(window.location);
                if (!useMSE) {
                  url.searchParams.set('mode', 'mse');
                } else {
                  url.searchParams.delete('mode');
                }
                window.history.replaceState({}, '', url);
              }}
            >
              {useMSE ? 'HLS View' : 'MSE View'}
            </button>
                )}

          </div>
        </div>
        <div className="controls flex items-center space-x-2">
          {availableGroups.length > 0 && (
            <div className="flex items-center gap-1.5">
              <label htmlFor="group-filter" className="text-sm whitespace-nowrap">Group:</label>
              <select
                id="group-filter"
                className="px-3 py-2 border border-border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                value={groupFilter}
                onChange={(e) => { setGroupFilter(e.target.value); setCurrentPage(0); }}
              >
                <option value="">All Groups</option>
                {availableGroups.map(g => <option key={g} value={g}>{g}</option>)}
              </select>
            </div>
          )}

          {/* Grid layout picker */}
          <div className="flex items-center gap-1.5">
            <label className="text-sm whitespace-nowrap">Layout:</label>
            <GridPicker
              cols={cols}
              rows={rows}
              onSelect={(c, r) => { setCols(c); setRows(r); setCurrentPage(0); setAutoGrid(false); }}
            />
          </div>

          {isSingleStream && (
            <div className="flex items-center gap-1.5">
              <label htmlFor="stream-selector" className="text-sm whitespace-nowrap">Stream:</label>
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
            title={showLabels ? 'Hide Stream Labels' : 'Show Stream Labels'}
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
            title={showControls ? 'Hide Stream Controls' : 'Show Stream Controls'}
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

          {orderedStreams.length > 1 && (
            <button
              className={`p-2 rounded-full focus:outline-none focus:ring-2 focus:ring-primary ${reorderMode ? 'bg-primary text-primary-foreground hover:bg-primary/90' : 'bg-secondary hover:bg-secondary/80 text-secondary-foreground'}`}
              onClick={toggleReorderMode}
              title={reorderMode ? 'Exit reorder mode' : 'Drag to reorder cameras'}
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
              title="Reset camera order to default"
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
            className="p-2 rounded-full bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none"
            onClick={() => toggleFullscreen()}
            title="Toggle Fullscreen"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path
                d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
            </svg>
          </button>
        </div>
      </div>

      <div className="flex flex-col space-y-4 h-full">
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
                <p className="mt-4 text-muted-foreground">Loading streams...</p>
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
                <p className="mt-4 text-muted-foreground">Loading streams...</p>
              </div>
            </div>
          ) : (streamsError) ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-card text-card-foreground rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-muted-foreground text-lg">Error loading streams: {streamsError.message}</p>
              <button
                onClick={() => window.location.reload()}
                className="btn-primary"
              >
                Retry
              </button>
            </div>
          ) : streams.length === 0 ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-card text-card-foreground rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-muted-foreground text-lg">No streams configured</p>
              <a href="streams.html" className="btn-primary">Configure Streams</a>
            </div>
          ) : (
            // Render video cells using MSEVideoCell (when go2rtc enabled) or HLSVideoCell (fallback)
            // Both modes stagger initialization to avoid a thundering-herd on go2rtc/backend:
            //   MSE:  300ms per stream (WebSocket + codec negotiation burst is lighter)
            //   HLS:  600ms per stream (manifest + segment fetches need more breathing room)
            streamsToShow.map((stream, index) => {
              const VideoCell = useMSE ? MSEVideoCell : HLSVideoCell;
              const initDelay = useMSE ? (index * 300) : (index * 600);
              // Global index in orderedStreams for drag-and-drop (pagination offset)
              const globalIndex = currentPage * maxStreams + index;

              return (
                <div
                  key={stream.name}
                  style={{ position: 'relative' }}
                  draggable={reorderMode}
                  onDragStart={reorderMode ? () => handleDragStart(globalIndex) : undefined}
                  onDragOver={reorderMode ? (e) => handleDragOver(e, globalIndex) : undefined}
                  onDrop={reorderMode ? handleDrop : undefined}
                  onDragEnd={reorderMode ? handleDragEnd : undefined}
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
                      {/* Drag handle bars icon */}
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                           fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                        <line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/>
                      </svg>
                      Drag to reorder
                    </div>
                  )}
                  <VideoCell
                    stream={stream}
                    onToggleFullscreen={toggleStreamFullscreen}
                    streamId={stream.name}
                    initDelay={initDelay}
                    showLabels={showLabels}
                    showControls={showControls}
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
              Previous
            </button>

            <span className="text-foreground">
              Page {currentPage + 1} of {Math.ceil(orderedStreams.length / maxStreams)}
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
              Next
            </button>
          </div>
        ) : null}
      </div>
    </section>
  );
}
