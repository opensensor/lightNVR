/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the live view page using preact-query
 */

import { html } from '../../html-helper.js';
import { useState, useEffect, useRef } from 'preact/hooks';
import { LoadingIndicator } from './LoadingIndicator.js';
import { QueryClientProvider, queryClient } from '../../query-client.js';

// Import modular components
import { useStreams, useStreamDetails, updateVideoGrid, filterStreamsForLiveView } from './StreamGridQuery.js';
import { useQuery } from '../../query-client.js';
import { stopAllStreams } from './VideoPlayer.js';
import { toggleFullscreen, exitFullscreenMode } from './FullscreenManager.js';

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView() {
  // Initialize layout from URL if available
  const [layout, setLayout] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get('layout') || '4';
  });

  // Initialize selectedStream from URL if available
  const [selectedStream, setSelectedStream] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get('stream') || '';
  });

  const [isFullscreen, setIsFullscreen] = useState(false);
  const [detailedStreams, setDetailedStreams] = useState([]);

  // Initialize currentPage from URL if available (URL uses 1-based indexing, internal state uses 0-based)
  const [currentPage, setCurrentPage] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const pageParam = urlParams.get('page');
    // Convert from 1-based (URL) to 0-based (internal)
    return pageParam ? Math.max(0, parseInt(pageParam, 10) - 1) : 0;
  });

  const videoGridRef = useRef(null);
  const videoPlayers = useRef({});
  const detectionIntervals = useRef({});

  // Fetch settings to check if WebRTC is disabled
  const {
    data: settings,
    isLoading: isLoadingSettings
  } = useQuery({
    queryKey: ['settings'],
    queryFn: async () => {
      const response = await fetch('/api/settings');
      if (!response.ok) {
        throw new Error('Failed to fetch settings');
      }
      return response.json();
    }
  });

  // Fetch streams using preact-query
  const {
    data: streamsData,
    isLoading: isLoadingStreams,
    error: streamsError
  } = useStreams();

  // Process streams data when it's loaded
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData)) {
      // Fetch details for each stream
      const fetchStreamDetails = async () => {
        const detailedStreamsPromises = streamsData.map(async (stream) => {
          try {
            const streamId = stream.id || stream.name;
            const response = await fetch(`/api/streams/${encodeURIComponent(streamId)}`);
            if (!response.ok) {
              throw new Error(`Failed to load details for stream ${stream.name}`);
            }
            return await response.json();
          } catch (error) {
            console.error(`Error loading details for stream ${stream.name}:`, error);
            // Return the basic stream info if we can't get details
            return stream;
          }
        });

        const detailedStreamsResult = await Promise.all(detailedStreamsPromises);
        console.log('Loaded detailed streams for live view:', detailedStreamsResult);

        // Filter streams for live view
        const filteredStreams = filterStreamsForLiveView(detailedStreamsResult);
        console.log('Filtered streams for live view:', filteredStreams);

        // Store filtered streams in state
        setDetailedStreams(filteredStreams || []);

        // Handle selectedStream based on URL parameter
        if (filteredStreams.length > 0) {
          // Check if the selectedStream from URL exists in the loaded streams
          const urlParams = new URLSearchParams(window.location.search);
          const streamParam = urlParams.get('stream');

          if (streamParam && filteredStreams.some(stream => stream.name === streamParam)) {
            // If the stream from URL exists in the loaded streams, use it
            setSelectedStream(streamParam);
          } else if (!selectedStream || !filteredStreams.some(stream => stream.name === selectedStream)) {
            // Otherwise use the first stream if selectedStream is not set or invalid
            setSelectedStream(filteredStreams[0].name);
          }
        }
      };

      fetchStreamDetails();
    }
  }, [streamsData]);

  // Set up event listeners
  useEffect(() => {
    // Set up Escape key to exit fullscreen mode
    const handleEscape = (e) => {
      if (e.key === 'Escape') {
        console.log("Escape key pressed, current fullscreen state:", isFullscreen);
        // Check if we're in fullscreen mode by checking the DOM directly
        const livePage = document.getElementById('live-page');
        if (livePage && livePage.classList.contains('fullscreen-mode')) {
          console.log("Detected fullscreen mode via DOM, exiting fullscreen");
          exitFullscreenMode(null, setIsFullscreen);
        }
      }
    };

    document.addEventListener('keydown', handleEscape);

    // Add event listener to stop streams when leaving the page
    const handleBeforeUnload = () => {
      stopAllStreams(detailedStreams, videoPlayers.current, detectionIntervals.current);
    };

    window.addEventListener('beforeunload', handleBeforeUnload);

    // Cleanup
    return () => {
      document.removeEventListener('keydown', handleEscape);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      stopAllStreams(detailedStreams, videoPlayers.current, detectionIntervals.current);
    };
  }, []);

  // Update video grid when layout, page, or streams change
  useEffect(() => {
    if (videoGridRef.current) {
      updateVideoGrid(
        videoGridRef.current,
        detailedStreams,
        layout,
        selectedStream,
        videoPlayers.current,
        detectionIntervals.current,
        currentPage
      );
    }
  }, [layout, selectedStream, detailedStreams, currentPage]);

  // Update URL when layout or selectedStream changes
  useEffect(() => {
    // Don't update URL during initial load
    if (detailedStreams.length === 0) return;

    const url = new URL(window.location.href);

    // Handle layout parameter
    if (layout !== '4') {
      // Only set layout in URL if it's not the default
      url.searchParams.set('layout', layout);
    } else {
      // Remove layout parameter if it's the default value
      url.searchParams.delete('layout');
    }

    // Handle stream parameter
    if (layout === '1' && selectedStream) {
      url.searchParams.set('stream', selectedStream);
    } else {
      // Remove stream parameter if not in single stream mode
      url.searchParams.delete('stream');
    }

    // Handle page parameter
    if (currentPage === 0) {
      url.searchParams.delete('page');
    } else {
      // Add 1 to convert from 0-based (internal) to 1-based (URL)
      url.searchParams.set('page', (currentPage + 1).toString());
    }

    window.history.replaceState({}, '', url);
  }, [layout, selectedStream, currentPage, detailedStreams.length]);

  // Handle layout change
  const handleLayoutChange = (e) => {
    const newLayout = e.target.value;
    setLayout(newLayout);

    // Reset to first page when changing layout
    setCurrentPage(0);

    // URL will be updated by the useEffect hook
  };

  // Handle stream selection change
  const handleStreamChange = (e) => {
    const newStream = e.target.value;
    setSelectedStream(newStream);

    // URL will be updated by the useEffect hook
  };

  // Handle page change
  const handlePageChange = (newPage) => {
    // Ensure page is within bounds
    const maxStreams = parseInt(layout);
    const totalPages = Math.ceil(detailedStreams.length / maxStreams);

    if (newPage < 0 || newPage >= totalPages) {
      return;
    }

    setCurrentPage(newPage);

    // URL will be updated by the useEffect hook
  };

  // Handle fullscreen toggle
  const handleFullscreenToggle = () => {
    toggleFullscreen(setIsFullscreen);
  };

  // Calculate pagination info
  const maxStreams = parseInt(layout);
  const totalPages = Math.ceil(detailedStreams.length / maxStreams);
  const startIdx = currentPage * maxStreams;
  const endIdx = Math.min(startIdx + maxStreams, detailedStreams.length);
  const visibleStreams = detailedStreams.slice(startIdx, endIdx);

  return html`
    <section id="live-page" class="page ${isFullscreen ? 'fullscreen-mode' : ''}">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center space-x-2">
          <h2 class="text-xl font-bold mr-4">Live View</h2>
          ${!settings?.webrtc_disabled && html`
            <div class="flex space-x-2">
              <button
                id="webrtc-toggle-btn"
                class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                onClick=${() => window.location.href = '/index.html'}
              >
                WebRTC View
              </button>
            </div>
          `}
        </div>

        <div class="flex space-x-4">
          <!-- Layout selector -->
          <div class="flex items-center">
            <label for="layout-selector" class="mr-2">Layout:</label>
            <select
              id="layout-selector"
              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value=${layout}
              onChange=${handleLayoutChange}
            >
              <option value="1">1 Stream</option>
              <option value="2">2 Streams</option>
              <option value="4">4 Streams</option>
              <option value="6">6 Streams</option>
              <option value="9">9 Streams</option>
              <option value="16">16 Streams</option>
            </select>
          </div>

          <!-- Stream selector (only visible in single-stream layout) -->
          ${layout === '1' && html`
            <div class="flex items-center">
              <label for="stream-selector" class="mr-2">Stream:</label>
              <select
                id="stream-selector"
                class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
                value=${selectedStream}
                onChange=${handleStreamChange}
              >
                ${detailedStreams.map(stream => html`
                  <option key=${stream.name} value=${stream.name}>${stream.name}</option>
                `)}
              </select>
            </div>
          `}

          <!-- Fullscreen button -->
          <button
            class="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
            onClick=${handleFullscreenToggle}
            title="Toggle Fullscreen"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
            </svg>
          </button>
        </div>
      </div>

      <!-- Video grid -->
      <div class="video-container layout-${layout}" ref=${videoGridRef}>
          ${isLoadingStreams && html`
            <div class="placeholder">
              <${LoadingIndicator} message="Loading streams..." />
            </div>
          `}

          ${!isLoadingStreams && detailedStreams.length === 0 && html`
            <div class="placeholder">
              <div class="flex flex-col items-center justify-center py-12 text-center">
                <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                </svg>
                <p class="text-gray-600 dark:text-gray-400 text-lg">No streams available</p>
                <p class="text-gray-500 dark:text-gray-500 mt-2">Configure streams in the settings page</p>
              </div>
            </div>
          `}
      </div>

      <!-- Pagination controls (only visible when needed) -->
      ${layout !== '1' && totalPages > 1 && html`
        <div class="pagination-controls flex justify-center items-center space-x-4 mt-4">
          <button
            class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
            onClick=${() => handlePageChange(currentPage - 1)}
            disabled=${currentPage === 0}
          >
            Previous
          </button>
          <span class="text-gray-700 dark:text-gray-300">
            Page ${currentPage + 1} of ${totalPages}
          </span>
          <button
            class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
            onClick=${() => handlePageChange(currentPage + 1)}
            disabled=${currentPage >= totalPages - 1}
          >
            Next
          </button>
        </div>
      `}
    </section>
  `;
}

/**
 * Load LiveView component
 */
export function loadLiveView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;

  // Import preact
  import('preact').then(({ render }) => {
    // Render the LiveView component wrapped with QueryClientProvider
    render(
      html`
        <${QueryClientProvider} client=${queryClient}>
          <${LiveView} />
        <//>
      `,
      mainContent
    );
  });
}
