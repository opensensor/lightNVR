/**
 * LightNVR Web Interface Batch Delete Modal Component
 * Preact component for displaying progress of batch delete operations
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';

/**
 * BatchDeleteModal component
 * @returns {JSX.Element} BatchDeleteModal component
 */
export function BatchDeleteModal() {
  // State for modal visibility and progress
  const [isVisible, setIsVisible] = useState(false);
  const [progress, setProgress] = useState({
    current: 0,
    total: 0,
    succeeded: 0,
    failed: 0,
    status: 'Preparing to delete recordings...',
    complete: false,
    error: false
  });

  // Use refs to store the current poll timer so it can be cancelled
  const pollTimerRef = useRef(null);
  // Track if we're currently running a batch delete
  const isRunningRef = useRef(false);

  /**
   * Show the modal
   */
  const showModal = useCallback(() => {
    // Reset progress state
    setProgress({
      current: 0,
      total: 0,
      succeeded: 0,
      failed: 0,
      status: 'Preparing to delete recordings...',
      complete: false,
      error: false
    });

    // Show modal
    setIsVisible(true);
  }, []);

  /**
   * Close the modal
   */
  const closeModal = useCallback(() => {
    setIsVisible(false);

    // Clear any active poll timer
    if (pollTimerRef.current) {
      clearInterval(pollTimerRef.current);
      pollTimerRef.current = null;
    }
  }, []);

  /**
   * Update progress state
   * @param {Object} newProgress - New progress data
   */
  const updateProgress = useCallback((newProgress) => {
    setProgress(prevProgress => ({
      ...prevProgress,
      ...newProgress
    }));
  }, []);

  /**
   * Cancel batch delete operation
   */
  const cancelBatchDelete = useCallback(() => {
    // Clear poll timer
    if (pollTimerRef.current) {
      clearInterval(pollTimerRef.current);
      pollTimerRef.current = null;
    }

    isRunningRef.current = false;

    // Close modal
    setIsVisible(false);

    // Show status message
    showStatusMessage('Batch delete operation cancelled', 'warning', 5000);
  }, []);

  /**
   * Delete recordings by HTTP request
   * @param {Object} params - Delete parameters (ids or filter)
   * @returns {Promise<Object>} Promise that resolves when the operation is complete
   */
  const batchDeleteRecordingsByHttpRequest = useCallback((params) => {
    console.log('Using HTTP for batch delete with params:', params);

    return new Promise((resolve, reject) => {
      // Clear any existing timer
      if (pollTimerRef.current) {
        clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }

      // Show modal with reset state
      setProgress({
        current: 0,
        total: 0,
        succeeded: 0,
        failed: 0,
        status: 'Preparing to delete recordings...',
        complete: false,
        error: false
      });
      setIsVisible(true);

      isRunningRef.current = true;

      // Calculate total count for progress bar
      let totalCount = 0;
      if (params.ids) {
        totalCount = params.ids.length;
      } else if (params.filter && params.totalCount) {
        totalCount = params.totalCount;
      }

      // Update progress to show we're starting
      setProgress(prev => ({
        ...prev,
        total: totalCount,
        status: 'Starting batch delete operation...'
      }));

      // Send HTTP request to start the batch delete job
      fetch('/api/recordings/batch-delete', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(params)
      })
      .then(response => {
        if (!response.ok) {
          throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
        }
        return response.json();
      })
      .then(result => {
        console.log('Batch delete job started:', result);

        if (!result.job_id) {
          throw new Error('No job_id received from server');
        }

        const jobId = result.job_id;

        // Start polling for progress
        const pollInterval = 500; // Poll every 500ms

        const pollProgress = () => {
          // Check if we've been cancelled
          if (!isRunningRef.current) {
            if (pollTimerRef.current) {
              clearInterval(pollTimerRef.current);
              pollTimerRef.current = null;
            }
            return;
          }

          fetch(`/api/recordings/batch-delete/progress/${jobId}`)
            .then(response => {
              if (!response.ok) {
                throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
              }
              return response.json();
            })
            .then(progressData => {
              console.log('Progress update:', progressData);

              // Update progress UI
              setProgress(prev => ({
                ...prev,
                current: progressData.current || 0,
                total: progressData.total || totalCount,
                succeeded: progressData.succeeded || 0,
                failed: progressData.failed || 0,
                status: progressData.status_message || 'Processing...',
                complete: progressData.complete || false
              }));

              // Check if complete
              if (progressData.complete) {
                // Stop polling
                if (pollTimerRef.current) {
                  clearInterval(pollTimerRef.current);
                  pollTimerRef.current = null;
                }

                isRunningRef.current = false;

                // Show status message
                const succeeded = progressData.succeeded || 0;
                const failed = progressData.failed || 0;
                const message = failed === 0
                  ? `Successfully deleted ${succeeded} recordings`
                  : `Deleted ${succeeded} recordings with ${failed} failures`;

                showStatusMessage(message, failed === 0 ? 'success' : 'warning', 5000);

                // Reload recordings after a short delay
                setTimeout(() => {
                  if (typeof window.loadRecordings === 'function') {
                    window.loadRecordings();
                  }
                }, 1000);

                resolve(progressData);
              }
            })
            .catch(error => {
              console.error('Progress polling error:', error);

              // Stop polling
              if (pollTimerRef.current) {
                clearInterval(pollTimerRef.current);
                pollTimerRef.current = null;
              }

              isRunningRef.current = false;

              // Update progress UI to show error
              setProgress({
                current: 0,
                total: 0,
                succeeded: 0,
                failed: 0,
                status: `Error: ${error.message || 'Unknown error'}`,
                complete: true,
                error: true
              });

              // Show error message
              showStatusMessage(`Error: ${error.message || 'Unknown error'}`, 'error', 5000);

              reject(error);
            });
        };

        // Start polling immediately and then at intervals
        pollProgress();
        pollTimerRef.current = setInterval(pollProgress, pollInterval);
      })
      .catch(error => {
        console.error('Batch delete start error:', error);

        isRunningRef.current = false;

        // Update progress UI to show error
        setProgress({
          current: 0,
          total: 0,
          succeeded: 0,
          failed: 0,
          status: `Error: ${error.message || 'Unknown error'}`,
          complete: true,
          error: true
        });

        // Show error message
        showStatusMessage(`Error: ${error.message || 'Unknown error'}`, 'error', 5000);

        reject(error);
      });
    });
  }, []);

  // Component initialization - register global functions
  useEffect(() => {
    console.log('BatchDeleteModal initialized');

    // Make functions globally available
    window.showBatchDeleteModal = showModal;
    window.closeBatchDeleteModal = closeModal;
    window.updateBatchDeleteProgress = updateProgress;
    window.batchDeleteRecordingsByHttpRequest = batchDeleteRecordingsByHttpRequest;

    // Cleanup on unmount
    return () => {
      // Clear any active poll timer
      if (pollTimerRef.current) {
        clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }

      // Clean up global functions
      delete window.showBatchDeleteModal;
      delete window.closeBatchDeleteModal;
      delete window.updateBatchDeleteProgress;
      delete window.batchDeleteRecordingsByHttpRequest;
    };
  }, [showModal, closeModal, updateProgress, batchDeleteRecordingsByHttpRequest]);

  // Calculate progress percentage
  const getProgressPercentage = () => {
    if (progress.total > 0) {
      return Math.round((progress.current / progress.total) * 100);
    } else if (progress.complete) {
      return 100;
    } else if (progress.current > 0) {
      return Math.min(90, progress.current / 10);
    } else {
      return 50; // Indeterminate
    }
  };

  // Get progress bar classes
  const getProgressBarClasses = () => {
    let classes = 'h-4 rounded-full text-center text-xs text-white';
    
    // Add color class
    if (progress.error) {
      classes += ' ' + 'bg-[hsl(var(--danger))]';
    } else {
      classes += ' ' + 'bg-[hsl(var(--success))]';
    }
    
    // Add animation class if needed
    if (!progress.complete && progress.total === 0) {
      classes += ' animate-pulse';
    }
    
    return classes;
  };

  // Get count text
  const getCountText = () => {
    if (progress.total > 0) {
      return `${progress.current} / ${progress.total}`;
    } else {
      return `${progress.current} / ?`;
    }
  };

  // Get percentage text
  const getPercentageText = () => {
    if (progress.total > 0) {
      return `${Math.round((progress.current / progress.total) * 100)}%`;
    } else if (progress.complete) {
      return '100%';
    } else {
      return 'In progress';
    }
  };

  // If not visible, don't render anything
  if (!isVisible) {
    return null;
  }

  return (
    <div 
      id="batch-delete-modal" 
      className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center z-50"
    >
      <div className="relative bg-card text-card-foreground rounded-lg shadow-xl max-w-md mx-auto p-6 w-full">
        <div className="flex justify-between items-center mb-4 pb-2 border-b border-border">
          <h3 id="batch-delete-modal-title" className="text-xl font-bold text-gray-900 dark:text-white">
            Batch Delete Progress
          </h3>
          <button 
            className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 text-2xl font-bold"
            onClick={closeModal}
          >
            &times;
          </button>
        </div>
        
        <div className="modal-body">
          <div className="mb-4 text-gray-700 dark:text-gray-300">
            {progress.status}
          </div>
          
          <div className="progress-container bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-4">
            <div 
              className={getProgressBarClasses()}
              style={{ width: `${getProgressPercentage()}%` }}
            ></div>
          </div>
          
          <div className="flex justify-between text-sm text-muted-foreground mb-6">
            <div>{getCountText()}</div>
            <div>{getPercentageText()}</div>
          </div>
          
          <div className="mb-4">
            <div className="flex justify-between mb-2">
              <span className="text-gray-700 dark:text-gray-300">Succeeded:</span>
              <span className="font-bold text-green-600 dark:text-green-400">{progress.succeeded}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-700 dark:text-gray-300">Failed:</span>
              <span className="font-bold text-red-600 dark:text-red-400">{progress.failed}</span>
            </div>
          </div>
          
          <div className="text-sm italic text-muted-foreground mb-4">
            {progress.message}
          </div>
        </div>
        
        <div className="flex justify-end pt-2 border-t border-border">
          {progress.complete ? (
            <button 
              className="btn-primary"
              onClick={closeModal}
            >
              Done
            </button>
          ) : (
            <button 
              className="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
              onClick={cancelBatchDelete}
            >
              Cancel
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

/**
 * Delete multiple recordings by IDs
 * @param {Array<number>} ids - Recording IDs to delete
 */
export function batchDeleteRecordings(ids) {
  if (!ids || ids.length === 0) {
    showStatusMessage('No recordings selected for deletion', 'warning', 5000);
    return;
  }

  // Confirm deletion
  if (!confirm(`Are you sure you want to delete ${ids.length} recordings?`)) {
    return;
  }

  // Show modal
  if (typeof window.showBatchDeleteModal === 'function') {
    window.showBatchDeleteModal();
  }

  // Use HTTP fallback for batch delete
  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ ids })
      .catch(error => {
        console.error('Error starting batch delete:', error);
        showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 'error', 5000);

        // Close modal if it's open
        if (typeof window.closeBatchDeleteModal === 'function') {
          window.closeBatchDeleteModal();
        }
      });
  } else {
    showStatusMessage('Batch delete function not available', 'error', 5000);
  }
}

/**
 * Delete recordings by filter
 * @param {Object} filter - Filter to delete by
 */
export function batchDeleteRecordingsByFilter(filter) {
  if (!filter) {
    showStatusMessage('No filter specified for deletion', 'warning', 5000);
    return;
  }

  // Confirm deletion
  if (!confirm('Are you sure you want to delete all recordings matching the current filter?')) {
    return;
  }

  // Show modal
  if (typeof window.showBatchDeleteModal === 'function') {
    window.showBatchDeleteModal();
  }

  // Use HTTP fallback for batch delete
  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ filter })
      .catch(error => {
        console.error('Error starting batch delete:', error);
        showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 'error', 5000);

        // Close modal if it's open
        if (typeof window.closeBatchDeleteModal === 'function') {
          window.closeBatchDeleteModal();
        }
      });
  } else {
    showStatusMessage('Batch delete function not available', 'error', 5000);
  }
}

// Export for global use
if (typeof window !== 'undefined') {
  window.batchDeleteRecordings = batchDeleteRecordings;
  window.batchDeleteRecordingsByFilter = batchDeleteRecordingsByFilter;
}
