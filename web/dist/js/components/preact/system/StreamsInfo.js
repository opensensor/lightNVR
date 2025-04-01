/**
 * StreamsInfo Component
 * Displays information about streams and recordings
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * StreamsInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @param {Function} props.formatBytes Function to format bytes to human-readable size
 * @returns {JSX.Element} StreamsInfo component
 */
export function StreamsInfo({ systemInfo, formatBytes }) {
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Streams & Recordings</h3>
      <div class="space-y-2">
        <div class="flex justify-between">
          <span class="font-medium">Active Streams:</span>
          <span>${systemInfo.streams?.active || 0} / ${systemInfo.streams?.total || 0}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Recordings:</span>
          <span>${systemInfo.recordings?.count || 0}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-medium">Recordings Size:</span>
          <span>${systemInfo.recordings?.size ? formatBytes(systemInfo.recordings.size) : '0'}</span>
        </div>
      </div>
    </div>
  `;
}
