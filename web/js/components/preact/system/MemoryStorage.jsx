/**
 * MemoryStorage Component
 * Displays memory and storage information with progress bars
 */

/**
 * MemoryStorage component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} MemoryStorage component
 */
export function MemoryStorage({ systemInfo, formatBytes }) {
  // Get memory usage values
  const lightNvrMemoryUsed = systemInfo.memory?.used || 0;
  const go2rtcMemoryUsed = systemInfo.go2rtcMemory?.used || 0;
  const detectorMemoryUsed = systemInfo.detectorMemory?.used || 0;
  const totalSystemMemory = systemInfo.memory?.total || 0;

  // Calculate combined memory usage (all three processes)
  const combinedMemoryUsed = lightNvrMemoryUsed + go2rtcMemoryUsed + detectorMemoryUsed;

  // Calculate the percentage of total system memory used by all processes combined
  const combinedMemoryPercent = totalSystemMemory ?
    (combinedMemoryUsed / totalSystemMemory * 100).toFixed(1) : 0;

  // Calculate the percentage of each process relative to their combined usage
  // This ensures the slivers add up to the total width of the progress bar
  const lightNvrSlicePercent = combinedMemoryUsed ?
    (lightNvrMemoryUsed / combinedMemoryUsed * 100).toFixed(1) : 0;

  const go2rtcSlicePercent = combinedMemoryUsed ?
    (go2rtcMemoryUsed / combinedMemoryUsed * 100).toFixed(1) : 0;

  const detectorSlicePercent = combinedMemoryUsed ?
    (detectorMemoryUsed / combinedMemoryUsed * 100).toFixed(1) : 0;

  // These variables ensure the slivers add up to 100% of the combined usage bar

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Memory & Storage</h3>
      <div className="space-y-4">
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">Process Memory:</span>
            <div className="flex flex-wrap justify-end gap-1">
              <span className="inline-block px-2 py-0.5 text-xs rounded" style={{backgroundColor: 'hsl(var(--primary-muted))', color: 'hsl(var(--primary))'}}>
                LightNVR: {formatBytes(lightNvrMemoryUsed)}
              </span>
              <span className="inline-block px-2 py-0.5 text-xs rounded badge-success">
                go2rtc: {formatBytes(go2rtcMemoryUsed)}
              </span>
              <span className="inline-block px-2 py-0.5 text-xs rounded badge-warning">
                detector: {formatBytes(detectorMemoryUsed)}
              </span>
            </div>
          </div>
          <div className="flex justify-between text-xs text-muted-foreground mb-1">
            <span>Combined: {formatBytes(combinedMemoryUsed)} / {formatBytes(totalSystemMemory)}</span>
            <span>{combinedMemoryPercent}% of total memory</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div className="flex h-full" style={{ width: `${combinedMemoryPercent}%` }}>
              <div className="h-2.5" style={{ width: `${lightNvrSlicePercent}%`, backgroundColor: 'hsl(var(--primary))' }}></div>
              <div className="h-2.5" style={{ width: `${go2rtcSlicePercent}%`, backgroundColor: 'hsl(var(--success))' }}></div>
              <div className="h-2.5" style={{ width: `${detectorSlicePercent}%`, backgroundColor: 'hsl(var(--warning))' }}></div>
            </div>
          </div>
        </div>
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">System Memory:</span>
            <span>
              {systemInfo.systemMemory?.used ? formatBytes(systemInfo.systemMemory.used) : '0'} /
              {systemInfo.systemMemory?.total ? formatBytes(systemInfo.systemMemory.total) : '0'}
            </span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="h-2.5 rounded-full" style={{backgroundColor: 'hsl(var(--primary))'}}
              style={{
                width: `${systemInfo.systemMemory?.total ?
                  (systemInfo.systemMemory.used / systemInfo.systemMemory.total * 100).toFixed(1) : 0}%`
              }}
            ></div>
          </div>
        </div>
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">LightNVR Storage:</span>
            <span>
              {systemInfo.disk?.used ? formatBytes(systemInfo.disk.used) : '0'} /
              {systemInfo.disk?.total ? formatBytes(systemInfo.disk.total) : '0'}
            </span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="h-2.5 rounded-full" style={{backgroundColor: 'hsl(var(--primary))'}}
              style={{
                width: `${systemInfo.disk?.total ?
                  (systemInfo.disk.used / systemInfo.disk.total * 100).toFixed(1) : 0}%`
              }}
            ></div>
          </div>
        </div>
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">System Storage:</span>
            <span>
              {systemInfo.systemDisk?.used ? formatBytes(systemInfo.systemDisk.used) : '0'} /
              {systemInfo.systemDisk?.total ? formatBytes(systemInfo.systemDisk.total) : '0'}
            </span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="h-2.5 rounded-full" style={{backgroundColor: 'hsl(var(--primary))'}}
              style={{
                width: `${systemInfo.systemDisk?.total ?
                  (systemInfo.systemDisk.used / systemInfo.systemDisk.total * 100).toFixed(1) : 0}%`
              }}
            ></div>
          </div>
        </div>
      </div>
    </div>
  );
}
