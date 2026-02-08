/**
 * RestartModal Component
 * A styled modal for confirming and showing restart progress
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
import { createPortal } from 'preact/compat';

/**
 * RestartModal component
 * @param {Object} props Component props
 * @param {boolean} props.isOpen Whether the modal is open
 * @param {Function} props.onClose Function to close the modal
 * @param {Function} props.onConfirm Function to call when restart is confirmed
 * @param {boolean} props.isRestarting Whether the system is currently restarting
 * @returns {JSX.Element} RestartModal component
 */
export function RestartModal({ isOpen, onClose, onConfirm, isRestarting }) {
  const modalRef = useRef(null);
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const [reconnectStatus, setReconnectStatus] = useState('');
  const maxReconnectAttempts = 30; // Try for up to 60 seconds (2s intervals)

  // Handle escape key (only when not restarting)
  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape' && isOpen && !isRestarting) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, isRestarting, onClose]);

  // Handle reconnection attempts after restart
  useEffect(() => {
    if (!isRestarting) {
      setReconnectAttempts(0);
      setReconnectStatus('');
      return;
    }

    // Wait a bit before starting to check
    const initialDelay = setTimeout(() => {
      setReconnectStatus('Waiting for server to restart...');
    }, 2000);

    // Start checking after 5 seconds
    const checkInterval = setInterval(async () => {
      setReconnectAttempts(prev => {
        const newAttempts = prev + 1;
        if (newAttempts >= maxReconnectAttempts) {
          setReconnectStatus('Server is taking longer than expected. Please refresh manually.');
          clearInterval(checkInterval);
          return newAttempts;
        }
        return newAttempts;
      });

      try {
        const response = await fetch('/api/system/status', {
          method: 'GET',
          cache: 'no-store',
          signal: AbortSignal.timeout(2000)
        });
        
        if (response.ok) {
          setReconnectStatus('Server is back online! Reloading...');
          clearInterval(checkInterval);
          setTimeout(() => {
            window.location.reload();
          }, 1000);
        }
      } catch (error) {
        // Server not ready yet, continue waiting
        setReconnectStatus(`Reconnecting... (attempt ${reconnectAttempts + 1})`);
      }
    }, 2000);

    return () => {
      clearTimeout(initialDelay);
      clearInterval(checkInterval);
    };
  }, [isRestarting]);

  // Handle background click (only when not restarting)
  const handleBackgroundClick = (e) => {
    if (e.target === e.currentTarget && !isRestarting) {
      onClose();
    }
  };

  if (!isOpen) return null;

  return createPortal(
    <div
      ref={modalRef}
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
    >
      <div className="bg-card text-card-foreground rounded-lg shadow-xl p-6 max-w-md w-full mx-4 transform transition-all duration-300 ease-out">
        {!isRestarting ? (
          // Confirmation state
          <>
            <div className="flex items-center mb-4">
              <div className="flex-shrink-0 w-12 h-12 rounded-full bg-yellow-100 dark:bg-yellow-900/30 flex items-center justify-center mr-4">
                <svg className="w-6 h-6 text-yellow-600 dark:text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </div>
              <div>
                <h3 className="text-lg font-semibold">Restart LightNVR</h3>
                <p className="text-sm text-muted-foreground">This will restart the NVR service</p>
              </div>
            </div>

            <p className="text-muted-foreground mb-6">
              Are you sure you want to restart LightNVR? All active streams will be temporarily interrupted and will resume automatically after restart.
            </p>

            <div className="flex justify-end space-x-3">
              <button
                className="px-4 py-2 bg-muted text-muted-foreground rounded-md hover:bg-muted/80 transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-muted"
                onClick={onClose}
              >
                Cancel
              </button>
              <button
                className="px-4 py-2 bg-yellow-500 text-white rounded-md hover:bg-yellow-600 transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500"
                onClick={onConfirm}
              >
                Restart
              </button>
            </div>
          </>
        ) : (
          // Restarting state
          <>
            <div className="flex flex-col items-center py-4">
              {/* Spinner */}
              <div className="relative w-16 h-16 mb-6">
                <div className="absolute inset-0 border-4 border-primary/20 rounded-full"></div>
                <div className="absolute inset-0 border-4 border-transparent border-t-primary rounded-full animate-spin"></div>
              </div>

              <h3 className="text-lg font-semibold mb-2">Restarting LightNVR</h3>
              <p className="text-sm text-muted-foreground text-center mb-4">
                {reconnectStatus || 'Initiating restart...'}
              </p>

              {reconnectAttempts >= maxReconnectAttempts && (
                <button
                  className="mt-4 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
                  onClick={() => window.location.reload()}
                >
                  Refresh Page
                </button>
              )}
            </div>
          </>
        )}
      </div>
    </div>,
    document.body
  );
}

