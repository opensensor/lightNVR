/**
 * StreamsInfo Component
 * Displays information about streams and recordings
 */

/**
 * StreamsInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} StreamsInfo component
 */
export function StreamsInfo({ systemInfo, formatBytes }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Streams & Recordings</h3>
      <div className="space-y-2">
        <div className="flex justify-between">
          <span className="font-medium">Active Streams:</span>
          <span>{systemInfo.streams?.active || 0} / {systemInfo.streams?.total || 0}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">Recordings:</span>
          <span>{systemInfo.recordings?.count || 0}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">Recordings Size:</span>
          <span>{systemInfo.recordings?.size ? formatBytes(systemInfo.recordings.size) : '0'}</span>
        </div>
      </div>
    </div>
  );
}
