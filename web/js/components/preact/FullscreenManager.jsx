/**
 * Fullscreen functionality for LiveView
 * React component for managing fullscreen mode
 */
import { useState, useEffect, useCallback, useRef } from 'preact/hooks';
import { tinykeys } from 'tinykeys';


/**
 * Show a brief grid-position overlay inside the newly-fullscreen cell so the
 * user can orient themselves after arrow-key navigation.
 *
 * The overlay is injected directly into the DOM (not via React/Preact) so it
 * works inside the native fullscreen element without needing a portal.
 *
 * @param {HTMLElement} nextCell       - The .video-cell element now in fullscreen
 * @param {string}      nextStreamName - The stream that is now visible
 * @param {Array}       streamsToShow  - All streams on the current page
 * @param {number}      cols           - Grid column count
 * @param {number}      rows           - Grid row count
 */
function showGridOverlay(nextCell, nextStreamName, streamsToShow, cols, rows) {
  // Remove any stale overlay left from rapid navigation.
  const existing = nextCell.querySelector('.fs-grid-overlay');
  if (existing) existing.remove();

  const overlay = document.createElement('div');
  overlay.className = 'fs-grid-overlay';
  Object.assign(overlay.style, {
    position: 'absolute',
    bottom: '20px', left: '20px', top: 'auto', right: 'auto',
    background: 'rgba(0,0,0,0.65)',
    padding: '8px',
    borderRadius: '8px',
    zIndex: '9999',
    display: 'grid',
    gridTemplateColumns: `repeat(${cols}, 1fr)`,
    gap: '4px',
    width: 'min(180px, 22vw)',
    backdropFilter: 'blur(4px)',
    pointerEvents: 'auto',
    boxSizing: 'border-box',
  });

  // Build the mini grid.
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      const idx = row * cols + col;
      const stream = streamsToShow[idx];
      const isCurrent = stream && stream.name === nextStreamName;
      const cell = document.createElement('div');
      Object.assign(cell.style, {
        aspectRatio: '16/9',
        background: !stream
          ? 'rgba(255,255,255,0.05)'
          : isCurrent
            ? 'rgba(255,255,255,0.65)'
            : 'rgba(255,255,255,0.18)',
        border: `1px solid ${isCurrent ? 'rgba(255,255,255,0.9)' : 'rgba(255,255,255,0.25)'}`,
        borderRadius: '2px',
        boxSizing: 'border-box',
      });
      overlay.appendChild(cell);
    }
  }

  // Cycle through four corners when the user hovers to keep it out of the way.
  const CORNERS = [
    { bottom: '20px', left:  '20px', top:  'auto', right: 'auto' }, // bottom-left (default)
    { bottom: 'auto', left:  'auto', top:  '20px', right: '20px' }, // top-right
    { bottom: '20px', left:  'auto', top:  'auto', right: '20px' }, // bottom-right
    { bottom: 'auto', left:  '20px', top:  '20px', right: 'auto' }, // top-left
  ];
  let cornerIdx = 0;
  overlay.addEventListener('mouseenter', () => {
    cornerIdx = (cornerIdx + 1) % CORNERS.length;
    Object.assign(overlay.style, CORNERS[cornerIdx]);
  });

  nextCell.appendChild(overlay);

  // Auto-remove after 3 s; also clean up when fullscreen changes (nav or exit).
  const hideTimer = setTimeout(() => overlay.remove(), 3000);
  document.addEventListener('fullscreenchange', () => {
    clearTimeout(hideTimer);
    overlay.remove();
  }, { once: true });
}

/**
 * Navigate to an adjacent stream in the native fullscreen grid.
 * Finds the currently fullscreen .video-cell, locates its grid position, then
 * steps in the requested direction (with wrap-around), skipping empty cells.
 *
 * IMPORTANT: The browser Fullscreen API is a *stack* — every requestFullscreen()
 * call pushes a new entry, and exitFullscreen() only pops the top.  To avoid
 * building up a stack [A, B, C, D] that forces the user to "unwind" every
 * visited stream, we drain the stack recursively: keep calling exitFullscreen()
 * on each fullscreenchange until document.fullscreenElement is null, then
 * requestFullscreen() on the next cell.  A guard flag prevents overlapping
 * transitions from rapid key presses.
 *
 * @param {'ArrowLeft'|'ArrowRight'|'ArrowUp'|'ArrowDown'} direction
 * @param {Array}  streamsToShow - streams visible in the current page
 * @param {number} cols          - grid column count
 * @param {number} rows          - grid row count
 */

// Guard: true while a fullscreen transition is in progress.
let _fsNavBusy = false;

function navigateFullscreenGrid(direction, streamsToShow, cols, rows) {
  if (_fsNavBusy) return; // drop key if transition already underway

  const fullscreenEl = document.fullscreenElement;
  if (!fullscreenEl) return;

  // The fullscreen element is the .video-cell div (data-stream-name is on it)
  const streamName = fullscreenEl.dataset.streamName;
  if (!streamName) return;

  const currentIndex = streamsToShow.findIndex(s => s.name === streamName);
  if (currentIndex === -1) return;

  let nextRow = Math.floor(currentIndex / cols);
  let nextCol = currentIndex % cols;

  // Walk one step at a time in the requested direction, wrapping around, until
  // we land on a populated cell (or exhaust all possibilities).
  const maxAttempts = cols * rows;
  let nextCell = null;
  let nextStreamName = null;
  for (let i = 0; i < maxAttempts; i++) {
    if (direction === 'ArrowRight') {
      nextCol = (nextCol + 1) % cols;
    } else if (direction === 'ArrowLeft') {
      nextCol = (nextCol - 1 + cols) % cols;
    } else if (direction === 'ArrowDown') {
      nextRow = (nextRow + 1) % rows;
    } else if (direction === 'ArrowUp') {
      nextRow = (nextRow - 1 + rows) % rows;
    }

    const nextIndex = nextRow * cols + nextCol;
    if (nextIndex < streamsToShow.length) {
      const nextStream = streamsToShow[nextIndex];
      // Query the DOM for the cell; it remains in the DOM even while another
      // element is in fullscreen mode.
      const candidate = document.querySelector(
        `[data-stream-name="${CSS.escape(nextStream.name)}"].video-cell`
      );
      if (candidate && candidate !== fullscreenEl) {
        nextCell = candidate;
        nextStreamName = nextStream.name;
      }
      break;
    }
  }

  if (!nextCell) return;

  _fsNavBusy = true;

  // Recursively drain the fullscreen stack until it is completely empty, then
  // enter fullscreen for the next cell.  This prevents the "reverse-order exit"
  // bug caused by browsers that implement exitFullscreen() as a single-pop.
  const drainAndEnter = () => {
    if (!document.fullscreenElement) {
      // Stack is fully empty — enter fullscreen for the next cell.
      nextCell.requestFullscreen()
        .then(() => showGridOverlay(nextCell, nextStreamName, streamsToShow, cols, rows))
        .catch(err => console.warn(`Grid nav fullscreen switch failed: ${err.message}`))
        .finally(() => { _fsNavBusy = false; });
    } else {
      // Stack still has entries — keep draining.
      document.addEventListener('fullscreenchange', drainAndEnter, { once: true });
      document.exitFullscreen().catch(err => {
        console.warn(`Grid nav fullscreen drain failed: ${err.message}`);
        _fsNavBusy = false;
      });
    }
  };

  document.addEventListener('fullscreenchange', drainAndEnter, { once: true });
  document.exitFullscreen().catch(err => {
    console.warn(`Grid nav fullscreen exit failed: ${err.message}`);
    _fsNavBusy = false;
  });
}

/**
 * Returns the stream name of the video cell currently in native fullscreen,
 * or null. Reads `data-stream-name` from `document.fullscreenElement` and
 * updates on every `fullscreenchange`. LiveView/WebRTCView use it to swap
 * the fullscreened cell off the sub-stream and onto the main stream (#366).
 *
 * @returns {string|null}
 */
export function useFullscreenCellStream() {
  const [fullscreenStream, setFullscreenStream] = useState(null);

  useEffect(() => {
    const update = () => {
      const el = document.fullscreenElement;
      setFullscreenStream(el && el.dataset ? (el.dataset.streamName || null) : null);
    };
    update();
    document.addEventListener('fullscreenchange', update);
    return () => document.removeEventListener('fullscreenchange', update);
  }, []);

  return fullscreenStream;
}

/**
 * Hook: bind arrow keys for grid navigation while a stream is in native
 * fullscreen.  Refs keep streamsToShow/cols/rows fresh without re-subscribing
 * tinykeys on every render.
 *
 * @param {Array}  streamsToShow
 * @param {number} cols
 * @param {number} rows
 */
export function useFullscreenGridNav(streamsToShow, cols, rows) {
  const streamsRef = useRef(streamsToShow);
  const colsRef    = useRef(cols);
  const rowsRef    = useRef(rows);

  useEffect(() => { streamsRef.current = streamsToShow; }, [streamsToShow]);
  useEffect(() => { colsRef.current = cols; },            [cols]);
  useEffect(() => { rowsRef.current = rows; },            [rows]);

  useEffect(() => {
    // Use a capture-phase listener instead of tinykeys so the handler runs
    // before the browser's native fullscreen-controls overlay can mark the
    // event as defaultPrevented (tinykeys v3 skips events that are already
    // defaultPrevented, which caused arrow keys to be silently ignored).
    const handler = (e) => {
      if (!document.fullscreenElement) return; // only act when a cell is in native fullscreen
      if (e.altKey || e.ctrlKey || e.metaKey || e.shiftKey) return;

      let direction;
      switch (e.key) {
        case 'ArrowLeft':  direction = 'ArrowLeft';  break;
        case 'ArrowRight': direction = 'ArrowRight'; break;
        case 'ArrowUp':    direction = 'ArrowUp';    break;
        case 'ArrowDown':  direction = 'ArrowDown';  break;
        default: return;
      }

      // Prevent the browser from scrolling the page or seeking the video.
      e.preventDefault();
      navigateFullscreenGrid(direction, streamsRef.current, colsRef.current, rowsRef.current);
    };

    window.addEventListener('keydown', handler, { capture: true });
    return () => window.removeEventListener('keydown', handler, { capture: true });
  }, []); // empty deps: refs keep values current
}

/**
 * Custom hook for fullscreen functionality
 * @returns {Object} Fullscreen functions and state
 */
export function useFullscreenManager() {
  const [isFullscreen, setIsFullscreen] = useState(false);

  /**
   * Exit fullscreen mode
   * @param {Event} e - Optional event object
   */
  const exitFullscreenMode = useCallback((e) => {
    // If this was called from an event, stop propagation
    if (e) {
      e.stopPropagation();
      e.preventDefault();
    }

    console.log("Exit fullscreen mode called");
    setIsFullscreen(false);
  }, []);

  /**
   * Toggle fullscreen mode
   */
  const toggleFullscreen = useCallback(() => {
    setIsFullscreen(prevState => {
      const nextState = !prevState;
      console.log("Toggle fullscreen called, new state:", nextState);
      return nextState;
    });
  }, []);

  return {
    isFullscreen,
    setIsFullscreen,
    exitFullscreenMode,
    toggleFullscreen
  };
}

/**
 * FullscreenManager component
 * @param {Object} props Component props
 * @param {boolean} props.isFullscreen Whether fullscreen mode is active
 * @param {Function} props.onExit Function to call when exiting fullscreen mode
 * @returns {JSX.Element} FullscreenManager component
 */
export function FullscreenExitButton({ isFullscreen, onExit }) {
  if (!isFullscreen) return null;

  return (
    <button 
      className="fullscreen-exit fixed top-4 right-4 w-10 h-10 bg-black/70 text-white rounded-full flex justify-center items-center cursor-pointer z-50 transition-all duration-200 hover:bg-black/85 hover:scale-110 shadow-md"
      onClick={onExit}
    >
      ✕
    </button>
  );
}

/**
 * FullscreenButton component
 * @param {Object} props Component props
 * @param {boolean} props.isFullscreen Whether fullscreen mode is active
 * @param {Function} props.onToggle Function to call when toggling fullscreen mode
 * @returns {JSX.Element} FullscreenButton component
 */
export function FullscreenButton({ isFullscreen, onToggle }) {
  if (isFullscreen) return null;

  return (
    <button 
      id="fullscreen-btn"
      className="fullscreen-btn px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
      onClick={onToggle}
    >
      <svg 
        xmlns="http://www.w3.org/2000/svg" 
        width="24" 
        height="24" 
        viewBox="0 0 24 24" 
        fill="none" 
        stroke="currentColor" 
        strokeWidth="2" 
        strokeLinecap="round" 
        strokeLinejoin="round"
      >
        <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
      </svg>
    </button>
  );
}

/**
 * FullscreenManager component
 * @param {Object} props Component props
 * @param {boolean} props.isFullscreen Whether fullscreen mode is active
 * @param {Function} props.setIsFullscreen Function to set fullscreen state
 * @param {string} props.targetId ID of the element to make fullscreen
 * @returns {JSX.Element} FullscreenManager component
 */
export function FullscreenManager({ isFullscreen, setIsFullscreen, targetId = 'live-page' }) {
  // Handle escape key (via tinykeys) and apply/remove fullscreen CSS
  useEffect(() => {
    if (!isFullscreen) return;

    const targetElement = document.getElementById(targetId);
    if (targetElement) {
      targetElement.classList.add('fullscreen-mode');
      document.body.style.overflow = 'hidden';
    }

    const unsub = tinykeys(window, {
      Escape: (e) => {
        console.log("Escape key pressed in fullscreen mode");
        e.preventDefault();
        setIsFullscreen(false);
      },
    });

    return () => {
      unsub();

      // Remove fullscreen styles when component unmounts or fullscreen is exited
      if (targetElement) {
        targetElement.classList.remove('fullscreen-mode');
        document.body.style.overflow = '';
      }
    };
  }, [isFullscreen, setIsFullscreen, targetId]);

  // Register the toggleFullscreen function globally for backward compatibility
  useEffect(() => {
    // Create a wrapper function that uses the current setIsFullscreen
    const toggleFullscreenWrapper = (currentIsFullscreen, setterFunc) => {
      // If setterFunc is provided, use it (for backward compatibility)
      if (typeof setterFunc === 'function') {
        setterFunc(!currentIsFullscreen);
      } else {
        // Otherwise toggle our internal state
        setIsFullscreen(prev => !prev);
      }
    };

    // Expose the wrapper globally for backward compatibility with existing code
    if (typeof window !== 'undefined') {
      window.toggleFullscreen = toggleFullscreenWrapper;
    }

    // Cleanup on unmount or when dependencies change
    return () => {
      if (typeof window !== 'undefined' && window.toggleFullscreen === toggleFullscreenWrapper) {
        delete window.toggleFullscreen;
      }
    };
  }, [setIsFullscreen]);

  return null;
}
