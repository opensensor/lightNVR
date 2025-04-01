/**
 * LightNVR Web Interface Video Detection
 * Contains functionality for object detection overlays
 */

/**
 * Start polling for detection results and draw bounding boxes
 */
function startDetectionPolling(streamName, canvasOverlay, videoElement) {
    // Store the polling interval ID on the canvas element for cleanup
    if (canvasOverlay.detectionInterval) {
        clearInterval(canvasOverlay.detectionInterval);
    }
    
    // Function to draw bounding boxes
    function drawDetectionBoxes(detections) {
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
    }
    
    // Use a more conservative polling interval (1000ms instead of 500ms)
    // and implement exponential backoff on errors
    let errorCount = 0;
    let currentInterval = 1000; // Start with 1 second
    
    // Poll for detection results
    canvasOverlay.detectionInterval = setInterval(() => {
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
                    clearInterval(canvasOverlay.detectionInterval);
                    currentInterval = Math.min(5000, currentInterval * 2); // Max 5 seconds
                    console.log(`Reducing detection polling frequency to ${currentInterval}ms due to errors`);
                    
                    canvasOverlay.detectionInterval = setInterval(arguments.callee, currentInterval);
                }
            });
    }, currentInterval);
}

// Export functions
window.startDetectionPolling = startDetectionPolling;
