/**
 * MemoryStorage Component
 * Displays memory and storage information with progress bars
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * MemoryStorage component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} MemoryStorage component
 */
export function MemoryStorage({ systemInfo, formatBytes }) {
  // Calculate the percentage of memory used by LightNVR and go2rtc
  const lightNvrMemoryPercent = systemInfo.memory?.total ? 
    (systemInfo.memory.used / systemInfo.memory.total * 100).toFixed(1) : 0;
  
  const go2rtcMemoryPercent = systemInfo.go2rtcMemory?.total ? 
    (systemInfo.go2rtcMemory.used / systemInfo.go2rtcMemory.total * 100).toFixed(1) : 0;
  
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Memory & Storage</h3>
      <div class="space-y-4">
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">Process Memory:</span>
            <div>
              <span class="inline-block px-2 py-0.5 mr-1 text-xs rounded bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                LightNVR: ${systemInfo.memory?.used ? formatBytes(systemInfo.memory.used) : '0'}
              </span>
              <span class="inline-block px-2 py-0.5 text-xs rounded bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200">
                go2rtc: ${systemInfo.go2rtcMemory?.used ? formatBytes(systemInfo.go2rtcMemory.used) : '0'}
              </span>
            </div>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 overflow-hidden">
            <div class="flex h-full">
              <div class="bg-blue-600 h-2.5" style=${`width: ${lightNvrMemoryPercent}%`}></div>
              <div class="bg-green-500 h-2.5" style=${`width: ${go2rtcMemoryPercent}%`}></div>
            </div>
          </div>
          <div class="flex justify-end mt-1">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Total: ${systemInfo.memory?.total ? formatBytes(systemInfo.memory.total) : '0'}
            </span>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Memory:</span>
            <span>${systemInfo.systemMemory?.used ? formatBytes(systemInfo.systemMemory.used) : '0'} / ${systemInfo.systemMemory?.total ? formatBytes(systemInfo.systemMemory.total) : '0'}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${systemInfo.systemMemory?.total ? (systemInfo.systemMemory.used / systemInfo.systemMemory.total * 100).toFixed(1) : 0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">LightNVR Storage:</span>
            <span>${systemInfo.disk?.used ? formatBytes(systemInfo.disk.used) : '0'} / ${systemInfo.disk?.total ? formatBytes(systemInfo.disk.total) : '0'}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${systemInfo.disk?.total ? (systemInfo.disk.used / systemInfo.disk.total * 100).toFixed(1) : 0}%`}></div>
          </div>
        </div>
        <div>
          <div class="flex justify-between mb-1">
            <span class="font-medium">System Storage:</span>
            <span>${systemInfo.systemDisk?.used ? formatBytes(systemInfo.systemDisk.used) : '0'} / ${systemInfo.systemDisk?.total ? formatBytes(systemInfo.systemDisk.total) : '0'}</span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${systemInfo.systemDisk?.total ? (systemInfo.systemDisk.used / systemInfo.systemDisk.total * 100).toFixed(1) : 0}%`}></div>
          </div>
        </div>
      </div>
    </div>
  `;
}
