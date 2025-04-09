/**
 * Stream grid functionality for LiveView using preact-query
 */

import { useQuery } from '../../query-client.js';
import { initializeVideoPlayer } from './VideoPlayer.js';
import { takeSnapshot } from './SnapshotManager.js';
import { toggleStreamFullscreen } from './FullscreenManager.js';
import { showStatusMessage } from './UI.js';

/**
 * Custom hook to fetch streams list
 * @returns {Object} Query result
 */
export function useStreams() {
  return useQuery(
    'streams',
    '/api/streams',
    {
      timeout: 15000, // 15 second timeout
      retries: 2,     // Retry twice
      retryDelay: 1000 // 1 second between retries
    }
  );
}

/**
 * Custom hook to fetch stream details
 * @param {string} streamId - Stream ID or name
 * @param {boolean} enabled - Whether to enable the query
 * @returns {Object} Query result
 */
export function useStreamDetails(streamId, enabled = true) {
  return useQuery(
    ['stream-details', streamId],
    `/api/streams/${encodeURIComponent(streamId)}`,
    {
      timeout: 10000, // 10 second timeout
      retries: 1,     // Retry once
      retryDelay: 1000 // 1 second between retries
    },
    {
      enabled: !!streamId && enabled,
      onError: (error) => {
        console.error(`Error loading details for stream ${streamId}:`, error);
      }
    }
  );
}

/**
 * Update video grid based on layout, streams, and pagination
 * @param {HTMLElement} videoGridRef - Reference to video grid element
 * @param {Array} streams - Array of stream objects
 * @param {string} layout - Layout type ('1', '4', '9', '16')
 * @param {string} selectedStream - Selected stream name for single view
 * @param {Object} videoPlayers - Reference to video player instances
 * @param {Object} detectionIntervals - Reference to detection intervals
 * @param {number} currentPage - Current page number (0-based)
 */
export function updateVideoGrid(
  videoGridRef,
  streams,
  layout,
  selectedStream,
  videoPlayers,
  detectionIntervals,
  currentPage = 0
) {
  if (!videoGridRef) return;

  // Clear existing content except placeholder
  const placeholder = videoGridRef.querySelector('.placeholder');
  videoGridRef.innerHTML = '';

  // If placeholder exists and no streams, add it back
  if (placeholder && streams.length === 0) {
    videoGridRef.appendChild(placeholder);
    return;
  }

  // Filter streams based on layout and selected stream
  let streamsToShow = streams;
  if (layout === '1' && selectedStream) {
    streamsToShow = streams.filter(stream => stream.name === selectedStream);
  } else {
    // Apply pagination
    const maxStreams = parseInt(layout);
    const totalPages = Math.ceil(streams.length / maxStreams);

    // Ensure current page is valid
    if (currentPage >= totalPages) {
      console.warn(`Current page ${currentPage} is invalid, max page is ${totalPages - 1}`);
      currentPage = Math.max(0, totalPages - 1);
    }

    // Get streams for current page
    const startIdx = currentPage * maxStreams;
    const endIdx = Math.min(startIdx + maxStreams, streams.length);
    streamsToShow = streams.slice(startIdx, endIdx);
  }

  // Get the names of streams that should be shown
  const streamsToShowNames = streamsToShow.map(stream => stream.name);

  // Clean up video players for streams that are no longer visible
  Object.keys(videoPlayers).forEach(streamName => {
    if (!streamsToShowNames.includes(streamName)) {
      console.log(`Cleaning up video player for stream ${streamName} as it's not on the current page`);

      // Stop the video player
      const videoPlayer = videoPlayers[streamName];
      if (videoPlayer && videoPlayer.hls) {
        videoPlayer.hls.destroy();
        delete videoPlayers[streamName];
      }

      // Clean up detection intervals
      if (detectionIntervals[streamName]) {
        clearInterval(detectionIntervals[streamName]);
        delete detectionIntervals[streamName];
      }
    }
  });

  // Add video elements for each stream
  streamsToShow.forEach(stream => {
    // Ensure we have an ID for the stream (use name as fallback if needed)
    const streamId = stream.id || stream.name;

    // Create video cell
    const videoCell = document.createElement('div');
    videoCell.className = 'video-cell';
    videoCell.dataset.streamName = stream.name;
    videoCell.style.position = 'relative'; // Create stacking context

    // Create video element
    const videoElement = document.createElement('video');
    videoElement.id = `video-${stream.name.replace(/\s+/g, '-')}`;
    videoElement.className = 'video-element';
    videoElement.playsInline = true;
    videoElement.autoplay = true;
    videoElement.muted = true;
    videoElement.style.pointerEvents = 'none'; // Allow clicks to pass through to controls

    // Create loading indicator
    const loadingIndicator = document.createElement('div');
    loadingIndicator.className = 'loading-indicator';
    loadingIndicator.innerHTML = `
      <div class="spinner"></div>
      <p>Connecting...</p>
    `;
    loadingIndicator.style.position = 'absolute';
    loadingIndicator.style.top = '0';
    loadingIndicator.style.left = '0';
    loadingIndicator.style.width = '100%';
    loadingIndicator.style.height = '100%';
    loadingIndicator.style.display = 'flex';
    loadingIndicator.style.flexDirection = 'column';
    loadingIndicator.style.justifyContent = 'center';
    loadingIndicator.style.alignItems = 'center';
    loadingIndicator.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
    loadingIndicator.style.color = 'white';
    loadingIndicator.style.zIndex = '20'; // Above video but below controls

    // Create stream name overlay
    const streamNameOverlay = document.createElement('div');
    streamNameOverlay.className = 'stream-name-overlay';
    streamNameOverlay.textContent = stream.name;
    streamNameOverlay.style.position = 'absolute';
    streamNameOverlay.style.top = '10px';
    streamNameOverlay.style.left = '10px';
    streamNameOverlay.style.padding = '5px 10px';
    streamNameOverlay.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
    streamNameOverlay.style.color = 'white';
    streamNameOverlay.style.borderRadius = '4px';
    streamNameOverlay.style.fontSize = '14px';
    streamNameOverlay.style.zIndex = '15'; // Above video but below controls

    // Create stream controls
    const streamControls = document.createElement('div');
    streamControls.className = 'stream-controls';
    streamControls.innerHTML = `
      <button class="snapshot-btn" title="Take Snapshot" data-id="${streamId}" data-name="${stream.name}" style="background-color: rgba(0, 0, 0, 0.5); border: none; color: white; width: 36px; height: 36px; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path><circle cx="12" cy="13" r="4"></circle></svg>
      </button>
      <button class="fullscreen-btn" title="Toggle Fullscreen" data-id="${streamId}" data-name="${stream.name}" style="background-color: rgba(0, 0, 0, 0.5); border: none; color: white; width: 36px; height: 36px; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer;">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
      </button>
    `;
    streamControls.style.position = 'absolute';
    streamControls.style.bottom = '10px';
    streamControls.style.right = '10px';
    streamControls.style.display = 'flex';
    streamControls.style.gap = '10px';
    streamControls.style.zIndex = '30'; // Above everything else

    // Add canvas for detection overlay
    const canvasOverlay = document.createElement('canvas');
    canvasOverlay.id = `canvas-${stream.name.replace(/\s+/g, '-')}`;
    canvasOverlay.className = 'detection-overlay';
    canvasOverlay.style.position = 'absolute';
    canvasOverlay.style.top = '0';
    canvasOverlay.style.left = '0';
    canvasOverlay.style.width = '100%';
    canvasOverlay.style.height = '100%';
    canvasOverlay.style.pointerEvents = 'none'; // Allow clicks to pass through
    canvasOverlay.style.zIndex = '5'; // Above video but below controls

    // Assemble the video cell
    videoCell.appendChild(videoElement);
    videoCell.appendChild(loadingIndicator);
    videoCell.appendChild(streamNameOverlay);
    videoCell.appendChild(streamControls);
    videoCell.appendChild(canvasOverlay);

    videoGridRef.appendChild(videoCell);

    // Initialize video player
    initializeVideoPlayer(stream, videoPlayers, detectionIntervals);

    // Add event listeners for buttons
    const snapshotBtns = videoCell.querySelectorAll('.snapshot-btn');
    snapshotBtns.forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        takeSnapshot(streamId);
      });
    });

    const fullscreenBtns = videoCell.querySelectorAll('.fullscreen-btn');
    fullscreenBtns.forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        toggleStreamFullscreen(stream.name);
      });
    });

    // Add click event to video cell for play/pause
    videoCell.addEventListener('click', (e) => {
      // Only handle clicks directly on the video cell, not on buttons
      if (e.target === videoCell || e.target === videoElement) {
        const video = videoCell.querySelector('video');
        if (video) {
          if (video.paused) {
            video.play().catch(err => console.error('Error playing video:', err));
          } else {
            video.pause();
          }
        }
      }
    });
  });
}

/**
 * Filter streams for live view
 * @param {Array} streams - Array of stream objects with details
 * @returns {Array} Filtered streams
 */
export function filterStreamsForLiveView(streams) {
  if (!streams || !Array.isArray(streams)) return [];

  // Filter out streams that are soft deleted, inactive, or not configured for HLS
  return streams.filter(stream => {
    // Filter out soft deleted streams
    if (stream.is_deleted) {
      console.log(`Stream ${stream.name} is soft deleted, filtering out`);
      return false;
    }

    // Filter out inactive streams
    if (!stream.enabled) {
      console.log(`Stream ${stream.name} is inactive, filtering out`);
      return false;
    }

    // Filter out streams not configured for HLS
    if (!stream.streaming_enabled) {
      console.log(`Stream ${stream.name} is not configured for HLS, filtering out`);
      return false;
    }

    return true;
  });
}
