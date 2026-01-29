/**
 * PaginationControls component for RecordingsView
 */

import { h } from 'preact';

/**
 * PaginationControls component
 * @param {Object} props Component props
 * @returns {JSX.Element} PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        Showing <span id="pagination-showing">{pagination.startItem}-{pagination.endItem}</span> of <span id="pagination-total">{pagination.totalItems}</span> recordings
      </div>
      <div className="pagination-buttons flex items-center space-x-1">
        <button id="pagination-first"
                className="w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                title="First Page"
                onClick={() => goToPage(1)}
                disabled={pagination.currentPage === 1}>
          «
        </button>
        <button id="pagination-prev"
                className="w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                title="Previous Page"
                onClick={() => goToPage(pagination.currentPage - 1)}
                disabled={pagination.currentPage === 1}>
          ‹
        </button>
        <span id="pagination-current" className="px-2 text-sm text-foreground">
          Page {pagination.currentPage} of {pagination.totalPages}
        </span>
        <button id="pagination-next"
                className="w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                title="Next Page"
                onClick={() => goToPage(pagination.currentPage + 1)}
                disabled={pagination.currentPage === pagination.totalPages}>
          ›
        </button>
        <button id="pagination-last"
                className="w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
                title="Last Page"
                onClick={() => goToPage(pagination.totalPages)}
                disabled={pagination.currentPage === pagination.totalPages}>
          »
        </button>
      </div>
    </div>
  );
}
