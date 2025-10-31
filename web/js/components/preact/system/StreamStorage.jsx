/**
 * StreamStorage Component
 * Displays storage usage per stream with slivers in a progress bar
 */

/**
 * StreamStorage component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} StreamStorage component
 */
export function StreamStorage({ systemInfo, formatBytes }) {
  // Check if stream storage information is available
  if (!systemInfo.streamStorage || !Array.isArray(systemInfo.streamStorage) || systemInfo.streamStorage.length === 0) {
    return (
      <div className="bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Stream Storage</h3>
        <div className="text-muted-foreground text-center py-4">
          No stream storage information available
        </div>
      </div>
    );
  }

  // Calculate total storage used by all streams
  const totalStreamStorage = systemInfo.streamStorage.reduce((total, stream) => total + stream.size, 0);

  // Calculate the percentage of total disk space used by all streams
  const totalDiskSpace = systemInfo.disk?.total || 0;
  const totalStreamStoragePercent = totalDiskSpace ?
    (totalStreamStorage / totalDiskSpace * 100).toFixed(1) : 0;

  // Calculate the percentage of each stream relative to the total stream storage
  const streamStorageData = systemInfo.streamStorage.map(stream => ({
    name: stream.name,
    size: stream.size,
    count: stream.count,
    slicePercent: totalStreamStorage ? (stream.size / totalStreamStorage * 100).toFixed(1) : 0
  }));

  // Sort streams by size (largest first)
  streamStorageData.sort((a, b) => b.size - a.size);

  // Generate theme-aware colors for each stream
  const getStreamColor = (index) => {
    const hues = [217, 142, 38, 0, 265, 350, 215, 180]; // primary, success, warning, danger, purple, rose, slate, teal
    const hue = hues[index % hues.length];
    return {
      bg: `hsl(${hue} 70% 50% / 0.2)`,
      text: `hsl(${hue} 70% 40%)`
    };
  };

  return (
    <div className="bg-card text-card-foreground rounded-lg shadow p-4">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-border">Stream Storage</h3>

      <div className="space-y-4">
        <div>
          <div className="flex justify-between mb-1">
            <span className="font-medium">Storage per Stream:</span>
            <div className="flex flex-wrap justify-end gap-1">
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                return (
                  <span
                    key={stream.name}
                    className="inline-block px-2 py-0.5 text-xs rounded"
                    style={{ backgroundColor: color.bg, color: color.text }}
                  >
                    {stream.name}: {formatBytes(stream.size)}
                  </span>
                );
              })}
            </div>
          </div>

          <div className="flex justify-between text-xs text-muted-foreground mb-1">
            <span>Combined: {formatBytes(totalStreamStorage)} / {formatBytes(totalDiskSpace)}</span>
            <span>{totalStreamStoragePercent}% of total storage</span>
          </div>

          <div className="w-full bg-muted rounded-full h-2.5 overflow-hidden">
            <div className="flex h-full" style={{ width: `${totalStreamStoragePercent}%` }}>
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                return (
                  <div
                    key={stream.name}
                    className="h-2.5"
                    style={{ width: `${stream.slicePercent}%`, backgroundColor: color.text }}
                  ></div>
                );
              })}
            </div>
          </div>

          <div className="mt-4">
            <h4 className="font-medium mb-2">Stream Details:</h4>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {streamStorageData.map((stream, index) => {
                const color = getStreamColor(index);
                return (
                  <a
                    key={stream.name}
                    href={`recordings.html?stream=${encodeURIComponent(stream.name)}`}
                    className="flex items-center p-2 rounded bg-muted hover:bg-muted/70 transition-colors cursor-pointer"
                  >
                    <div className="w-3 h-3 rounded-full mr-2" style={{ backgroundColor: color.text }}></div>
                    <div>
                      <div className="font-medium">{stream.name}</div>
                      <div className="text-xs text-muted-foreground">
                        {formatBytes(stream.size)} ({stream.slicePercent}%) • {stream.count} recordings
                      </div>
                    </div>
                  </a>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
