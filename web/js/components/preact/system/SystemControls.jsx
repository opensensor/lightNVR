/**
 * SystemControls Component
 * Provides system restart control
 */

/**
 * SystemControls component
 * @param {Object} props Component props
 * @param {Function} props.restartSystem Function to restart lightNVR
 * @param {boolean} props.isRestarting Whether lightNVR is currently restarting
 * @param {boolean} props.canControlSystem Whether the user has permission to control the system
 * @returns {JSX.Element} SystemControls component
 */
export function SystemControls({ restartSystem, isRestarting, canControlSystem = true }) {
  return (
    <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
      <h2 className="text-xl font-bold">System</h2>
      <div className="controls space-x-2">
        {canControlSystem && (
          <button
            id="restart-btn"
            className="btn-warning focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
            onClick={restartSystem}
            disabled={isRestarting}
          >
            Restart lightNVR
          </button>
        )}
      </div>
    </div>
  );
}
