/**
 * Snapshot functionality for LiveView
 * React component for managing snapshots
 */
import { h } from 'preact';
import { useState, useCallback, useEffect } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { showSnapshotPreview } from './UI.jsx';

/**
 * Custom hook for snapshot functionality
 * @returns {Object} Snapshot functions and state
 */
export function useSnapshotManager() {
  const [snapshotData, setSnapshotData] = useState({
    canvas: null,
    fileName: null
  });

  /**
   * Take a snapshot of a stream
   * @param {string} streamId - ID of the stream
   * @param {Event} event - Optional click event
   */
  const takeSnapshot = useCallback((streamId, event) => {
    console.log(`Taking snapshot of stream with ID: ${streamId}`);

    // Find the stream by ID or name
    const streamElement = document.querySelector(`.snapshot-btn[data-id="${streamId}"]`);
    let streamName;

    if (streamElement) {
      // Get the stream name from the data attribute
      streamName = streamElement.getAttribute('data-name');
    } else if (event) {
      // If we can't find by data-id, try to find from the event target
      const clickedButton = event.currentTarget || event.target;
      const videoCell = clickedButton ? clickedButton.closest('.video-cell') : null;

      if (videoCell) {
        streamName = videoCell.dataset.streamName;
      }
    }

    if (!streamName) {
      console.error('Stream name not found for ID:', streamId);
      showStatusMessage('Cannot take snapshot: Stream not identified', 'error');
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
      // Generate a filename
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
      const fileName = `snapshot-${streamName.replace(/\s+/g, '-')}-${timestamp}.jpg`;

      // Store the canvas and filename in state
      setSnapshotData({
        canvas,
        fileName
      });

      // Show the standard preview
      showSnapshotPreview(canvas.toDataURL('image/jpeg', 0.95), `Snapshot: ${streamName}`);

      // Show success message
      showStatusMessage('Snapshot taken successfully');
    } catch (error) {
      console.error('Error creating snapshot:', error);
      showStatusMessage('Failed to create snapshot: ' + error.message);
    }
  }, []);

  /**
   * Download the snapshot as JPEG
   */
  const downloadSnapshot = useCallback(() => {
    const { canvas, fileName } = snapshotData;

    if (!canvas) {
      console.error('No snapshot canvas available');
      showStatusMessage('Download failed: No snapshot data available');
      return;
    }

    // Convert canvas to blob and download
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
      link.download = fileName || 'snapshot.jpg';

      // Add the link to the document
      document.body.appendChild(link);
      console.log('Added download link to document');

      // Trigger click
      link.click();

      // Clean up
      setTimeout(() => {
        if (document.body.contains(link)) {
          document.body.removeChild(link);
        }
        URL.revokeObjectURL(blobUrl);
        console.log('Cleaned up download resources');
      }, 1000);

      showStatusMessage('Download started');
    }, 'image/jpeg', 0.95); // High quality JPEG
  }, [snapshotData]);

  return {
    takeSnapshot,
    downloadSnapshot,
    hasSnapshot: !!snapshotData.canvas
  };
}

/**
 * SnapshotManager component
 * @returns {JSX.Element} SnapshotManager component
 */
export function SnapshotManager() {
  const { takeSnapshot, downloadSnapshot } = useSnapshotManager();

  // Register the takeSnapshot function globally for backward compatibility
  useEffect(() => {
    window.takeSnapshot = takeSnapshot;

    return () => {
      // Clean up global function when component unmounts
      delete window.takeSnapshot;
    };
  }, [takeSnapshot]);

  // This component doesn't render anything visible
  return null;
}

/**
 * SnapshotButton component
 * @param {Object} props Component props
 * @param {string} props.streamId Stream ID
 * @param {string} props.streamName Stream name
 * @param {Function} props.onSnapshot Optional custom snapshot handler
 * @returns {JSX.Element} SnapshotButton component
 */
export function SnapshotButton({ streamId, streamName, onSnapshot }) {
  const { takeSnapshot } = useSnapshotManager();

  const handleClick = (event) => {
    event.preventDefault();
    event.stopPropagation();

    // Use custom snapshot handler if provided, otherwise use default
    if (typeof onSnapshot === 'function') {
      onSnapshot(event);
    } else {
      takeSnapshot(streamId, event);
    }
  };

  return (
    <button
      className="snapshot-btn"
      title="Take Snapshot"
      data-id={streamId}
      data-name={streamName}
      onClick={handleClick}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="white"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path>
        <circle cx="12" cy="13" r="4"></circle>
      </svg>
    </button>
  );
}

/**
 * DownloadButton component
 * @param {Object} props Component props
 * @returns {JSX.Element} DownloadButton component
 */
export function DownloadButton() {
  const { downloadSnapshot, hasSnapshot } = useSnapshotManager();

  if (!hasSnapshot) return null;

  return (
    <button
      className="download-btn px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
      onClick={downloadSnapshot}
    >
      Download
    </button>
  );
}
