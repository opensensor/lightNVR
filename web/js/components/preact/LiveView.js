/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the live view page
 */

import { h } from '../../preact.min.js';
import { html } from '../../preact-app.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { showStatusMessage, showSnapshotPreview } from './UI.js';

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView() {
  const [streams, setStreams] = useState([]);
  const [layout, setLayout] = useState('4');
  const [selectedStream, setSelectedStream] = useState('');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const videoGridRef = useRef(null);
  const videoPlayers = useRef({});
  const detectionIntervals = useRef({});
  
  // Load streams on mount
  useEffect(() => {
    loadStreams();
    
    // Set up Escape key to exit fullscreen mode
    const handleEscape = (e) => {
      if (e.key === 'Escape') {
        console.log("Escape key pressed, current fullscreen state:", isFullscreen);
        // Check if we're in fullscreen mode by checking the DOM directly
        const livePage = document.getElementById('live-page');
        if (livePage && livePage.classList.contains('fullscreen-mode')) {
          console.log("Detected fullscreen mode via DOM, exiting fullscreen");
          // Force exit fullscreen mode
          livePage.classList.remove('fullscreen-mode');
          document.body.style.overflow = '';
          
          // Remove exit button
          const exitBtn = document.querySelector('.fullscreen-exit');
          if (exitBtn) {
            exitBtn.remove();
          }
          
          // Show the fullscreen button again
          const fullscreenBtn = document.getElementById('fullscreen-btn');
          if (fullscreenBtn) {
            fullscreenBtn.style.display = '';
          }
          
          // Update state
          setIsFullscreen(false);
        }
      }
    };
    
    document.addEventListener('keydown', handleEscape);
    
    // Add event listener to stop streams when leaving the page
    const handleBeforeUnload = () => {
      stopAllStreams();
    };
    
    window.addEventListener('beforeunload', handleBeforeUnload);
    
    // Cleanup
    return () => {
      document.removeEventListener('keydown', handleEscape);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      stopAllStreams();
    };
  }, []);
  
  // Update video grid when layout or streams change
  useEffect(() => {
    updateVideoGrid();
  }, [layout, selectedStream, streams]);
  
  // Load streams from API
  async function loadStreams() {
    try {
      // Show loading state
      if (videoGridRef.current) {
        showLoading(videoGridRef.current);
      }
      
      // Fetch streams from API
      const response = await fetch('/api/streams');
      if (!response.ok) {
        throw new Error('Failed to load streams');
      }
      
      const data = await response.json();
      
      // For live view, we need to fetch full details for each stream
      // to get detection settings
      const streamPromises = (data || []).map(stream => {
        return fetch(`/api/streams/${encodeURIComponent(stream.id || stream.name)}`)
          .then(response => {
            if (!response.ok) {
              throw new Error(`Failed to load details for stream ${stream.name}`);
            }
            return response.json();
          })
          .catch(error => {
            console.error(`Error loading details for stream ${stream.name}:`, error);
            // Return the basic stream info if we can't get details
            return stream;
          });
      });
      
      const detailedStreams = await Promise.all(streamPromises);
      console.log('Loaded detailed streams for live view:', detailedStreams);
      
      // Store streams in state
      setStreams(detailedStreams || []);
      
      // If we have streams, set the first one as selected for single view
      if (detailedStreams.length > 0 && !selectedStream) {
        setSelectedStream(detailedStreams[0].name);
      }
    } catch (error) {
      console.error('Error loading streams for live view:', error);
      showStatusMessage('Error loading streams: ' + error.message);
      
      // Show error placeholder
      if (videoGridRef.current) {
        videoGridRef.current.innerHTML = `
          <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
            <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">Error loading streams</p>
            <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
          </div>
        `;
      }
    } finally {
      // Hide loading state
      if (videoGridRef.current) {
        hideLoading(videoGridRef.current);
      }
    }
  };
  
  // Update video grid
  function updateVideoGrid() {
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
    }
    
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
          <span>Connecting...</span>
        </div>
      `;
      
      videoGridRef.current.appendChild(videoCell);
      
      // Initialize video player
      initializeVideoPlayer(stream);
      
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
  
  // Initialize video player
  function initializeVideoPlayer(stream) {
    const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;
    
    if (!videoElement || !videoCell) return;
    
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
    
    // Start detection polling if detection is enabled for this stream
    console.log(`Stream ${stream.name} detection settings:`, {
      detection_based_recording: stream.detection_based_recording,
      detection_model: stream.detection_model,
      detection_threshold: stream.detection_threshold
    });
    
    if (stream.detection_based_recording && stream.detection_model) {
      console.log(`Starting detection polling for stream ${stream.name}`);
      startDetectionPolling(stream.name, canvasOverlay, videoElement);
    } else {
      console.log(`Detection not enabled for stream ${stream.name}`);
    }
    
    // Show loading state
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
      loadingIndicator.style.display = 'flex';
    }
    
    // Build the HLS stream URL with cache-busting timestamp to prevent stale data
    const timestamp = Date.now();
    const hlsStreamUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${timestamp}`;
    
    // Check if HLS is supported natively
    if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
      // Native HLS support (Safari)
      videoElement.src = hlsStreamUrl;
      videoElement.addEventListener('loadedmetadata', function() {
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }
      });
      
      videoElement.addEventListener('error', () => {
        handleVideoError(stream.name);
      });
    }
    // Use HLS.js for browsers that don't support HLS natively
    else if (window.Hls && window.Hls.isSupported()) {
      const hls = new window.Hls({
        maxBufferLength: 30,
        maxMaxBufferLength: 60,
        liveSyncDurationCount: 4,
        liveMaxLatencyDurationCount: 10,
        liveDurationInfinity: false,
        lowLatencyMode: false,
        enableWorker: true,
        fragLoadingTimeOut: 30000,
        manifestLoadingTimeOut: 20000,
        levelLoadingTimeOut: 20000,
        backBufferLength: 60,
        startLevel: -1,
        abrEwmaDefaultEstimate: 500000,
        abrBandWidthFactor: 0.7,
        abrBandWidthUpFactor: 0.5
      });
      
      hls.loadSource(hlsStreamUrl);
      hls.attachMedia(videoElement);
      
      hls.on(window.Hls.Events.MANIFEST_PARSED, () => {
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }
        videoElement.play().catch(error => {
          console.warn('Auto-play prevented:', error);
          // Add play button overlay for user interaction
          addPlayButtonOverlay(videoCell, videoElement);
        });
      });
      
      hls.on(window.Hls.Events.ERROR, (event, data) => {
        if (data.fatal) {
          console.error('HLS error:', data);
          hls.destroy();
          
          // Check if the stream was recently enabled
          const videoCell = videoElement.closest('.video-cell');
          const loadingIndicator = videoCell.querySelector('.loading-indicator');
          
          // If the stream was recently enabled (indicated by the loading message),
          // automatically retry after a short delay
          if (loadingIndicator && 
              loadingIndicator.querySelector('span').textContent === 'Starting stream...') {
            
            console.log(`Stream ${stream.name} failed to load after enabling, retrying in 2 seconds...`);
            
            // Show retry message
            loadingIndicator.querySelector('span').textContent = 'Retrying connection...';
            
            // Retry after a delay
            setTimeout(() => {
              console.log(`Retrying stream ${stream.name} after failure`);
              // Fetch updated stream info and reinitialize
              fetch(`/api/streams/${encodeURIComponent(stream.name)}`)
                .then(response => response.json())
                .then(updatedStream => {
                  // Cleanup existing player
                  cleanupVideoPlayer(stream.name);
                  // Reinitialize with updated stream info
                  initializeVideoPlayer(updatedStream);
                })
                .catch(error => {
                  console.error(`Error fetching stream info for retry: ${error}`);
                  handleVideoError(stream.name, 'Failed to reconnect after enabling');
                });
            }, 2000);
          } else {
            // Regular error handling for non-startup errors
            handleVideoError(stream.name);
          }
        }
      });
      
      // Set up less frequent refresh to reduce load on low-power devices
      const refreshInterval = 60000; // 60 seconds
      const refreshTimer = setInterval(() => {
        if (videoCell && videoCell.hlsPlayer) {
          console.log(`Refreshing HLS stream for ${stream.name}`);
          const newTimestamp = Date.now();
          const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
          videoCell.hlsPlayer.loadSource(newUrl);
        } else {
          // Clear interval if video cell or player no longer exists
          clearInterval(refreshTimer);
        }
      }, refreshInterval);
      
      // Store hls instance and timer for cleanup
      videoCell.hlsPlayer = hls;
      videoCell.refreshTimer = refreshTimer;
      
      // Store in ref for cleanup
      videoPlayers.current[stream.name] = { hls, refreshTimer };
    }
    // Fallback for unsupported browsers
    else {
      handleVideoError(stream.name, 'HLS not supported by your browser');
    }
  };
  
  // Add play button overlay
  function addPlayButtonOverlay(videoCell, videoElement) {
    const playOverlay = document.createElement('div');
    playOverlay.className = 'play-overlay';
    
    const playButton = document.createElement('div');
    playButton.className = 'play-button';
    playButton.innerHTML = `
      <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
      </svg>
    `;
    
    playOverlay.appendChild(playButton);
    videoCell.appendChild(playOverlay);
    
    playOverlay.addEventListener('click', function() {
      videoElement.play()
        .then(() => {
          playOverlay.remove();
        })
        .catch(error => {
          console.error('Play failed:', error);
          showStatusMessage('Auto-play blocked by browser. Please adjust your browser settings.');
        });
    });
  };
  
  // Handle video error
  function handleVideoError(streamName, message) {
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
      <p>${message || 'Stream connection failed'}</p>
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
        
        // Fetch stream info again and reinitialize
        fetch(`/api/streams/${encodeURIComponent(streamName)}`)
          .then(response => response.json())
          .then(streamInfo => {
            // Cleanup existing player if any
            cleanupVideoPlayer(streamName);
            
            // Reinitialize
            initializeVideoPlayer(streamInfo);
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
  
  // Cleanup video player
  function cleanupVideoPlayer(streamName) {
    const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
    const videoElement = document.getElementById(videoElementId);
    const videoCell = videoElement ? videoElement.closest('.video-cell') : null;
    
    if (!videoCell) return;
    
    // Destroy HLS instance and clear refresh timer if they exist
    if (videoCell.hlsPlayer) {
      videoCell.hlsPlayer.destroy();
      delete videoCell.hlsPlayer;
    }
    
    if (videoCell.refreshTimer) {
      clearInterval(videoCell.refreshTimer);
      delete videoCell.refreshTimer;
    }
    
    // Reset video element
    if (videoElement) {
      videoElement.pause();
      videoElement.removeAttribute('src');
      videoElement.load();
    }
    
    // Reset loading indicator
    const loadingIndicator = videoCell.querySelector('.loading-indicator');
    if (loadingIndicator) {
      loadingIndicator.style.display = 'none';
    }
    
    // Remove error indicator if any
    const errorIndicator = videoCell.querySelector('.error-indicator');
    if (errorIndicator) {
      errorIndicator.remove();
    }
    
    // Remove play overlay if any
    const playOverlay = videoCell.querySelector('.play-overlay');
    if (playOverlay) {
      playOverlay.remove();
    }
    
    // Clean up detection polling if active
    const canvasId = `canvas-${streamName.replace(/\s+/g, '-')}`;
    const canvasOverlay = document.getElementById(canvasId);
    if (canvasOverlay && canvasOverlay.detectionInterval) {
      clearInterval(canvasOverlay.detectionInterval);
      delete canvasOverlay.detectionInterval;
    }
    
    // Clean up from refs
    if (videoPlayers.current[streamName]) {
      const { hls, refreshTimer } = videoPlayers.current[streamName];
      if (hls) {
        hls.destroy();
      }
      if (refreshTimer) {
        clearInterval(refreshTimer);
      }
      delete videoPlayers.current[streamName];
    }
    
    if (detectionIntervals.current[streamName]) {
      clearInterval(detectionIntervals.current[streamName]);
      delete detectionIntervals.current[streamName];
    }
  };
  
  // Start detection polling
  function startDetectionPolling(streamName, canvasOverlay, videoElement) {
    // Clear existing interval if any
    if (detectionIntervals.current[streamName]) {
      clearInterval(detectionIntervals.current[streamName]);
    }
    
    // Function to draw bounding boxes
    const drawDetectionBoxes = (detections) => {
      const canvas = canvasOverlay;
      const ctx = canvas.getContext('2d');
      
      // Set canvas dimensions to match the displayed video element
      canvas.width = videoElement.clientWidth;
      canvas.height = videoElement.clientHeight;
      
      // Clear previous drawings
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      // No detections, just return
      if (!detections || detections.length === 0) {
        return;
      }
      
      // Get the actual video dimensions
      const videoWidth = videoElement.videoWidth;
      const videoHeight = videoElement.videoHeight;
      
      // If video dimensions aren't available yet, skip drawing
      if (!videoWidth || !videoHeight) {
        console.log('Video dimensions not available yet, skipping detection drawing');
        return;
      }
      
      // Calculate the scaling and positioning to maintain aspect ratio
      const videoAspect = videoWidth / videoHeight;
      const canvasAspect = canvas.width / canvas.height;
      
      let drawWidth, drawHeight, offsetX = 0, offsetY = 0;
      
      if (videoAspect > canvasAspect) {
        // Video is wider than canvas (letterboxing - black bars on top and bottom)
        drawWidth = canvas.width;
        drawHeight = canvas.width / videoAspect;
        offsetY = (canvas.height - drawHeight) / 2;
      } else {
        // Video is taller than canvas (pillarboxing - black bars on sides)
        drawHeight = canvas.height;
        drawWidth = canvas.height * videoAspect;
        offsetX = (canvas.width - drawWidth) / 2;
      }
      
      // Draw each detection
      detections.forEach(detection => {
        // Calculate pixel coordinates based on normalized values (0-1)
        // and adjust for the actual display area
        const x = (detection.x * drawWidth) + offsetX;
        const y = (detection.y * drawHeight) + offsetY;
        const width = detection.width * drawWidth;
        const height = detection.height * drawHeight;
        
        // Draw bounding box
        ctx.strokeStyle = 'rgba(255, 0, 0, 0.8)';
        ctx.lineWidth = 3;
        ctx.strokeRect(x, y, width, height);
        
        // Draw label background
        const label = `${detection.label} (${Math.round(detection.confidence * 100)}%)`;
        ctx.font = '14px Arial';
        const textWidth = ctx.measureText(label).width;
        ctx.fillStyle = 'rgba(255, 0, 0, 0.7)';
        ctx.fillRect(x, y - 20, textWidth + 10, 20);
        
        // Draw label text
        ctx.fillStyle = 'white';
        ctx.fillText(label, x + 5, y - 5);
      });
    };
    
    // Use a more conservative polling interval (1000ms instead of 500ms)
    // and implement exponential backoff on errors
    let errorCount = 0;
    let currentInterval = 1000; // Start with 1 second
    
    // Poll for detection results
    const intervalId = setInterval(() => {
      if (!videoElement.videoWidth) {
        // Video not loaded yet, skip this cycle
        return;
      }
      
      // Fetch detection results from API
      fetch(`/api/detection/results/${encodeURIComponent(streamName)}`)
        .then(response => {
          if (!response.ok) {
            throw new Error(`Failed to fetch detection results: ${response.status}`);
          }
          // Reset error count on success
          errorCount = 0;
          return response.json();
        })
        .then(data => {
          // Draw bounding boxes if we have detections
          if (data && data.detections) {
            drawDetectionBoxes(data.detections);
          }
        })
        .catch(error => {
          console.error(`Error fetching detection results for ${streamName}:`, error);
          // Clear canvas on error
          const ctx = canvasOverlay.getContext('2d');
          ctx.clearRect(0, 0, canvasOverlay.width, canvasOverlay.height);
          
          // Implement backoff strategy on errors
          errorCount++;
          if (errorCount > 3) {
            // After 3 consecutive errors, slow down polling to avoid overwhelming the server
            clearInterval(intervalId);
            currentInterval = Math.min(5000, currentInterval * 2); // Max 5 seconds
            console.log(`Reducing detection polling frequency to ${currentInterval}ms due to errors`);
            
            detectionIntervals.current[streamName] = setInterval(arguments.callee, currentInterval);
          }
        });
    }, currentInterval);
    
    // Store interval ID for cleanup
    detectionIntervals.current[streamName] = intervalId;
    canvasOverlay.detectionInterval = intervalId;
  };
  
  // Take snapshot
  function takeSnapshot(streamId) {
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
      // Convert the canvas to a data URL (JPEG image)
      const imageUrl = canvas.toDataURL('image/jpeg', 0.95);
      
      console.log('Taking snapshot, imageUrl created successfully');
      
      // Show snapshot preview
      showSnapshotPreview(imageUrl, `Snapshot: ${streamName}`);
      
      // Show success message
      showStatusMessage('Snapshot taken successfully');
    } catch (error) {
      console.error('Error creating snapshot:', error);
      showStatusMessage('Failed to create snapshot: ' + error.message);
    }
  };
  
  // Direct function to exit fullscreen mode
  function exitFullscreenMode(e) {
    // If this was called from an event, stop propagation
    if (e) {
      e.stopPropagation();
      e.preventDefault();
    }
    
    console.log("DIRECT EXIT FUNCTION CALLED");
    
    const livePage = document.getElementById('live-page');
    if (!livePage) {
      console.error("Live page element not found");
      return;
    }
    
    // Exit fullscreen
    livePage.classList.remove('fullscreen-mode');
    document.body.style.overflow = '';
    
    // Remove exit button
    const exitBtn = document.querySelector('.fullscreen-exit');
    if (exitBtn) {
      exitBtn.remove();
    } else {
      console.warn("Exit button not found when trying to remove it");
    }
    
    // Show the fullscreen button again
    const fullscreenBtn = document.getElementById('fullscreen-btn');
    if (fullscreenBtn) {
      fullscreenBtn.style.display = '';
    } else {
      console.warn("Fullscreen button not found when trying to show it again");
    }
    
    // Update state
    setIsFullscreen(false);
    
    console.log("Fullscreen mode exited, state set to false");
  }
  
  // Toggle fullscreen
  function toggleFullscreen() {
    console.log("toggleFullscreen called, current state:", isFullscreen);
    
    const livePage = document.getElementById('live-page');
    
    if (!livePage) {
      console.error("Live page element not found");
      return;
    }
    
    // Check the actual DOM state rather than relying on the React state
    const isCurrentlyInFullscreen = livePage.classList.contains('fullscreen-mode');
    console.log("DOM check for fullscreen mode:", isCurrentlyInFullscreen);
    
    if (!isCurrentlyInFullscreen) {
      console.log("Entering fullscreen mode");
      // Enter fullscreen
      livePage.classList.add('fullscreen-mode');
      document.body.style.overflow = 'hidden';
      
      // Add exit button - IMPORTANT: Use a standalone function for the click handler
      const exitBtn = document.createElement('button');
      exitBtn.className = 'fullscreen-exit fixed top-4 right-4 w-10 h-10 bg-black/70 text-white rounded-full flex justify-center items-center cursor-pointer z-50 transition-all duration-200 hover:bg-black/85 hover:scale-110 shadow-md';
      exitBtn.innerHTML = '✕';
      
      // Create a standalone function for the click handler
      const exitClickHandler = function(e) {
        console.log("Exit button clicked - STANDALONE HANDLER");
        exitFullscreenMode(e);
      };
      
      // Add the event listener with the standalone function
      exitBtn.addEventListener('click', exitClickHandler);
      
      livePage.appendChild(exitBtn);
      
      // Hide the fullscreen button in the controls when in fullscreen mode
      const fullscreenBtn = document.getElementById('fullscreen-btn');
      if (fullscreenBtn) {
        fullscreenBtn.style.display = 'none';
      }
      
      // Update state
      setIsFullscreen(true);
      console.log("Fullscreen mode entered, state set to true");
    } else {
      exitFullscreenMode();
    }
  };
  
  // Toggle stream fullscreen
  function toggleStreamFullscreen(streamName) {
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
  
  // Stop all streams
  function stopAllStreams() {
    streams.forEach(stream => {
      cleanupVideoPlayer(stream.name);
    });
  };
  
  // Show loading state
  function showLoading(element) {
    if (!element) return;
    console.log('Showing loading for element:', element);
    
    // Add loading class to element
    element.classList.add('loading');
    
    // Optionally add a loading spinner
    const spinner = document.createElement('div');
    spinner.className = 'spinner';
    element.appendChild(spinner);
  };
  
  // Hide loading state
  function hideLoading(element) {
    if (!element) return;
    console.log('Hiding loading for element:', element);
    
    // Remove loading class from element
    element.classList.remove('loading');
    
    // Remove loading spinner if exists
    const spinner = element.querySelector('.spinner');
    if (spinner) {
      spinner.remove();
    }
  };
  
  return html`
    <section id="live-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center space-x-2">
          <h2 class="text-xl font-bold mr-4">Live View</h2>
          <button 
            id="fullscreen-btn" 
            class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
            onClick=${toggleFullscreen}
          >
            Fullscreen
          </button>
        </div>
        <div class="controls flex items-center space-x-2">
          <select 
            id="layout-selector" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
            value=${layout}
            onChange=${(e) => setLayout(e.target.value)}
          >
            <option value="1">Single View</option>
            <option value="4" selected>2x2 Grid</option>
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
      
      <div 
        id="video-grid" 
        class=${`video-container layout-${layout}`}
        ref=${videoGridRef}
      >
        ${streams.length === 0 && html`
          <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
            <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
            <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
          </div>
        `}
        <!-- Video cells will be dynamically added by the updateVideoGrid function -->
      </div>
    </section>
  `;
}

/**
 * Load LiveView component
 */
export function loadLiveView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the LiveView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${LiveView} />`, mainContent);
  });
}
