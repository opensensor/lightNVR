/**
 * LightNVR Web Interface WebRTCView Component
 * Preact component for the WebRTC view page
 */


import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { showSnapshotPreview, setupModals, addModalStyles } from './UI.jsx';
import { showStatusMessage } from './ToastContainer.jsx';
import { useFullscreenManager, FullscreenManager } from './FullscreenManager.jsx';
import { startDetectionPolling, cleanupDetectionPolling } from './DetectionOverlay.js';
import { useMutation, useQuery, useQueryClient } from '../../query-client.js';
import { WebRTCVideoCell } from './WebRTCVideoCell.jsx';
import { SnapshotManager, useSnapshotManager } from './SnapshotManager.jsx';

/**
 * WebRTCView component
 * @returns {JSX.Element} WebRTCView component
 */
export function WebRTCView() {
  // Use the snapshot manager hook
  const { takeSnapshot } = useSnapshotManager();
  // Use the fullscreen manager hook
  const { isFullscreen, setIsFullscreen, toggleFullscreen } = useFullscreenManager();
  // WebRTC offer mutation hook - we don't specify the URL here as it will be dynamic based on the stream
  const webrtcOfferMutation = useMutation({
    mutationFn: async (data) => {
      const { streamName, ...offerData } = data;
      const auth = localStorage.getItem('auth');

      const response = await fetch(`/api/webrtc?src=${encodeURIComponent(streamName)}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
        },
        body: JSON.stringify(offerData),
        signal: data.signal
      });

      if (!response.ok) {
        throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
      }

      const text = await response.text();
      try {
        return JSON.parse(text);
      } catch (jsonError) {
        console.error(`Error parsing JSON for stream ${streamName}:`, jsonError);
        console.log(`Raw response text: ${text}`);
        throw new Error(`Failed to parse WebRTC answer: ${jsonError.message}`);
      }
    },
    onError: (error, variables) => {
      console.error(`Error sending WebRTC offer for stream ${variables.streamName}:`, error);
    }
  });

  const [streams, setStreams] = useState([]);
  // Initialize layout from URL or sessionStorage if available
  const [layout, setLayout] = useState(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const layoutParam = urlParams.get('layout');
    if (layoutParam) {
      return layoutParam;
    }
    // Check sessionStorage as a backup
    const storedLayout = sessionStorage.getItem('webrtc_layout');
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
    const storedStream = sessionStorage.getItem('webrtc_selected_stream');
    return storedStream || '';
  });

  // isFullscreen state is now managed by useFullscreenManager
  const [isLoading, setIsLoading] = useState(true);

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
  const videoGridRef = useRef(null);
  const webrtcConnections = useRef({});
  const detectionIntervals = useRef({});

  // Set up event listeners and UI components
  useEffect(() => {
    // Set up modals for snapshot preview
    setupModals();
    addModalStyles();

    // Add event listener to preserve URL parameters when page is reloaded
    const handleBeforeUnload = () => {
      console.log('Preserving URL parameters before page reload');

      // Create a URL with the current parameters
      const url = new URL(window.location);

      // Ensure page parameter is set correctly (convert from 0-based internal to 1-based URL)
      if (currentPage > 0) {
        url.searchParams.set('page', currentPage + 1);
      } else {
        url.searchParams.delete('page');
      }

      // Ensure layout parameter is set if not default
      if (layout !== '4') {
        url.searchParams.set('layout', layout);
      } else {
        url.searchParams.delete('layout');
      }

      // Ensure stream parameter is set if in single stream mode
      if (layout === '1' && selectedStream) {
        url.searchParams.set('stream', selectedStream);
      } else {
        url.searchParams.delete('stream');
      }

      // Update URL without triggering navigation
      window.history.replaceState({}, '', url);

      // Store the current page in sessionStorage as a backup
      if (currentPage > 0) {
        sessionStorage.setItem('webrtc_current_page', (currentPage + 1).toString());
      } else {
        sessionStorage.removeItem('webrtc_current_page');
      }

      // Store layout in sessionStorage
      if (layout !== '4') {
        sessionStorage.setItem('webrtc_layout', layout);
      } else {
        sessionStorage.removeItem('webrtc_layout');
      }

      // Store selected stream in sessionStorage
      if (layout === '1' && selectedStream) {
        sessionStorage.setItem('webrtc_selected_stream', selectedStream);
      } else {
        sessionStorage.removeItem('webrtc_selected_stream');
      }
    };

    // Register the beforeunload handler
    window.addEventListener('beforeunload', handleBeforeUnload);

    // Set up periodic connection check
    const connectionCheckInterval = setInterval(() => {
      Object.keys(webrtcConnections.current).forEach(streamName => {
        const pc = webrtcConnections.current[streamName];
        if (pc) {
          // Log connection state for debugging
          console.debug(`WebRTC connection state for ${streamName}: ${pc.connectionState}, ICE state: ${pc.iceConnectionState}`);

          // If connection is failed or disconnected for too long, try to reconnect
          if (pc.iceConnectionState === 'failed' || pc.iceConnectionState === 'disconnected') {
            console.warn(`WebRTC connection for ${streamName} is in ${pc.iceConnectionState} state, will attempt reconnect`);

            // Clean up the old connection
            cleanupWebRTCPlayer(streamName);

            // Find the stream info and reinitialize
            const stream = streams.find(s => s.name === streamName);
            if (stream) {
              console.log(`Attempting to reconnect WebRTC for stream ${streamName}`);
              initializeWebRTCPlayer(stream);
            }
          }
        }
      });
    }, 30000); // Check every 30 seconds

    // Cleanup
    return () => {
      // Remove event listeners
      window.removeEventListener('beforeunload', handleBeforeUnload);
      clearInterval(connectionCheckInterval);
      stopAllWebRTCStreams();
    };
  }, [streams, currentPage, layout, selectedStream]); // Add all relevant dependencies

  // Get query client for fetching and invalidating queries
  const queryClient = useQueryClient();

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
    // Only set loading state based on streams loading, but don't block UI interactions
    setIsLoading(isLoadingStreams);
  }, [isLoadingStreams]);

  // Process streams data when it's loaded
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData)) {
      // Process the streams data
      const processStreams = async () => {
        try {
          // Filter and process the streams
          const filteredStreams = await filterStreamsForWebRTC(streamsData);

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
            console.warn('No streams available for WebRTC view after filtering');
          }
        } catch (error) {
          console.error('Error processing streams:', error);
          showStatusMessage('Error processing streams: ' + error.message);
        }
      };

      processStreams();
    }
  }, [streamsData, selectedStream, queryClient]);

  // Use a ref to track previous values to prevent unnecessary updates
  const previousValues = useRef({ layout, selectedStream, currentPage, streamsLength: streams.length });

  useEffect(() => {
    // Only update if something actually changed
    const prev = previousValues.current;
    if (
      prev.layout !== layout ||
      prev.selectedStream !== selectedStream ||
      prev.currentPage !== currentPage ||
      prev.streamsLength !== streams.length
    ) {
      console.log('Layout, selectedStream, currentPage, or streams changed, updating video grid');
      updateVideoGrid();

      // Update previous values
      previousValues.current = { layout, selectedStream, currentPage, streamsLength: streams.length };
    }
  }, [layout, selectedStream, streams, currentPage]);

  // Update URL when layout, page, or selectedStream changes
  useEffect(() => {
    // Don't update URL during initial load or when streams are empty
    if (streams.length === 0) return;

    // Use a debounce to prevent multiple URL updates in quick succession
    const updateURLTimeout = setTimeout(() => {
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
        sessionStorage.setItem('webrtc_current_page', (currentPage + 1).toString());
      } else {
        sessionStorage.removeItem('webrtc_current_page');
      }

      if (layout !== '4') {
        sessionStorage.setItem('webrtc_layout', layout);
      } else {
        sessionStorage.removeItem('webrtc_layout');
      }

      if (layout === '1' && selectedStream) {
        sessionStorage.setItem('webrtc_selected_stream', selectedStream);
      } else {
        sessionStorage.removeItem('webrtc_selected_stream');
      }
    }, 300); // 300ms debounce

    // Clean up the timeout if the component re-renders before the timeout completes
    return () => clearTimeout(updateURLTimeout);
  }, [currentPage, layout, selectedStream, streams.length]);

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

      // For WebRTC view, we need to fetch full details for each stream
      const streamPromises = streams.map(async (stream) => {
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
          // Return the basic stream info if we can't get details
          return stream;
        }
      });

      const detailedStreams = await Promise.all(streamPromises);
      console.log('Loaded detailed streams for WebRTC view:', detailedStreams);

      // Filter out streams that are soft deleted, inactive, or not configured for streaming
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
      showStatusMessage('Error processing streams: ' + error.message);
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
   * Get streams to show based on layout, selected stream, and pagination
   * @returns {Array} Streams to show
   */
  const getStreamsToShow = () => {
    // Filter streams based on layout and selected stream
    let streamsToShow = streams;
    if (layout === '1' && selectedStream) {
      streamsToShow = streams.filter(stream => stream.name === selectedStream);
    } else {
      // Apply pagination
      const maxStreams = getMaxStreamsForLayout();
      const totalPages = Math.ceil(streams.length / maxStreams);

      // Ensure current page is valid
      if (currentPage >= totalPages && totalPages > 0) {
        // We'll handle this in updateVideoGrid
        return [];
      }

      // Get streams for current page
      const startIdx = currentPage * maxStreams;
      const endIdx = Math.min(startIdx + maxStreams, streams.length);
      streamsToShow = streams.slice(startIdx, endIdx);
    }

    return streamsToShow;
  };

  /**
   * Update video grid based on layout, streams, and pagination
   */
  const updateVideoGrid = () => {
    if (!videoGridRef.current) return;

    // Filter streams based on layout and selected stream
    let streamsToShow = getStreamsToShow();

    // If no streams to show and we have streams, check if page is invalid
    if (streamsToShow.length === 0 && streams.length > 0) {
      const maxStreams = getMaxStreamsForLayout();
      const totalPages = Math.ceil(streams.length / maxStreams);

      if (currentPage >= totalPages) {
        setCurrentPage(Math.max(0, totalPages - 1));
        return; // Will re-render with corrected page
      }
    }

    // Get the names of streams that should be shown
    const streamsToShowNames = streamsToShow.map(stream => stream.name);

    // Log page change for debugging
    console.log(`Updating video grid for page ${currentPage + 1}, showing streams:`, streamsToShowNames);

    // Clean up connections for streams that are no longer visible
    const connectionsToCleanup = Object.keys(webrtcConnections.current).filter(
      streamName => !streamsToShowNames.includes(streamName)
    );

    if (connectionsToCleanup.length > 0) {
      console.log(`Cleaning up ${connectionsToCleanup.length} WebRTC connections that are no longer visible:`, connectionsToCleanup);
      connectionsToCleanup.forEach(streamName => {
        cleanupWebRTCPlayer(streamName);
      });
    }
  };

  /**
   * Initialize WebRTC player for a stream
   * @param {Object} stream - Stream object
   * @param {HTMLVideoElement} videoElement - Video element
   * @param {HTMLCanvasElement} canvasOverlay - Canvas overlay for detection
   * @param {Object} callbacks - Callback functions
   */
  const initializeWebRTCPlayer = (stream, videoElement, canvasOverlay, callbacks = {}) => {
    if (!stream || !videoElement) {
      console.error(`Cannot initialize WebRTC player: missing stream or video element`);
      return;
    }

    // Check if there's already a connection for this stream
    if (webrtcConnections.current[stream.name]) {
      console.log(`WebRTC connection for stream ${stream.name} already exists, cleaning up first`);
      cleanupWebRTCPlayer(stream.name);
    }

    console.log(`Initializing WebRTC player for stream ${stream.name}`);

    // Create a new RTCPeerConnection with ICE servers
    const pc = new RTCPeerConnection({
      iceServers: [
        { urls: 'stun:stun.l.google.com:19302' }
      ],
      // Add additional configuration to ensure proper ICE credentials
      iceTransportPolicy: 'all',
      bundlePolicy: 'balanced',
      rtcpMuxPolicy: 'require',
      sdpSemantics: 'unified-plan'
    });

    // Store the connection for cleanup
    webrtcConnections.current[stream.name] = pc;

    // Add event listeners
    pc.ontrack = (event) => {
      console.log(`Track received for stream ${stream.name}:`, event);
      if (event.track.kind === 'video') {
        videoElement.srcObject = event.streams[0];

        // Add event handlers for video element
        videoElement.onloadeddata = () => {
          console.log(`Video data loaded for stream ${stream.name}`);
          if (callbacks.onLoadedData) {
            callbacks.onLoadedData();
          }
        };

        videoElement.onplaying = () => {
          console.log(`Video playing for stream ${stream.name}`);
          if (callbacks.onPlaying) {
            callbacks.onPlaying();
          }

          // Start detection polling now that the video is playing
          if (stream.detection_based_recording && stream.detection_model && canvasOverlay) {
            console.log(`Starting detection polling for stream ${stream.name} now that video is playing`);
            startDetectionPolling(stream.name, canvasOverlay, videoElement, detectionIntervals.current);
          } else {
            console.log(`Detection not enabled for stream ${stream.name}`);
          }
        };

        videoElement.onerror = (e) => {
          console.error(`Video error for stream ${stream.name}:`, e);
          if (callbacks.onError) {
            callbacks.onError('Video playback error');
          }
        };
      }
    };

    pc.onicecandidate = (event) => {
      if (event.candidate) {
        console.log(`ICE candidate for stream ${stream.name}:`, event.candidate);
        // go2rtc doesn't use a separate ICE endpoint, so we don't need to send ICE candidates
      }
    };

    pc.oniceconnectionstatechange = () => {
      console.log(`ICE connection state for stream ${stream.name}:`, pc.iceConnectionState);

      // Handle different ICE connection states
      if (pc.iceConnectionState === 'failed') {
        console.warn(`ICE failed for stream ${stream.name}`);
        if (callbacks.onError) {
          callbacks.onError('WebRTC ICE connection failed');
        }
      } else if (pc.iceConnectionState === 'disconnected') {
        console.warn(`ICE disconnected for stream ${stream.name}`);
        // Don't immediately handle as error, as disconnected can be temporary
      }
    };

    // Also monitor connection state changes
    pc.onconnectionstatechange = () => {
      console.log(`Connection state changed for stream ${stream.name}:`, pc.connectionState);

      if (pc.connectionState === 'failed') {
        console.warn(`Connection failed for stream ${stream.name}`);
        if (callbacks.onError) {
          callbacks.onError('WebRTC connection failed');
        }
      }
    };

    // Add transceivers to ensure we get both audio and video tracks
    pc.addTransceiver('video', {direction: 'recvonly'});
    pc.addTransceiver('audio', {direction: 'recvonly'});

    // Create an offer with specific codec requirements
    const offerOptions = {
      offerToReceiveAudio: true,
      offerToReceiveVideo: true
    };

    // Create a timeout for the entire WebRTC setup process
    const setupTimeoutId = setTimeout(() => {
      console.warn(`WebRTC setup timed out for stream ${stream.name}`);
      if (callbacks.onError) {
        callbacks.onError('WebRTC setup timed out');
      }

      // Clean up the connection if it exists
      if (webrtcConnections.current[stream.name]) {
        cleanupWebRTCPlayer(stream.name);
      }
    }, 30000); // 30 second timeout for the entire setup process

    // Create a separate timeout for video playback
    const videoPlaybackTimeoutId = setTimeout(() => {
      // Only show error if the connection still exists but video isn't playing
      if (webrtcConnections.current[stream.name] &&
          (!videoElement.srcObject || videoElement.readyState < 2)) { // HAVE_CURRENT_DATA = 2
        console.warn(`Video playback timed out for stream ${stream.name}`);
        if (callbacks.onError) {
          callbacks.onError('Video playback timed out');
        }
      }
    }, 20000); // 20 second timeout for video playback

    // Add a check to ensure the connection still exists before proceeding
    const checkConnectionExists = () => {
      return webrtcConnections.current[stream.name] === pc;
    };

    pc.createOffer(offerOptions)
      .then(offer => {
        if (!checkConnectionExists()) {
          throw new Error('Connection was cleaned up during offer creation');
        }
        console.log(`Created offer for stream ${stream.name}`);
        return pc.setLocalDescription(offer);
      })
      .then(() => {
        if (!checkConnectionExists()) {
          throw new Error('Connection was cleaned up after setting local description');
        }
        console.log(`Set local description for stream ${stream.name}`);
        // Send the offer to the server
        return sendOffer(stream.name, pc.localDescription);
      })
      .then(answer => {
        if (!checkConnectionExists()) {
          throw new Error('Connection was cleaned up after receiving answer');
        }
        console.log(`Received answer for stream ${stream.name}`);
        // Set the remote description
        return pc.setRemoteDescription(new RTCSessionDescription(answer));
      })
      .then(() => {
        if (!checkConnectionExists()) {
          throw new Error('Connection was cleaned up after setting remote description');
        }
        console.log(`Set remote description for stream ${stream.name}`);

        // Clear both timeouts since we've successfully set up the connection
        clearTimeout(setupTimeoutId);
        clearTimeout(videoPlaybackTimeoutId);
      })
      .catch(error => {
        // Clear both timeouts
        clearTimeout(setupTimeoutId);
        clearTimeout(videoPlaybackTimeoutId);

        // Only log and call error callback if the connection still exists
        if (checkConnectionExists()) {
          console.error(`Error setting up WebRTC for stream ${stream.name}:`, error);
          if (callbacks.onError) {
            callbacks.onError(error.message);
          }
        } else {
          console.log(`WebRTC setup for stream ${stream.name} was cancelled: ${error.message}`);
        }
      });

    // Add event listener to clear the video playback timeout when video starts playing
    videoElement.addEventListener('playing', () => {
      clearTimeout(videoPlaybackTimeoutId);
    }, { once: true }); // Use once: true to ensure it only fires once
  };

  /**
   * Send WebRTC offer to server
   * @param {string} streamName - Stream name
   * @param {RTCSessionDescription} offer - WebRTC offer
   * @returns {Promise<RTCSessionDescription>} Promise resolving to WebRTC answer
   */
  const sendOffer = useCallback(async (streamName, offer) => {
    try {
      // Format the offer according to go2rtc expectations
      const formattedOffer = {
        type: offer.type,
        sdp: offer.sdp
      };

      console.log(`Sending formatted offer for stream ${streamName}`);

      // Create an AbortController to allow cancellation of the request
      const abortController = new AbortController();
      const signal = abortController.signal;

      // Store the abort controller in the connection object for later cleanup
      if (webrtcConnections.current[streamName]) {
        webrtcConnections.current[streamName].abortController = abortController;
      } else {
        // If the connection no longer exists, abort immediately
        console.log(`Connection for stream ${streamName} no longer exists, aborting offer`);
        abortController.abort();
        return Promise.reject(new Error('Connection no longer exists'));
      }

      // Use the mutation to send the offer with the abort signal
      const result = await webrtcOfferMutation.mutateAsync({
        ...formattedOffer,
        streamName, // Add streamName for the URL construction in mutationFn
        signal      // Add signal for request cancellation
      });

      // Check if the connection still exists before returning the result
      if (webrtcConnections.current[streamName]) {
        return result;
      } else {
        // If the connection was cleaned up during the request, abort and reject
        console.log(`Connection for stream ${streamName} was cleaned up during offer, rejecting result`);
        return Promise.reject(new Error('Connection was cleaned up during offer'));
      }
    } catch (error) {
      // Check if this was an abort error, which we can safely ignore
      if (error.name === 'AbortError') {
        console.log(`WebRTC offer request for stream ${streamName} was aborted`);
        // Return a rejected promise to stop the WebRTC connection process
        return Promise.reject(new Error('Request aborted'));
      }

      console.error(`Error sending offer for stream ${streamName}:`, error);
      throw error;
    }
  }, [webrtcOfferMutation]);

  // ICE candidates are handled internally by the browser for go2rtc

  /**
   * Handle WebRTC error
   * @param {string} streamName - Stream name
   * @param {string} message - Error message
   */
  const handleWebRTCError = async (streamName, message) => {
    console.error(`WebRTC error for stream ${streamName}:`, message);

    // The WebRTCVideoCell component now handles its own error state
    // We just need to update the connection state and let the component handle the UI

    // Cleanup existing connection to trigger the error state in the component
    cleanupWebRTCPlayer(streamName);

    // Find the stream in our streams array
    const stream = streams.find(s => s.name === streamName);

    if (stream) {
      console.log(`Found stream ${streamName} in local state, will be reinitialized by component`);
      // The component will handle retrying through its error UI
    } else {
      console.log(`Stream ${streamName} not found in local state, this is unexpected`);
      // This is an edge case that shouldn't happen in normal operation
      // The component will show an error state
    }
  };

  /**
   * Cleanup WebRTC player
   * @param {string} streamName - Stream name
   */
  const cleanupWebRTCPlayer = (streamName) => {
    console.log(`Cleaning up WebRTC player for stream ${streamName}`);

    // Close and remove the RTCPeerConnection
    if (webrtcConnections.current[streamName]) {
      // Create a local reference to the connection before deleting it
      const connection = webrtcConnections.current[streamName];

      // Abort any pending fetch requests
      if (connection.abortController) {
        console.log(`Aborting pending WebRTC requests for stream ${streamName}`);
        try {
          connection.abortController.abort();
        } catch (e) {
          console.error(`Error aborting WebRTC request for stream ${streamName}:`, e);
        }
      }

      // Remove all event listeners to prevent memory leaks
      if (connection.onicecandidate) connection.onicecandidate = null;
      if (connection.oniceconnectionstatechange) connection.oniceconnectionstatechange = null;
      if (connection.onconnectionstatechange) connection.onconnectionstatechange = null;
      if (connection.ontrack) connection.ontrack = null;

      // Close the connection
      connection.close();

      // Remove from our reference object
      delete webrtcConnections.current[streamName];

      console.log(`Closed WebRTC connection for stream ${streamName}`);
    }

    // Clean up detection polling
    cleanupDetectionPolling(streamName, detectionIntervals.current);
  };

  /**
   * Stop all WebRTC streams
   */
  const stopAllWebRTCStreams = () => {
    console.log('Stopping all WebRTC streams');
    // Close all RTCPeerConnections
    Object.keys(webrtcConnections.current).forEach(streamName => {
      cleanupWebRTCPlayer(streamName);
    });
    console.log('All WebRTC streams stopped');
  };

// Note: takeSnapshot is now provided by the useSnapshotManager hook

  /**
   * Toggle fullscreen mode for a specific stream
   * @param {string} streamName - Stream name
   * @param {Event} event - Click event
   * @param {HTMLElement} cellElement - The video cell element (passed from the component)
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
  };

  return (
    <section
      id="live-page"
      className={`page ${isFullscreen ? 'fullscreen-mode' : ''}`}
      style={{
        // Ensure the section doesn't block navigation during loading
        position: 'relative',
        zIndex: 1
      }}
    >
      {/* Include the SnapshotManager component */}
      <SnapshotManager />
      {/* Include the FullscreenManager component */}
      <FullscreenManager isFullscreen={isFullscreen} setIsFullscreen={setIsFullscreen} targetId="live-page" />
      <div className="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div className="flex items-center space-x-2">
          <h2 className="text-xl font-bold mr-4">Live View</h2>
          <div className="flex space-x-2">
            <button
              id="hls-toggle-btn"
              className="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick={() => window.location.href = '/hls.html'}
            >
              HLS View
            </button>
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
            style={{
              // Ensure the video grid doesn't block navigation during loading
              position: 'relative',
              zIndex: 1
            }}
        >
          {isLoadingStreams ? (
              <div className="flex justify-center items-center col-span-full row-span-full h-64 w-full">
                <div className="flex flex-col items-center justify-center py-8">
                  <div
                      className="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                  <p className="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
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
                    className="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                <p className="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
              </div>
            </div>
          ) : (streamsError) ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-gray-600 dark:text-gray-300 text-lg">Error loading streams: {streamsError.message}</p>
              <button
                onClick={() => window.location.reload()}
                className="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
              >
                Retry
              </button>
            </div>
          ) : streams.length === 0 ? (
            <div className="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p className="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
              <a href="streams.html" className="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
            </div>
          ) : (
            // Render video cells using our WebRTCVideoCell component
            getStreamsToShow().map(stream => (
              <WebRTCVideoCell
                key={stream.name}
                stream={stream}
                onTakeSnapshot={takeSnapshot}
                onToggleFullscreen={toggleStreamFullscreen}
                webrtcConnections={webrtcConnections}
                detectionIntervals={detectionIntervals}
                initializeWebRTCPlayer={initializeWebRTCPlayer}
                cleanupWebRTCPlayer={cleanupWebRTCPlayer}
              />
            ))
          )}
        </div>

        {layout !== '1' && streams.length > getMaxStreamsForLayout() ? (
          <div className="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button
              className="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => {
                console.log('Changing to previous page');
                setCurrentPage(Math.max(0, currentPage - 1));

                // Update URL and sessionStorage
                const url = new URL(window.location);
                const newPage = currentPage - 1;

                if (newPage > 0) {
                  url.searchParams.set('page', newPage + 1);
                  sessionStorage.setItem('webrtc_current_page', (newPage + 1).toString());
                } else {
                  url.searchParams.delete('page');
                  sessionStorage.removeItem('webrtc_current_page');
                }

                window.history.replaceState({}, '', url);
              }}
              disabled={currentPage === 0}
            >
              Previous
            </button>

            <span className="text-gray-700 dark:text-gray-300">
              Page {currentPage + 1} of {Math.ceil(streams.length / getMaxStreamsForLayout())}
            </span>

            <button
              className="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => {
                console.log('Changing to next page');
                const newPage = Math.min(Math.ceil(streams.length / getMaxStreamsForLayout()) - 1, currentPage + 1);
                setCurrentPage(newPage);

                // Update URL and sessionStorage
                const url = new URL(window.location);
                url.searchParams.set('page', newPage + 1);
                sessionStorage.setItem('webrtc_current_page', (newPage + 1).toString());

                window.history.replaceState({}, '', url);
              }}
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
