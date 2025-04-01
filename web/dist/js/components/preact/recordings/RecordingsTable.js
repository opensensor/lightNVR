/**
 * RecordingsTable component for RecordingsView
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { formatUtils } from './formatUtils.js';

/**
 * RecordingsTable component
 * @param {Object} props Component props
 * @returns {JSX.Element} RecordingsTable component
 */
export function RecordingsTable({
  recordings,
  sortField,
  sortDirection,
  sortBy,
  selectedRecordings,
  toggleRecordingSelection,
  selectAll,
  toggleSelectAll,
  getSelectedCount,
  openDeleteModal,
  playRecording,
  downloadRecording,
  deleteRecording,
  recordingsTableBodyRef,
  pagination
}) {
  return html`
    <div class="recordings-container bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
      <div class="batch-actions p-3 border-b border-gray-200 dark:border-gray-700 flex flex-wrap gap-2 items-center">
        <div class="selected-count text-sm text-gray-600 dark:text-gray-400 mr-2">
          ${getSelectedCount() > 0 ? 
            `${getSelectedCount()} recording${getSelectedCount() !== 1 ? 's' : ''} selected` : 
            'No recordings selected'}
        </div>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          disabled=${getSelectedCount() === 0}
          onClick=${() => openDeleteModal('selected')}>
          Delete Selected
        </button>
        <button 
          class="px-3 py-1.5 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
          onClick=${() => openDeleteModal('all')}>
          Delete All Filtered
        </button>
      </div>
      <div class="overflow-x-auto">
        <table id="recordings-table" class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th class="w-10 px-4 py-3">
                <input 
                  type="checkbox" 
                  checked=${selectAll}
                  onChange=${toggleSelectAll}
                  class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                />
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${() => sortBy('stream_name')}>
                <div class="flex items-center">
                  Stream
                  ${sortField === 'stream_name' && html`
                    <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${() => sortBy('start_time')}>
                <div class="flex items-center">
                  Start Time
                  ${sortField === 'start_time' && html`
                    <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Duration
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider cursor-pointer"
                  onClick=${() => sortBy('size_bytes')}>
                <div class="flex items-center">
                  Size
                  ${sortField === 'size_bytes' && html`
                    <span class="sort-icon ml-1">${sortDirection === 'asc' ? '▲' : '▼'}</span>
                  `}
                </div>
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Detections
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody ref=${recordingsTableBodyRef} class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
            ${recordings.length === 0 ? html`
              <tr>
                <td colspan="6" class="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                  ${pagination.totalItems === 0 ? 'No recordings found' : 'Loading recordings...'}
                </td>
              </tr>
            ` : recordings.map(recording => html`
              <tr key=${recording.id} class="hover:bg-gray-50 dark:hover:bg-gray-700">
                <td class="px-4 py-4 whitespace-nowrap">
                  <input 
                    type="checkbox" 
                    checked=${!!selectedRecordings[recording.id]}
                    onChange=${() => toggleRecordingSelection(recording.id)}
                    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                  />
                </td>
                <td class="px-6 py-4 whitespace-nowrap">${recording.stream || ''}</td>
                <td class="px-6 py-4 whitespace-nowrap">${formatUtils.formatDateTime(recording.start_time)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${formatUtils.formatDuration(recording.duration)}</td>
                <td class="px-6 py-4 whitespace-nowrap">${recording.size || ''}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                  ${recording.has_detections ? html`
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 dark:bg-green-800 dark:text-green-100">
                      <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                        <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                      </svg>
                      Yes
                    </span>
                  ` : ''}
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex space-x-2">
                    <button class="p-1 rounded-full text-blue-600 hover:bg-blue-100 dark:text-blue-400 dark:hover:bg-blue-900 focus:outline-none"
                            onClick=${() => playRecording(recording)}
                            title="Play">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button class="p-1 rounded-full text-green-600 hover:bg-green-100 dark:text-green-400 dark:hover:bg-green-900 focus:outline-none"
                            onClick=${() => downloadRecording(recording)}
                            title="Download">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button class="p-1 rounded-full text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900 focus:outline-none"
                            onClick=${() => deleteRecording(recording)}
                            title="Delete">
                      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                        <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
            `)}
          </tbody>
        </table>
      </div>
    </div>
  `;
}
