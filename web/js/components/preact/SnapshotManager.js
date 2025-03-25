/**
 * Snapshot functionality for LiveView
 * This version focuses exclusively on making direct download work
 */
import { showStatusMessage, showSnapshotPreview } from './UI.js';

/**
 * Take a snapshot of a stream
 * @param {string} streamId - ID of the stream
 */
export function takeSnapshot(streamId) {
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
    
    // Find and enhance the download button
    setTimeout(() => enhanceDownloadButton(), 100);
    
    // Show success message
    showStatusMessage('Snapshot taken successfully');
  } catch (error) {
    console.error('Error creating snapshot:', error);
    showStatusMessage('Failed to create snapshot: ' + error.message);
  }
}

/**
 * Find and enhance the download button in the preview
 */
function enhanceDownloadButton() {
  const overlay = document.getElementById('snapshot-preview-overlay');
  if (!overlay) {
    // Try again in 100ms if not found
    setTimeout(enhanceDownloadButton, 100);
    return;
  }
  
  // Find download button
  const buttons = overlay.querySelectorAll('button');
  let downloadButton = null;
  
  for (const button of buttons) {
    if (button.textContent.includes('Download')) {
      downloadButton = button;
      break;
    }
  }
  
  if (!downloadButton) {
    // Try again if button not found
    setTimeout(enhanceDownloadButton, 100);
    return;
  }
  
  // Replace the download button with our enhanced version
  const newButton = downloadButton.cloneNode(true);
  downloadButton.parentNode.replaceChild(newButton, downloadButton);
  
  // Add our download handler using Blob+URL.createObjectURL approach
  newButton.addEventListener('click', () => {
    // Check if we have a canvas available
    if (!window.__snapshotCanvas) {
      console.error('No snapshot canvas available');
      showStatusMessage('Download failed: No snapshot data available');
      return;
    }
    
    // Convert canvas to blob and download
    downloadCanvasAsJpeg(window.__snapshotCanvas, window.__snapshotFileName || 'snapshot.jpg');
  });
  
  console.log('Download button enhanced');
}

/**
 * Convert canvas to blob and download
 * @param {HTMLCanvasElement} canvas - The canvas to download
 * @param {string} fileName - The filename to use
 */
function downloadCanvasAsJpeg(canvas, fileName) {
  // Use canvas.toBlob for better browser compatibility
  canvas.toBlob(function(blob) {
    if (!blob) {
      console.error('Failed to create blob from canvas');
      showStatusMessage('Download failed: Unable to create image data');
      return;
    }
    
    console.log('Created blob:', blob.size, 'bytes');
    
    // Create object URL from blob
    const blobUrl = URL.createObjectURL(blob);
    console.log('Created blob URL:', blobUrl);
    
    // Create download link
    const link = document.createElement('a');
    link.href = blobUrl;
    link.download = fileName;
    
    // Style the link to make it more visible
    link.style.position = 'absolute';
    link.style.top = '0';
    link.style.left = '0';
    link.style.opacity = '0.01'; // Almost invisible but still technically visible
    
    // Add the link to the document
    document.body.appendChild(link);
    console.log('Added download link to document');
    
    // Trigger click after a short delay
    setTimeout(() => {
      console.log('Clicking download link');
      link.click();
      
      // Keep the link in the document for a while
      setTimeout(() => {
        // Clean up
        if (document.body.contains(link)) {
          document.body.removeChild(link);
        }
        URL.revokeObjectURL(blobUrl);
        console.log('Cleaned up download resources');
      }, 10000); // Keep resources around for 10 seconds
    }, 100);
    
    showStatusMessage('Download started');
  }, 'image/jpeg', 0.95); // High quality JPEG
}