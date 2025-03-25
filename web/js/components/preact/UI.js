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
    <div class="modal-content bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out scale-95 opacity-0" style="width: 90%;">
      <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
        <h3 id="video-preview-title" class="text-lg font-semibold text-gray-900 dark:text-white">Video</h3>
        <button class="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200">✕</button>
      </div>
      <div class="p-4 overflow-auto flex-grow">
        <video id="video-preview-player" class="max-w-full max-h-[70vh] mx-auto" controls autoplay></video>
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
  // Check if styles already exist
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
      bottom: 1rem;
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
      transform: translateY(0.5rem);
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
  `;
  
  // Add to document head
  document.head.appendChild(style);
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
  
  // Create modal container
  const modalContainer = document.createElement('div');
  modalContainer.className = 'bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col';
  modalContainer.style.width = '90%';
  
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
  
  // Create video container
  const videoContainer = document.createElement('div');
  videoContainer.className = 'p-4 overflow-auto flex-grow';
  
  const video = document.createElement('video');
  video.src = videoUrl;
  video.className = 'max-w-full max-h-[70vh] mx-auto';
  video.controls = true;
  video.autoplay = true;
  
  videoContainer.appendChild(video);
  
  // Create speed controls container with Tailwind classes
  const speedControlsContainer = document.createElement('div');
  speedControlsContainer.id = 'recordings-speed-controls';
  speedControlsContainer.className = 'mt-6 mb-8 p-4 border-2 border-green-500 rounded-lg bg-white dark:bg-gray-800 shadow-md mx-4';
  // Add inline styles to ensure visibility
  speedControlsContainer.style.width = 'calc(100% - 2rem)';
  speedControlsContainer.style.display = 'block';
  speedControlsContainer.style.position = 'relative';
  speedControlsContainer.style.zIndex = '100';
  
  // Create heading
  const heading = document.createElement('h3');
  heading.className = 'text-lg font-bold text-center mb-4 text-gray-800 dark:text-white';
  heading.textContent = 'PLAYBACK SPEED CONTROLS';
  speedControlsContainer.appendChild(heading);
  
  // Create speed buttons container
  const speedButtonsContainer = document.createElement('div');
  speedButtonsContainer.className = 'flex flex-wrap justify-center gap-2';
  
  // Create speed buttons
  const speeds = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];
  speeds.forEach(speed => {
    const button = document.createElement('button');
    button.textContent = speed === 1.0 ? '1× (Normal)' : `${speed}×`;
    button.className = speed === 1.0 
      ? 'speed-btn px-4 py-2 rounded-full bg-green-500 text-white font-bold transition-all transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50'
      : 'speed-btn px-4 py-2 rounded-full bg-gray-200 hover:bg-gray-300 font-bold transition-all transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50';
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
  
  // Add buttons container to speed controls
  speedControlsContainer.appendChild(speedButtonsContainer);
  
  // Add current speed indicator
  const currentSpeedIndicator = document.createElement('div');
  currentSpeedIndicator.id = 'current-speed-indicator';
  currentSpeedIndicator.className = 'mt-4 text-center font-bold text-green-600 dark:text-green-400';
  currentSpeedIndicator.textContent = 'Current Speed: 1× (Normal)';
  speedControlsContainer.appendChild(currentSpeedIndicator);
  
  // Create footer with actions
  const footer = document.createElement('div');
  footer.className = 'p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2';
  
  if (downloadUrl) {
    const downloadButton = document.createElement('a');
    downloadButton.className = 'px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors';
    downloadButton.textContent = 'Download';
    downloadButton.href = downloadUrl;
    downloadButton.download = `video-${Date.now()}.mp4`;
    
    footer.appendChild(downloadButton);
  }
  
  // Assemble the modal
  modalContainer.appendChild(header);
  modalContainer.appendChild(videoContainer);
  modalContainer.appendChild(speedControlsContainer); // Add speed controls
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
 * @param {number} duration - Duration in milliseconds (default: 3000)
 */
export function showStatusMessage(message, duration = 3000) {
  // Check if a status message container already exists
  let statusContainer = document.getElementById('status-message-container');
  
  // Create container if it doesn't exist
  if (!statusContainer) {
    statusContainer = document.createElement('div');
    statusContainer.id = 'status-message-container';
    statusContainer.className = 'fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col items-center';
    document.body.appendChild(statusContainer);
  }
  
  // Create message element
  const messageElement = document.createElement('div');
  messageElement.className = 'bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg mb-2 transition-all duration-300 opacity-0 transform translate-y-2';
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
      if (statusContainer.children.length === 0) {
        document.body.removeChild(statusContainer);
      }
    }, 300);
  }, duration);
}
 
/**
 * Show a snapshot preview
 * @param {string} imageUrl - Data URL of the image
 * @param {string} title - Title for the snapshot
 */
export function showSnapshotPreview(imageUrl, title) {
  // Create overlay container
  const overlay = document.createElement('div');
  overlay.className = 'fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50';
  overlay.id = 'snapshot-preview-overlay';
  
  // Create preview container
  const previewContainer = document.createElement('div');
  previewContainer.className = 'bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col';
  previewContainer.style.width = '90%';
  
  // Create header
  const header = document.createElement('div');
  header.className = 'flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700';
  
  const titleElement = document.createElement('h3');
  titleElement.className = 'text-lg font-semibold text-gray-900 dark:text-white';
  titleElement.textContent = title || 'Snapshot';
  
  const closeButton = document.createElement('button');
  closeButton.className = 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200';
  closeButton.innerHTML = '✕';
  closeButton.addEventListener('click', () => {
    document.body.removeChild(overlay);
  });
  
  header.appendChild(titleElement);
  header.appendChild(closeButton);
  
  // Create image container
  const imageContainer = document.createElement('div');
  imageContainer.className = 'p-4 overflow-auto flex-grow';
  
  const image = document.createElement('img');
  image.src = imageUrl;
  image.className = 'max-w-full max-h-[70vh] mx-auto';
  image.alt = title || 'Snapshot';
  
  imageContainer.appendChild(image);
  
  // Create footer with actions
  const footer = document.createElement('div');
  footer.className = 'p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-2';
  
  const downloadButton = document.createElement('button');
  downloadButton.className = 'px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors';
  downloadButton.textContent = 'Download';
  downloadButton.addEventListener('click', () => {
    try {
      // For data URLs, we need to ensure proper handling
      // First, check if it's a valid data URL
      if (!imageUrl.startsWith('data:image/')) {
        console.error('Invalid image URL format for download');
        showStatusMessage('Failed to download: Invalid image format', 3000);
        return;
      }
      
      // Convert data URL to Blob
      // Extract the base64 data from the data URL
      const byteString = atob(imageUrl.split(',')[1]);
      
      // Get the MIME type from the data URL
      const mimeType = imageUrl.split(',')[0].split(':')[1].split(';')[0];
      
      // Create an array buffer from the binary string
      const arrayBuffer = new ArrayBuffer(byteString.length);
      const intArray = new Uint8Array(arrayBuffer);
      
      // Fill the array buffer with the binary data
      for (let i = 0; i < byteString.length; i++) {
        intArray[i] = byteString.charCodeAt(i);
      }
      
      // Create a Blob from the array buffer
      const blob = new Blob([arrayBuffer], { type: mimeType });
      
      // Create an object URL from the Blob
      const blobUrl = URL.createObjectURL(blob);
      
      // Create a download link with proper attributes
      const link = document.createElement('a');
      link.href = blobUrl;
      link.download = `snapshot-${Date.now()}.jpg`;
      
      // IMPORTANT CHANGE: Don't hide the link, and keep it in the DOM longer
      // Add link to the document in a way that won't disrupt the UI but will remain active
      link.style.position = 'absolute';
      link.style.top = '0';
      link.style.left = '0';
      link.style.opacity = '0.01'; // Nearly invisible but technically visible
      
      // Add to document, click, but DON'T remove immediately
      document.body.appendChild(link);
      
      // Click after a short delay to ensure the link is properly registered
      setTimeout(() => {
        link.click();
        console.log('Download initiated with Blob URL');
        showStatusMessage('Download started', 3000);
        
        // Clean up after a much longer delay to ensure download starts
        // This is the key fix - keeping the link around longer
        setTimeout(() => {
          if (document.body.contains(link)) {
            document.body.removeChild(link);
          }
          URL.revokeObjectURL(blobUrl);
        }, 5000); // Wait 5 seconds before cleaning up
      }, 100);
    } catch (error) {
      console.error('Error downloading snapshot:', error);
      showStatusMessage('Failed to download: ' + error.message, 3000);
    }
  });
  
  footer.appendChild(downloadButton);
  
  // Assemble the preview
  previewContainer.appendChild(header);
  previewContainer.appendChild(imageContainer);
  previewContainer.appendChild(footer);
  overlay.appendChild(previewContainer);
  
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
