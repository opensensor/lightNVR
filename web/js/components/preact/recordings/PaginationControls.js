/**
 * PaginationControls component for RecordingsView
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * PaginationControls component
 * @param {Object} props Component props
 * @returns {JSX.Element} PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  return html`
    <div class="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-gray-200 dark:border-gray-700">
      <div class="pagination-info text-sm text-gray-600 dark:text-gray-400 mb-2 sm:mb-0">
        Showing <span id="pagination-showing">${pagination.startItem}-${pagination.endItem}</span> of <span id="pagination-total">${pagination.totalItems}</span> recordings
      </div>
      <div class="pagination-buttons flex items-center space-x-1">
        <button id="pagination-first" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="First Page"
                onClick=${() => goToPage(1)}
                disabled=${pagination.currentPage === 1}>
          «
        </button>
        <button id="pagination-prev" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Previous Page"
                onClick=${() => goToPage(pagination.currentPage - 1)}
                disabled=${pagination.currentPage === 1}>
          ‹
        </button>
        <span id="pagination-current" class="px-2 text-sm text-gray-700 dark:text-gray-300">
          Page ${pagination.currentPage} of ${pagination.totalPages}
        </span>
        <button id="pagination-next" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Next Page"
                onClick=${() => goToPage(pagination.currentPage + 1)}
                disabled=${pagination.currentPage === pagination.totalPages}>
          ›
        </button>
        <button id="pagination-last" 
                class="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                title="Last Page"
                onClick=${() => goToPage(pagination.totalPages)}
                disabled=${pagination.currentPage === pagination.totalPages}>
          »
        </button>
      </div>
    </div>
  `;
}
