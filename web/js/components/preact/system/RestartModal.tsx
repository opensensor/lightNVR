import {getSystemStatus} from '../../../api/system.ts';
/**
 * RestartModal Component
 * A styled modal for confirming and showing restart progress
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { createPortal } from 'preact/compat';
import { useI18n } from '../../../i18n.ts';
import { AsyncButton } from '../AsyncButton.tsx';

export interface RestartModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => unknown;
  isRestarting: boolean;
}

/**
 * RestartModal component
 * @param {Object} props Component props
 * @param {boolean} props.isOpen Whether the modal is open
 * @param {Function} props.onClose Function to close the modal
 * @param {Function} props.onConfirm Function to call when restart is confirmed
 * @param {boolean} props.isRestarting Whether the system is currently restarting
 * @returns {JSX.Element} RestartModal component
 */
export function RestartModal({ isOpen, onClose, onConfirm, isRestarting }: RestartModalProps) {
  const modalRef = useRef<HTMLDivElement | null>(null);
  const [observationExpired, setObservationExpired] = useState(false);
  const [reconnectStatus, setReconnectStatus] = useState('');
  const [serverDown, setServerDown] = useState(false);
  const { t } = useI18n();

  // Handle escape key (only when not restarting)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen && !isRestarting) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, isRestarting, onClose]);

  // Bound observation even when a fast restart's outage is never seen.
  useEffect(() => {
    setObservationExpired(false);
    setServerDown(false);
    if (!isRestarting) {
      setReconnectStatus('');
      return;
    }

    setReconnectStatus(t('system.initiatingRestart'));
    const startedAt = Date.now();
    const controller = new AbortController();
    let finished = false;
    let observedOutage = false;
    let pollTimer: ReturnType<typeof setTimeout>;
    let reloadTimer: ReturnType<typeof setTimeout> | undefined;
    const deadline = setTimeout(() => {
      finished = true;
      clearTimeout(pollTimer);
      controller.abort();
      setObservationExpired(true);
      setReconnectStatus(t('system.serverTakingLonger'));
    }, 120000);

    const poll = async () => {
      try {
        await getSystemStatus({
          signal: controller.signal,
          timeout: observedOutage ? 2000 : 1000,
          retries: 0
        });
        if (finished) return;
        // Healthy probes alone cannot confirm that a restart occurred.
        if (observedOutage) {
          finished = true;
          clearTimeout(deadline);
          setReconnectStatus(t('system.serverBackOnline'));
          reloadTimer = setTimeout(() => window.location.reload(), 1000);
          return;
        }
      } catch {
        if (finished) return;
        if (!observedOutage) {
          observedOutage = true;
          setServerDown(true);
          setReconnectStatus(t('system.serverShuttingDown'));
        } else {
          setReconnectStatus(t('system.waitingForServerRestart', {
            seconds: Math.floor((Date.now() - startedAt) / 1000)
          }));
        }
      }
      pollTimer = setTimeout(poll, observedOutage ? 2000 : 500);
    };
    pollTimer = setTimeout(poll, 500);

    return () => {
      finished = true;
      controller.abort();
      clearTimeout(deadline);
      clearTimeout(pollTimer);
      clearTimeout(reloadTimer);
    };
  }, [isRestarting, t]);

  // Handle background click (only when not restarting)
  const handleBackgroundClick = (e: MouseEvent) => {
    if (e.target === e.currentTarget && !isRestarting) {
      onClose();
    }
  };

  if (!isOpen) return null;

  return createPortal(
    <div role="presentation"
      ref={modalRef}
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={handleBackgroundClick}
    >
      <div className="bg-card text-card-foreground rounded-lg shadow-xl p-6 max-w-md w-full mx-4 transform transition-all duration-300 ease-out">
        {!isRestarting ? (
          // Confirmation state
          <>
            <div className="flex items-center mb-4">
              <div className="flex-shrink-0 w-12 h-12 rounded-full bg-yellow-100 dark:bg-yellow-900/30 flex items-center justify-center me-4">
                <svg className="w-6 h-6 text-yellow-600 dark:text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </div>
              <div>
                <h3 className="text-lg font-semibold">{t('system.restartLightNvr')}</h3>
                <p className="text-sm text-muted-foreground">{t('system.restartServiceDescription')}</p>
              </div>
            </div>

            <p className="text-muted-foreground mb-6">
              {t('system.restartConfirmation')}
            </p>

            <div className="flex justify-end space-x-3">
              <button
                className="px-4 py-2 bg-muted text-muted-foreground rounded-md hover:bg-muted/80 transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-muted"
                onClick={onClose}
              >
                {t('common.cancel')}
              </button>
              {/* AsyncButton — locks + spins the brief window between
                  click and the parent flipping isRestarting=true so a
                  rapid second tap can't fire two restart POSTs (#399 /
                  PRD UXD_01 §5.1 / T1). */}
              <AsyncButton
                className="btn-warning focus:outline-none focus:ring-2 focus:ring-offset-2"
                onClick={() => Promise.resolve(onConfirm())}
              >
                {t('system.restart')}
              </AsyncButton>
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

              <h3 className="text-lg font-semibold mb-2">{t('system.restartingLightNvr')}</h3>
              <p className="text-sm text-muted-foreground text-center mb-2">
                {reconnectStatus || t('system.initiatingRestart')}
              </p>

              {!serverDown && (
                <p className="text-xs text-muted-foreground/70 text-center mb-4">
                  {t('system.restartMayTakeOneToTwoMinutes')}
                </p>
              )}

              {observationExpired && (
                <button
                  className="mt-4 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
                  onClick={() => window.location.reload()}
                >
                  {t('system.refreshPage')}
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

