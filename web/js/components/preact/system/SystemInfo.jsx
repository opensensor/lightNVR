/**
 * SystemInfo Component
 * Displays basic system information like version, uptime, and CPU details
 */

/**
 * SystemInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatUptime Function to format uptime
 * @returns {JSX.Element} SystemInfo component
 */
export function SystemInfo({ systemInfo, formatUptime }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">System Information</h3>
      <div className="space-y-2">
        <div className="flex justify-between">
          <span className="font-medium">Version:</span>
          <span>{systemInfo.version || 'Unknown'}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">Uptime:</span>
          <span>{systemInfo.uptime ? formatUptime(systemInfo.uptime) : 'Unknown'}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">CPU Model:</span>
          <span>{systemInfo.cpu?.model || 'Unknown'}</span>
        </div>
        <div className="flex justify-between">
          <span className="font-medium">CPU Cores:</span>
          <span>{systemInfo.cpu?.cores || 'Unknown'}</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="font-medium">CPU Usage:</span>
          <div className="w-32 bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="h-2.5 rounded-full"
              style={{ backgroundColor: 'hsl(var(--primary))', width: `${systemInfo.cpu?.usage || 0}%` }}
            ></div>
          </div>
          <span>{systemInfo.cpu?.usage ? `${systemInfo.cpu.usage.toFixed(1)}%` : 'Unknown'}</span>
        </div>
      </div>
    </div>
  );
}
