/**
 * StreamStorage Component
 * Displays storage usage per stream with slivers in a progress bar
 */

import { html } from '../../../html-helper.js';

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
    return html`
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
        <div class="text-gray-500 dark:text-gray-400 text-center py-4">
          No stream storage information available
        </div>
      </div>
    `;
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
  
  // Generate a color for each stream
  const colors = [
    'bg-blue-600',
    'bg-green-500',
    'bg-yellow-500',
    'bg-red-500',
    'bg-purple-500',
    'bg-pink-500',
    'bg-indigo-500',
    'bg-teal-500'
  ];
  
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Stream Storage</h3>
      
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Storage per Stream:</span>
            <div class="flex flex-wrap justify-end gap-1">
              ${streamStorageData.map((stream, index) => html`
                <span class="inline-block px-2 py-0.5 text-xs rounded ${colors[index % colors.length].replace('bg-', 'bg-opacity-20 bg-')} ${colors[index % colors.length].replace('bg-', 'text-')}">
                  ${stream.name}: ${formatBytes(stream.size)}
                </span>
              `)}
            </div>
          </div>
          
          <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mb-1">
            <span>Combined: ${formatBytes(totalStreamStorage)} / ${formatBytes(totalDiskSpace)}</span>
            <span>${totalStreamStoragePercent}% of total storage</span>
          </div>
          
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full" style=${`width: ${totalStreamStoragePercent}%`}>
              ${streamStorageData.map((stream, index) => html`
                <div class="${colors[index % colors.length]} h-2.5" style=${`width: ${stream.slicePercent}%`}></div>
              `)}
            </div>
          </div>
          
          <div class="mt-4">
            <h4 class="font-medium mb-2">Stream Details:</h4>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
              ${streamStorageData.map((stream, index) => html`
                <a href="recordings.html?stream=${encodeURIComponent(stream.name)}" 
                   class="flex items-center p-2 rounded bg-gray-50 dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors cursor-pointer">
                  <div class="w-3 h-3 rounded-full mr-2 ${colors[index % colors.length]}"></div>
                  <div>
                    <div class="font-medium">${stream.name}</div>
                    <div class="text-xs text-gray-500 dark:text-gray-400">
                      ${formatBytes(stream.size)} (${stream.slicePercent}%) • ${stream.count} recordings
                    </div>
                  </div>
                </a>
              `)}
            </div>
          </div>
        </div>
      </div>
    </div>
  `;
}
