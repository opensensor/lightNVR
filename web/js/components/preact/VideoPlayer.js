/**
 * Video player functionality for LiveView
 */

import { showStatusMessage } from './UI.js';
import { startDetectionPolling, cleanupDetectionPolling } from './DetectionOverlay.js';
import Hls from 'hls.js';

// Make Hls available globally for backward compatibility
window.Hls = Hls;

/**
 * Initialize video player for a stream
 * @param {Object} stream - Stream object
 * @param {Object} videoPlayers - Reference to store video player instances
 * @param {Object} detectionIntervals - Reference to store detection intervals
 */
export function initializeVideoPlayer(stream, videoPlayers, detectionIntervals) {
  const videoElementId = `video-${stream.name.replace(/\s+/g, '-')}`;
  const videoElement = document.getElementById(videoElementId);
  const videoCell = videoElement ? videoElement.closest('.video-cell') : null;
  
  if (!videoElement || !videoCell) return;
  
  // Ensure proper stacking context for the video cell
  videoCell.style.position = 'relative';
  
  // Make sure video doesn't block clicks to controls
  videoElement.style.pointerEvents = 'none';
  
  // Add stream name overlay to the upper left corner
  let streamNameOverlay = videoCell.querySelector('.stream-name-overlay');
  if (!streamNameOverlay) {
    streamNameOverlay = document.createElement('div');
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
    videoCell.appendChild(streamNameOverlay);
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
    canvasOverlay.style.zIndex = '5'; // Above video but below controls
    videoCell.appendChild(canvasOverlay);
  }
  
  // Ensure all controls are visible and clickable
  ensureControlsVisibility(videoCell);
  
  // Start detection polling if detection is enabled for this stream
  console.log(`Stream ${stream.name} detection settings:`, {
    detection_based_recording: stream.detection_based_recording,
    detection_model: stream.detection_model,
    detection_threshold: stream.detection_threshold
  });
  
  if (stream.detection_based_recording && stream.detection_model) {
    console.log(`Starting detection polling for stream ${stream.name}`);
    startDetectionPolling(stream.name, canvasOverlay, videoElement, detectionIntervals);
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
    
    // Get auth from localStorage
    const auth = localStorage.getItem('auth');
    
    // Check if the stream is already running or needs to be started
    console.log(`Initializing video player for stream ${stream.name}`);
    
    // Show loading indicator with starting message
    if (loadingIndicator) {
      loadingIndicator.style.display = 'flex';
      const messageSpan = loadingIndicator.querySelector('span');
      if (messageSpan) {
        messageSpan.textContent = 'Starting stream...';
      }
    }
  
  // Check if HLS is supported natively
  if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
    // Native HLS support (Safari)
    console.log(`Using native HLS support for stream ${stream.name}`);
    
    // Set source directly
    videoElement.src = hlsStreamUrl;
    
    // Hide loading indicator when metadata is loaded
    videoElement.addEventListener('loadedmetadata', function() {
      console.log(`Metadata loaded for stream ${stream.name}`);
      if (loadingIndicator) {
        loadingIndicator.style.display = 'none';
      }
    });
    
    // Handle errors
    videoElement.addEventListener('error', (e) => {
      console.error(`Video error for stream ${stream.name}:`, videoElement.error);
      handleVideoError(stream.name, videoElement.error ? videoElement.error.message : 'Unknown error');
    });
    
    // Try to play automatically
    videoElement.play().catch(error => {
      console.warn('Auto-play prevented:', error);
      // Add play button overlay for user interaction
      addPlayButtonOverlay(videoCell, videoElement);
    });
  }
    // Use HLS.js for browsers that don't support HLS natively
    else if (window.Hls && window.Hls.isSupported()) {
        // Get auth from localStorage
        const auth = localStorage.getItem('auth');
        
        // Check if this is a mobile device
        const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);
        
        // Configure HLS.js with settings optimized for the device type
        const hls = new window.Hls({
            // Increase buffer settings for mobile devices to improve stability
            maxBufferLength: isMobile ? 30 : 20,
            maxMaxBufferLength: isMobile ? 60 : 30,
            // Increase sync settings for mobile to handle network fluctuations
            liveSyncDurationCount: isMobile ? 4 : 3,
            liveMaxLatencyDurationCount: isMobile ? 10 : 6,
            liveDurationInfinity: false,
            // Disable low latency mode as it can cause issues on mobile devices
            lowLatencyMode: false,
            // Enable worker for better performance
            enableWorker: true,
            // Increase timeouts for mobile devices to handle slower networks
            fragLoadingTimeOut: isMobile ? 60000 : 30000,
            manifestLoadingTimeOut: isMobile ? 60000 : 30000,
            levelLoadingTimeOut: isMobile ? 60000 : 30000,
            // Increase back buffer length for mobile
            backBufferLength: isMobile ? 60 : 30,
            // Start with lower quality on mobile for faster initial load
            startLevel: isMobile ? 0 : -1,
            // More conservative ABR settings for mobile
            abrEwmaDefaultEstimate: isMobile ? 1000000 : 500000,
            abrBandWidthFactor: isMobile ? 0.5 : 0.7,
            abrBandWidthUpFactor: isMobile ? 0.3 : 0.5,
            // Add custom headers to all HLS requests
            xhrSetup: function(xhr, url) {
                // Add Authorization header if we have auth in localStorage
                if (auth) {
                    xhr.setRequestHeader('Authorization', 'Basic ' + auth);
                }
                // Always include credentials (cookies)
                xhr.withCredentials = true;
            }
        });
    
    hls.loadSource(hlsStreamUrl);
    hls.attachMedia(videoElement);
    
            hls.on(window.Hls.Events.MANIFEST_PARSED, () => {
                console.log(`Manifest parsed for stream ${stream.name}`);
                
                // On mobile, we need to be more careful with autoplay
                const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);
                
                if (isMobile) {
                    console.log(`Mobile device detected for stream ${stream.name}, using muted autoplay`);
                    // For mobile, always mute the video first to allow autoplay
                    videoElement.muted = true;
                    
                    // Hide loading indicator after a short delay to ensure UI is ready
                    setTimeout(() => {
                        if (loadingIndicator) {
                            loadingIndicator.style.display = 'none';
                        }
                        
                        // Try to play with muted audio first (more likely to succeed on mobile)
                        videoElement.play().then(() => {
                            console.log(`Autoplay succeeded for stream ${stream.name} on mobile`);
                        }).catch(error => {
                            console.warn(`Autoplay failed for stream ${stream.name} on mobile:`, error);
                            // Show play button if autoplay fails even with muted audio
                            addPlayButtonOverlay(videoCell, videoElement);
                        });
                    }, 500);
                } else {
                    // For desktop, proceed as before
                    if (loadingIndicator) {
                        loadingIndicator.style.display = 'none';
                    }
                    
                    // Try to play automatically
                    videoElement.play().catch(error => {
                        console.warn('Auto-play prevented:', error);
                        // Add play button overlay for user interaction
                        addPlayButtonOverlay(videoCell, videoElement);
                    });
                }
            });
    
    hls.on(window.Hls.Events.ERROR, (event, data) => {
      console.warn('HLS error:', data);
      
      // Handle fatal errors
      if (data.fatal) {
        console.error('Fatal HLS error:', data);
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
                cleanupVideoPlayer(stream.name, videoPlayers, detectionIntervals);
                // Reinitialize with updated stream info
                initializeVideoPlayer(updatedStream, videoPlayers, detectionIntervals);
              })
              .catch(error => {
                console.error(`Error fetching stream info for retry: ${error}`);
                handleVideoError(stream.name, 'Failed to reconnect after enabling');
              });
          }, 2000);
        } else {
          // Use a standard retry strategy for all devices
          console.log(`Implementing standard retry for stream ${stream.name}`);
          
          // Show loading indicator with retry message
          if (loadingIndicator) {
            loadingIndicator.style.display = 'flex';
            const messageSpan = loadingIndicator.querySelector('span');
            if (messageSpan) {
              messageSpan.textContent = 'Reconnecting to stream...';
            }
          }
          
          // Try to recover with a new HLS instance after a delay
          setTimeout(() => {
            try {
              // Create a new timestamp to avoid caching issues
              const newTimestamp = Date.now();
              const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
              
              // Create a new HLS instance with the same settings as the main instance
              const newHls = new window.Hls({
                // Increase buffer settings for mobile devices to improve stability
                maxBufferLength: isMobile ? 30 : 20,
                maxMaxBufferLength: isMobile ? 60 : 30,
                // Increase sync settings for mobile to handle network fluctuations
                liveSyncDurationCount: isMobile ? 4 : 3,
                liveMaxLatencyDurationCount: isMobile ? 10 : 6,
                liveDurationInfinity: false,
                // Disable low latency mode as it can cause issues on mobile devices
                lowLatencyMode: false,
                // Enable worker for better performance
                enableWorker: true,
                // Increase timeouts for mobile devices to handle slower networks
                fragLoadingTimeOut: isMobile ? 60000 : 30000,
                manifestLoadingTimeOut: isMobile ? 60000 : 30000,
                levelLoadingTimeOut: isMobile ? 60000 : 30000,
                // Increase back buffer length for mobile
                backBufferLength: isMobile ? 60 : 30,
                // Start with lower quality on mobile for faster initial load
                startLevel: isMobile ? 0 : -1,
                // More conservative ABR settings for mobile
                abrEwmaDefaultEstimate: isMobile ? 1000000 : 500000,
                abrBandWidthFactor: isMobile ? 0.5 : 0.7,
                abrBandWidthUpFactor: isMobile ? 0.3 : 0.5,
                // Add custom headers to all HLS requests
                xhrSetup: function(xhr, url) {
                  // Add Authorization header if we have auth in localStorage
                  if (auth) {
                    xhr.setRequestHeader('Authorization', 'Basic ' + auth);
                  }
                  // Always include credentials (cookies)
                  xhr.withCredentials = true;
                }
              });
              
              // Load the new source
              newHls.loadSource(newUrl);
              newHls.attachMedia(videoElement);
              
              // Store the new HLS instance
              if (videoCell) {
                if (videoCell.hlsPlayer) {
                  videoCell.hlsPlayer.destroy();
                }
                videoCell.hlsPlayer = newHls;
              }
              
              // Store in ref for cleanup
              videoPlayers[stream.name] = { 
                hls: newHls, 
                refreshTimer: videoPlayers[stream.name] ? videoPlayers[stream.name].refreshTimer : null 
              };
              
              // Hide loading indicator when media is attached
              newHls.on(window.Hls.Events.MEDIA_ATTACHED, () => {
                console.log(`New HLS instance attached for stream ${stream.name}`);
                if (loadingIndicator) {
                  loadingIndicator.style.display = 'none';
                }
                
                // Show play button for all devices to ensure consistent behavior
                addPlayButtonOverlay(videoCell, videoElement);
              });
              
              // Handle errors in the new instance
              newHls.on(window.Hls.Events.ERROR, (event, newData) => {
                if (newData.fatal) {
                  console.error('Fatal error in recovery HLS instance:', newData);
                  newHls.destroy();
                  handleVideoError(stream.name, 'Failed to reconnect after multiple attempts');
                }
              });
            } catch (error) {
              console.error('Error during HLS recovery:', error);
              handleVideoError(stream.name, 'Failed to reconnect: ' + error.message);
            }
          }, 3000);
        }
      } else if (data.type === window.Hls.ErrorTypes.NETWORK_ERROR) {
        // For non-fatal network errors, try to recover
        console.warn('Network error, attempting to recover:', data);
        
        // Try to recover by seeking slightly
        if (videoElement.currentTime > 0) {
          try {
            // Seek to live edge
            hls.recoverMediaError();
            videoElement.currentTime = videoElement.duration - 1;
          } catch (e) {
            console.error('Error during recovery seek:', e);
          }
        }
        
        // For fragment load errors, try to switch to a lower quality
        if (data.details === window.Hls.ErrorDetails.FRAG_LOAD_ERROR ||
            data.details === window.Hls.ErrorDetails.FRAG_LOAD_TIMEOUT) {
          
          try {
            // Get current level
            const currentLevel = hls.currentLevel;
            
            // If not already at lowest level, switch to a lower one
            if (currentLevel > 0) {
              console.log(`Switching from level ${currentLevel} to level 0 due to fragment error`);
              hls.currentLevel = 0;
            }
          } catch (e) {
            console.error('Error during level switching:', e);
          }
        }
      } else if (data.type === window.Hls.ErrorTypes.MEDIA_ERROR) {
        // For media errors, try to recover
        console.warn('Media error, attempting to recover:', data);
        try {
          hls.recoverMediaError();
        } catch (e) {
          console.error('Error during media error recovery:', e);
        }
      }
    });
    
    // Set up a universal refresh interval
    const refreshInterval = 30000; // 30 seconds for all devices
    const refreshTimer = setInterval(() => {
      if (videoCell && videoCell.hlsPlayer) {
        console.log(`Refreshing HLS stream for ${stream.name}`);
        const newTimestamp = Date.now();
        const newUrl = `/hls/${encodeURIComponent(stream.name)}/index.m3u8?_t=${newTimestamp}`;
        
        // Check if the player is in a good state before refreshing
        if (!videoCell.hlsPlayer.autoLevelCapping) {
          videoCell.hlsPlayer.loadSource(newUrl);
        } else {
          console.log(`Skipping refresh for ${stream.name} as it appears to be in recovery mode`);
        }
      } else {
        // Clear interval if video cell or player no longer exists
        clearInterval(refreshTimer);
      }
    }, refreshInterval);
    
    // Store hls instance and timer for cleanup
    videoCell.hlsPlayer = hls;
    videoCell.refreshTimer = refreshTimer;
    
    // Store in ref for cleanup
    videoPlayers[stream.name] = { hls, refreshTimer };
  }
  // Fallback for unsupported browsers
  else {
    handleVideoError(stream.name, 'HLS not supported by your browser');
  }
}

/**
 * Ensure all controls in the video cell are visible and clickable
 * @param {HTMLElement} videoCell - The video cell element
 */
function ensureControlsVisibility(videoCell) {
  if (!videoCell) return;
  
  // Find all controls within this cell
  const controls = videoCell.querySelector('.stream-controls');
  if (controls) {
    controls.style.position = 'absolute';
    controls.style.zIndex = '30';
    controls.style.pointerEvents = 'auto';
    
    // Remove any duplicate fullscreen button if needed
    const fullscreenBtns = controls.querySelectorAll('.fullscreen-btn');
    if (fullscreenBtns.length > 1) {
      fullscreenBtns[1].remove();
    }
  }
  
  // Find and fix all buttons in the video cell
  const allButtons = videoCell.querySelectorAll('button');
  allButtons.forEach(button => {
    button.style.position = 'relative';
    button.style.zIndex = '30';
    button.style.pointerEvents = 'auto';
  });
  
  // Find and fix snapshot button specifically
  const snapshotBtn = videoCell.querySelector('.snapshot-btn');
  if (snapshotBtn) {
    snapshotBtn.style.position = 'relative';
    snapshotBtn.style.zIndex = '30';
    snapshotBtn.style.pointerEvents = 'auto';
  }
  
  // Make sure loading and error indicators are visible
  const indicators = videoCell.querySelectorAll('.loading-indicator, .error-indicator');
  indicators.forEach(indicator => {
    indicator.style.position = 'absolute';
    indicator.style.zIndex = '20';
    indicator.style.pointerEvents = 'auto';
  });
}

/**
 * Add play button overlay
 * @param {HTMLElement} videoCell - Video cell element
 * @param {HTMLVideoElement} videoElement - Video element
 */
export function addPlayButtonOverlay(videoCell, videoElement) {
  // Check if play overlay already exists
  if (videoCell.querySelector('.play-overlay')) {
    return;
  }
  
  // Create play button overlay
  const playOverlay = document.createElement('div');
  playOverlay.className = 'play-overlay';
  playOverlay.style.position = 'absolute';
  playOverlay.style.top = '0';
  playOverlay.style.left = '0';
  playOverlay.style.width = '100%';
  playOverlay.style.height = '100%';
  playOverlay.style.display = 'flex';
  playOverlay.style.justifyContent = 'center';
  playOverlay.style.alignItems = 'center';
  playOverlay.style.backgroundColor = 'rgba(0, 0, 0, 0.3)';
  playOverlay.style.cursor = 'pointer';
  playOverlay.style.zIndex = '25'; // Above video but below controls
  
  // Create play button
  const playButton = document.createElement('div');
  playButton.className = 'play-button';
  playButton.innerHTML = '<svg width="64" height="64" viewBox="0 0 24 24"><path fill="white" d="M8 5v14l11-7z"/></svg>';
  playOverlay.appendChild(playButton);
  
  // Add to video cell
  videoCell.appendChild(playOverlay);
  
  // Get loading indicator
  const loadingIndicator = videoCell.querySelector('.loading-indicator');
  
  // Add click handler
  playOverlay.onclick = function() {
    // Show loading indicator
    if (loadingIndicator) {
      loadingIndicator.style.display = 'flex';
    }
    
    // Disable overlay while attempting to play
    playOverlay.style.pointerEvents = 'none';
    playButton.style.transform = 'scale(0.8)';
    
    videoElement.play()
      .then(() => {
        playOverlay.remove();
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }
      })
      .catch(error => {
        console.error('Play failed:', error);
        
        // Re-enable the overlay if play fails
        playOverlay.style.pointerEvents = 'auto';
        playButton.style.transform = '';
        
        if (loadingIndicator) {
          loadingIndicator.style.display = 'none';
        }
        
        // Show error message
        showStatusMessage('Auto-play blocked by browser. Please try again or check your browser settings.');
        
        // On iOS, we need to mute the video to allow playback without user gesture
        if (/iPhone|iPad|iPod/i.test(navigator.userAgent)) {
          videoElement.muted = true;
          showStatusMessage('Video muted to allow playback on iOS. Tap again to play.');
        }
      });
  };
}

/**
 * Handle video error
 * @param {string} streamName - Name of the stream
 * @param {string} message - Error message
 */
export function handleVideoError(streamName, message) {
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
          cleanupVideoPlayer(streamName, videoPlayers, detectionIntervals);
          
          // Reinitialize
          initializeVideoPlayer(streamInfo, videoPlayers, detectionIntervals);
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
}

/**
 * Cleanup video player
 * @param {string} streamName - Name of the stream
 * @param {Object} videoPlayers - Reference to video player instances
 * @param {Object} detectionIntervals - Reference to detection intervals
 */
export function cleanupVideoPlayer(streamName, videoPlayers, detectionIntervals) {
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
  
  // Clean up detection polling
  cleanupDetectionPolling(streamName, detectionIntervals);
  
  // Clean up from refs
  if (videoPlayers[streamName]) {
    const { hls, refreshTimer } = videoPlayers[streamName];
    if (hls) {
      hls.destroy();
    }
    if (refreshTimer) {
      clearInterval(refreshTimer);
    }
    delete videoPlayers[streamName];
  }
}

/**
 * Stop all streams
 * @param {Array} streams - Array of stream objects
 * @param {Object} videoPlayers - Reference to video player instances
 * @param {Object} detectionIntervals - Reference to detection intervals
 */
export function stopAllStreams(streams, videoPlayers, detectionIntervals) {
  streams.forEach(stream => {
    cleanupVideoPlayer(stream.name, videoPlayers, detectionIntervals);
  });
}

/**
 * Update video grid based on layout and streams with staggered loading
 * @param {HTMLElement} videoGridRef - Reference to video grid element
 * @param {Array} streams - Array of stream objects
 * @param {string} layout - Layout type ('1', '4', '9', '16')
 * @param {string} selectedStream - Selected stream name for single view
 * @param {Object} videoPlayers - Reference to video player instances
 * @param {Object} detectionIntervals - Reference to detection intervals
 */
export function updateVideoGrid(
  videoGridRef, 
  streams, 
  layout, 
  selectedStream, 
  videoPlayers, 
  detectionIntervals
) {
  if (!videoGridRef) return;
  
  // Clear existing content except placeholder
  const placeholder = videoGridRef.querySelector('.placeholder');
  videoGridRef.innerHTML = '';
  
  // If placeholder exists and no streams, add it back
  if (placeholder && streams.length === 0) {
    videoGridRef.appendChild(placeholder);
    return;
  }
  
  // Filter streams based on layout and selected stream
  let streamsToShow = streams;
  if (layout === '1' && selectedStream) {
    streamsToShow = streams.filter(stream => stream.name === selectedStream);
  }
  
  // Create video cells for all streams first (for UI responsiveness)
  streamsToShow.forEach(stream => {
    createVideoCell(videoGridRef, stream);
  });
  
  // Then initialize players with staggered timing
  streamsToShow.forEach((stream, index) => {
    setTimeout(() => {
      const videoCell = videoGridRef.querySelector(`.video-cell[data-stream-name="${stream.name}"]`);
      if (videoCell) {
        const videoElement = videoCell.querySelector('.video-element');
        if (videoElement) {
          initializeVideoPlayer(stream, videoPlayers, detectionIntervals);
        }
      }
    }, index * 500); // 500ms delay between each stream initialization
  });
}

/**
 * Create video cell without initializing player
 * @param {HTMLElement} videoGridRef - Reference to video grid element
 * @param {Object} stream - Stream object
 */
function createVideoCell(videoGridRef, stream) {
  // Ensure we have an ID for the stream (use name as fallback if needed)
  const streamId = stream.id || stream.name;
  
  const videoCell = document.createElement('div');
  videoCell.className = 'video-cell';
  videoCell.dataset.streamName = stream.name;
  
  // Create video element with object-fit: cover to fill available space
  const videoElement = document.createElement('video');
  videoElement.id = `video-${stream.name.replace(/\s+/g, '-')}`;
  videoElement.className = 'video-element';
  videoElement.playsInline = true;
  videoElement.autoplay = true;
  videoElement.muted = true;
  videoElement.style.objectFit = 'cover'; // Ensure video fills the space
  
  // Add loading indicator
  const loadingIndicator = document.createElement('div');
  loadingIndicator.className = 'loading-indicator';
  loadingIndicator.innerHTML = '<div class="spinner"></div><span>Preparing stream...</span>';
  
  // Add error indicator (hidden by default)
  const errorIndicator = document.createElement('div');
  errorIndicator.className = 'error-indicator';
  errorIndicator.style.display = 'none';
  errorIndicator.innerHTML = '<p>Connection failed</p><button class="retry-button">Retry</button>';
  
  // Add stream name overlay - make it more compact
  const streamNameOverlay = document.createElement('div');
  streamNameOverlay.className = 'stream-name-overlay';
  streamNameOverlay.textContent = stream.name;
  streamNameOverlay.style.padding = '0.25rem 0.5rem';
  streamNameOverlay.style.fontSize = '0.9rem';
  streamNameOverlay.style.opacity = '0.8';
  
  // Add stream controls (snapshot and fullscreen buttons) - make more compact
  const streamControls = document.createElement('div');
  streamControls.className = 'stream-controls';
  streamControls.style.padding = '0.25rem';
  streamControls.innerHTML = `
    <button class="snapshot-btn" data-id="${streamId}" data-name="${stream.name}">
      <span>📷</span>
    </button>
    <button class="fullscreen-btn" data-id="${streamId}" data-name="${stream.name}">
      <span>⛶</span>
    </button>
  `;
  
  // Add canvas for detection overlay
  const canvasOverlay = document.createElement('canvas');
  canvasOverlay.id = `canvas-${stream.name.replace(/\s+/g, '-')}`;
  canvasOverlay.className = 'detection-overlay';
  
  // Assemble the video cell
  videoCell.appendChild(videoElement);
  videoCell.appendChild(loadingIndicator);
  videoCell.appendChild(errorIndicator);
  videoCell.appendChild(streamNameOverlay);
  videoCell.appendChild(streamControls);
  videoCell.appendChild(canvasOverlay);
  
  // Add to grid
  videoGridRef.appendChild(videoCell);
  
  // Add event listeners for buttons
  const snapshotBtn = videoCell.querySelector('.snapshot-btn');
  if (snapshotBtn) {
    snapshotBtn.addEventListener('click', () => {
      // Import from SnapshotManager.js
      const { takeSnapshot } = window.AugmentCode || {};
      if (typeof takeSnapshot === 'function') {
        takeSnapshot(streamId);
      } else {
        // Fallback to global function if module import not available
        if (typeof window.takeSnapshot === 'function') {
          window.takeSnapshot(streamId);
        }
      }
    });
  }
  
  const fullscreenBtn = videoCell.querySelector('.fullscreen-btn');
  if (fullscreenBtn) {
    fullscreenBtn.addEventListener('click', () => {
      // Import from FullscreenManager.js
      const { toggleStreamFullscreen } = window.AugmentCode || {};
      if (typeof toggleStreamFullscreen === 'function') {
        toggleStreamFullscreen(stream.name);
      } else {
        // Fallback to global function if module import not available
        if (typeof window.toggleStreamFullscreen === 'function') {
          window.toggleStreamFullscreen(stream.name);
        }
      }
    });
  }
}
