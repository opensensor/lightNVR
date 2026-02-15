/**
 * PaginationControls component for RecordingsView
 */

import { h } from 'preact';

const NAV_BTN = "w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed text-sm";
const PAGE_BTN = "w-8 h-8 flex items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-primary text-sm";
const PAGE_BTN_ACTIVE = "bg-primary text-primary-foreground";
const PAGE_BTN_INACTIVE = "bg-secondary text-secondary-foreground hover:bg-secondary/80";

/**
 * Build the list of page numbers to display.
 * Always shows first page, last page, current page, and up to 3 pages
 * on each side of the current page. Gaps are represented by null.
 */
function getPageNumbers(current, total) {
  if (total <= 1) return [1];

  const pages = new Set();
  // Always include first and last
  pages.add(1);
  pages.add(total);
  // Include current and up to 3 on each side
  for (let i = current - 3; i <= current + 3; i++) {
    if (i >= 1 && i <= total) pages.add(i);
  }

  const sorted = Array.from(pages).sort((a, b) => a - b);

  // Insert nulls for gaps
  const result = [];
  for (let i = 0; i < sorted.length; i++) {
    if (i > 0 && sorted[i] - sorted[i - 1] > 1) {
      result.push(null); // ellipsis
    }
    result.push(sorted[i]);
  }
  return result;
}

/**
 * PaginationControls component
 * @param {Object} props Component props
 * @returns {JSX.Element} PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  const { currentPage, totalPages } = pagination;
  const pageNumbers = getPageNumbers(currentPage, totalPages);

  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        Showing <span>{pagination.startItem}-{pagination.endItem}</span> of <span>{pagination.totalItems}</span> recordings
      </div>
      <div className="pagination-buttons flex items-center space-x-1">
        {/* First */}
        <button className={NAV_BTN} title="First Page"
                onClick={() => goToPage(1)} disabled={currentPage === 1}>
          «
        </button>
        {/* Previous */}
        <button className={NAV_BTN} title="Previous Page"
                onClick={() => goToPage(currentPage - 1)} disabled={currentPage === 1}>
          ‹
        </button>

        {/* Numbered pages */}
        {pageNumbers.map((page, idx) =>
          page === null ? (
            <span key={`ellipsis-${idx}`} className="w-6 text-center text-muted-foreground text-sm select-none">…</span>
          ) : (
            <button
              key={page}
              className={`${PAGE_BTN} ${page === currentPage ? PAGE_BTN_ACTIVE : PAGE_BTN_INACTIVE}`}
              onClick={() => goToPage(page)}
              title={`Page ${page}`}
            >
              {page}
            </button>
          )
        )}

        {/* Next */}
        <button className={NAV_BTN} title="Next Page"
                onClick={() => goToPage(currentPage + 1)} disabled={currentPage === totalPages}>
          ›
        </button>
        {/* Last */}
        <button className={NAV_BTN} title="Last Page"
                onClick={() => goToPage(totalPages)} disabled={currentPage === totalPages}>
          »
        </button>
      </div>
    </div>
  );
}
