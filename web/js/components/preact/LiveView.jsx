/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the HLS live view page
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useMemo } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { setupModals, addModalStyles } from './UI.jsx';
import { useFullscreenManager, FullscreenManager } from './FullscreenManager.jsx';
import { useQuery, useQueryClient } from '../../query-client.js';
import { SnapshotManager, useSnapshotManager } from './SnapshotManager.jsx';
import { HLSVideoCell } from './HLSVideoCell.jsx';

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView({isWebRTCDisabled}) {
  // Use the snapshot manager hook
  const { takeSnapshot } = useSnapshotManager();

  // Use the fullscreen manager hook
  const { isFullscreen, setIsFullscreen, toggleFullscreen } = useFullscreenManager();

  // State for streams and layout
  const [streams, setStreams] = useState([]);
  const [isLoading, setIsLoading] = useState(true);

  // Initialize layout from URL or localStorage if available
  const [layout, setLayout] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const layoutParam = urlParams.get('layout');
    if (layoutParam) {
      return layoutParam;
    }
    // Check localStorage for persisted layout preference
    const storedLayout = localStorage.getItem('lightnvr-hls-layout');
    return storedLayout || '4';
  });

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

  // Set up event listeners and UI components
  useEffect(() => {
    // Set up modals for snapshot preview
    setupModals();
    addModalStyles();
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

          // Fetch details for all streams
          const detailedStreams = await Promise.all(streamsData.map(fetchStreamDetails));
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

  // Update URL when layout, page, or selectedStream changes
  useEffect(() => {
    // Don't update URL during initial load or when streams are empty
    if (streams.length === 0) return;

    console.log('Updating URL parameters');
    const url = new URL(window.location);

    // Handle page parameter (convert from 0-based internal to 1-based URL)
    if (currentPage === 0) {
      url.searchParams.delete('page');
    } else {
      // Add 1 to convert from 0-based (internal) to 1-based (URL)
      url.searchParams.set('page', currentPage + 1);
    }

    // Handle layout parameter
    if (layout !== '4') { // Only set if not the default
      url.searchParams.set('layout', layout);
    } else {
      // Remove layout parameter if it's the default value
      url.searchParams.delete('layout');
    }

    // Handle selectedStream parameter
    if (layout === '1' && selectedStream) {
      url.searchParams.set('stream', selectedStream);
    } else {
      // Remove stream parameter if not in single stream mode
      url.searchParams.delete('stream');
    }

    // Update URL without reloading the page
    window.history.replaceState({}, '', url);

    // Also update sessionStorage
    if (currentPage > 0) {
      sessionStorage.setItem('hls_current_page', (currentPage + 1).toString());
    } else {
      sessionStorage.removeItem('hls_current_page');
    }

    // Save layout to localStorage for persistence across sessions
    localStorage.setItem('lightnvr-hls-layout', layout);

    if (layout === '1' && selectedStream) {
      sessionStorage.setItem('hls_selected_stream', selectedStream);
    } else {
      sessionStorage.removeItem('hls_selected_stream');
    }
  }, [currentPage, layout, selectedStream, streams.length]);

  // Memoize max streams for layout to use throughout the component
  const maxStreams = useMemo(() => {
    switch (layout) {
      case '1': return 1;
      case '2': return 2;
      case '4': return 4;
      case '6': return 6;
      case '9': return 9;
      case '16': return 16;
      default: return 4;
    }
  }, [layout]);

  // Ensure current page is valid when streams or layout changes
  useEffect(() => {
    if (streams.length === 0) return;

    const totalPages = Math.ceil(streams.length / maxStreams);

    if (currentPage >= totalPages) {
      setCurrentPage(Math.max(0, totalPages - 1));
    }
  }, [streams, maxStreams, currentPage]);

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
    let result = streams;
    if (layout === '1' && selectedStream) {
      result = streams.filter(stream => stream.name === selectedStream);
    } else {
      // Apply pagination
      const totalPages = Math.ceil(streams.length / maxStreams);

      // Ensure current page is valid
      if (currentPage >= totalPages && totalPages > 0) {
        result = []; // Will be handled by the effect that watches currentPage
      } else {
        // Get streams for current page
        const startIdx = currentPage * maxStreams;
        const endIdx = Math.min(startIdx + maxStreams, streams.length);
        result = streams.slice(startIdx, endIdx);
      }
    }

    console.log(`[LiveView] streamsToShow computed: ${result.length} streams`, result.map(s => s.name));
    console.log(`[LiveView] layout=${layout}, currentPage=${currentPage}, totalStreams=${streams.length}`);
    return result;
  }, [streams, layout, selectedStream, currentPage, maxStreams]);

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
          <h2 className="text-xl font-bold mr-4">Live View (HLS)</h2>
          <div className="flex space-x-2">
            {!isWebRTCDisabled && (
            <button
              id="hls-toggle-btn"
              className="btn-secondary focus:outline-none focus:ring-2 focus:ring-primary inline-block text-center"
              style={{ position: 'relative', zIndex: 50 }} // Very high z-index to ensure clickability
              onClick={() => {
                window.location.href = '/index.html';
              }}
            >
              WebRTC View
            </button>
                )}
          </div>
        </div>
        <div className="controls flex items-center space-x-2">
          <div className="flex items-center">
            <label htmlFor="layout-selector" className="mr-2">Layout:</label>
            <select
              id="layout-selector"
              className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
              value={layout}
              onChange={(e) => {
                const newLayout = e.target.value;
                setLayout(newLayout);
                setCurrentPage(0); // Reset to first page when layout changes
              }}
            >
              <option value="1">1 Stream</option>
              <option value="2">2 Streams</option>
              <option value="4">4 Streams</option>
              <option value="6">6 Streams</option>
              <option value="9">9 Streams</option>
              <option value="16">16 Streams</option>
            </select>
          </div>

          {layout === '1' && (
            <div className="flex items-center">
              <label htmlFor="stream-selector" className="mr-2">Stream:</label>
              <select
                id="stream-selector"
                className="px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none bg-background text-foreground"
                value={selectedStream}
                onChange={(e) => {
                  const newStream = e.target.value;
                  setSelectedStream(newStream);
                }}
              >
                {streams.map(stream => (
                  <option key={stream.name} value={stream.name}>{stream.name}</option>
                ))}
              </select>
            </div>
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
          className={`video-container layout-${layout}`}
        >
          {isLoadingStreams ? (
              <div className="flex justify-center items-center col-span-full row-span-full h-64 w-full" style={{ pointerEvents: 'none', zIndex: 1 }}>
                <div className="flex flex-col items-center justify-center py-8">
                <div
                  className="inline-block animate-spin rounded-full border-4 border-secondary border-t-primary w-16 h-16"></div>
                <p className="mt-4 text-muted-foreground">Loading streams...</p>
              </div>
            </div>
          ) : (isLoading && !isLoadingStreams) ? (
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
            // Render video cells using our self-contained HLSVideoCell component
            // Pass index for staggered initialization to avoid overwhelming go2rtc
            streamsToShow.map((stream, index) => (
              <HLSVideoCell
                key={stream.name}
                stream={stream}
                onToggleFullscreen={toggleStreamFullscreen}
                streamId={stream.name} // Add explicit streamId prop to prevent re-renders
                initDelay={index * 500} // Stagger initialization by 500ms per stream
              />
            ))
          )}
        </div>

        {layout !== '1' && streams.length > maxStreams ? (
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
              Page {currentPage + 1} of {Math.ceil(streams.length / maxStreams)}
            </span>

            <button
              className="btn-primary focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => {
                console.log('Changing to next page');
                const totalPages = Math.ceil(streams.length / maxStreams);
                setCurrentPage(Math.min(totalPages - 1, currentPage + 1));
              }}
              disabled={currentPage >= Math.ceil(streams.length / maxStreams) - 1}
            >
              Next
            </button>
          </div>
        ) : null}
      </div>
    </section>
  );
}
