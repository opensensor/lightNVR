/**
 * UI utility functions for LiveView
 */

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
    // Create a download link
    const link = document.createElement('a');
    link.href = imageUrl;
    link.download = `snapshot-${Date.now()}.jpg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
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
