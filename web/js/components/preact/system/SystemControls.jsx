/**
 * SystemControls Component
 * Provides system restart and shutdown controls
 */

/**
 * SystemControls component
 * @param {Object} props Component props
 * @param {Function} props.restartSystem Function to restart the system
 * @param {Function} props.shutdownSystem Function to shut down the system
 * @param {boolean} props.isRestarting Whether the system is currently restarting
 * @param {boolean} props.isShuttingDown Whether the system is currently shutting down
 * @param {boolean} props.canControlSystem Whether the user has permission to control the system
 * @returns {JSX.Element} SystemControls component
 */
export function SystemControls({ restartSystem, shutdownSystem, isRestarting, isShuttingDown, canControlSystem = true }) {
  return (
    <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
      <h2 className="text-xl font-bold">System</h2>
      <div className="controls space-x-2">
        {canControlSystem && (
          <>
            <button
              id="restart-btn"
              className="btn-warning focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={restartSystem}
              disabled={isRestarting || isShuttingDown}
            >
              Restart
            </button>
            <button
              id="shutdown-btn"
              className="btn-danger focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={shutdownSystem}
              disabled={isRestarting || isShuttingDown}
            >
              Shutdown
            </button>
          </>
        )}
      </div>
    </div>
  );
}
