/**
 * LightNVR Web Interface Batch Delete Modal Component
 * Preact component for displaying progress of batch delete operations
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.tsx';
import { batchDeleteStatus } from '../../utils/batch-delete-status.ts';
import { useQueryClient } from '../../query-client.ts';
import { t as translate, useI18n } from '../../i18n.ts';
import type {
  BatchDeleteFilter,
  BatchDeleteParams,
  BatchDeleteProgressResponse,
  BatchDeleteStartResponse,
} from '../../types/recordings.ts';
import { ModalShell } from './common/ModalDialog.tsx';

declare global {
  interface Window {
    closeBatchDeleteModal?: () => void;
    batchDeleteRecordingsByHttpRequest?: (params: BatchDeleteParams) => Promise<BatchDeleteProgressResponse>;
    batchDeleteRecordings?: typeof batchDeleteRecordings;
    batchDeleteRecordingsByFilter?: typeof batchDeleteRecordingsByFilter;
  }
}

type BatchDeletePhase = 'hidden' | 'confirm' | 'progress';

interface PendingBatchDelete {
  params: BatchDeleteParams;
  resolve: (result: BatchDeleteProgressResponse) => void;
  reject: (reason: unknown) => void;
  running: boolean;
  controller: AbortController;
}

function cancelledResult(): BatchDeleteProgressResponse {
  return {status: 'cancelled', complete: true, current: 0, total: 0,
    succeeded: 0, failed: 0, pending_deletions: 0};
}

interface BatchDeleteProgressState {
  current: number;
  total: number;
  succeeded: number;
  failed: number;
  pending_deletions?: number;
  status: string;
  complete: boolean;
  error?: boolean;
}

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
  const { t } = useI18n();
  const queryClient = useQueryClient();

  // Modal phase: 'hidden' | 'confirm' | 'progress'
  const [phase, setPhase] = useState<BatchDeletePhase>('hidden');
  const cancelButtonRef = useRef<HTMLButtonElement>(null);

  // Each invocation owns its promise and requests; stale callbacks cannot touch a later one.
  const pendingRef = useRef<PendingBatchDelete | null>(null);
  const pollTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [progress, setProgress] = useState<BatchDeleteProgressState>({
    current: 0, total: 0, succeeded: 0, failed: 0,
    status: 'Preparing to delete recordings...', complete: false, error: false
  });

  const clearPollTimer = useCallback(() => {
    if (pollTimerRef.current !== null) {
      clearTimeout(pollTimerRef.current);
      pollTimerRef.current = null;
    }
  }, []);

  const getConfirmDescription = () => {
    const params = pendingRef.current?.params;
    if (!params) return '';
    if (params.ids) return t(params.ids.length === 1 ? 'recordings.selectedRecordingOne' : 'recordings.selectedRecordingMany', { count: params.ids.length });
    if (params.totalCount) return t(params.totalCount === 1 ? 'recordings.filteredRecordingOne' : 'recordings.filteredRecordingMany', { count: params.totalCount });
    return t('recordings.allMatchingRecordings');
  };

  // Closing confirmation settles its caller; a running server job cannot be cancelled.
  const closeModal = useCallback(() => {
    const pending = pendingRef.current;
    if (pending?.running) return;
    pendingRef.current = null;
    pending?.resolve(cancelledResult());
    clearPollTimer();
    setPhase('hidden');
  }, [clearPollTimer]);

  const cancelConfirm = useCallback(() => {
    if (!pendingRef.current || pendingRef.current.running) return;
    closeModal();
    showStatusMessage(t('recordings.batchDeleteCancelled'), 'info', 3000);
  }, [closeModal, t]);

  // Dismissed confirmations resolve with status "cancelled" and zero deletion counts.
  const batchDeleteRecordingsByHttpRequest = useCallback((params: BatchDeleteParams) => {
    // Do not replace another caller's pending confirmation or active operation.
    if (pendingRef.current) return Promise.reject(new Error('A batch deletion is already pending'));
    return new Promise<BatchDeleteProgressResponse>((resolve, reject) => {
      pendingRef.current = {params, resolve, reject, running: false, controller: new AbortController()};
      setPhase('confirm');
    });
  }, []);

  const proceedWithDelete = useCallback(async () => {
    const pending = pendingRef.current;
    if (!pending || pending.running) return;
    pending.running = true; // Guard a second click before the progress view renders.
    const {params, controller} = pending;
    const totalCount = params.ids?.length ?? params.totalCount ?? 0;
    const isCurrent = () => pendingRef.current === pending;
    setProgress({current: 0, total: totalCount, succeeded: 0, failed: 0,
      status: 'Starting batch delete operation...', complete: false, error: false});
    setPhase('progress');

    const fail = (reason: unknown) => {
      if (!isCurrent()) return;
      const error = reason instanceof Error ? reason : new Error(String(reason));
      pendingRef.current = null;
      clearPollTimer();
      setProgress({current: 0, total: 0, succeeded: 0, failed: 0,
        status: `Error: ${error.message}`, complete: true, error: true});
      showStatusMessage(t('recordings.errorMessage', {message: error.message}), 'error', 5000);
      pending.reject(error);
    };

    try {
      const response = await fetch('/api/recordings/batch-delete', {
        method: 'POST', headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(params), signal: controller.signal,
      });
      if (!response.ok) throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
      const result: BatchDeleteStartResponse = await response.json();
      if (!isCurrent()) return;
      if (!result.job_id) throw new Error('No job_id received from server');
      const jobId = result.job_id;

      const pollProgress = async () => {
        if (!isCurrent()) return;
        pollTimerRef.current = null;
        try {
          const response = await fetch(`/api/recordings/batch-delete/progress/${jobId}`, {signal: controller.signal});
          if (!response.ok) throw new Error(`HTTP error ${response.status}: ${response.statusText}`);
          const data: BatchDeleteProgressResponse = await response.json();
          if (!isCurrent()) return;
          if (data.status === 'error') throw new Error(data.error_message || 'Batch deletion failed');
          setProgress({current: data.current ?? 0, total: data.total ?? totalCount,
            succeeded: data.succeeded ?? 0, failed: data.failed ?? 0,
            pending_deletions: data.pending_deletions ?? 0,
            status: data.status_message || 'Processing...', complete: data.complete ?? false});
          if (data.complete) {
            pendingRef.current = null;
            pending.resolve(data);
            showStatusMessage(batchDeleteStatus(data), (data.failed ?? 0) === 0 ? 'success' : 'warning', 5000);
            void queryClient.invalidateQueries({queryKey: ['recordings']});
          } else {
            // Wait after each response, keeping at most one progress request in flight.
            pollTimerRef.current = setTimeout(pollProgress, 500);
          }
        } catch (error) {
          fail(error);
        }
      };
      await pollProgress();
    } catch (error) {
      fail(error);
    }
  }, [clearPollTimer, queryClient, t]);

  useEffect(() => {
    window.closeBatchDeleteModal = closeModal;
    window.batchDeleteRecordingsByHttpRequest = batchDeleteRecordingsByHttpRequest;
    return () => {
      const pending = pendingRef.current;
      pendingRef.current = null;
      clearPollTimer();
      if (pending) {
        // Abort client monitoring only; deletion may already be running on the server.
        pending.controller.abort();
        if (pending.running) pending.reject(new Error('Deletion monitoring stopped; the server job may continue'));
        else pending.resolve(cancelledResult());
      }
      if (window.closeBatchDeleteModal === closeModal) delete window.closeBatchDeleteModal;
      if (window.batchDeleteRecordingsByHttpRequest === batchDeleteRecordingsByHttpRequest) {
        delete window.batchDeleteRecordingsByHttpRequest;
      }
    };
  }, [closeModal, batchDeleteRecordingsByHttpRequest, clearPollTimer]);

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
      return t('recordings.inProgress');
    }
  };

  return (
    <ModalShell isOpen={phase !== 'hidden'}
      title={t(phase === 'confirm' ? 'recordings.confirmBatchDelete' : progress.complete ? 'recordings.batchDeleteComplete' : 'recordings.deletingRecordings')}
      onClose={phase === 'confirm' ? cancelConfirm : closeModal}
      closeDisabled={phase === 'progress' && !progress.complete} initialFocusRef={cancelButtonRef}>
      <div id="batch-delete-modal" className="p-5 max-h-[75dvh] overflow-y-auto">
        {/* ===== PHASE 1: Confirmation ===== */}
        {phase === 'confirm' && (
          <>
            <div className="mb-4">
              <div className="flex items-start gap-3 p-3 rounded-lg bg-red-500/10 border border-red-500/30 mb-4">
                <svg className="w-6 h-6 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
                </svg>
                <div>
                  <p className="font-semibold text-red-500 mb-1">{t('recordings.actionCannotBeUndone')}</p>
                  <p className="text-sm text-muted-foreground">
                    {t('recordings.aboutToPermanentlyDelete', { description: getConfirmDescription() })}
                    {' '}{t('recordings.batchDeleteIrreversible')}
                  </p>
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-2 pt-2 border-t border-border">
              <button
                className="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors"
                ref={cancelButtonRef}
                onClick={cancelConfirm}
              >
                {t('common.cancel')}
              </button>
              <button
                className="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors font-medium"
                onClick={proceedWithDelete}
              >
                {t('recordings.deleteDescription', { description: getConfirmDescription() })}
              </button>
            </div>
          </>
        )}

        {/* ===== PHASE 2: Progress (non-cancellable) ===== */}
        {phase === 'progress' && (
          <>
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
                  <span className="text-muted-foreground">{t('recordings.succeededLabel')}</span>
                  <span className="font-bold text-green-600 dark:text-green-400">{progress.succeeded}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">{t('recordings.failedLabel')}</span>
                  <span className="font-bold text-red-600 dark:text-red-400">{progress.failed}</span>
                </div>
              </div>

              {!progress.complete && (
                <div className="text-xs text-muted-foreground italic mb-4">
                  {t('recordings.batchDeleteCannotCancel')}
                </div>
              )}
            </div>

            <div className="flex justify-end pt-2 border-t border-border">
              {progress.complete ? (
                <button className="btn-primary" onClick={closeModal}>{t('common.done')}</button>
              ) : (
                <span className="text-sm text-muted-foreground italic">{t('recordings.pleaseWait')}</span>
              )}
            </div>
          </>
        )}

      </div>
    </ModalShell>
  );
}

/**
 * Delete multiple recordings by IDs.
 * Opens the confirmation modal — no browser confirm() used.
 * @param {Array<number | string>} ids - Recording IDs to delete
 */
export function batchDeleteRecordings(ids: Array<number | string>) {
  if (!ids || ids.length === 0) {
    showStatusMessage(translate('recordings.noSelectionForDeletion'), 'warning', 5000);
    return;
  }

  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ ids })
      .catch(error => {
        console.error('Error in batch delete:', error);
      });
  } else {
    showStatusMessage(translate('recordings.batchDeleteUnavailable'), 'error', 5000);
  }
}

/**
 * Delete recordings by filter.
 * Opens the confirmation modal — no browser confirm() used.
 * @param {Object} filter - Filter to delete by
 */
export function batchDeleteRecordingsByFilter(filter: BatchDeleteFilter) {
  if (!filter) {
    showStatusMessage(translate('recordings.noFilterForDeletion'), 'warning', 5000);
    return;
  }

  if (typeof window.batchDeleteRecordingsByHttpRequest === 'function') {
    window.batchDeleteRecordingsByHttpRequest({ filter })
      .catch(error => {
        console.error('Error in batch delete:', error);
      });
  } else {
    showStatusMessage(translate('recordings.batchDeleteUnavailable'), 'error', 5000);
  }
}

// Export for global use
if (typeof window !== 'undefined') {
  window.batchDeleteRecordings = batchDeleteRecordings;
  window.batchDeleteRecordingsByFilter = batchDeleteRecordingsByFilter;
}
