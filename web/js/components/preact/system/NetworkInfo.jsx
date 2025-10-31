/**
 * NetworkInfo Component
 * Displays network interface information
 */

/**
 * NetworkInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @returns {JSX.Element} NetworkInfo component
 */
export function NetworkInfo({ systemInfo }) {
  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Network Interfaces</h3>
      <div className="space-y-2">
        {systemInfo.network?.interfaces?.length ? (
          systemInfo.network.interfaces.map(iface => (
            <div key={iface.name} className="mb-2 pb-2 border-b border-gray-100 dark:border-gray-700 last:border-0">
              <div className="flex justify-between">
                <span className="font-medium">{iface.name}:</span>
                <span>{iface.address || 'No IP'}</span>
              </div>
              <div className="text-sm text-muted-foreground">
                MAC: {iface.mac || 'Unknown'} | {iface.up ? 'Up' : 'Down'}
              </div>
            </div>
          ))
        ) : (
          <div className="text-muted-foreground">No network interfaces found</div>
        )}
      </div>
    </div>
  );
}
