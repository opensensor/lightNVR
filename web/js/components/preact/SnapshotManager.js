/**
 * Snapshot functionality for LiveView
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
}
