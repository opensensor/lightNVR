/**
 * LightNVR Web Interface Batch Delete Modal Component
 * Preact component for displaying progress of batch delete operations
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { useQueryClient } from '../../query-client.js';

/**
 * BatchDeleteModal component
 *
 * Two-phase modal:
 *  1. Confirmation — shows warning and Proceed / Cancel buttons
 *  2. Progress    — shows progress bar with no cancel (operation cannot be stopped)
 *
 * @returns {JSX.Element} BatchDeleteModal component
 */
export function BatchDeleteModal() {
  const queryClient = useQueryClient();

  // Modal phase: 'hidden' | 'confirm' | 'progress'
  const [phase, setPhase] = useState('hidden');

  // Pending delete params (stored during confirm phase, used when user clicks Proceed)
  const pendingParamsRef = useRef(null);
  const pendingResolveRef = useRef(null);
  const pendingRejectRef = useRef(null);

  // Progress state
  const [progress, setProgress] = useState({
    current: 0,
    total: 0,
    succeeded: 0,
    failed: 0,
    status: 'Preparing to delete recordings...',
    complete: false,
    error: false
  });

  // Poll timer ref
  const pollTimerRef = useRef(null);
  const isRunningRef = useRef(false);

  /** Derive a human-readable description for the confirm dialog */
  const getConfirmDescription = () => {
    const params = pendingParamsRef.current;
    if (!params) return '';
    if (params.ids) return `${params.ids.length} selected recording${params.ids.length !== 1 ? 's' : ''}`;
    if (params.totalCount) return `${params.totalCount} filtered recording${params.totalCount !== 1 ? 's' : ''}`;
    return 'all matching recordings';
  };

  /**
   * Close/reset the modal (only allowed when not running, or when complete)
   */
  const closeModal = useCallback(() => {
    if (pollTimerRef.current) {
      clearInterval(pollTimerRef.current);
      pollTimerRef.current = null;
    }
    setPhase('hidden');
    pendingParamsRef.current = null;
    pendingResolveRef.current = null;
    pendingRejectRef.current = null;
  }, []);

  /**
   * Cancel during the confirmation phase (before delete starts)
   */
  const cancelConfirm = useCallback(() => {
    closeModal();
    showStatusMessage('Batch delete cancelled', 'info', 3000);
  }, [closeModal]);

  /**
   * Open the modal in confirmation phase. The actual HTTP request is only
   * fired when the user clicks "Proceed".
   *
   * @param {Object} params - Delete parameters (ids or filter)
   * @returns {Promise<Object>} Resolves when the operation completes
   */
  const batchDeleteRecordingsByHttpRequest = useCallback((params) => {
    console.log('Batch delete requested with params:', params);

    return new Promise((resolve, reject) => {
      pendingParamsRef.current = params;
      pendingResolveRef.current = resolve;
      pendingRejectRef.current = reject;
      setPhase('confirm');
    });
  }, []);

  /**
   * User confirmed — start the actual delete operation.
   */
  const proceedWithDelete = useCallback(() => {
    const params = pendingParamsRef.current;
    const resolve = pendingResolveRef.current;
    const reject = pendingRejectRef.current;
    if (!params) return;

    // Switch to progress phase
    setProgress({
      current: 0,
      total: 0,
      succeeded: 0,
      failed: 0,
      status: 'Preparing to delete recordings...',
      complete: false,
      error: false
    });
    setPhase('progress');
    isRunningRef.current = true;

    // Calculate total count for progress bar
    let totalCount = 0;
    if (params.ids) {
      totalCount = params.ids.length;
    } else if (params.filter && params.totalCount) {
      totalCount = params.totalCount;
    }

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

                // Invalidate the recordings query cache so the list re-fetches immediately
                queryClient.invalidateQueries({ queryKey: ['recordings'] });

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
    window.closeBatchDeleteModal = closeModal;
    window.batchDeleteRecordingsByHttpRequest = batchDeleteRecordingsByHttpRequest;

    // Cleanup on unmount
    return () => {
      if (pollTimerRef.current) {
        clearInterval(pollTimerRef.current);
        pollTimerRef.current = null;
      }
      delete window.closeBatchDeleteModal;
      delete window.batchDeleteRecordingsByHttpRequest;
    };
  }, [closeModal, batchDeleteRecordingsByHttpRequest]);

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

  // If hidden, don't render anything
  if (phase === 'hidden') {
    return null;
  }

  return (
    <div
      id="batch-delete-modal"
      className="fixed inset-0 bg-black/50 overflow-y-auto h-full w-full flex items-center justify-center z-50"
    >
      <div className="relative bg-card text-card-foreground rounded-lg shadow-xl max-w-md mx-auto p-6 w-full">

        {/* ===== PHASE 1: Confirmation ===== */}
        {phase === 'confirm' && (
          <>
            <div className="flex justify-between items-center mb-4 pb-2 border-b border-border">
              <h3 className="text-xl font-bold">Confirm Batch Delete</h3>
              <button
                className="text-muted-foreground hover:text-foreground text-2xl font-bold"
                onClick={cancelConfirm}
              >&times;</button>
            </div>

            <div className="mb-4">
              <div className="flex items-start gap-3 p-3 rounded-lg bg-red-500/10 border border-red-500/30 mb-4">
                <svg className="w-6 h-6 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
                </svg>
                <div>
                  <p className="font-semibold text-red-500 mb-1">This action cannot be undone</p>
                  <p className="text-sm text-muted-foreground">
                    You are about to permanently delete <strong>{getConfirmDescription()}</strong>.
                    Once started, this operation cannot be cancelled or reversed.
                  </p>
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-2 pt-2 border-t border-border">
              <button
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                onClick={cancelConfirm}
              >
                Cancel
              </button>
              <button
                className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors font-medium"
                onClick={proceedWithDelete}
              >
                Delete {getConfirmDescription()}
              </button>
            </div>
          </>
        )}

        {/* ===== PHASE 2: Progress (non-cancellable) ===== */}
        {phase === 'progress' && (
          <>
            <div className="flex justify-between items-center mb-4 pb-2 border-b border-border">
              <h3 className="text-xl font-bold">
                {progress.complete ? 'Batch Delete Complete' : 'Deleting Recordings…'}
              </h3>
            </div>

            <div className="modal-body">
              <div className="progress-container bg-gray-200 dark:bg-gray-700 rounded-full h-4 mb-4">
                <div
                  className={getProgressBarClasses()}
                  style={{ width: `${getProgressPercentage()}%` }}
                ></div>
              </div>

              <div className="flex justify-between text-muted-foreground mb-6">
                <div>{getCountText()}</div>
                <div>{getPercentageText()}</div>
              </div>

              <div className="mb-4">
                <div className="flex justify-between mb-2">
                  <span className="text-muted-foreground">Succeeded:</span>
                  <span className="font-bold text-green-600 dark:text-green-400">{progress.succeeded}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Failed:</span>
                  <span className="font-bold text-red-600 dark:text-red-400">{progress.failed}</span>
                </div>
              </div>

              {!progress.complete && (
                <div className="text-xs text-muted-foreground italic mb-4">
                  This operation cannot be cancelled once started.
                </div>
              )}
            </div>

            <div className="flex justify-end pt-2 border-t border-border">
              {progress.complete ? (
                <button className="btn-primary" onClick={closeModal}>Done</button>
              ) : (
                <span className="text-sm text-muted-foreground italic">Please wait…</span>
              )}
            </div>
          </>
        )}

      </div>
    </div>
  );
}

/**
 * Delete multiple recordings by IDs.
 * Opens the confirmation modal — no browser confirm() used.
 * @param {Array<number>} ids - Recording IDs to delete
 */
export function batchDeleteRecordings(ids) {
  if (!ids || ids.length === 0) {
    showStatusMessage('No recordings selected for deletion', 'warning', 5000);
    return;
  }

  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ ids })
      .catch(error => {
        console.error('Error in batch delete:', error);
        showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 'error', 5000);
        if (typeof window.closeBatchDeleteModal === 'function') {
          window.closeBatchDeleteModal();
        }
      });
  } else {
    showStatusMessage('Batch delete function not available', 'error', 5000);
  }
}

/**
 * Delete recordings by filter.
 * Opens the confirmation modal — no browser confirm() used.
 * @param {Object} filter - Filter to delete by
 */
export function batchDeleteRecordingsByFilter(filter) {
  if (!filter) {
    showStatusMessage('No filter specified for deletion', 'warning', 5000);
    return;
  }

  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ filter })
      .catch(error => {
        console.error('Error in batch delete:', error);
        showStatusMessage(`Error: ${error.message || 'Failed to start batch delete operation'}`, 'error', 5000);
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
