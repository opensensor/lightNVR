/**
 * SystemInfo Component
 * Displays basic system information like version, uptime, and CPU details
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * SystemInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatUptime Function to format uptime
 * @returns {JSX.Element} SystemInfo component
 */
export function SystemInfo({ systemInfo, formatUptime }) {
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">System Information</h3>
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Version:</span>
          <span>${systemInfo.version || 'Unknown'}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Uptime:</span>
          <span>${systemInfo.uptime ? formatUptime(systemInfo.uptime) : 'Unknown'}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">CPU Model:</span>
          <span>${systemInfo.cpu?.model || 'Unknown'}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">CPU Cores:</span>
          <span>${systemInfo.cpu?.cores || 'Unknown'}</span>
        </div>
        <div class="flex justify-between items-center">
          <span class="font-medium">CPU Usage:</span>
          <div class="w-32 bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${systemInfo.cpu?.usage || 0}%`}></div>
          </div>
          <span>${systemInfo.cpu?.usage ? `${systemInfo.cpu.usage.toFixed(1)}%` : 'Unknown'}</span>
        </div>
      </div>
    </div>
  `;
}
