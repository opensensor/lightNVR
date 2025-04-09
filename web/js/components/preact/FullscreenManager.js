/**
 * Fullscreen functionality for LiveView
 */

import { showStatusMessage } from './UI.js';

/**
 * Exit fullscreen mode
 * @param {Event} e - Optional event object
 * @param {Function} setIsFullscreen - State setter for fullscreen state
 */
export function exitFullscreenMode(e, setIsFullscreen) {
  // If this was called from an event, stop propagation
  if (e) {
    e.stopPropagation();
    e.preventDefault();
  }

  console.log("DIRECT EXIT FUNCTION CALLED");

  const livePage = document.getElementById('live-page');
  if (!livePage) {
    console.error("Live page element not found");
    return;
  }

  // Exit fullscreen
  livePage.classList.remove('fullscreen-mode');
  document.body.style.overflow = '';

  // Remove exit button
  const exitBtn = document.querySelector('.fullscreen-exit');
  if (exitBtn) {
    exitBtn.remove();
  } else {
    console.warn("Exit button not found when trying to remove it");
  }

  // Show the fullscreen button again
  const fullscreenBtn = document.getElementById('fullscreen-btn');
  if (fullscreenBtn) {
    fullscreenBtn.style.display = '';
  } else {
    console.warn("Fullscreen button not found when trying to show it again");
  }

  // Remove the escape key handler if it exists
  if (window._fullscreenEscapeHandler) {
    document.removeEventListener('keydown', window._fullscreenEscapeHandler);
    delete window._fullscreenEscapeHandler;
  }

  // Update state
  setIsFullscreen(false);

  console.log("Fullscreen mode exited, state set to false");
}

/**
 * Toggle fullscreen mode for the entire live view
 * @param {boolean} isFullscreen - Current fullscreen state
 * @param {Function} setIsFullscreen - State setter for fullscreen state
 */
export function toggleFullscreen(isFullscreen, setIsFullscreen) {
  console.log("toggleFullscreen called, current state:", isFullscreen);

  const livePage = document.getElementById('live-page');

  if (!livePage) {
    console.error("Live page element not found");
    return;
  }

  const isCurrentlyInFullscreen = livePage.classList.contains('fullscreen-mode');
  console.log("DOM check for fullscreen mode:", isCurrentlyInFullscreen);

  if (!isCurrentlyInFullscreen) {
    console.log("Entering fullscreen mode");
    // Enter fullscreen
    livePage.classList.add('fullscreen-mode');
    document.body.style.overflow = 'hidden';

    // Add exit button - IMPORTANT: Use a standalone function for the click handler
    const exitBtn = document.createElement('button');
    exitBtn.className = 'fullscreen-exit fixed top-4 right-4 w-10 h-10 bg-black/70 text-white rounded-full flex justify-center items-center cursor-pointer z-50 transition-all duration-200 hover:bg-black/85 hover:scale-110 shadow-md';
    exitBtn.innerHTML = '✕';

    // Create a standalone function for the click handler
    const exitClickHandler = function(e) {
      console.log("Exit button clicked - STANDALONE HANDLER");
      exitFullscreenMode(e, setIsFullscreen);
    };

    // Add the event listener with the standalone function
    exitBtn.addEventListener('click', exitClickHandler);

    livePage.appendChild(exitBtn);

    // Hide the fullscreen button in the controls when in fullscreen mode
    const fullscreenBtn = document.getElementById('fullscreen-btn');
    if (fullscreenBtn) {
      fullscreenBtn.style.display = 'none';
    }

    // Add event listener for Escape key
    const escapeHandler = function(e) {
      if (e.key === 'Escape') {
        console.log("Escape key pressed in fullscreen mode");
        exitFullscreenMode(null, setIsFullscreen);
      }
    };

    // Store the handler on the window object so we can remove it later
    window._fullscreenEscapeHandler = escapeHandler;
    document.addEventListener('keydown', escapeHandler);

    // Update state
    setIsFullscreen(true);
    console.log("Fullscreen mode entered, state set to true");
  } else {
    exitFullscreenMode(null, setIsFullscreen);
  }
}

/**
 * Toggle fullscreen mode for a specific stream
 * @param {string} streamName - Name of the stream
 */
export function toggleStreamFullscreen(streamName) {
  const videoElementId = `video-${streamName.replace(/\s+/g, '-')}`;
  const videoElement = document.getElementById(videoElementId);
  const videoCell = videoElement ? videoElement.closest('.video-cell') : null;

  if (!videoCell) {
    console.error('Stream not found:', streamName);
    return;
  }

  if (!document.fullscreenElement) {
    videoCell.requestFullscreen().catch(err => {
      console.error(`Error attempting to enable fullscreen: ${err.message}`);
      showStatusMessage(`Could not enable fullscreen mode: ${err.message}`);
    });
  } else {
    document.exitFullscreen();
  }
}
