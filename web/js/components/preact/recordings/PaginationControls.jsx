/**
 * PaginationControls component for RecordingsView
 */

import { h } from 'preact';

const NAV_BTN = "w-8 h-8 flex items-center justify-center rounded-full bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed text-sm";
const PAGE_BTN = "w-8 h-8 flex items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-primary text-sm";
const PAGE_BTN_ACTIVE = "bg-primary text-primary-foreground";
const PAGE_BTN_INACTIVE = "bg-secondary text-secondary-foreground hover:bg-secondary/80";
const EDGE_BTN = "h-8 px-2 flex items-center justify-center gap-0.5 rounded-full focus:outline-none focus:ring-2 focus:ring-primary text-sm";

/**
 * Build the list of interior page numbers to display (excluding first & last).
 * Shows up to 3 pages on each side of the current page.
 */
function getMiddlePages(current, total) {
  if (total <= 2) return [];

  const pages = new Set();
  for (let i = current - 3; i <= current + 3; i++) {
    if (i > 1 && i < total) pages.add(i);
  }

  return Array.from(pages).sort((a, b) => a - b);
}

/**
 * PaginationControls component
 * @param {Object} props Component props
 * @returns {JSX.Element} PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  const { currentPage, totalPages } = pagination;
  const middlePages = getMiddlePages(currentPage, totalPages);

  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        Showing <span>{pagination.startItem}-{pagination.endItem}</span> of <span>{pagination.totalItems}</span> recordings
      </div>
      <div className="pagination-buttons flex items-center space-x-1">
        {/* First page hybrid: number then « */}
        <button
          className={`${EDGE_BTN} ${currentPage === 1 ? PAGE_BTN_ACTIVE : PAGE_BTN_INACTIVE}`}
          onClick={() => goToPage(1)}
          title="First Page"
        >
          <span>1</span>
          <span className="text-xs opacity-60">«</span>
        </button>

        {/* Previous */}
        <button className={NAV_BTN} title="Previous Page"
                onClick={() => goToPage(currentPage - 1)} disabled={currentPage === 1}>
          ‹
        </button>

        {/* Middle numbered pages */}
        {middlePages.map((page) => (
          <button
            key={page}
            className={`${PAGE_BTN} ${page === currentPage ? PAGE_BTN_ACTIVE : PAGE_BTN_INACTIVE}`}
            onClick={() => goToPage(page)}
            title={`Page ${page}`}
          >
            {page}
          </button>
        ))}

        {/* Next */}
        <button className={NAV_BTN} title="Next Page"
                onClick={() => goToPage(currentPage + 1)} disabled={currentPage === totalPages}>
          ›
        </button>

        {/* Last page hybrid: » then number */}
        {totalPages > 1 && (
          <button
            className={`${EDGE_BTN} ${currentPage === totalPages ? PAGE_BTN_ACTIVE : PAGE_BTN_INACTIVE}`}
            onClick={() => goToPage(totalPages)}
            title="Last Page"
          >
            <span className="text-xs opacity-60">»</span>
            <span>{totalPages}</span>
          </button>
        )}
      </div>
    </div>
  );
}
