/**
 * UI utility functions for LiveView
 */

import { h } from '../../preact.min.js';
import { html } from '../../html-helper.js';

/**
 * Set up modals for the application
 */
export function setupModals() {
  // Create snapshot modal container if it doesn't exist
  let snapshotModalContainer = document.getElementById('snapshot-modal-container');
  if (!snapshotModalContainer) {
    snapshotModalContainer = document.createElement('div');
    snapshotModalContainer.id = 'snapshot-modal-container';
    document.body.appendChild(snapshotModalContainer);
  }
  
  // Create snapshot preview modal
  const snapshotModal = document.createElement('div');
  snapshotModal.id = 'snapshot-preview-modal';
  snapshotModal.className = 'fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50 hidden';
  
  snapshotModal.innerHTML = `
    <div class="modal-content bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out scale-95 opacity-0" style="width: 90%;">
      <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 id="snapshot-preview-title" class="text-lg font-semibold text-gray-900 dark:text-white">Snapshot</h3>
        <button class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">✕</button>
      </div>
      <div class="p-4 overflow-auto flex-grow">
        <img id="snapshot-preview-image" class="max-w-full max-h-[70vh] mx-auto" src="" alt="Snapshot">
      </div>
      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2">
        <button id="snapshot-download-btn" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Download</button>
        <button id="snapshot-close-btn" class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">Close</button>
      </div>
    </div>
  `;
  
  snapshotModalContainer.appendChild(snapshotModal);
  
  // Create video modal container if it doesn't exist
  let videoModalContainer = document.getElementById('video-modal-container');
  if (!videoModalContainer) {
    videoModalContainer = document.createElement('div');
    videoModalContainer.id = 'video-modal-container';
    document.body.appendChild(videoModalContainer);
  }
  
  // Create video preview modal
  const videoModal = document.createElement('div');
  videoModal.id = 'video-preview-modal';
  videoModal.className = 'fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50 hidden';
  
  videoModal.innerHTML = `
    <div class="modal-content bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out scale-95 opacity-0 w-full md:w-[90%]">
      <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 id="video-preview-title" class="text-lg font-semibold text-gray-900 dark:text-white">Video</h3>
        <button class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">✕</button>
      </div>
      <div class="p-4 flex-grow">
        <video id="video-preview-player" class="w-full h-auto max-w-full object-contain mx-auto" controls autoplay></video>
      </div>
      <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2">
        <a id="video-download-btn" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors" href="#" download>Download</a>
        <button id="video-close-btn" class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600">Close</button>
      </div>
    </div>
  `;
  
  videoModalContainer.appendChild(videoModal);
  
  // Set up event listeners for modals
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      // Close any open modals
      const modals = document.querySelectorAll('.fixed.inset-0.bg-black.bg-opacity-75');
      modals.forEach(modal => {
        if (!modal.classList.contains('hidden')) {
          modal.classList.add('hidden');
        }
      });
    }
  });
}

/**
 * Add status message styles to the document
 */
export function addStatusMessageStyles() {
  // Import the toast module to ensure it's loaded
  import("./toast.js").then(({ initToastContainer }) => {
    // Initialize the toast container
    initToastContainer(false);
  }).catch(error => {
    console.error('Error loading toast module:', error);
    
    // Fallback to basic styles if toast module fails to load
    if (document.getElementById('status-message-styles')) {
      return;
    }
    
    // Create style element
    const style = document.createElement('style');
    style.id = 'status-message-styles';
    
    // Add CSS rules
    style.textContent = `
      #status-message-container {
        position: fixed;
        top: 1rem;
        left: 50%;
        transform: translateX(-50%);
        z-index: 50;
        display: flex;
        flex-direction: column;
        align-items: center;
      }
      
      .status-message {
        background-color: #1f2937;
        color: white;
        padding: 0.5rem 1rem;
        border-radius: 0.375rem;
        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        margin-bottom: 0.5rem;
        transition: all 0.3s ease;
        opacity: 0;
        transform: translateY(-0.5rem);
      }
      
      .status-message.show {
        opacity: 1;
        transform: translateY(0);
      }
      
      .status-message.success {
        background-color: #10b981;
      }
      
      .status-message.error {
        background-color: #ef4444;
      }
      
      .status-message.warning {
        background-color: #f59e0b;
      }
      
      .status-message.info {
        background-color: #3b82f6;
      }
    `;
    
    // Add to document head
    document.head.appendChild(style);
  });
}

/**
 * Add modal styles to the document
 */
export function addModalStyles() {
  // Check if styles already exist
  if (document.getElementById('modal-styles')) {
    return;
  }
  
  // Create style element
  const style = document.createElement('style');
  style.id = 'modal-styles';
  
  // Add CSS rules
  style.textContent = `
    .modal-content {
      transition: all 0.3s ease-out;
    }
    
    .modal-content.scale-95 {
      transform: scale(0.95);
      opacity: 0;
    }
    
    .modal-content.scale-100 {
      transform: scale(1);
      opacity: 1;
    }
  `;
  
  // Add to document head
  document.head.appendChild(style);
}

/**
 * DeleteConfirmationModal component for Preact
 * This is the declarative version for use with Preact JSX
 * @param {Object} props - Component props
 * @param {boolean} props.isOpen - Whether the modal is open
 * @param {Function} props.onClose - Function to call when the modal is closed
 * @param {Function} props.onConfirm - Function to call when delete is confirmed
 * @param {string} props.mode - Delete mode ('selected' or 'all')
 * @param {number} props.count - Number of items selected (for 'selected' mode)
 * @returns {JSX.Element} DeleteConfirmationModal component
 */
export function DeleteConfirmationModal(props) {
  const { isOpen, onClose, onConfirm, mode, count } = props;
  
  if (!isOpen) return null;
  
  // Determine title and message based on mode
  let title = 'Confirm Delete';
  let message = 'Are you sure you want to delete this item?';
  
  if (mode === 'selected') {
    title = 'Delete Selected Recordings';
    message = `Are you sure you want to delete ${count} selected recording${count !== 1 ? 's' : ''}?`;
  } else if (mode === 'all') {
    title = 'Delete All Filtered Recordings';
    message = 'Are you sure you want to delete all recordings matching the current filters? This action cannot be undone.';
  }
  
  // Handle escape key
  const handleKeyDown = (e) => {
    if (e.key === 'Escape') {
      onClose();
    }
  };
  
  // Handle background click
  const handleBackgroundClick = (e) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };
  
  return html`
    <div 
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick=${handleBackgroundClick}
      onKeyDown=${handleKeyDown}
    >
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 max-w-md mx-auto">
        <div class="mb-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">${title}</h3>
        </div>
        
        <p class="text-gray-600 dark:text-gray-300 mb-6">${message}</p>
        
        <div class="flex justify-end space-x-3">
          <button 
            class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
            onClick=${onClose}
          >
            Cancel
          </button>
          <button 
            class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
            onClick=${onConfirm}
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  `;
}

/**
 * Show a video modal
 * @param {string} videoUrl - URL of the video to display
 * @param {string} title - Title for the video
 * @param {string} downloadUrl - Optional URL for downloading the video
 */
export function showVideoModal(videoUrl, title, downloadUrl) {
  // Create overlay container
  const overlay = document.createElement('div');
  overlay.className = 'fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50';
  overlay.id = 'video-modal-overlay';
  
  // Create modal container with responsive classes
  const modalContainer = document.createElement('div');
  modalContainer.className = 'bg-gray-50 dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl flex flex-col w-full md:w-[90%]';
  
  // Create header
  const header = document.createElement('div');
  header.className = 'flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700';
  
  const titleElement = document.createElement('h3');
  titleElement.className = 'text-lg font-semibold text-gray-900 dark:text-white';
  titleElement.textContent = title || 'Video';
  
  const closeButton = document.createElement('button');
  closeButton.className = 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200';
  closeButton.innerHTML = '✕';
  closeButton.addEventListener('click', () => {
    document.body.removeChild(overlay);
  });
  
  header.appendChild(titleElement);
  header.appendChild(closeButton);
  
  // Create video container with responsive classes
  const videoContainer = document.createElement('div');
  videoContainer.className = 'p-4';
  
  const video = document.createElement('video');
  video.src = videoUrl;
  video.className = 'w-full h-auto max-w-full object-contain mx-auto';
  video.controls = true;
  video.autoplay = true;
  
  videoContainer.appendChild(video);
  
  // Create controls container with responsive Tailwind classes
  const controlsContainer = document.createElement('div');
  controlsContainer.id = 'recordings-controls';
  controlsContainer.className = 'mx-4 mb-4 p-4 border border-green-500 rounded-lg bg-white dark:bg-gray-700 shadow-md relative z-10';
  
  // Create heading
  const heading = document.createElement('h3');
  heading.className = 'text-lg font-bold text-center mb-4 text-gray-800 dark:text-white';
  heading.textContent = 'PLAYBACK CONTROLS';
  controlsContainer.appendChild(heading);
  
  // Create controls grid container
  const controlsGrid = document.createElement('div');
  controlsGrid.className = 'grid grid-cols-1 md:grid-cols-2 gap-4 mb-2';
  
  // Create speed controls section
  const speedControlsSection = document.createElement('div');
  speedControlsSection.className = 'border-b pb-4 md:border-b-0 md:border-r md:pr-4 md:pb-0';
  
  // Create speed section heading
  const speedHeading = document.createElement('h4');
  speedHeading.className = 'font-bold text-center mb-3 text-gray-700 dark:text-gray-300';
  speedHeading.textContent = 'Playback Speed';
  speedControlsSection.appendChild(speedHeading);
  
  // Create speed buttons container
  const speedButtonsContainer = document.createElement('div');
  speedButtonsContainer.className = 'flex flex-wrap justify-center gap-2';
  
  // Create speed buttons
  const speeds = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];
  speeds.forEach(speed => {
    const button = document.createElement('button');
    button.textContent = speed === 1.0 ? '1× (Normal)' : `${speed}×`;
    button.className = speed === 1.0 
      ? 'speed-btn px-3 py-2 rounded-full bg-green-500 text-white text-sm font-medium transition-all focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50'
      : 'speed-btn px-3 py-2 rounded-full bg-gray-200 hover:bg-gray-300 text-sm font-medium transition-all focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50';
    button.setAttribute('data-speed', speed);
    
    // Add click event
    button.addEventListener('click', () => {
      // Set playback speed
      video.playbackRate = speed;
      
      // Update button styles
      speedButtonsContainer.querySelectorAll('.speed-btn').forEach(btn => {
        if (btn.getAttribute('data-speed') === speed.toString()) {
          btn.classList.remove('bg-gray-200', 'hover:bg-gray-300');
          btn.classList.add('bg-green-500', 'text-white');
        } else {
          btn.classList.remove('bg-green-500', 'text-white');
          btn.classList.add('bg-gray-200', 'hover:bg-gray-300');
        }
      });
      
      // Update current speed indicator
      currentSpeedIndicator.textContent = `Current Speed: ${speed}× ${speed === 1.0 ? '(Normal)' : ''}`;
    });
    
    speedButtonsContainer.appendChild(button);
  });
  
  // Add buttons container to speed controls section
  speedControlsSection.appendChild(speedButtonsContainer);
  
  // Add current speed indicator
  const currentSpeedIndicator = document.createElement('div');
  currentSpeedIndicator.id = 'current-speed-indicator';
  currentSpeedIndicator.className = 'mt-3 text-center font-medium text-green-600 dark:text-green-400 text-sm';
  currentSpeedIndicator.textContent = 'Current Speed: 1× (Normal)';
  speedControlsSection.appendChild(currentSpeedIndicator);
  
  // Add speed controls section to grid
  controlsGrid.appendChild(speedControlsSection);
  
  // Create detection overlay section
  const detectionSection = document.createElement('div');
  detectionSection.className = 'pt-4 md:pt-0 md:pl-4';
  
  // Create detection section heading
  const detectionHeading = document.createElement('h4');
  detectionHeading.className = 'font-bold text-center mb-2 text-gray-700 dark:text-gray-300';
  detectionHeading.textContent = 'Detection Overlays';
  detectionSection.appendChild(detectionHeading);
  
  // Create detection controls container
  const detectionControlsContainer = document.createElement('div');
  detectionControlsContainer.className = 'flex flex-col items-center gap-2';
  
  // Create detection checkbox container
  const detectionCheckboxContainer = document.createElement('div');
  detectionCheckboxContainer.className = 'flex items-center gap-2 mb-2';
  
  // Create detection checkbox
  const detectionCheckbox = document.createElement('input');
  detectionCheckbox.type = 'checkbox';
  detectionCheckbox.id = 'detection-overlay-checkbox';
  detectionCheckbox.className = 'w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600';
  
  // Create detection checkbox label
  const detectionCheckboxLabel = document.createElement('label');
  detectionCheckboxLabel.htmlFor = 'detection-overlay-checkbox';
  detectionCheckboxLabel.className = 'text-sm font-medium text-gray-700 dark:text-gray-300';
  detectionCheckboxLabel.textContent = 'Show Detection Overlays';
  
  // Add checkbox and label to container
  detectionCheckboxContainer.appendChild(detectionCheckbox);
  detectionCheckboxContainer.appendChild(detectionCheckboxLabel);
  
  // Add checkbox container to detection controls
  detectionControlsContainer.appendChild(detectionCheckboxContainer);
  
  // Create sensitivity slider container
  const sensitivityContainer = document.createElement('div');
  sensitivityContainer.className = 'flex flex-col w-full mt-2 mb-2';
  
  // Create sensitivity label
  const sensitivityLabel = document.createElement('label');
  sensitivityLabel.htmlFor = 'detection-sensitivity-slider';
  sensitivityLabel.className = 'text-sm font-medium text-gray-700 dark:text-gray-300 mb-1';
  sensitivityLabel.textContent = 'Detection Sensitivity';
  
  // Create sensitivity value display
  const sensitivityValue = document.createElement('div');
  sensitivityValue.id = 'detection-sensitivity-value';
  sensitivityValue.className = 'text-xs text-gray-600 dark:text-gray-400 text-center mb-1';
  sensitivityValue.textContent = 'Time Window: 2 seconds';
  
  // Create sensitivity slider
  const sensitivitySlider = document.createElement('input');
  sensitivitySlider.type = 'range';
  sensitivitySlider.id = 'detection-sensitivity-slider';
  sensitivitySlider.className = 'w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700';
  sensitivitySlider.min = '1';
  sensitivitySlider.max = '10';
  sensitivitySlider.step = '1';
  sensitivitySlider.value = '2';
  
  // Add elements to sensitivity container
  sensitivityContainer.appendChild(sensitivityLabel);
  sensitivityContainer.appendChild(sensitivitySlider);
  sensitivityContainer.appendChild(sensitivityValue);
  
  // Add event listener to the sensitivity slider
  sensitivitySlider.addEventListener('input', () => {
    // Update the time window value
    timeWindow = parseInt(sensitivitySlider.value);
    
    // Update the sensitivity value display
    sensitivityValue.textContent = `Time Window: ${timeWindow} second${timeWindow !== 1 ? 's' : ''}`;
    
    // Redraw detections if enabled
    if (detectionOverlayEnabled) {
      drawDetections();
    }
  });
  
  // Add sensitivity container to detection controls
  detectionControlsContainer.appendChild(sensitivityContainer);
  
  // Create detection status indicator
  const detectionStatusIndicator = document.createElement('div');
  detectionStatusIndicator.id = 'detection-status-indicator';
  detectionStatusIndicator.className = 'text-center text-sm text-gray-600 dark:text-gray-400';
  detectionStatusIndicator.textContent = 'No detections loaded';
  
  // Add status indicator to detection controls
  detectionControlsContainer.appendChild(detectionStatusIndicator);
  
  // Add detection controls to detection section
  detectionSection.appendChild(detectionControlsContainer);
  
  // Add detection section to grid
  controlsGrid.appendChild(detectionSection);
  
  // Add controls grid to container
  controlsContainer.appendChild(controlsGrid);
  
  // Add download button to controls container
  if (downloadUrl) {
    const downloadSection = document.createElement('div');
    downloadSection.className = 'flex justify-center mt-4 pt-2 border-t border-gray-200 dark:border-gray-700';
    
    const downloadButton = document.createElement('a');
    downloadButton.className = 'px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors flex items-center text-sm';
    downloadButton.innerHTML = `
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
      </svg>
      Download Video
    `;
    downloadButton.href = downloadUrl;
    downloadButton.download = `video-${Date.now()}.mp4`;
    
    downloadSection.appendChild(downloadButton);
    controlsContainer.appendChild(downloadSection);
  }
  
  // Create empty footer for spacing
  const footer = document.createElement('div');
  footer.className = 'p-2';
  
  // Create detection overlay canvas container
  const canvasContainer = document.createElement('div');
  canvasContainer.className = 'relative';
  canvasContainer.style.position = 'relative';
  canvasContainer.style.width = '100%';
  canvasContainer.style.height = '100%';
  
  // Create detection overlay canvas
  const detectionCanvas = document.createElement('canvas');
  detectionCanvas.id = 'detection-overlay-canvas';
  detectionCanvas.className = 'absolute top-0 left-0 w-full h-full pointer-events-none';
  detectionCanvas.style.position = 'absolute';
  detectionCanvas.style.top = '0';
  detectionCanvas.style.left = '0';
  detectionCanvas.style.width = '100%';
  detectionCanvas.style.height = '100%';
  detectionCanvas.style.pointerEvents = 'none';
  detectionCanvas.style.display = 'none'; // Hidden by default
  
  // Wrap video in canvas container
  canvasContainer.appendChild(video);
  canvasContainer.appendChild(detectionCanvas);
  videoContainer.appendChild(canvasContainer);
  
  // Store recording data for detection overlay
  let recordingData = null;
  let detections = [];
  let detectionOverlayEnabled = false;
  let timeWindow = parseInt(sensitivitySlider.value); // Time window in seconds for detection visibility
  
  // Extract recording ID from URL
  const recordingIdMatch = videoUrl.match(/\/play\/(\d+)/);
  if (recordingIdMatch && recordingIdMatch[1]) {
    const recordingId = parseInt(recordingIdMatch[1], 10);
    
    // Fetch recording data
    fetch(`/api/recordings/${recordingId}`)
      .then(response => response.json())
      .then(data => {
        recordingData = data;
        
        // Check if recording has detections
        if (recordingData && recordingData.stream && recordingData.start_time && recordingData.end_time) {
          // Convert timestamps to seconds
          const startTime = Math.floor(new Date(recordingData.start_time).getTime() / 1000);
          const endTime = Math.floor(new Date(recordingData.end_time).getTime() / 1000);
          
          // Query the detections API
          const params = new URLSearchParams({
            start: startTime,
            end: endTime
          });
          
          return fetch(`/api/detection/results/${recordingData.stream}?${params.toString()}`);
        }
        
        throw new Error('Recording data incomplete');
      })
      .then(response => response.json())
      .then(data => {
        detections = data.detections || [];
        
        if (detections.length > 0) {
          detectionStatusIndicator.textContent = `${detections.length} detection${detections.length !== 1 ? 's' : ''} available`;
          detectionStatusIndicator.className = 'text-center text-sm font-medium text-green-600 dark:text-green-400';
          
          // Enable checkbox
          detectionCheckbox.disabled = false;
        } else {
          detectionStatusIndicator.textContent = 'No detections found for this recording';
          
          // Disable checkbox
          detectionCheckbox.disabled = true;
        }
      })
      .catch(error => {
        console.error('Error fetching detection data:', error);
        detectionStatusIndicator.textContent = 'Error loading detections';
        detectionStatusIndicator.className = 'text-center text-sm font-medium text-red-600 dark:text-red-400';
        
        // Disable checkbox
        detectionCheckbox.disabled = true;
      });
  }
  
  // Handle detection overlay checkbox change
  detectionCheckbox.addEventListener('change', () => {
    detectionOverlayEnabled = detectionCheckbox.checked;
    
    if (detectionOverlayEnabled) {
      // Show canvas
      detectionCanvas.style.display = 'block';
      
      // Draw detections
      drawDetections();
      
      // Update status
      detectionStatusIndicator.textContent = `Showing ${detections.length} detection${detections.length !== 1 ? 's' : ''}`;
    } else {
      // Hide canvas
      detectionCanvas.style.display = 'none';
      
      // Update status
      detectionStatusIndicator.textContent = `${detections.length} detection${detections.length !== 1 ? 's' : ''} available`;
    }
  });
  
  // Function to draw detections on canvas
  function drawDetections() {
    if (!detectionOverlayEnabled || !video || !detectionCanvas || detections.length === 0) {
      return;
    }
    
    // Get video dimensions
    const videoWidth = video.videoWidth;
    const videoHeight = video.videoHeight;
    
    if (videoWidth === 0 || videoHeight === 0) {
      // Video dimensions not available yet, try again later
      requestAnimationFrame(drawDetections);
      return;
    }
    
    // Set canvas dimensions to match video
    detectionCanvas.width = videoWidth;
    detectionCanvas.height = videoHeight;
    
    // Get canvas context
    const ctx = detectionCanvas.getContext('2d');
    ctx.clearRect(0, 0, detectionCanvas.width, detectionCanvas.height);
    
    // Get current video time in seconds
    const currentTime = video.currentTime;
    
    // Calculate the timestamp for the current video position
    // We need to convert from video time (seconds from start of video) to unix timestamp
    if (!recordingData || !recordingData.start_time) {
      return;
    }
    
    // Convert recording start time to seconds
    const recordingStartTime = Math.floor(new Date(recordingData.start_time).getTime() / 1000);
    
    // Calculate current timestamp in the video
    const currentTimestamp = recordingStartTime + Math.floor(currentTime);
    
    // Use the time window value from the slider
    // timeWindow is already defined at the top level
    
    // Filter detections to only show those within the time window
    const visibleDetections = detections.filter(detection => {
      if (!detection.timestamp) return false;
      
      // Check if detection is within the time window
      return Math.abs(detection.timestamp - currentTimestamp) <= timeWindow;
    });
    
    // Update status indicator with count of visible detections
    if (detectionStatusIndicator) {
      if (visibleDetections.length > 0) {
        detectionStatusIndicator.textContent = `Showing ${visibleDetections.length} detection${visibleDetections.length !== 1 ? 's' : ''} at current time`;
      } else {
        detectionStatusIndicator.textContent = `No detections at current time (${detections.length} total)`;
      }
    }
    
    // Draw each visible detection
    visibleDetections.forEach(detection => {
      // Calculate coordinates based on relative positions
      const x = detection.x * videoWidth;
      const y = detection.y * videoHeight;
      const width = detection.width * videoWidth;
      const height = detection.height * videoHeight;
      
      // Draw bounding box
      ctx.strokeStyle = 'rgba(0, 255, 0, 0.8)';
      ctx.lineWidth = 2;
      ctx.strokeRect(x, y, width, height);
      
      // Draw label background
      ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
      const labelText = `${detection.label} (${Math.round(detection.confidence * 100)}%)`;
      const labelWidth = ctx.measureText(labelText).width + 10;
      ctx.fillRect(x, y - 20, labelWidth, 20);
      
      // Draw label text
      ctx.fillStyle = 'white';
      ctx.font = '12px Arial';
      ctx.fillText(labelText, x + 5, y - 5);
    });
    
    // Request next frame if video is playing
    if (!video.paused && !video.ended) {
      requestAnimationFrame(drawDetections);
    }
  }
  
  // Handle video events
  video.addEventListener('play', () => {
    if (detectionOverlayEnabled) {
      drawDetections();
    }
  });
  
  video.addEventListener('seeked', () => {
    if (detectionOverlayEnabled) {
      drawDetections();
    }
  });
  
  // Add timeupdate event to update detections as video plays
  video.addEventListener('timeupdate', () => {
    if (detectionOverlayEnabled) {
      // Don't redraw on every timeupdate as it's too frequent
      // Instead, redraw every 0.5 seconds
      const currentTime = Math.floor(video.currentTime * 2) / 2;
      if (video.lastDrawnTime !== currentTime) {
        video.lastDrawnTime = currentTime;
        drawDetections();
      }
    }
  });
  
  // Assemble the modal
  modalContainer.appendChild(header);
  modalContainer.appendChild(videoContainer);
  modalContainer.appendChild(controlsContainer); // Add controls
  modalContainer.appendChild(footer);
  overlay.appendChild(modalContainer);
  
  // Add to document
  document.body.appendChild(overlay);
  
  // Add event listener to close on escape key
  const handleEscape = (e) => {
    if (e.key === 'Escape') {
      if (document.body.contains(overlay)) {
        document.body.removeChild(overlay);
      }
      document.removeEventListener('keydown', handleEscape);
    }
  };
  
  document.addEventListener('keydown', handleEscape);
  
  // Add event listener to close on background click
  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) {
      document.body.removeChild(overlay);
    }
  });
}

/**
 * Show a status message to the user
 * @param {string} message - Message to display
 * @param {string} type - Message type ('success', 'error', 'warning', 'info')
 * @param {number} duration - Duration in milliseconds (default: 4000)
 */
export function showStatusMessage(message, type = 'info', duration = 4000) {
  console.log(`Showing status message: ${message} (${type})`);
  
  // Try to use the global toast functions first (they should be available)
  if (window.showToast) {
    window.showToast(message, type, duration);
    return;
  }
  
  // If global functions aren't available, import the toast module dynamically
  import("./toast.js").then(({ showStatusMessage: showToast }) => {
    // Use the toast system
    showToast(message, type, duration);
  }).catch(error => {
    console.error('Error loading toast module:', error);
    
    // Fallback to basic implementation if toast module fails to load
    let statusContainer = document.getElementById('status-message-container');
    
    // Create container if it doesn't exist
    if (!statusContainer) {
      statusContainer = document.createElement('div');
      statusContainer.id = 'status-message-container';
      statusContainer.className = 'fixed top-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col items-center';
      document.body.appendChild(statusContainer);
    }
  
    // Create message element
    const messageElement = document.createElement('div');
    
    // Set class based on type
    let typeClass = 'bg-blue-500'; // Default for info
    if (type === 'success') {
      typeClass = 'bg-green-500';
    } else if (type === 'error') {
      typeClass = 'bg-red-500';
    } else if (type === 'warning') {
      typeClass = 'bg-yellow-500';
    }
    
    messageElement.className = `${typeClass} text-white px-4 py-2 rounded-lg shadow-lg mb-2 transition-all duration-300 opacity-0 transform translate-y-2`;
    messageElement.textContent = message;
    
    // Add to container
    statusContainer.appendChild(messageElement);
    
    // Trigger animation to show message
    setTimeout(() => {
      messageElement.classList.remove('opacity-0', 'translate-y-2');
    }, 10);
    
    // Set timeout to remove message
    setTimeout(() => {
      // Trigger animation to hide message
      messageElement.classList.add('opacity-0', 'translate-y-2');
      
      // Remove element after animation completes
      setTimeout(() => {
        if (messageElement.parentNode === statusContainer) {
          statusContainer.removeChild(messageElement);
        }
        
        // Remove container if no more messages
        if (statusContainer.children.length === 0 && statusContainer.parentNode === document.body) {
          document.body.removeChild(statusContainer);
        }
      }, 300);
    }, duration);
  });
}

/**
 * Show a snapshot preview in a modal
 * @param {string} imageData - The image data URL
 * @param {string} streamName - The name of the stream
 */
export function showSnapshotPreview(imageData, streamName) {
  // Get the existing snapshot modal created by setupModals()
  const snapshotModal = document.getElementById('snapshot-preview-modal');
  
  if (!snapshotModal) {
    console.error('Snapshot modal not found. Make sure setupModals() has been called.');
    return;
  }
  
  // Get modal elements
  const snapshotTitle = document.getElementById('snapshot-preview-title');
  const snapshotImage = document.getElementById('snapshot-preview-image');
  const snapshotDownloadBtn = document.getElementById('snapshot-download-btn');
  const snapshotCloseBtn = document.getElementById('snapshot-close-btn');
  const closeButtons = snapshotModal.querySelectorAll('.close');
  const modalContent = snapshotModal.querySelector('.modal-content');
  
  // Set the title and image
  if (snapshotTitle) snapshotTitle.textContent = `Snapshot: ${streamName}`;
  
  // Store the image data directly on the modal for later access
  snapshotModal.dataset.imageData = imageData;
  snapshotModal.dataset.streamName = streamName;
  
  // Ensure the image is properly displayed
  if (snapshotImage) {
    snapshotImage.src = imageData;
    snapshotImage.style.display = 'block'; // Make sure it's visible
    snapshotImage.onload = () => {
      console.log('Snapshot image loaded successfully');
    };
    snapshotImage.onerror = (e) => {
      console.error('Error loading snapshot image:', e);
    };
  }
  
  // Define close modal function
  function closeModal() {
    snapshotModal.classList.add('hidden');
    if (modalContent) {
      modalContent.classList.remove('scale-100');
      modalContent.classList.add('scale-95');
    }
  }
  
  // Set up close button event handlers
  if (snapshotCloseBtn) {
    snapshotCloseBtn.onclick = closeModal;
  }
  
  closeButtons.forEach(button => {
    button.onclick = closeModal;
  });
  
  // Close on click outside
  snapshotModal.onclick = function(event) {
    if (event.target === snapshotModal) {
      closeModal();
    }
  };
  
  // Close on escape key
  document.addEventListener('keydown', function(event) {
    if (event.key === 'Escape' && !snapshotModal.classList.contains('hidden')) {
      closeModal();
    }
  });
  
  // Set up download button
  if (snapshotDownloadBtn) {
    snapshotDownloadBtn.onclick = function() {
      // Get the stored image data from the modal
      const storedImageData = snapshotModal.dataset.imageData;
      const storedStreamName = snapshotModal.dataset.streamName || streamName;
      
      if (!storedImageData) {
        console.error('No image data available for download');
        showStatusMessage('Download failed: No image data available', 5000);
        return;
      }
      
      console.log('Downloading snapshot with data URL length:', storedImageData.length);
      
      try {
        // Create a temporary download iframe - this approach often works better on HTTP
        console.log('Using iframe download method');
        
        // Create a canvas to resize and optimize the image
        const canvas = document.createElement('canvas');
        const img = new Image();
        
        img.onload = function() {
          try {
            console.log('Image loaded for optimization, original dimensions:', img.width, 'x', img.height);
            
            // Use original dimensions as requested by the user
            const newWidth = img.width;
            const newHeight = img.height;
            console.log('Using original image dimensions for maximum quality:', newWidth, 'x', newHeight);
            
            // Set canvas dimensions to the new size
            canvas.width = newWidth;
            canvas.height = newHeight;
            
            // Draw the image on the canvas with the new dimensions
            const ctx = canvas.getContext('2d');
            ctx.fillStyle = 'black';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            ctx.drawImage(img, 0, 0, newWidth, newHeight);
            
            // Convert to JPEG with maximum quality for best image quality
            const optimizedJpegData = canvas.toDataURL('image/jpeg', 1.0);
            console.log('Optimized JPEG data URL length:', optimizedJpegData.length);
            
            // Create a download link with the optimized image
            const filename = `snapshot_${storedStreamName}_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`;
            
            // Method 1: Try the iframe approach
            const iframe = document.createElement('iframe');
            iframe.style.display = 'none';
            document.body.appendChild(iframe);
            
            try {
              // Write a simple HTML document to the iframe with a download link
              const iframeDoc = iframe.contentWindow.document;
              iframeDoc.open();
              iframeDoc.write(`
                <!DOCTYPE html>
                <html>
                <head>
                  <title>Downloading...</title>
                  <script>
                    function startDownload() {
                      const a = document.getElementById('download-link');
                      a.click();
                      setTimeout(function() {
                        window.parent.postMessage('download-complete', '*');
                      }, 1000);
                    }
                  </script>
                </head>
                <body onload="startDownload()">
                  <a id="download-link" href="${optimizedJpegData}" download="${filename}">Download</a>
                </body>
                </html>
              `);
              iframeDoc.close();
              
              // Listen for the download complete message
              window.addEventListener('message', function onMessage(e) {
                if (e.data === 'download-complete') {
                  window.removeEventListener('message', onMessage);
                  document.body.removeChild(iframe);
                  showStatusMessage('Snapshot downloaded successfully', 3000);
                }
              }, { once: true });
              
              // Fallback cleanup in case the message isn't received
              setTimeout(() => {
                if (document.body.contains(iframe)) {
                  document.body.removeChild(iframe);
                }
              }, 5000);
              
            } catch (iframeError) {
              console.error('Iframe download method failed:', iframeError);
              document.body.removeChild(iframe);
              
              // Fallback to direct link method
              tryDirectLinkDownload(optimizedJpegData, filename);
            }
          } catch (canvasError) {
            console.error('Canvas processing error:', canvasError);
            showStatusMessage('Download failed: Error processing image', 5000);
            
            // Try direct download with original data
            tryDirectLinkDownload(storedImageData, `snapshot_${storedStreamName}_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`);
          }
        };
        
        img.onerror = function(e) {
          console.error('Error loading image for optimization:', e);
          showStatusMessage('Download failed: Could not load image', 5000);
          
          // Try direct download with original data
          tryDirectLinkDownload(storedImageData, `snapshot_${storedStreamName}_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`);
        };
        
        // Set the source to trigger loading
        img.src = storedImageData;
        
      } catch (error) {
        console.error('Error in snapshot download process:', error);
        showStatusMessage('Download failed: ' + error.message, 5000);
        
        // Try direct download with original data as last resort
        tryDirectLinkDownload(storedImageData, `snapshot_${storedStreamName}_${new Date().toISOString().replace(/[:.]/g, '-')}.jpg`);
      }
      
      // Helper function for direct link download
      function tryDirectLinkDownload(dataUrl, filename) {
        console.log('Attempting direct link download');
        try {
          const link = document.createElement('a');
          link.href = dataUrl;
          link.download = filename;
          link.target = '_blank'; // Try opening in a new tab
          link.rel = 'noopener noreferrer';
          document.body.appendChild(link);
          link.click();
          
          // Clean up
          setTimeout(() => {
            document.body.removeChild(link);
          }, 100);
          
          showStatusMessage('Download initiated', 3000);
        } catch (linkError) {
          console.error('Direct link download failed:', linkError);
          showStatusMessage('Download failed: Browser restrictions', 5000);
        }
      }
    };
  }
  
  // Show the modal
  snapshotModal.classList.remove('hidden');
  
  // Animate the modal content
  if (modalContent) {
    setTimeout(() => {
      modalContent.classList.remove('scale-95', 'opacity-0');
      modalContent.classList.add('scale-100', 'opacity-100');
    }, 10);
  }
}
