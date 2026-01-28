/**
 * UI utility components for LightNVR
 * JSX version of UI components
 */

import { h, createContext } from 'preact';
import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { createPortal } from 'preact/compat';
import { showStatusMessage } from './ToastContainer.jsx';
import { useSnapshotManager } from './SnapshotManager.jsx';

// Create contexts for modals
export const ModalContext = createContext({
  showVideoModal: () => {},
  showSnapshotPreview: () => {}
});

/**
 * DeleteConfirmationModal component for Preact
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

  return (
    <div
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
      onKeyDown={handleKeyDown}
    >
      <div class="bg-card text-card-foreground rounded-lg shadow-xl p-6 max-w-md mx-auto">
        <div class="mb-4">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{title}</h3>
        </div>

        <p class="text-gray-600 dark:text-gray-300 mb-6">{message}</p>

        <div class="flex justify-end space-x-3">
          <button
            class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
            onClick={onConfirm}
          >
            Delete
          </button>
        </div>
      </div>
    </div>
  );
}

/**
 * SnapshotPreviewModal component
 * @param {Object} props - Component props
 * @param {boolean} props.isOpen - Whether the modal is open
 * @param {Function} props.onClose - Function to call when the modal is closed
 * @param {string} props.imageData - Image data URL
 * @param {string} props.streamName - Stream name
 * @returns {JSX.Element} SnapshotPreviewModal component
 */
export function SnapshotPreviewModal({ isOpen, onClose, imageData, streamName, onDownload }) {
  const modalRef = useRef(null);

  useEffect(() => {
    // Handle escape key
    const handleKeyDown = (e) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);

    // Animate in
    if (isOpen && modalRef.current) {
      setTimeout(() => {
        const modalContent = modalRef.current.querySelector('.modal-content');
        if (modalContent) {
          modalContent.classList.remove('scale-95', 'opacity-0');
          modalContent.classList.add('scale-100', 'opacity-100');
        }
      }, 10);
    }

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [isOpen, onClose]);

  // Handle background click
  const handleBackgroundClick = (e) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  if (!isOpen) return null;

  return createPortal(
    <div
      ref={modalRef}
      id="snapshot-preview-modal"
      className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
    >
      <div className="modal-content bg-card text-card-foreground rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out scale-95 opacity-0" style={{ width: '90%' }}>
        <div className="flex justify-between items-center p-4 border-b border-border">
          <h3 id="snapshot-preview-title" className="text-lg font-semibold text-gray-900 dark:text-white">
            {streamName ? `Snapshot: ${streamName}` : 'Snapshot'}
          </h3>
          <button
            className="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            onClick={onClose}
          >
            ✕
          </button>
        </div>
        <div className="p-4 overflow-auto flex-grow">
          <img
            id="snapshot-preview-image"
            className="max-w-full max-h-[70vh] mx-auto"
            src={imageData}
            alt="Snapshot"
          />
        </div>
        <div className="p-4 border-t border-border flex justify-end space-x-2">
          <button
            id="snapshot-download-btn"
            className="btn-primary"
            onClick={onDownload}
          >
            Download
          </button>
          <button
            id="snapshot-close-btn"
            className="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
            onClick={onClose}
          >
            Close
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

/**
 * VideoModal component
 * @param {Object} props - Component props
 * @param {boolean} props.isOpen - Whether the modal is open
 * @param {Function} props.onClose - Function to call when the modal is closed
 * @param {string} props.videoUrl - URL of the video to display
 * @param {string} props.title - Title for the video
 * @param {string} props.downloadUrl - Optional URL for downloading the video
 * @returns {JSX.Element} VideoModal component
 */
export function VideoModal({ isOpen, onClose, videoUrl, title, downloadUrl }) {
  console.log('VideoModal rendered with props:', { isOpen, videoUrl, title });

  const [detectionOverlayEnabled, setDetectionOverlayEnabled] = useState(false);
  const [timeWindow, setTimeWindow] = useState(2);
  const [detections, setDetections] = useState([]);
  const [recordingData, setRecordingData] = useState(null);
  const [detectionStatus, setDetectionStatus] = useState('No detections loaded');
  const [currentSpeed, setCurrentSpeed] = useState(1.0);

  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const modalRef = useRef(null);

  // Handle escape key and cleanup
  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    if (isOpen) {
      console.log('VideoModal opened, setting up event listeners');
      document.addEventListener('keydown', handleKeyDown);

      // Reset video element when modal opens
      if (videoRef.current) {
        videoRef.current.load();
      }
    }

    return () => {
      console.log('VideoModal cleanup');
      document.removeEventListener('keydown', handleKeyDown);

      // Cleanup video element when component unmounts or modal closes
      if (videoRef.current) {
        // Pause and reset video src to stop any ongoing requests
        try {
          videoRef.current.pause();
          videoRef.current.removeAttribute('src');
          videoRef.current.load();
        } catch (e) {
          console.error('Error cleaning up video element:', e);
        }
      }
    };
  }, [isOpen, onClose]);

  // Fetch recording data and detections
  useEffect(() => {
    if (!isOpen || !videoUrl) return;

    // Extract recording ID from URL
    const recordingIdMatch = videoUrl.match(/\/play\/(\d+)/);
    if (!recordingIdMatch || !recordingIdMatch[1]) return;

    const recordingId = parseInt(recordingIdMatch[1], 10);

    // Fetch recording data
    const fetchRecordingData = async () => {
      try {
        const response = await fetch(`/api/recordings/${recordingId}`);
        if (!response.ok) throw new Error('Failed to fetch recording data');

        const data = await response.json();
        setRecordingData(data);

        // If we have recording data, fetch detections
        if (data && data.stream && data.start_time && data.end_time) {
          const startTime = Math.floor(new Date(data.start_time).getTime() / 1000);
          const endTime = Math.floor(new Date(data.end_time).getTime() / 1000);

          const detectionsResponse = await fetch(
            `/api/detection/results/${data.stream}?start=${startTime}&end=${endTime}`
          );

          if (!detectionsResponse.ok) throw new Error('Failed to fetch detections');

          const detectionsData = await detectionsResponse.json();
          const fetchedDetections = detectionsData.detections || [];
          setDetections(fetchedDetections);

          if (fetchedDetections.length > 0) {
            setDetectionStatus(`${fetchedDetections.length} detection${fetchedDetections.length !== 1 ? 's' : ''} available`);
          } else {
            setDetectionStatus('No detections found for this recording');
          }
        }
      } catch (error) {
        console.error('Error fetching data:', error);
        setDetectionStatus('Error loading detections');
      }
    };

    fetchRecordingData();
  }, [isOpen, videoUrl]);

  // Draw detections on canvas
  const drawDetections = useCallback(() => {
    if (!detectionOverlayEnabled || !videoRef.current || !canvasRef.current || detections.length === 0) {
      return;
    }

    const video = videoRef.current;
    const canvas = canvasRef.current;

    // Get video dimensions
    const videoWidth = video.videoWidth;
    const videoHeight = video.videoHeight;

    if (videoWidth === 0 || videoHeight === 0) {
      // Video dimensions not available yet, try again later
      requestAnimationFrame(drawDetections);
      return;
    }

    // Set canvas dimensions to match video
    canvas.width = videoWidth;
    canvas.height = videoHeight;

    // Get canvas context
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Get current video time in seconds
    const currentTime = video.currentTime;

    // Calculate the timestamp for the current video position
    if (!recordingData || !recordingData.start_time) {
      return;
    }

    // Convert recording start time to seconds
    const recordingStartTime = Math.floor(new Date(recordingData.start_time).getTime() / 1000);

    // Calculate current timestamp in the video
    const currentTimestamp = recordingStartTime + Math.floor(currentTime);

    // Filter detections to only show those within the time window
    const visibleDetections = detections.filter(detection => {
      if (!detection.timestamp) return false;

      // Check if detection is within the time window
      return Math.abs(detection.timestamp - currentTimestamp) <= timeWindow;
    });

    // Update status with count of visible detections
    if (visibleDetections.length > 0) {
      setDetectionStatus(`Showing ${visibleDetections.length} detection${visibleDetections.length !== 1 ? 's' : ''} at current time`);
    } else {
      setDetectionStatus(`No detections at current time (${detections.length} total)`);
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
  }, [detectionOverlayEnabled, detections, recordingData, timeWindow]);

  // Set up video event listeners
  useEffect(() => {
    if (!isOpen || !videoRef.current) return;

    const video = videoRef.current;

    const handlePlay = () => {
      if (detectionOverlayEnabled) {
        drawDetections();
      }
    };

    const handleSeeked = () => {
      if (detectionOverlayEnabled) {
        drawDetections();
      }
    };

    const handleTimeUpdate = () => {
      if (detectionOverlayEnabled) {
        // Don't redraw on every timeupdate as it's too frequent
        // Instead, redraw every 0.5 seconds
        const currentTime = Math.floor(video.currentTime * 2) / 2;
        if (video.lastDrawnTime !== currentTime) {
          video.lastDrawnTime = currentTime;
          drawDetections();
        }
      }
    };

    video.addEventListener('play', handlePlay);
    video.addEventListener('seeked', handleSeeked);
    video.addEventListener('timeupdate', handleTimeUpdate);

    return () => {
      video.removeEventListener('play', handlePlay);
      video.removeEventListener('seeked', handleSeeked);
      video.removeEventListener('timeupdate', handleTimeUpdate);
    };
  }, [isOpen, detectionOverlayEnabled, drawDetections]);

  // Handle video URL changes
  useEffect(() => {
    if (isOpen && videoUrl && videoRef.current) {
      console.log('Video URL changed, loading new video');
      videoRef.current.load();
    }
  }, [isOpen, videoUrl]);

  // Update detection overlay when enabled/disabled
  useEffect(() => {
    if (detectionOverlayEnabled) {
      if (canvasRef.current) {
        canvasRef.current.style.display = 'block';
      }
      drawDetections();
    } else {
      if (canvasRef.current) {
        canvasRef.current.style.display = 'none';
      }
    }
  }, [detectionOverlayEnabled, drawDetections]);

  // Handle background click
  const handleBackgroundClick = (e) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  // Handle speed change
  const handleSpeedChange = (speed) => {
    if (videoRef.current) {
      videoRef.current.playbackRate = speed;
      setCurrentSpeed(speed);
    }
  };

  // Handle time window change
  const handleTimeWindowChange = (e) => {
    const newTimeWindow = parseInt(e.target.value, 10);
    setTimeWindow(newTimeWindow);
    if (detectionOverlayEnabled) {
      drawDetections();
    }
  };

  if (!isOpen) return null;

  // Add useEffect to handle modal animation and cleanup
  useEffect(() => {
    let animationTimeout;

    if (isOpen && modalRef.current) {
      // Animate in - small delay to ensure DOM is ready
      animationTimeout = setTimeout(() => {
        const modalContent = modalRef.current?.querySelector('.modal-content');
        if (modalContent) {
          modalContent.classList.remove('scale-95', 'opacity-0');
          modalContent.classList.add('scale-100', 'opacity-100');
          console.log('Modal animation applied');
        }
      }, 10);
    }

    // Cleanup function
    return () => {
      if (animationTimeout) {
        clearTimeout(animationTimeout);
      }
    };
  }, [isOpen]);

  return createPortal(
    <div
      ref={modalRef}
      id="video-preview-modal"
      className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
    >
      <div className={`modal-content bg-card text-card-foreground rounded-lg shadow-xl max-w-4xl max-h-[90vh] flex flex-col transform transition-all duration-300 ease-out ${isOpen ? 'scale-100 opacity-100' : 'scale-95 opacity-0'} w-full md:w-[90%]`}>
        <div className="flex justify-between items-center p-4 border-b border-border">
          <h3 id="video-preview-title" className="text-lg font-semibold text-gray-900 dark:text-white">
            {title || 'Video'}
          </h3>
          <button
            className="close text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            onClick={onClose}
          >
            ✕
          </button>
        </div>

        <div className="p-4 flex-grow">
          <div className="relative">
            <video
              ref={videoRef}
              className="w-full h-auto max-w-full object-contain mx-auto"
              controls
              autoPlay
              key={videoUrl} /* Add key to force re-render when URL changes */
              onError={(e) => {
                console.error('Video error:', e);
                showStatusMessage('Error loading video. Please try again.', 'error');
              }}
              onLoadStart={() => console.log('Video load started')}
              onLoadedData={() => console.log('Video data loaded')}
            >
              {/* Use source element instead of src attribute for better control */}
              {videoUrl && <source src={videoUrl} type="video/mp4" />}
            </video>
            <canvas
              ref={canvasRef}
              className="absolute top-0 left-0 w-full h-full pointer-events-none"
              style={{ display: 'none' }}
            />
          </div>
        </div>

        <div id="recordings-controls" className="mx-4 mb-4 p-4 border border-green-500 rounded-lg bg-card text-card-foreground shadow-md relative z-10">
          <h3 className="text-lg font-bold text-center mb-4 text-foreground">
            PLAYBACK CONTROLS
          </h3>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-2">
            {/* Speed controls section */}
            <div className="border-b border-border pb-4 md:border-b-0 md:border-r md:pr-4 md:pb-0">
              <h4 className="font-bold text-center mb-3 text-foreground">
                Playback Speed
              </h4>

              <div className="flex flex-wrap justify-center gap-2">
                {[0.25, 0.5, 1.0, 1.5, 2.0, 4.0].map(speed => (
                  <button
                    key={speed}
                    className={`speed-btn px-3 py-2 rounded-full ${
                      speed === currentSpeed
                        ? 'badge-success'
                        : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
                    } text-sm font-medium transition-all focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-50`}
                    data-speed={speed}
                    onClick={() => handleSpeedChange(speed)}
                  >
                    {speed === 1.0 ? '1× (Normal)' : `${speed}×`}
                  </button>
                ))}
              </div>

              <div id="current-speed-indicator" className="mt-3 text-center font-medium text-green-600 dark:text-green-400 text-sm">
                Current Speed: {currentSpeed}× {currentSpeed === 1.0 ? '(Normal)' : ''}
              </div>
            </div>

            {/* Detection overlay section */}
            <div className="pt-4 md:pt-0 md:pl-4">
              <h4 className="font-bold text-center mb-2 text-foreground">
                Detection Overlays
              </h4>

              <div className="flex flex-col items-center gap-2">
                <div className="flex items-center gap-2 mb-2">
                  <input
                    type="checkbox"
                    id="detection-overlay-checkbox"
                    className="w-4 h-4 accent-primary bg-secondary border-border rounded focus:ring-primary focus:ring-2"
                    checked={detectionOverlayEnabled}
                    onChange={(e) => setDetectionOverlayEnabled(e.target.checked)}
                    disabled={detections.length === 0}
                  />
                  <label
                    htmlFor="detection-overlay-checkbox"
                    className="text-sm font-medium text-foreground"
                  >
                    Show Detection Overlays
                  </label>
                </div>

                <div className="flex flex-col w-full mt-2 mb-2">
                  <label
                    htmlFor="detection-sensitivity-slider"
                    className="text-sm font-medium text-foreground mb-1"
                  >
                    Detection Sensitivity
                  </label>

                  <input
                    type="range"
                    id="detection-sensitivity-slider"
                    className="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary"
                    min="1"
                    max="10"
                    step="1"
                    value={timeWindow}
                    onChange={handleTimeWindowChange}
                  />

                  <div id="detection-sensitivity-value" className="text-xs text-muted-foreground text-center mb-1">
                    Time Window: {timeWindow} second{timeWindow !== 1 ? 's' : ''}
                  </div>
                </div>

                <div id="detection-status-indicator" className={`text-center text-sm ${
                  detections.length > 0
                    ? 'font-medium text-green-600 dark:text-green-400'
                    : 'text-muted-foreground'
                }`}>
                  {detectionStatus}
                </div>
              </div>
            </div>
          </div>

          {/* Download button */}
          {downloadUrl && (
            <div className="flex justify-center mt-4 pt-2 border-t border-border">
              <a
                className="px-4 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors flex items-center text-sm"
                href={downloadUrl}
                download={`video-${Date.now()}.mp4`}
              >
                <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
                Download Video
              </a>
            </div>
          )}
        </div>

        <div className="p-2"></div>
      </div>
    </div>,
    document.body
  );
}

/**
 * ModalProvider component
 * Provides modal context and renders modal components
 */
export function ModalProvider({ children }) {
  const [snapshotModal, setSnapshotModal] = useState({
    isOpen: false,
    imageData: '',
    streamName: '',
  });

  const [videoModal, setVideoModal] = useState({
    isOpen: false,
    videoUrl: '',
    title: '',
    downloadUrl: '',
  });

  // Show snapshot preview
  const showSnapshotPreview = useCallback((imageData, streamName) => {
    setSnapshotModal({
      isOpen: true,
      imageData,
      streamName,
    });
  }, []);

  // Show video modal
  const showVideoModal = useCallback((videoUrl, title, downloadUrl) => {
    console.log('ModalProvider.showVideoModal called with:', { videoUrl, title, downloadUrl });

    // First reset the modal state completely
    setVideoModal({
      isOpen: false,
      videoUrl: '',
      title: '',
      downloadUrl: ''
    });

    // Then set the new state after a small delay to ensure clean rendering
    setTimeout(() => {
      setVideoModal({
        isOpen: true,
        videoUrl,
        title,
        downloadUrl,
      });
      console.log('Video modal state updated with new content');
    }, 50);
  }, []);

  // Use the snapshot manager hook for download functionality
  const { downloadSnapshot } = useSnapshotManager();

  // Handle snapshot download
  const handleSnapshotDownload = useCallback(() => {
    try {
      const { imageData, streamName } = snapshotModal;

      if (!imageData) {
        console.error('No image data available for download');
        showStatusMessage('Download failed: No image data available', 'error', 5000);
        return;
      }

      // Use the downloadSnapshot function from the hook if available
      if (downloadSnapshot) {
        downloadSnapshot();
        return;
      }

      // Fallback to the old implementation
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const fileName = `snapshot_${streamName}_${timestamp}.jpg`;

      const link = document.createElement('a');
      link.href = imageData;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();

      // Clean up
      setTimeout(() => {
        document.body.removeChild(link);
      }, 100);

      showStatusMessage('Snapshot downloaded successfully', 'success', 3000);
    } catch (error) {
      console.error('Error in snapshot download process:', error);
      showStatusMessage('Download failed: ' + error.message, 'error', 5000);
    }
  }, [snapshotModal, downloadSnapshot]);

  // Close snapshot modal
  const closeSnapshotModal = useCallback(() => {
    setSnapshotModal(prev => ({ ...prev, isOpen: false }));
  }, []);

  // Close video modal
  const closeVideoModal = useCallback(() => {
    console.log('Closing video modal');

    // First just set isOpen to false to trigger the closing animation
    setVideoModal(prev => ({ ...prev, isOpen: false }));

    // Then completely reset the modal state after animation completes
    setTimeout(() => {
      setVideoModal({
        isOpen: false,
        videoUrl: '',
        title: '',
        downloadUrl: ''
      });
      console.log('Video modal state fully reset');
    }, 300);
  }, []);

  // Create context value
  const contextValue = {
    showSnapshotPreview,
    showVideoModal,
  };

  return (
    <ModalContext.Provider value={contextValue}>
      {children}

      {/* Render modals */}
      <SnapshotPreviewModal
        isOpen={snapshotModal.isOpen}
        onClose={closeSnapshotModal}
        imageData={snapshotModal.imageData}
        streamName={snapshotModal.streamName}
        onDownload={handleSnapshotDownload}
      />

      <VideoModal
        isOpen={videoModal.isOpen}
        onClose={closeVideoModal}
        videoUrl={videoModal.videoUrl}
        title={videoModal.title}
        downloadUrl={videoModal.downloadUrl}
      />
    </ModalContext.Provider>
  );
}

// Export functions for backward compatibility
export function setupModals() {
  console.warn('setupModals() is deprecated. Use <ModalProvider> component instead.');
  // This function is kept for backward compatibility
}

export function addModalStyles() {
  console.warn('addModalStyles() is deprecated. Modal styles are now included in components.css');
  // This function is kept for backward compatibility
}

export function showVideoModal(videoUrl, title, downloadUrl) {
  console.warn('Direct showVideoModal() is deprecated. Use ModalContext.showVideoModal instead.');

  // Get the modal context from the global variable if available
  if (window.__modalContext && window.__modalContext.showVideoModal) {
    console.log('Using modal context to show video modal');
    window.__modalContext.showVideoModal(videoUrl, title, downloadUrl);
    return;
  }

  console.log('Falling back to direct modal rendering');

  // Fallback to creating a modal directly
  const modalRoot = document.getElementById('modal-root');
  if (!modalRoot) {
    console.log('Creating modal root element');
    const root = document.createElement('div');
    root.id = 'modal-root';
    document.body.appendChild(root);
  }

  // Create a temporary div to render the modal
  const modalContainer = document.createElement('div');
  modalContainer.id = 'temp-modal-container';
  document.body.appendChild(modalContainer);

  // Import preact to render the modal
  console.log('Dynamically importing preact to render modal');
  import('preact').then(({ render, h }) => {
    console.log('Rendering VideoModal component');
    render(
      h(VideoModal, {
        isOpen: true,
        onClose: () => {
          console.log('Closing modal');
          render(null, modalContainer);
          document.body.removeChild(modalContainer);
        },
        videoUrl,
        title,
        downloadUrl
      }),
      modalContainer
    );
  }).catch(err => {
    console.error('Error rendering video modal:', err);
    showStatusMessage('Error showing video modal: ' + err.message, 'error');
  });
}

export function showSnapshotPreview(imageData, streamName) {
  console.warn('Direct showSnapshotPreview() is deprecated. Use ModalContext.showSnapshotPreview instead.');

  // Get the modal context from the global variable if available
  if (window.__modalContext && window.__modalContext.showSnapshotPreview) {
    window.__modalContext.showSnapshotPreview(imageData, streamName);
    return;
  }

  // Fallback to creating a modal directly
  const modalRoot = document.getElementById('modal-root');
  if (!modalRoot) {
    const root = document.createElement('div');
    root.id = 'modal-root';
    document.body.appendChild(root);
  }

  // Create a temporary div to render the modal
  const modalContainer = document.createElement('div');
  modalContainer.id = 'temp-snapshot-container';
  document.body.appendChild(modalContainer);

  // Import preact to render the modal
  import('preact').then(({ render, h }) => {
    const handleDownload = () => {
      try {
        // Import the snapshot manager hook
        import('./SnapshotManager.jsx').then(({ useSnapshotManager }) => {
          const { downloadSnapshot } = useSnapshotManager();
          if (downloadSnapshot) {
            downloadSnapshot();
          } else {
            // Fallback to the old implementation
            const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
            const fileName = `snapshot_${streamName}_${timestamp}.jpg`;

            const link = document.createElement('a');
            link.href = imageData;
            link.download = fileName;
            document.body.appendChild(link);
            link.click();

            // Clean up
            setTimeout(() => {
              document.body.removeChild(link);
            }, 100);

            showStatusMessage('Snapshot downloaded successfully', 'success', 3000);
          }
        }).catch(error => {
          console.error('Error importing SnapshotManager:', error);
          // Fallback to the old implementation
          const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
          const fileName = `snapshot_${streamName}_${timestamp}.jpg`;

          const link = document.createElement('a');
          link.href = imageData;
          link.download = fileName;
          document.body.appendChild(link);
          link.click();

          // Clean up
          setTimeout(() => {
            document.body.removeChild(link);
          }, 100);

          showStatusMessage('Snapshot downloaded successfully', 'success', 3000);
        });
      } catch (error) {
        console.error('Error downloading snapshot:', error);
        showStatusMessage('Download failed: ' + error.message, 'error', 5000);
      }
    };

    render(
      h(SnapshotPreviewModal, {
        isOpen: true,
        onClose: () => {
          render(null, modalContainer);
          document.body.removeChild(modalContainer);
        },
        imageData,
        streamName,
        onDownload: handleDownload
      }),
      modalContainer
    );
  }).catch(err => {
    console.error('Error rendering snapshot modal:', err);
    showStatusMessage('Error showing snapshot preview: ' + err.message, 'error');
  });
}
