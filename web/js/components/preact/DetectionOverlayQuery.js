/**
 * Detection overlay functionality for LiveView using preact-query
 */

import { useQuery } from '../../query-client.js';

/**
 * Custom hook to fetch detection results
 * @param {string} streamName - Name of the stream
 * @param {boolean} enabled - Whether to enable the query
 * @param {number} pollingInterval - Polling interval in milliseconds
 * @returns {Object} Query result
 */
export function useDetectionResults(streamName, enabled = true, pollingInterval = 1000) {
  return useQuery(
    ['detection-results', streamName],
    `/api/detection/results/${encodeURIComponent(streamName)}`,
    {
      timeout: 5000, // 5 second timeout
      retries: 1,     // Retry once
      retryDelay: 1000 // 1 second between retries
    },
    {
      enabled: !!streamName && enabled,
      refetchInterval: pollingInterval,
      refetchIntervalInBackground: false,
      onError: (error) => {
        console.error(`Error fetching detection results for ${streamName}:`, error);
      }
    }
  );
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
  
  // Set canvas dimensions to match the displayed video element
  canvas.width = videoElement.clientWidth;
  canvas.height = videoElement.clientHeight;
  
  // Clear previous drawings
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  
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
}

/**
 * DetectionOverlay component
 * @param {Object} props Component props
 * @param {string} props.streamName Stream name
 * @param {HTMLVideoElement} props.videoElement Video element
 * @param {HTMLCanvasElement} props.canvasOverlay Canvas element
 */
export function DetectionOverlay({ streamName, videoElement, canvasOverlay }) {
  // Use the detection results hook
  const { 
    data: detectionData,
    isLoading,
    error
  } = useDetectionResults(streamName, true, 1000);
  
  // Draw detections when data is received
  if (detectionData && detectionData.detections && videoElement && canvasOverlay) {
    drawDetections(canvasOverlay, videoElement, detectionData.detections);
  }
  
  // Clear canvas on error
  if (error && canvasOverlay) {
    const ctx = canvasOverlay.getContext('2d');
    if (ctx) {
      ctx.clearRect(0, 0, canvasOverlay.width, canvasOverlay.height);
    }
  }
  
  return null; // This is a non-rendering component
}
