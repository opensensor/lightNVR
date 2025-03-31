/**
 * LightNVR Web Interface WebRTCView Component
 * Preact component for the WebRTC view page
 */

import { h } from '../../preact.min.js';
import { html } from '../../html-helper.js';
import { useState, useEffect, useRef } from '../../preact.hooks.module.js';
import { showStatusMessage, showSnapshotPreview, setupModals, addStatusMessageStyles, addModalStyles } from './UI.js';
import { toggleFullscreen, exitFullscreenMode } from './FullscreenManager.js';
import { startDetectionPolling, cleanupDetectionPolling } from './DetectionOverlay.js';

/**
 * WebRTCView component
 * @returns {JSX.Element} WebRTCView component
 */
export function WebRTCView() {
  const [streams, setStreams] = useState([]);
  const [layout, setLayout] = useState('4');
  const [selectedStream, setSelectedStream] = useState('');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [currentPage, setCurrentPage] = useState(0);
  const videoGridRef = useRef(null);
  const webrtcConnections = useRef({});
  const detectionIntervals = useRef({});
  
  // Set up event listeners and UI components
  useEffect(() => {
    // Set up modals for snapshot preview
    setupModals();
    addStatusMessageStyles();
    addModalStyles();
    
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
      stopAllWebRTCStreams();
    };
    
    // Add event listener for visibility change to handle tab switching
    const handleVisibilityChange = () => {
      if (document.hidden) {
        console.log("Page hidden, pausing WebRTC streams");
        // Mark connections as inactive but don't close them yet
        Object.keys(webrtcConnections.current).forEach(streamName => {
          const pc = webrtcConnections.current[streamName];
          if (pc && pc.connectionState !== 'closed') {
            // Pause video elements to reduce resource usage
            const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
            const videoElement = document.getElementById(videoElementId);
            if (videoElement) {
              videoElement.pause();
            }
          }
        });
      } else {
        console.log("Page visible, resuming WebRTC streams");
        // Resume video playback
        Object.keys(webrtcConnections.current).forEach(streamName => {
          const pc = webrtcConnections.current[streamName];
          if (pc && pc.connectionState !== 'closed') {
            const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
            const videoElement = document.getElementById(videoElementId);
            if (videoElement) {
              videoElement.play().catch(e => {
                console.warn(`Could not resume video for ${streamName}:`, e);
              });
            }
          }
        });
      }
    };
    
    window.addEventListener('beforeunload', handleBeforeUnload);
    document.addEventListener('visibilitychange', handleVisibilityChange);
    
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
      document.removeEventListener('keydown', handleEscape);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      clearInterval(connectionCheckInterval);
      stopAllWebRTCStreams();
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
            setSelectedStream(streamData[0].name);
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
      
      // For WebRTC view, we need to fetch full details for each stream
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
      console.log('Loaded detailed streams for WebRTC view:', detailedStreams);
      
      return detailedStreams || [];
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
    
    // Get names of streams that should be shown
    const streamsToShowNames = streamsToShow.map(stream => stream.name);
    
    // Clean up connections for streams that are no longer visible
    Object.keys(webrtcConnections.current).forEach(streamName => {
      if (!streamsToShowNames.includes(streamName)) {
        console.log(`Cleaning up WebRTC connection for stream ${streamName} (not visible in current view)`);
        cleanupWebRTCPlayer(streamName);
      }
    });
    
    // Add video elements for each stream
    streamsToShow.forEach(stream => {
      // Ensure we have an ID for the stream (use name as fallback if needed)
      const streamId = stream.id || stream.name;
      
      const videoCell = document.createElement('div');
      videoCell.className = 'video-cell';
      
      videoCell.innerHTML = `
        <video id="video-${stream.name.replace(/\s+/g, '-')}" autoplay muted></video>
        <div class="stream-info">
          <span>${stream.name}</span>
          <span>${stream.width}x${stream.height} · ${stream.fps}fps</span>
          <div class="stream-controls">
            <button class="snapshot-btn" data-id="${streamId}" data-name="${stream.name}">
              <span>📷</span> Snapshot
            </button>
            <button class="fullscreen-btn" data-id="${streamId}" data-name="${stream.name}">
              <span>⛶</span> Fullscreen
            </button>
          </div>
        </div>
        <div class="loading-indicator">
          <div class="loading-spinner"></div>
          <span>Connecting WebRTC...</span>
        </div>
      `;
      
      videoGridRef.current.appendChild(videoCell);
      
      // Initialize WebRTC player
      initializeWebRTCPlayer(stream);
      
      // Add event listeners for buttons
      const snapshotBtn = videoCell.querySelector('.snapshot-btn');
      if (snapshotBtn) {
        snapshotBtn.addEventListener('click', () => {
          takeSnapshot(streamId);
        });
      }
      
      const fullscreenBtn = videoCell.querySelector('.fullscreen-btn');
      if (fullscreenBtn) {
        fullscreenBtn.addEventListener('click', () => {
          toggleStreamFullscreen(stream.name);
        });
      }
    });
  };
  
  /**
   * Initialize WebRTC player for a stream
   * @param {Object} stream - Stream object
   */
  const initializeWebRTCPlayer = (stream) => {
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
      canvasOverlay.style.pointerEvents = 'none'; // Allow clicks to pass through
      videoCell.appendChild(canvasOverlay);
    }
    
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
        
        // Hide loading indicator when video starts playing
        videoElement.onloadeddata = () => {
          if (loadingIndicator) {
            loadingIndicator.style.display = 'none';
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
      if (pc.iceConnectionState === 'failed' || pc.iceConnectionState === 'disconnected') {
        // Handle connection failure
        handleWebRTCError(stream.name, 'WebRTC connection failed');
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
      handleWebRTCError(stream.name, 'WebRTC setup timed out');
      
      // Clean up the connection if it exists
      if (webrtcConnections.current[stream.name]) {
        cleanupWebRTCPlayer(stream.name);
      }
    }, 15000); // 15 second timeout for the entire setup process
    
    pc.createOffer(offerOptions)
      .then(offer => {
        console.log(`Created offer for stream ${stream.name}:`, offer);
        
        // Log the original SDP to ensure it has ice-ufrag and ice-pwd
        console.log(`Original SDP for stream ${stream.name}:`, offer.sdp);
        
        // Check if the SDP has ice-ufrag and ice-pwd
        if (!offer.sdp.includes('a=ice-ufrag:') || !offer.sdp.includes('a=ice-pwd:')) {
          console.warn(`SDP for stream ${stream.name} is missing ice-ufrag or ice-pwd!`);
        }
        
        // We'll use the original offer without modifications to preserve ice-ufrag and ice-pwd
        console.log(`Using original offer for stream ${stream.name}`);
        return pc.setLocalDescription(offer);
      })
      .then(() => {
        console.log(`Set local description for stream ${stream.name}`);
        // Send the offer to the server
        return sendOffer(stream.name, pc.localDescription);
      })
      .then(answer => {
        console.log(`Received answer for stream ${stream.name}:`, answer);
        // Set the remote description
        return pc.setRemoteDescription(new RTCSessionDescription(answer));
      })
      .then(() => {
        console.log(`Set remote description for stream ${stream.name}`);
        
        // Clear the setup timeout since we've successfully set up the connection
        clearTimeout(setupTimeoutId);
        
        // Start detection polling if detection is enabled for this stream
        console.log(`Stream ${stream.name} detection settings:`, {
          detection_based_recording: stream.detection_based_recording,
          detection_model: stream.detection_model,
          detection_threshold: stream.detection_threshold
        });
        
        if (stream.detection_based_recording && stream.detection_model) {
          console.log(`Starting detection polling for stream ${stream.name}`);
          startDetectionPolling(stream.name, canvasOverlay, videoElement, detectionIntervals.current);
        } else {
          console.log(`Detection not enabled for stream ${stream.name}`);
        }
      })
      .catch(error => {
        // Clear the setup timeout
        clearTimeout(setupTimeoutId);
        
        console.error(`Error setting up WebRTC for stream ${stream.name}:`, error);
        handleWebRTCError(stream.name, error.message);
      });
  };
  
  /**
   * Send WebRTC offer to server
   * @param {string} streamName - Stream name
   * @param {RTCSessionDescription} offer - WebRTC offer
   * @returns {Promise<RTCSessionDescription>} Promise resolving to WebRTC answer
   */
  const sendOffer = async (streamName, offer) => {
    try {
      // Get auth from localStorage
      const auth = localStorage.getItem('auth');
      
      // Send the offer to the server
      // Format the offer according to go2rtc expectations
      const formattedOffer = {
        type: offer.type,
        sdp: offer.sdp
      };
      
      console.log(`Sending formatted offer for stream ${streamName}:`, formattedOffer);
      
      // Create an AbortController for the fetch request
      const controller = new AbortController();
      const signal = controller.signal;
      
      // Set a timeout to abort the fetch after 8 seconds
      const timeoutId = setTimeout(() => {
        console.warn(`Aborting WebRTC offer request for stream ${streamName} due to timeout`);
        controller.abort();
      }, 8000);
      
      try {
        // Note: Session cookie is automatically included in fetch requests
        // We only need to add the Authorization header if we have it in localStorage
        const response = await fetch(`/api/webrtc?src=${encodeURIComponent(streamName)}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
          },
          body: JSON.stringify(formattedOffer),
          signal: signal
        });
        
        // Clear the timeout since the request completed
        clearTimeout(timeoutId);
        
        if (!response.ok) {
          throw new Error(`Failed to send offer: ${response.status} ${response.statusText}`);
        }
        
        // Create another AbortController for the JSON parsing
        const jsonController = new AbortController();
        const jsonSignal = jsonController.signal;
        
        // Set a timeout to abort the JSON parsing after 5 seconds
        const jsonTimeoutId = setTimeout(() => {
          console.warn(`Aborting JSON parsing for stream ${streamName} due to timeout`);
          jsonController.abort();
        }, 5000);
        
        try {
          // Use a separate try/catch for the JSON parsing
          const text = await response.text();
          
          // Clear the JSON timeout
          clearTimeout(jsonTimeoutId);
          
          // Try to parse the JSON
          try {
            const answer = JSON.parse(text);
            return answer;
          } catch (jsonError) {
            console.error(`Error parsing JSON for stream ${streamName}:`, jsonError);
            console.log(`Raw response text: ${text}`);
            throw new Error(`Failed to parse WebRTC answer: ${jsonError.message}`);
          }
        } catch (textError) {
          // Clear the JSON timeout if it hasn't been cleared yet
          clearTimeout(jsonTimeoutId);
          
          if (textError.name === 'AbortError') {
            throw new Error(`WebRTC answer parsing timed out for stream ${streamName}`);
          }
          throw textError;
        }
      } catch (fetchError) {
        // Clear the timeout if it hasn't been cleared yet
        clearTimeout(timeoutId);
        
        if (fetchError.name === 'AbortError') {
          throw new Error(`WebRTC offer request timed out for stream ${streamName}`);
        }
        throw fetchError;
      }
    } catch (error) {
      console.error(`Error sending offer for stream ${streamName}:`, error);
      throw error;
    }
  };
  
  // ICE candidates are handled internally by the browser for go2rtc
  
  /**
   * Handle WebRTC error
   * @param {string} streamName - Stream name
   * @param {string} message - Error message
   */
  const handleWebRTCError = (streamName, message) => {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;
    
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
      videoCell.appendChild(errorIndicator);
    }
    
    errorIndicator.innerHTML = `
      <div class="error-icon">!</div>
      <p>${message || 'WebRTC connection failed'}</p>
      <button class="retry-button mt-4 px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Retry</button>
    `;
    
    // Add retry button handler
    const retryButton = errorIndicator.querySelector('.retry-button');
    if (retryButton) {
      retryButton.addEventListener('click', () => {
        // Show loading indicator again
        if (loadingIndicator) {
          loadingIndicator.style.display = 'flex';
        }
        
        // Hide error indicator
        errorIndicator.style.display = 'none';
        
        // Cleanup existing connection
        cleanupWebRTCPlayer(streamName);
        
        // Fetch stream info again and reinitialize
        fetch(`/api/streams/${encodeURIComponent(streamName)}`)
          .then(response => response.json())
          .then(streamInfo => {
            // Reinitialize
            initializeWebRTCPlayer(streamInfo);
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
   * Cleanup WebRTC player
   * @param {string} streamName - Stream name
   */
  const cleanupWebRTCPlayer = (streamName) => {
    // Close and remove the RTCPeerConnection
    if (webrtcConnections.current[streamName]) {
      webrtcConnections.current[streamName].close();
      delete webrtcConnections.current[streamName];
    }
    
    // Reset video element
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    if (videoElement) {
      videoElement.srcObject = null;
    }
    
    // Clean up detection polling
    cleanupDetectionPolling(streamName, detectionIntervals.current);
  };
  
  /**
   * Stop all WebRTC streams
   */
  const stopAllWebRTCStreams = () => {
    // Close all RTCPeerConnections
    Object.keys(webrtcConnections.current).forEach(streamName => {
      cleanupWebRTCPlayer(streamName);
    });
  };
  
/**
 * Take snapshot of a stream
 * @param {string} streamId - Stream ID
 */
const takeSnapshot = (streamId) => {
  // Find the stream by ID or name
  const streamElement = document.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
  if (!streamElement) {
    console.error('Stream element not found for ID:', streamId);
    return;
  }

  // Get the stream name from the data attribute
  const streamName = streamElement.getAttribute('data-name');
  if (!streamName) {
    console.error('Stream name not found for ID:', streamId);
    return;
  }

  // Find the video element
  const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
  const videoElement = document.getElementById(videoElementId);
  if (!videoElement) {
    console.error('Video element not found for stream:', streamName);
    return;
  }

  // Create a canvas element to capture the frame
  const canvas = document.createElement('canvas');
  canvas.width = videoElement.videoWidth;
  canvas.height = videoElement.videoHeight;

  // Check if we have valid dimensions
  if (canvas.width === 0 || canvas.height === 0) {
    console.error('Invalid video dimensions:', canvas.width, canvas.height);
    showStatusMessage('Cannot take snapshot: Video not loaded or has invalid dimensions');
    return;
  }

  // Draw the current frame to the canvas
  const ctx = canvas.getContext('2d');
  ctx.drawImage(videoElement, 0, 0, canvas.width, canvas.height);

  try {
    // Save the canvas to global scope for direct access in the overlay
    window.__snapshotCanvas = canvas;
    
    // Generate a filename
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const fileName = `snapshot-${streamName.replace(/\s+/g, '-')}-${timestamp}.jpg`;
    window.__snapshotFileName = fileName;
    
    // Show the standard preview
    showSnapshotPreview(canvas.toDataURL('image/jpeg', 0.95), `Snapshot: ${streamName}`);
    
    // Show success message
    showStatusMessage('Snapshot taken successfully');
  } catch (error) {
    console.error('Error creating snapshot:', error);
    showStatusMessage('Failed to create snapshot: ' + error.message);
  }
};
  
  /**
   * Toggle fullscreen mode for a specific stream
   * @param {string} streamName - Stream name
   */
  const toggleStreamFullscreen = (streamName) => {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;
    
    if (!videoCell) {
      console.error('Stream not found:', streamName);
      return;
    }
    
    if (!document.fullscreenElement) {
      videoCell.requestFullscreen().catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
        showStatusMessage(`Could not enable fullscreen mode: ${err.message}`);
      });
    } else {
      document.exitFullscreen();
    }
  };
  
  return html`
    <section id="live-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center space-x-2">
          <h2 class="text-xl font-bold mr-4">Live View</h2>
          <div class="flex space-x-2">
            <button 
              id="hls-toggle-btn" 
              class="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${() => window.location.href = '/hls.html'}
            >
              HLS View
            </button>
            <button 
              id="fullscreen-btn" 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${() => toggleFullscreen(isFullscreen, setIsFullscreen)}
            >
              Fullscreen
            </button>
          </div>
        </div>
        <div class="controls flex items-center space-x-2">
          <select 
            id="layout-selector" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
            value=${layout}
            onChange=${(e) => {
              setLayout(e.target.value);
              setCurrentPage(0); // Reset to first page when layout changes
            }}
          >
            <option value="1">Single View</option>
            <option value="2">2x1 Grid</option>
            <option value="4" selected>2x2 Grid</option>
            <option value="6">2x3 Grid</option>
            <option value="9">3x3 Grid</option>
            <option value="16">4x4 Grid</option>
          </select>
          
          ${layout === '1' && html`
            <select 
              id="stream-selector" 
              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value=${selectedStream}
              onChange=${(e) => setSelectedStream(e.target.value)}
            >
              ${streams.map(stream => html`
                <option key=${stream.name} value=${stream.name}>${stream.name}</option>
              `)}
            </select>
          `}
        </div>
      </div>
      
      <div class="flex flex-col space-y-4">
        <div 
          id="video-grid" 
          class=${`video-container layout-${layout}`}
          ref=${videoGridRef}
        >
          ${isLoading ? html`
            <div class="flex justify-center items-center col-span-full row-span-full h-64 w-full">
              <div class="flex flex-col items-center justify-center py-8">
                <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 w-16 h-16"></div>
                <p class="mt-4 text-gray-700 dark:text-gray-300">Loading streams...</p>
              </div>
            </div>
          ` : streams.length === 0 ? html`
            <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
              <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
              <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
            </div>
          ` : null}
          <!-- Video cells will be dynamically added by the updateVideoGrid function -->
        </div>
        
        ${layout !== '1' && streams.length > getMaxStreamsForLayout() ? html`
          <div class="pagination-controls flex justify-center items-center space-x-4 mt-4">
            <button 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${() => setCurrentPage(Math.max(0, currentPage - 1))}
              disabled=${currentPage === 0}
            >
              Previous
            </button>
            <span class="text-gray-700 dark:text-gray-300">
              Page ${currentPage + 1} of ${Math.ceil(streams.length / getMaxStreamsForLayout())}
            </span>
            <button 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick=${() => setCurrentPage(Math.min(Math.ceil(streams.length / getMaxStreamsForLayout()) - 1, currentPage + 1))}
              disabled=${currentPage >= Math.ceil(streams.length / getMaxStreamsForLayout()) - 1}
            >
              Next
            </button>
          </div>
        ` : null}
      </div>
    </section>
  `;
}

/**
 * Load WebRTCView component
 */
export function loadWebRTCView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the WebRTCView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${WebRTCView} />`, mainContent);
  });
}

// The component is initialized by preact-app.js when needed
