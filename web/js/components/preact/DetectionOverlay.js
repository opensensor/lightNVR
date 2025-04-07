/**
 * Detection overlay functionality for LiveView
 */

/**
 * Start detection polling for a stream
 * @param {string} streamName - Name of the stream
 * @param {HTMLCanvasElement} canvasOverlay - Canvas element for drawing detection boxes
 * @param {HTMLVideoElement} videoElement - Video element
 * @param {Object} detectionIntervals - Reference to store interval IDs
 * @returns {number} Interval ID
 */
export function startDetectionPolling(streamName, canvasOverlay, videoElement, detectionIntervals) {
  // Clear existing interval if any
  if (detectionIntervals[streamName]) {
    clearInterval(detectionIntervals[streamName]);
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
  
  // Create a polling function that we can reference for recreating intervals
  const pollDetections = () => {
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
          clearInterval(detectionIntervals[streamName]);
          currentInterval = Math.min(5000, currentInterval * 2); // Max 5 seconds
          console.log(`Reducing detection polling frequency to ${currentInterval}ms due to errors`);
          
          // Create a new interval with the updated taming
          detectionIntervals[streamName] = setInterval(pollDetections, currentInterval);
        }
      });
  };
  
  // Start the polling interval
  const intervalId = setInterval(pollDetections, currentInterval);
  
  // Store interval ID for cleanup
  detectionIntervals[streamName] = intervalId;
  canvasOverlay.detectionInterval = intervalId;
  
  return intervalId;
}

/**
 * Clean up detection polling
 * @param {string} streamName - Name of the stream
 * @param {Object} detectionIntervals - Reference to stored interval IDs
 */
export function cleanupDetectionPolling(streamName, detectionIntervals) {
  const canvasId = `canvas-${streamName.replace(/\s+/g, '-')}`;
  const canvasOverlay = document.getElementById(canvasId);
  
  if (canvasOverlay && canvasOverlay.detectionInterval) {
    clearInterval(canvasOverlay.detectionInterval);
    delete canvasOverlay.detectionInterval;
  }
  
  if (detectionIntervals[streamName]) {
    clearInterval(detectionIntervals[streamName]);
    delete detectionIntervals[streamName];
  }
}

/**
 * Draw detections on canvas overlay
 * @param {HTMLCanvasElement} canvas - Canvas element
 * @param {HTMLVideoElement} videoElement - Video element
 * @param {Array} detections - Array of detection objects
 */
export function drawDetections(canvas, videoElement, detections) {
  if (!canvas || !videoElement || !detections || !detections.length) return;
  
  const ctx = canvas.getContext('2d');
  if (!ctx) return;
  
  // Clear previous drawings
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  
  // Get video dimensions
  const videoWidth = videoElement.videoWidth;
  const videoHeight = videoElement.videoHeight;
  
  if (videoWidth === 0 || videoHeight === 0) return;
  
  // Ensure canvas dimensions match video container dimensions
  if (canvas.width !== canvas.offsetWidth || canvas.height !== canvas.offsetHeight) {
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
  }
  
  // For object-fit: cover, we need to calculate the visible portion of the video
  const videoAspect = videoWidth / videoHeight;
  const canvasAspect = canvas.width / canvas.height;
  
  let scale, offsetX = 0, offsetY = 0;
  
  if (videoAspect > canvasAspect) {
    // Video is wider than canvas - some width is cropped
    scale = canvas.height / videoHeight;
    offsetX = (videoWidth * scale - canvas.width) / 2;
  } else {
    // Video is taller than canvas - some height is cropped
    scale = canvas.width / videoWidth;
    offsetY = (videoHeight * scale - canvas.height) / 2;
  }
  
  // Draw each detection
  detections.forEach(detection => {
    // Convert normalized coordinates to canvas coordinates
    const x = (detection.x * videoWidth * scale) - offsetX;
    const y = (detection.y * videoHeight * scale) - offsetY;
    const width = detection.width * videoWidth * scale;
    const height = detection.height * videoHeight * scale;
    
    // Draw bounding box
    ctx.strokeStyle = detection.color || '#00FF00';
    ctx.lineWidth = 2;
    ctx.strokeRect(x, y, width, height);
    
    // Draw label background
    ctx.fillStyle = detection.color || '#00FF00';
    const label = `${detection.class} ${Math.round(detection.confidence * 100)}%`;
    const textWidth = ctx.measureText(label).width + 10;
    ctx.fillRect(x, y - 20, textWidth, 20);
    
    // Draw label text
    ctx.fillStyle = '#000000';
    ctx.font = '12px Arial';
    ctx.fillText(label, x + 5, y - 5);
  });
}

/**
 * Handle fullscreen change events for detection overlay
 * @param {Event} event - Fullscreen change event
 * @param {HTMLCanvasElement} canvasOverlay - Canvas element for drawing detection boxes
 * @param {HTMLVideoElement} videoElement - Video element
 */
export function handleFullscreenChange(event, canvasOverlay, videoElement) {
  if (document.fullscreenElement) {
    // When entering fullscreen, resize the canvas to match the fullscreen dimensions
    setTimeout(() => {
      if (canvasOverlay && videoElement) {
        canvasOverlay.width = videoElement.clientWidth;
        canvasOverlay.height = videoElement.clientHeight;
        
        // Ensure the canvas is positioned correctly in fullscreen
        canvasOverlay.style.position = 'absolute';
        canvasOverlay.style.top = '0';
        canvasOverlay.style.left = '0';
        canvasOverlay.style.width = '100%';
        canvasOverlay.style.height = '100%';
        canvasOverlay.style.zIndex = '10'; // Ensure it's above the video but below controls
      }
    }, 100); // Small delay to allow fullscreen to complete
  } else {
    // When exiting fullscreen, reset the canvas dimensions
    setTimeout(() => {
      if (canvasOverlay && videoElement) {
        canvasOverlay.width = videoElement.clientWidth;
        canvasOverlay.height = videoElement.clientHeight;
      }
    }, 100);
  }
}

/**
 * Ensure controls are visible above the detection overlay
 * @param {string} streamName - Name of the stream
 */
export function ensureControlsVisibility(streamName) {
  const streamId = streamName.replace(/\s+/g, '-');
  const videoCell = document.querySelector(`.video-cell[data-stream="${streamId}"]`);
  
  if (!videoCell) return;
  
  // Find all controls within this cell
  const controls = videoCell.querySelector('.stream-controls');
  if (controls) {
    // Ensure controls are above the canvas overlay
    controls.style.position = 'relative';
    controls.style.zIndex = '30'; // Higher than the canvas overlay
    controls.style.pointerEvents = 'auto'; // Ensure clicks are registered
    
    // Remove any fullscreen button as it's redundant
    const fullscreenBtn = controls.querySelector('.fullscreen-btn');
    if (fullscreenBtn) {
      fullscreenBtn.remove();
    }
  }
  
  // Find and fix snapshot button specifically
  const snapshotBtn = videoCell.querySelector('.snapshot-btn');
  if (snapshotBtn) {
    snapshotBtn.style.position = 'relative';
    snapshotBtn.style.zIndex = '30';
    snapshotBtn.style.pointerEvents = 'auto';
  }
  
  // Find and fix all buttons in the video cell
  const allButtons = videoCell.querySelectorAll('button');
  allButtons.forEach(button => {
    button.style.position = 'relative';
    button.style.zIndex = '30';
    button.style.pointerEvents = 'auto';
  });
  
  // Make sure the video element itself doesn't block clicks
  const video = videoCell.querySelector('video');
  if (video) {
    video.style.pointerEvents = 'none';
  }
}

/**
 * Initialize detection overlay for a stream
 * @param {string} streamName - Name of the stream
 * @param {HTMLCanvasElement} canvasOverlay - Canvas element for drawing detection boxes
 * @param {HTMLVideoElement} videoElement - Video element
 * @param {Object} detectionIntervals - Reference to store interval IDs
 */
export function initializeDetectionOverlay(streamName, canvasOverlay, videoElement, detectionIntervals) {
  // Ensure the canvas is properly positioned
  canvasOverlay.style.position = 'absolute';
  canvasOverlay.style.top = '0';
  canvasOverlay.style.left = '0';
  canvasOverlay.style.width = '100%';
  canvasOverlay.style.height = '100%';
  canvasOverlay.style.pointerEvents = 'none'; // Allow clicks to pass through
  canvasOverlay.style.zIndex = '5'; // Above video but below controls
  
  // Find the parent container and ensure proper stacking context
  const parentContainer = canvasOverlay.parentElement;
  if (parentContainer) {
    parentContainer.style.position = 'relative'; // Create stacking context
  }
  
  // Ensure controls are visible
  ensureControlsVisibility(streamName);
  
  // Add fullscreen change listener
  document.addEventListener('fullscreenchange', (event) => {
    handleFullscreenChange(event, canvasOverlay, videoElement);
    
    // Re-ensure controls visibility after fullscreen change
    setTimeout(() => ensureControlsVisibility(streamName), 200);
  });
  
  // Start detection polling
  return startDetectionPolling(streamName, canvasOverlay, videoElement, detectionIntervals);
}

/**
 * Add snapshot functionality to detection overlay
 * @param {string} streamName - Name of the stream
 * @param {HTMLCanvasElement} canvasOverlay - Canvas element for drawing detection boxes
 * @param {HTMLVideoElement} videoElement - Video element
 */
export function addSnapshotWithDetections(streamName, canvasOverlay, videoElement) {
  const streamId = streamName.replace(/\s+/g, '-');
  const videoCell = document.querySelector(`.video-cell[data-stream="${streamId}"]`);
  
  if (!videoCell) return;
  
  // Find the snapshot button
  const snapshotBtn = videoCell.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
  if (!snapshotBtn) return;
  
  // Create a wrapper for the original click handler
  const originalOnClick = snapshotBtn.onclick;
  
  // Replace with our enhanced version that includes detections
  snapshotBtn.onclick = function(event) {
    event.preventDefault();
    event.stopPropagation();
    
    // Create a combined canvas with video and detections
    const combinedCanvas = document.createElement('canvas');
    combinedCanvas.width = videoElement.videoWidth;
    combinedCanvas.height = videoElement.videoHeight;
    const ctx = combinedCanvas.getContext('2d');
    
    // Draw the video frame
    ctx.drawImage(videoElement, 0, 0, combinedCanvas.width, combinedCanvas.height);
    
    // Draw the detections from the overlay canvas
    if (canvasOverlay.width > 0 && canvasOverlay.height > 0) {
      ctx.drawImage(canvasOverlay, 0, 0, canvasOverlay.width, canvasOverlay.height, 
                   0, 0, combinedCanvas.width, combinedCanvas.height);
    }
    
    // Store the canvas for the snapshot functionality
    window.__snapshotCanvas = combinedCanvas;
    
    // Generate a filename
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const fileName = `snapshot-${streamName.replace(/\s+/g, '-')}-${timestamp}.jpg`;
    window.__snapshotFileName = fileName;
    
    // Show the standard preview
    const dataUrl = combinedCanvas.toDataURL('image/jpeg', 0.95);
    showSnapshotPreview(dataUrl, `Snapshot: ${streamName}`);
    
    // Call the original handler if it exists
    if (typeof originalOnClick === 'function') {
      originalOnClick.call(this, event);
    }
    
    return false;
  };
  
  // Make sure the button is visible
  snapshotBtn.style.position = 'relative';
  snapshotBtn.style.zIndex = '30';
  snapshotBtn.style.pointerEvents = 'auto';
  
  console.log('Enhanced snapshot button with detection overlay capability');
}
