/**
 * NetworkInfo Component
 * Displays network interface information
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * NetworkInfo component
 * @param {Object} props Component props
 * @param {Object} props.systemInfo System information object
 * @returns {JSX.Element} NetworkInfo component
 */
export function NetworkInfo({ systemInfo }) {
  return html`
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-gray-200 dark:border-gray-700">Network Interfaces</h3>
      <div class="space-y-2">
        ${systemInfo.network?.interfaces?.length ? systemInfo.network.interfaces.map(iface => html`
          <div key=${iface.name} class="mb-2 pb-2 border-b border-gray-100 dark:border-gray-700 last:border-0">
            <div class="flex justify-between">
              <span class="font-medium">${iface.name}:</span>
              <span>${iface.address || 'No IP'}</span>
            </div>
            <div class="text-sm text-gray-500 dark:text-gray-400">
              MAC: ${iface.mac || 'Unknown'} | ${iface.up ? 'Up' : 'Down'}
            </div>
          </div>
        `) : html`<div class="text-gray-500 dark:text-gray-400">No network interfaces found</div>`}
      </div>
    </div>
  `;
}
