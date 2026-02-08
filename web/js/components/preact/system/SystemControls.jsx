/**
 * SystemControls Component
 * Provides system restart control with a styled modal
 */

import { h } from 'preact';
import { useState } from 'preact/hooks';
import { RestartModal } from './RestartModal.jsx';

/**
 * SystemControls component
 * @param {Object} props Component props
 * @param {Function} props.onRestartConfirm Function to call when restart is confirmed
 * @param {boolean} props.isRestarting Whether lightNVR is currently restarting
 * @param {boolean} props.canControlSystem Whether the user has permission to control the system
 * @returns {JSX.Element} SystemControls component
 */
export function SystemControls({ onRestartConfirm, isRestarting, canControlSystem = true }) {
  const [showRestartModal, setShowRestartModal] = useState(false);

  const handleRestartClick = () => {
    setShowRestartModal(true);
  };

  const handleRestartConfirm = () => {
    if (onRestartConfirm) {
      onRestartConfirm();
    }
  };

  const handleCloseModal = () => {
    if (!isRestarting) {
      setShowRestartModal(false);
    }
  };

  return (
    <>
      <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
        <h2 className="text-xl font-bold">System</h2>
        <div className="controls space-x-2">
          {canControlSystem && (
            <button
              id="restart-btn"
              className="btn-warning focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              style={{ '--tw-ring-color': 'hsl(var(--warning))' }}
              onClick={handleRestartClick}
              disabled={isRestarting}
            >
              Restart lightNVR
            </button>
          )}
        </div>
      </div>

      <RestartModal
        isOpen={showRestartModal}
        onClose={handleCloseModal}
        onConfirm={handleRestartConfirm}
        isRestarting={isRestarting}
      />
    </>
  );
}
