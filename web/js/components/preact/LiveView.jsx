/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the HLS live view page
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { showSnapshotPreview, setupModals, addModalStyles } from './UI.jsx';
import { useFullscreenManager, FullscreenManager } from './FullscreenManager.jsx';
import { startDetectionPolling, cleanupDetectionPolling } from './DetectionOverlay.js';
import { SnapshotManager, useSnapshotManager } from './SnapshotManager.jsx';
import Hls from 'hls.js';

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView({isWebRTCDisabled}) {
  // Use the snapshot manager hook
  const { takeSnapshot } = useSnapshotManager();
  // Use the fullscreen manager hook
  const { isFullscreen, setIsFullscreen, toggleFullscreen } = useFullscreenManager();
  const [streams, setStreams] = useState([]);
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
  // isFullscreen state is now managed by useFullscreenManager
  const [isLoading, setIsLoading] = useState(true);
  // Initialize currentPage from URL if available (URL uses 1-based indexing, internal state uses 0-based)
  const [currentPage, setCurrentPage] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const pageParam = urlParams.get('page');
    // Convert from 1-based (URL) to 0-based (internal)
    return pageParam ? Math.max(0, parseInt(pageParam, 10) - 1) : 0;
  });
  const videoGridRef = useRef(null);
  const hlsPlayers = useRef({});
  const detectionIntervals = useRef({});
  const refreshTimers = useRef({});

  // Set up event listeners and UI components
  useEffect(() => {
    // Set up modals for snapshot preview
    setupModals();
    addModalStyles();

    // Add event listener to stop streams when leaving the page
    const handleBeforeUnload = () => {
      stopAllHLSStreams();
    };

    // Add event listener for visibility change to handle tab switching
    const handleVisibilityChange = () => {
      if (document.hidden) {
        console.log("Page hidden, pausing HLS streams");
        // Pause video elements to reduce resource usage
        Object.keys(hlsPlayers.current).forEach(streamName => {
          const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
          const videoElement = document.getElementById(videoElementId);
          if (videoElement) {
            videoElement.pause();
          }
        });
      } else {
        console.log("Page visible, resuming HLS streams");
        // Resume video playback
        Object.keys(hlsPlayers.current).forEach(streamName => {
          const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
          const videoElement = document.getElementById(videoElementId);
          if (videoElement) {
            videoElement.play().catch(e => {
              console.warn(`Could not resume video for ${streamName}:`, e);
            });
          }
        });
      }
    };

    window.addEventListener('beforeunload', handleBeforeUnload);
    document.addEventListener('visibilitychange', handleVisibilityChange);

    // Cleanup
    return () => {
      // No need to remove handleEscape as it's now handled in FullscreenManager.js
      window.removeEventListener('beforeunload', handleBeforeUnload);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      stopAllHLSStreams();
    };
  }, [streams]); // Add streams as dependency to ensure we have the latest stream data

  // Load streams after the component has rendered and videoGridRef is available
  useEffect(() => {
      // Set loading state initially
      setIsLoading(true);

      // Create a timeout to handle potential stalls in loading
      const timeoutId = setTimeout(() => {
        console.warn('Stream loading timed out');
        setIsLoading(false);
        showStatusMessage('Loading streams timed out. Please try refreshing the page.');
      }, 15000); // 15 second timeout

      // Load streams from API with timeout handling
      loadStreams()
        .then((streamData) => {
          clearTimeout(timeoutId);
          if (streamData && streamData.length > 0) {
            setStreams(streamData);

            // Set selectedStream based on URL parameter if it exists and is valid
            const urlParams = new URLSearchParams(window.location.search);
            const streamParam = urlParams.get('stream');

            if (streamParam && streamData.some(stream => stream.name === streamParam)) {
              // If the stream from URL exists in the loaded streams, use it
              setSelectedStream(streamParam);
            } else if (!selectedStream || !streamData.some(stream => stream.name === selectedStream)) {
              // Otherwise use the first stream if selectedStream is not set or invalid
              setSelectedStream(streamData[0].name);
            }
          } else {
            console.warn('No streams returned from API');
          }
          setIsLoading(false);
        })
        .catch((error) => {
          clearTimeout(timeoutId);
          console.error('Error loading streams:', error);
          showStatusMessage('Error loading streams: ' + error.message);
          setIsLoading(false);
        });
  }, []);

  // Update video grid when layout, page, or streams change
  useEffect(() => {
    updateVideoGrid();
  }, [layout, selectedStream, streams, currentPage]);

  // Update URL when page, layout, or selectedStream changes
  useEffect(() => {
    // Update URL with current page (convert from 0-based internal to 1-based URL)
    const url = new URL(window.location);
    if (currentPage === 0) {
      url.searchParams.delete('page');
    } else {
      // Add 1 to convert from 0-based (internal) to 1-based (URL)
      url.searchParams.set('page', currentPage + 1);
    }

    // Ensure layout parameter is preserved
    if (layout && layout !== '4') { // Only set if not the default
      url.searchParams.set('layout', layout);
    } else if (layout === '4') {
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
  }, [currentPage, layout, selectedStream]);

  /**
   * Load streams from API
   * @returns {Promise<Array>} Promise resolving to array of streams
   */
  const loadStreams = async () => {
    try {
      // Create a timeout promise to handle potential stalls
      const timeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error('Request timed out')), 5000); // 5 second timeout
      });

      // Fetch streams from API with timeout
      const fetchPromise = fetch('/api/streams');
      const response = await Promise.race([fetchPromise, timeoutPromise]);

      if (!response.ok) {
        throw new Error('Failed to load streams');
      }

      // Create another timeout for the JSON parsing
      const jsonTimeoutPromise = new Promise((_, reject) => {
        setTimeout(() => reject(new Error('JSON parsing timed out')), 3000); // 3 second timeout
      });

      const jsonPromise = response.json();
      const data = await Promise.race([jsonPromise, jsonTimeoutPromise]);

      // For HLS view, we need to fetch full details for each stream
      const streamPromises = (data || []).map(stream => {
        // Create a timeout promise for this stream's details fetch
        const detailsTimeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error(`Timeout fetching details for stream ${stream.name}`)), 3000);
        });

        // Fetch stream details with timeout
        const detailsFetchPromise = fetch(`/api/streams/${encodeURIComponent(stream.id || stream.name)}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to load details for stream ${stream.name}`);
            }
            return response.json();
          });

        // Race the fetch against the timeout
        return Promise.race([detailsFetchPromise, detailsTimeoutPromise])
          .catch(error => {
            console.error(`Error loading details for stream ${stream.name}:`, error);
            // Return the basic stream info if we can't get details
            return stream;
          });
      });

      const detailedStreams = await Promise.all(streamPromises);
      console.log('Loaded detailed streams for HLS view:', detailedStreams);

      // Filter out streams that are soft deleted, inactive, or not configured for HLS
      const filteredStreams = detailedStreams.filter(stream => {
        // Filter out soft deleted streams
        if (stream.is_deleted) {
          console.log(`Stream ${stream.name} is soft deleted, filtering out`);
          return false;
        }

        // Filter out inactive streams
        if (!stream.enabled) {
          console.log(`Stream ${stream.name} is inactive, filtering out`);
          return false;
        }

        // Filter out streams not configured for HLS
        if (!stream.streaming_enabled) {
          console.log(`Stream ${stream.name} is not configured for HLS, filtering out`);
          return false;
        }

        return true;
      });

      console.log('Filtered streams for HLS view:', filteredStreams);

      return filteredStreams || [];
    } catch (error) {
      console.error('Error loading streams for WebRTC view:', error);
      showStatusMessage('Error loading streams: ' + error.message);

      return [];
    }
  };

  /**
   * Get maximum number of streams to display based on layout
   * @returns {number} Maximum number of streams
   */
  const getMaxStreamsForLayout = () => {
    switch (layout) {
      case '1': return 1;  // Single view
      case '2': return 2;  // 2x1 grid
      case '4': return 4;  // 2x2 grid
      case '6': return 6;  // 2x3 grid
      case '9': return 9;  // 3x3 grid
      case '16': return 16; // 4x4 grid
      default: return 4;
    }
  };

  /**
   * Update video grid based on layout, streams, and pagination
   */
  const updateVideoGrid = () => {
    if (!videoGridRef.current) return;

    // Clear existing content except placeholder
    const placeholder = videoGridRef.current.querySelector('.placeholder');
    videoGridRef.current.innerHTML = '';

    // If placeholder exists and no streams, add it back
    if (placeholder && streams.length === 0) {
      videoGridRef.current.appendChild(placeholder);
      return;
    }

    // Filter streams based on layout and selected stream
    let streamsToShow = streams;
    if (layout === '1' && selectedStream) {
      streamsToShow = streams.filter(stream => stream.name === selectedStream);
    } else {
      // Apply pagination
      const maxStreams = getMaxStreamsForLayout();
      const totalPages = Math.ceil(streams.length / maxStreams);

      // Ensure current page is valid
      if (currentPage >= totalPages) {
        setCurrentPage(Math.max(0, totalPages - 1));
        return; // Will re-render with corrected page
      }

      // Get streams for current page
      const startIdx = currentPage * maxStreams;
      const endIdx = Math.min(startIdx + maxStreams, streams.length);
      streamsToShow = streams.slice(startIdx, endIdx);
    }

    // Get the names of streams that should be shown
    const streamsToShowNames = streamsToShow.map(stream => stream.name);

    // Clean up connections for streams that are no longer visible
    Object.keys(hlsPlayers.current).forEach(streamName => {
      if (!streamsToShowNames.includes(streamName)) {
        console.log(`Cleaning up HLS player for stream ${streamName} as it's not on the current page`);
        cleanupHLSPlayer(streamName);
      }
    });

    // Stagger initialization of HLS players
    streamsToShow.forEach((stream, index) => {
      // Create video cell immediately for UI responsiveness
      createVideoCell(stream);

      // Only initialize HLS if it's not already connected
      if (!hlsPlayers.current[stream.name]) {
        // Stagger the actual HLS initialization
        setTimeout(() => {
          initializeHLSPlayer(stream);
        }, index * 500); // 500ms delay between each stream initialization
      } else {
        console.log(`HLS player for stream ${stream.name} already exists, reusing`);
      }
    });
  };

  /**
   * Create video cell without initializing WebRTC
   * @param {Object} stream - Stream object
   */
  const createVideoCell = (stream) => {
    // Ensure we have an ID for the stream (use name as fallback if needed)
    const streamId = stream.id || stream.name;

    const videoCell = document.createElement('div');
    videoCell.className = 'video-cell';
    videoCell.dataset.streamName = stream.name;
    videoCell.dataset.streamId = streamId;
    videoCell.style.position = 'relative'; // Create stacking context

    // Create video element
    const videoElement = document.createElement('video');
    videoElement.id = `video-${stream.name.replace(/\s+/g, '-')}`;
    videoElement.className = 'video-element';
    videoElement.playsInline = true;
    videoElement.autoplay = true;
    videoElement.muted = true;

    // Create loading indicator
    const loadingIndicator = document.createElement('div');
    loadingIndicator.className = 'loading-indicator';
    loadingIndicator.innerHTML = `
      <div className="spinner"></div>
      <p>Connecting...</p>
    `;
    loadingIndicator.style.position = 'absolute';
    loadingIndicator.style.top = '0';
    loadingIndicator.style.left = '0';
    loadingIndicator.style.width = '100%';
    loadingIndicator.style.height = '100%';
    loadingIndicator.style.display = 'flex';
    loadingIndicator.style.flexDirection = 'column';
    loadingIndicator.style.justifyContent = 'center';
    loadingIndicator.style.alignItems = 'center';
    loadingIndicator.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
    loadingIndicator.style.color = 'white';
    loadingIndicator.style.zIndex = '20'; // Above video but below controls

    // Create error indicator (hidden by default)
    const errorIndicator = document.createElement('div');
    errorIndicator.className = 'error-indicator';
    errorIndicator.style.display = 'none';
    errorIndicator.style.position = 'absolute';
    errorIndicator.style.top = '0';
    errorIndicator.style.left = '0';
    errorIndicator.style.width = '100%';
    errorIndicator.style.height = '100%';
    errorIndicator.style.flexDirection = 'column';
    errorIndicator.style.justifyContent = 'center';
    errorIndicator.style.alignItems = 'center';
    errorIndicator.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
    errorIndicator.style.color = 'white';
    errorIndicator.style.zIndex = '20'; // Above video but below controls

    // Create stream name overlay
    const streamNameOverlay = document.createElement('div');
    streamNameOverlay.className = 'stream-name-overlay';
    streamNameOverlay.textContent = stream.name;
    streamNameOverlay.style.position = 'absolute';
    streamNameOverlay.style.top = '10px';
    streamNameOverlay.style.left = '10px';
    streamNameOverlay.style.padding = '5px 10px';
    streamNameOverlay.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
    streamNameOverlay.style.color = 'white';
    streamNameOverlay.style.borderRadius = '4px';
    streamNameOverlay.style.fontSize = '14px';
    streamNameOverlay.style.zIndex = '15'; // Above video but below controls

    // Create stream controls
    const streamControls = document.createElement('div');
    streamControls.className = 'stream-controls';
    streamControls.innerHTML = `
      <button class="snapshot-btn" title="Take Snapshot" data-id="${streamId}" data-name="${stream.name}">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path><circle cx="12" cy="13" r="4"></circle></svg>
      </button>
      <button class="fullscreen-btn" title="Toggle Fullscreen" data-id="${streamId}" data-name="${stream.name}">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
      </button>
    `;
    streamControls.style.position = 'absolute';
    streamControls.style.bottom = '10px';
    streamControls.style.right = '10px';
    streamControls.style.display = 'flex';
    streamControls.style.gap = '10px';
    streamControls.style.zIndex = '30'; // Above everything else
    streamControls.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
    streamControls.style.padding = '5px';
    streamControls.style.borderRadius = '4px';

    // Add canvas for detection overlay
    const canvasOverlay = document.createElement('canvas');
    canvasOverlay.id = `canvas-${stream.name.replace(/\s+/g, '-')}`;
    canvasOverlay.className = 'detection-overlay';
    canvasOverlay.style.position = 'absolute';
    canvasOverlay.style.top = '0';
    canvasOverlay.style.left = '0';
    canvasOverlay.style.width = '100%';
    canvasOverlay.style.height = '100%';
    canvasOverlay.style.zIndex = '5'; // Above video but below controls

    // Assemble the video cell
    videoCell.appendChild(videoElement);
    videoCell.appendChild(loadingIndicator);
    videoCell.appendChild(errorIndicator);
    videoCell.appendChild(streamNameOverlay);
    videoCell.appendChild(streamControls);
    videoCell.appendChild(canvasOverlay);

    // Add to grid
    videoGridRef.current.appendChild(videoCell);

    // Make sure all buttons have proper z-index and pointer events
    const allButtons = videoCell.querySelectorAll('button');
    allButtons.forEach(button => {
      button.style.position = 'relative';
      button.style.zIndex = '30';
      button.style.pointerEvents = 'auto';
      button.style.cursor = 'pointer';
      button.style.backgroundColor = 'transparent';
      button.style.border = 'none';
      button.style.padding = '5px';
      button.style.borderRadius = '4px';
      button.style.color = 'white';
      button.style.transition = 'background-color 0.2s';

      // Add hover effect
      button.addEventListener('mouseover', () => {
        button.style.backgroundColor = 'rgba(255, 255, 255, 0.2)';
      });

      button.addEventListener('mouseout', () => {
        button.style.backgroundColor = 'transparent';
      });
    });

    // Add event listeners for buttons
    const snapshotBtn = videoCell.querySelector('.snapshot-btn');
    if (snapshotBtn) {
      snapshotBtn.addEventListener('click', (event) => {
        console.log('Snapshot button clicked for stream:', stream.name);
        event.preventDefault();
        event.stopPropagation();
        takeSnapshot(streamId, event);
      });
    }

    const fullscreenBtn = videoCell.querySelector('.fullscreen-btn');
    if (fullscreenBtn) {
      fullscreenBtn.addEventListener('click', (event) => {
        console.log('Fullscreen button clicked for stream:', stream.name);
        event.preventDefault();
        event.stopPropagation();
        toggleStreamFullscreen(stream.name);
      });
    }
  };

  /**
   * Initialize HLS player for a stream
   * @param {Object} stream - Stream object
   */
  const initializeHLSPlayer = (stream) => {
    const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoElement || !videoCell) return;

    // Show loading state
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
      loadingIndicator.style.display = 'flex';
    }

    // Create canvas overlay for detection bounding boxes
    const canvasId = `canvas-${stream.name.replace(/\s+/g, '-')}`;
    let canvasOverlay = document.getElementById(canvasId);

    if (!canvasOverlay) {
      canvasOverlay = document.createElement('canvas');
      canvasOverlay.id = canvasId;
      canvasOverlay.className = 'detection-overlay';
      canvasOverlay.style.position = 'absolute';
      canvasOverlay.style.top = '0';
      canvasOverlay.style.left = '0';
      canvasOverlay.style.width = '100%';
      canvasOverlay.style.height = '100%';
      videoCell.appendChild(canvasOverlay);
    }

    // Build the HLS stream URL with cache-busting timestamp to prevent stale data
    const timestamp = Date.now();
    const hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${timestamp}`;

    // Check if HLS.js is supported
    if (Hls.isSupported()) {
      console.log(`Using HLS.js for stream ${stream.name}`);
      const hls = new Hls({
        maxBufferLength: 30,            // Increased from 10 to 30 seconds for better buffering on low-power devices
        maxMaxBufferLength: 60,         // Increased from 20 to 60 seconds for better buffering on low-power devices
        liveSyncDurationCount: 4,       // Increased from 3 to 4 segments for better stability
        liveMaxLatencyDurationCount: 10, // Increased from 5 to 10 segments for better stability on low-power devices
        liveDurationInfinity: false,    // Don't treat live streams as infinite duration
        lowLatencyMode: false,          // Disable low latency mode for better stability on low-power devices
        enableWorker: true,
        fragLoadingTimeOut: 30000,      // Increased from 20 to 30 seconds timeout for fragment loading
        manifestLoadingTimeOut: 20000,  // Increased from 15 to 20 seconds timeout for manifest loading
        levelLoadingTimeOut: 20000,     // Increased from 15 to 20 seconds timeout for level loading
        backBufferLength: 60,           // Add back buffer length to keep more segments in memory
        startLevel: -1,                 // Auto-select quality level based on network conditions
        abrEwmaDefaultEstimate: 500000, // Start with a lower bandwidth estimate (500kbps)
        abrBandWidthFactor: 0.7,        // Be more conservative with bandwidth estimates
        abrBandWidthUpFactor: 0.5       // Be more conservative when increasing quality
      });

      hls.loadSource(hlsStreamUrl);
      hls.attachMedia(videoElement);

      hls.on(Hls.Events.MANIFEST_PARSED, function() {
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }

        videoElement.play().catch(error => {
          console.warn('Auto-play prevented:', error);
          // Add play button overlay for user interaction
          addPlayButtonOverlay(videoCell, videoElement);
        });
      });

      hls.on(Hls.Events.ERROR, function(event, data) {
        if (data.fatal) {
          console.error('HLS error:', data);
          hls.destroy();

          // Check if the stream was recently enabled
          const videoCell = videoElement.closest('.video-cell');
          const loadingIndicator = videoCell.querySelector('.loading-indicator');

          // If the stream was recently enabled (indicated by the loading message),
          // automatically retry after a short delay
          if (loadingIndicator &&
              loadingIndicator.style.display === 'flex') {

            console.log(`Stream ${stream.name} failed to load, retrying in 2 seconds...`);

            // Retry after a delay
            setTimeout(() => {
              console.log(`Retrying stream ${stream.name} after failure`);
              // Fetch updated stream info and reinitialize
              fetch(`/api/streams/${encodeURIComponent(stream.name)}`)
                .then(response => response.json())
                .then(updatedStream => {
                  // Cleanup existing player
                  cleanupHLSPlayer(stream.name);
                  // Reinitialize with updated stream info
                  initializeHLSPlayer(updatedStream);
                })
                .catch(error => {
                  console.error(`Error fetching stream info for retry: ${error}`);
                  handleHLSError(stream.name, 'Failed to reconnect');
                });
            }, 2000);
          } else {
            // Regular error handling for non-startup errors
            handleHLSError(stream.name, data.details || 'HLS playback error');
          }
        }
      });

      // Set up less frequent refresh to reduce load on low-power devices
      const refreshInterval = 60000; // 60 seconds
      const refreshTimer = setInterval(() => {
        if (videoCell && hls) {
          console.log(`Refreshing HLS stream for ${stream.name}`);
          const newTimestamp = Date.now();
          const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
          hls.loadSource(newUrl);
        } else {
          // Clear interval if video cell or player no longer exists
          clearInterval(refreshTimer);
        }
      }, refreshInterval);

      // Store hls instance and timer for cleanup
      hlsPlayers.current[stream.name] = hls;
      refreshTimers.current[stream.name] = refreshTimer;

      // Start detection polling if detection is enabled for this stream
      if (stream.detection_based_recording && stream.detection_model) {
        console.log(`Starting detection polling for stream ${stream.name}`);
        startDetectionPolling(stream.name, canvasOverlay, videoElement, detectionIntervals.current);
      } else {
        console.log(`Detection not enabled for stream ${stream.name}`);
      }
    }
    // Check if HLS is supported natively (Safari)
    else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
      console.log(`Using native HLS support for stream ${stream.name}`);
      // Native HLS support (Safari)
      videoElement.src = hlsStreamUrl;
      videoElement.addEventListener('loadedmetadata', function() {
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }
      });

      videoElement.addEventListener('error', function() {
        handleHLSError(stream.name, 'HLS stream failed to load');
      });
    } else {
      // Fallback for truly unsupported browsers
      console.error(`HLS not supported for stream ${stream.name} - neither HLS.js nor native support available`);
      handleHLSError(stream.name, 'HLS not supported by your browser - please use a modern browser');
    }
  };

  /**
   * Add play button overlay for browsers that block autoplay
   * @param {HTMLElement} videoCell - Video cell element
   * @param {HTMLVideoElement} videoElement - Video element
   */
  const addPlayButtonOverlay = (videoCell, videoElement) => {
    const playOverlay = document.createElement('div');
    playOverlay.className = 'play-overlay';
    playOverlay.innerHTML = '<div class="play-button"></div>';
    playOverlay.style.position = 'absolute';
    playOverlay.style.top = '0';
    playOverlay.style.left = '0';
    playOverlay.style.width = '100%';
    playOverlay.style.height = '100%';
    playOverlay.style.display = 'flex';
    playOverlay.style.justifyContent = 'center';
    playOverlay.style.alignItems = 'center';
    playOverlay.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
    playOverlay.style.zIndex = '25';
    playOverlay.style.cursor = 'pointer';

    const playButton = document.createElement('div');
    playButton.className = 'play-button';
    playButton.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polygon points="5 3 19 12 5 21 5 3"></polygon>
      </svg>
    `;
    playOverlay.appendChild(playButton);

    videoCell.appendChild(playOverlay);

    playOverlay.addEventListener('click', () => {
      videoElement.play()
        .then(() => {
          playOverlay.remove();
        })
        .catch(error => {
          console.error('Play failed:', error);
        });
    });
  };

  /**
   * Handle HLS error
   * @param {string} streamName - Stream name
   * @param {string} message - Error message
   */
  const handleHLSError = (streamName, message) => {
    console.error(`HLS error for stream ${streamName}:`, message);

    // Find the video cell
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    if (!videoElement) return;

    const videoCell = videoElement.closest('.video-cell');
    if (!videoCell) return;

    // Hide loading indicator
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
      loadingIndicator.style.display = 'none';
    }

    // Create error indicator if it doesn't exist
    let errorIndicator = videoCell.querySelector('.error-indicator');
    if (!errorIndicator) {
      errorIndicator = document.createElement('div');
      errorIndicator.className = 'error-indicator';
      errorIndicator.style.position = 'absolute';
      errorIndicator.style.top = '0';
      errorIndicator.style.left = '0';
      errorIndicator.style.width = '100%';
      errorIndicator.style.height = '100%';
      errorIndicator.style.display = 'flex';
      errorIndicator.style.flexDirection = 'column';
      errorIndicator.style.justifyContent = 'center';
      errorIndicator.style.alignItems = 'center';
      errorIndicator.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
      errorIndicator.style.color = 'white';
      errorIndicator.style.zIndex = '20'; // Above video but below controls
      videoCell.appendChild(errorIndicator);
    }

    errorIndicator.innerHTML = `
      <div class="error-icon">!</div>
      <p>${message || 'HLS stream failed'}</p>
      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>
    `;
    errorIndicator.style.display = 'flex';

    // Make sure retry button is clickable
    const retryButton = errorIndicator.querySelector('.retry-button');
    if (retryButton) {
      retryButton.style.position = 'relative';
      retryButton.style.zIndex = '30';
      retryButton.style.pointerEvents = 'auto';

      retryButton.addEventListener('click', () => {
        // Show loading indicator
        if (loadingIndicator) {
          loadingIndicator.style.display = 'flex';
        }

        // Hide error indicator
        errorIndicator.style.display = 'none';

        // Cleanup existing player
        cleanupHLSPlayer(streamName);

        // Fetch stream info again and reinitialize
        fetch(`/api/streams/${encodeURIComponent(streamName)}`)
          .then(response => response.json())
          .then(streamInfo => {
            // Reinitialize
            initializeHLSPlayer(streamInfo);
          })
          .catch(error => {
            console.error('Error fetching stream info:', error);

            // Show error indicator again with new message
            errorIndicator.style.display = 'flex';
            const errorMsg = errorIndicator.querySelector('p');
            if (errorMsg) {
              errorMsg.textContent = 'Could not reconnect: ' + error.message;
            }

            // Hide loading indicator
            if (loadingIndicator) {
              loadingIndicator.style.display = 'none';
            }
          });
      });
    }
  };

  /**
   * Cleanup HLS player
   * @param {string} streamName - Stream name
   */
  const cleanupHLSPlayer = (streamName) => {
    // Destroy HLS instance if it exists
    if (hlsPlayers.current[streamName]) {
      hlsPlayers.current[streamName].destroy();
      delete hlsPlayers.current[streamName];
    }

    // Clear refresh timer if it exists
    if (refreshTimers.current[streamName]) {
      clearInterval(refreshTimers.current[streamName]);
      delete refreshTimers.current[streamName];
    }

    // Reset video element
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    if (videoElement) {
      videoElement.pause();
      videoElement.removeAttribute('src');
      videoElement.load();
    }

    // Clean up detection polling
    cleanupDetectionPolling(streamName, detectionIntervals.current);
  };

  /**
   * Stop all HLS streams
   */
  const stopAllHLSStreams = () => {
    // Close all HLS players
    Object.keys(hlsPlayers.current).forEach(streamName => {
      cleanupHLSPlayer(streamName);
    });
  };


// Note: takeSnapshot is now provided by the useSnapshotManager hook

  /**
   * Toggle fullscreen mode for a specific stream
   * @param {string} streamName - Stream name
   */
  const toggleStreamFullscreen = (streamName) => {
    console.log(`Toggling fullscreen for stream: ${streamName}`);
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

    if (!videoCell) {
      console.error('Stream not found:', streamName);
      return;
    }

    if (!document.fullscreenElement) {
      console.log('Entering fullscreen mode for video cell');
      videoCell.requestFullscreen().catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
        showStatusMessage(`Could not enable fullscreen mode: ${err.message}`);
      });
    } else {
      console.log('Exiting fullscreen mode');
      document.exitFullscreen();
    }
  };

  return (
    <section id="live-page" className={`page ${isFullscreen ? 'fullscreen-mode' : ''}`}>
      {/* Include the SnapshotManager component */}
      <SnapshotManager />
      {/* Include the FullscreenManager component */}
      <FullscreenManager isFullscreen={isFullscreen} setIsFullscreen={setIsFullscreen} targetId="live-page" />
      <div className="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div className="flex items-center space-x-2">
          <h2 className="text-xl font-bold mr-4">Live View</h2>
          <div className="flex space-x-2">
            {!isWebRTCDisabled && (
            <button
              id="hls-toggle-btn"
              className="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick={() => window.location.href = '/index.html'}
            >
              WebRTC View
            </button>
                )}
          </div>
        </div>
        <div className="controls flex items-center space-x-2">
          <div className="flex items-center">
            <label for="layout-selector" className="mr-2">Layout:</label>
            <select
              id="layout-selector"
              className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value={layout}
              onChange={(e) => {
                const newLayout = e.target.value;
                setLayout(newLayout);
                setCurrentPage(0); // Reset to first page when layout changes
                // URL will be updated by the useEffect hook
              }}
            >
              <option value="1">1 Stream</option>
              <option value="2">2 Streams</option>
              <option value="4" selected>4 Streams</option>
              <option value="6">6 Streams</option>
              <option value="9">9 Streams</option>
              <option value="16">16 Streams</option>
            </select>
          </div>

          {layout === '1' && (
            <div className="flex items-center">
              <label for="stream-selector" className="mr-2">Stream:</label>
              <select
                id="stream-selector"
                className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
                value={selectedStream}
                onChange={(e) => {
                  const newStream = e.target.value;
                  setSelectedStream(newStream);
                  // URL will be updated by the useEffect hook
                }}
              >
                {streams.map(stream =>
                  <option key={stream.name} value={stream.name}>{stream.name}</option>
                )}
              </select>
            </div>
          )}

          <button
              id="fullscreen-btn"
              className="p-2 rounded-full bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 focus:outline-none"
              onClick={() => toggleFullscreen()}
              title="Toggle Fullscreen"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path
                  d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
            </svg>
          </button>
        </div>
      </div>

      <div className="flex flex-col space-y-4">
        <div
          id="video-grid"
          className={`video-container layout-${layout}`}
          ref={videoGridRef}
        >
          {isLoading ? (
            <div className="flex justify-center items-center col-span-full row-span-full h-64 w-full">
              <div className="flex flex-col items-center justify-center py-8">
                <div className="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                <p className="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
              </div>
            </div>
          ) : streams.length === 0 ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
              <a href="streams.html" className="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
            </div>
          ) : null}
          {/* Video cells will be dynamically added by the updateVideoGrid function */}
        </div>

        {layout !== '1' && streams.length > getMaxStreamsForLayout() ? (
          <div className="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button
              className="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => setCurrentPage(Math.max(0, currentPage - 1))}
              disabled={currentPage === 0}
            >
              Previous
            </button>
            <span className="text-gray-700 dark:text-gray-300">
              Page {currentPage + 1} of {Math.ceil(streams.length / getMaxStreamsForLayout())}
            </span>
            <button
              className="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => setCurrentPage(Math.min(Math.ceil(streams.length / getMaxStreamsForLayout()) - 1, currentPage + 1))}
              disabled={currentPage >= Math.ceil(streams.length / getMaxStreamsForLayout()) - 1}
            >
              Next
            </button>
          </div>
        ) : null}
      </div>
    </section>
  );
}
