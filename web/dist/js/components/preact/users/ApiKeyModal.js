/**
 * API Key Modal Component
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * API Key Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user for API key generation
 * @param {string} props.newApiKey - Newly generated API key
 * @param {Function} props.handleGenerateApiKey - Function to handle API key generation
 * @param {Function} props.copyApiKey - Function to copy API key to clipboard
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} API key modal
 */
export function ApiKeyModal({ currentUser, newApiKey, handleGenerateApiKey, copyApiKey, onClose }) {
  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  return html`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${onClose}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick=${stopPropagation}>
        <h2 class="text-xl font-bold mb-4">API Key for ${currentUser.username}</h2>
        
        <div class="mb-6">
          ${newApiKey ? html`
            <div class="mb-4">
              <label class="block text-sm font-bold mb-2">
                API Key
              </label>
              <div class="flex">
                <input
                  class="shadow appearance-none border rounded-l w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                  type="text"
                  value=${newApiKey}
                  readonly
                />
                <button
                  class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-r"
                  onClick=${copyApiKey}
                >
                  Copy
                </button>
              </div>
              <p class="text-sm text-gray-600 dark:text-gray-300 mt-2">
                This key will only be shown once. Save it securely.
              </p>
            </div>
          ` : html`
            <p class="mb-4">
              Generate a new API key for this user. This will invalidate any existing API key.
            </p>
            <button
              class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
              onClick=${handleGenerateApiKey}
            >
              Generate New API Key
            </button>
          `}
        </div>
        
        <div class="flex justify-end">
          <button
            class="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500"
            onClick=${onClose}
          >
            Close
          </button>
        </div>
      </div>
    </div>
  `;
}
