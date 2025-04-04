/**
 * Stream grid functionality for LiveView
 */

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
    
    // Filter out streams that are soft deleted, inactive, or not configured for HLS
    const filteredStreams = detailedStreams.filter(stream => {
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
    
    console.log('Filtered streams for live view:', filteredStreams);
    
    // Store filtered streams in state
    setStreams(filteredStreams || []);
    
    // If we have filtered streams, set the first one as selected for single view
    if (filteredStreams.length > 0) {
      setSelectedStream(filteredStreams[0].name);
    }
    
    return filteredStreams;
  } catch (error) {
    console.error('Error loading streams for live view:', error);
    showStatusMessage('Error loading streams: ' + error.message);
    
    return [];
  }
}
