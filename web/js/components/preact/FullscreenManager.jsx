/**
 * Fullscreen functionality for LiveView
 * React component for managing fullscreen mode
 */
import { h } from 'preact';
import { useState, useEffect, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';

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
    console.log("Toggle fullscreen called, current state:", isFullscreen);
    setIsFullscreen(prevState => !prevState);
  }, [isFullscreen]);

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
        stroke-width="2" 
        stroke-linecap="round" 
        stroke-linejoin="round"
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
  // Handle escape key
  useEffect(() => {
    if (!isFullscreen) return;

    const handleEscapeKey = (e) => {
      if (e.key === 'Escape') {
        console.log("Escape key pressed in fullscreen mode");
        setIsFullscreen(false);
      }
    };

    document.addEventListener('keydown', handleEscapeKey);

    // Apply fullscreen styles to the target element
    const targetElement = document.getElementById(targetId);
    if (targetElement) {
      targetElement.classList.add('fullscreen-mode');
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscapeKey);
      
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
        if (!currentIsFullscreen) {
          setterFunc(true);
        } else {
          setterFunc(false);
        }
      } else {
        // Otherwise toggle our internal state
        setIsFullscreen(prev => !prev);
      }
    };

    // Create a wrapper function for exitFullscreenMode
    const exitFullscreenModeWrapper = (e, setterFunc) => {
      // If this was called from an event, stop propagation
      if (e) {
        e.stopPropagation();
        e.preventDefault();
      }

      // If setterFunc is provided, use it (for backward compatibility)
      if (typeof setterFunc === 'function') {
        setterFunc(false);
      } else {
        // Otherwise use our internal state
        setIsFullscreen(false);
      }
    };
  }, [setIsFullscreen]);

  // Render the exit button if in fullscreen mode
  return isFullscreen ? (
    <FullscreenExitButton 
      isFullscreen={isFullscreen} 
      onExit={() => setIsFullscreen(false)} 
    />
  ) : null;
}
