/**
 * Stream grid functionality for LiveView
 */

import { showLoading, hideLoading } from './utils.js';
import { initializeVideoPlayer } from './VideoPlayer.js';
import { takeSnapshot } from './SnapshotManager.js';
import { toggleStreamFullscreen } from './FullscreenManager.js';
import { showStatusMessage } from './UI.js';

/**
 * Update video grid based on layout and streams
 * @param {HTMLElement} videoGridRef - Reference to video grid element
 * @param {Array} streams - Array of stream objects
 * @param {string} layout - Layout type ('1', '4', '9', '16')
 * @param {string} selectedStream - Selected stream name for single view
 * @param {Object} videoPlayers - Reference to video player instances
 * @param {Object} detectionIntervals - Reference to detection intervals
 */
export function updateVideoGrid(
  videoGridRef, 
  streams, 
  layout, 
  selectedStream, 
  videoPlayers, 
  detectionIntervals
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
  }
  
  // Add video elements for each stream
  streamsToShow.forEach(stream => {
    // Ensure we have an ID for the stream (use name as fallback if needed)
    const streamId = stream.id || stream.name;
    
    const videoCell = document.createElement('div');
    videoCell.className = 'video-cell';
    
    videoCell.innerHTML = `
      <video id="video-${stream.name.replace(/\s+/g, '-')}" autoplay muted></video>
      <div class="stream-info">
        <span>${stream.name}</span>
        <span>${stream.width}x${stream.height} · ${stream.fps}fps</span>
        <div class="stream-controls">
          <button class="snapshot-btn" data-id="${streamId}" data-name="${stream.name}">
            <span>📷</span> Snapshot
          </button>
          <button class="fullscreen-btn" data-id="${streamId}" data-name="${stream.name}">
            <span>⛶</span> Fullscreen
          </button>
        </div>
      </div>
      <div class="loading-indicator">
        <div class="loading-spinner"></div>
        <span>Connecting...</span>
      </div>
    `;
    
    videoGridRef.appendChild(videoCell);
    
    // Initialize video player
    initializeVideoPlayer(stream, videoPlayers, detectionIntervals);
    
    // Add event listeners for buttons
    const snapshotBtn = videoCell.querySelector('.snapshot-btn');
    if (snapshotBtn) {
      snapshotBtn.addEventListener('click', () => {
        takeSnapshot(streamId);
      });
    }
    
    const fullscreenBtn = videoCell.querySelector('.fullscreen-btn');
    if (fullscreenBtn) {
      fullscreenBtn.addEventListener('click', () => {
        toggleStreamFullscreen(stream.name);
      });
    }
  });
}

/**
 * Load streams from API
 * @param {Function} setStreams - State setter for streams
 * @param {Function} setSelectedStream - State setter for selected stream
 * @param {HTMLElement} videoGridRef - Reference to video grid element
 * @returns {Promise<Array>} Promise resolving to array of streams
 */
export async function loadStreams(setStreams, setSelectedStream, videoGridRef) {
  try {
    // Show loading state
    if (videoGridRef) {
      showLoading(videoGridRef);
    }
    
    // Fetch streams from API
    const response = await fetch('/api/streams');
    if (!response.ok) {
      throw new Error('Failed to load streams');
    }
    
    const data = await response.json();
    
    // For live view, we need to fetch full details for each stream
    // to get detection settings
    const streamPromises = (data || []).map(stream => {
      return fetch(`/api/streams/${encodeURIComponent(stream.id || stream.name)}`)
        .then(response => {
          if (!response.ok) {
            throw new Error(`Failed to load details for stream ${stream.name}`);
          }
          return response.json();
        })
        .catch(error => {
          console.error(`Error loading details for stream ${stream.name}:`, error);
          // Return the basic stream info if we can't get details
          return stream;
        });
    });
    
    const detailedStreams = await Promise.all(streamPromises);
    console.log('Loaded detailed streams for live view:', detailedStreams);
    
    // Store streams in state
    setStreams(detailedStreams || []);
    
    // If we have streams, set the first one as selected for single view
    if (detailedStreams.length > 0) {
      setSelectedStream(detailedStreams[0].name);
    }
    
    return detailedStreams;
  } catch (error) {
    console.error('Error loading streams for live view:', error);
    showStatusMessage('Error loading streams: ' + error.message);
    
    // Show error placeholder
    if (videoGridRef) {
      videoGridRef.innerHTML = `
        <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
          <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">Error loading streams</p>
          <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
        </div>
      `;
    }
    
    return [];
  } finally {
    // Hide loading state
    if (videoGridRef) {
      hideLoading(videoGridRef);
    }
  }
}
