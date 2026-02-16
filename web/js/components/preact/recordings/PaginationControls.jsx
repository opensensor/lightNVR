/**
 * PaginationControls component for RecordingsView
 */

import { h } from 'preact';

/* shared base for every clickable element */
const BASE = "h-8 inline-flex items-center justify-center text-sm rounded cursor-pointer select-none border border-transparent focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed";
/* nav arrows (‹ ›) — square */
const NAV = `${BASE} w-8 bg-secondary text-secondary-foreground hover:bg-secondary/80`;
/* page number — square */
const PAGE = `${BASE} min-w-8 px-1`;
const PAGE_ON = "bg-primary text-primary-foreground";
const PAGE_OFF = "bg-secondary text-secondary-foreground hover:bg-secondary/80";
/* edge hybrid (1« / »N) — slightly wider */
const EDGE = `${BASE} px-2 gap-0.5`;

/**
 * Interior page numbers (excluding first & last).
 * Up to 3 each side of current page.
 */
function getMiddlePages(current, total) {
  if (total <= 2) return [];
  const pages = [];
  for (let i = current - 2; i <= current + 2; i++) {
    if (i > 1 && i < total) pages.push(i);
  }
  return pages;
}

/**
 * PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  const { currentPage, totalPages } = pagination;
  const middle = getMiddlePages(currentPage, totalPages);

  const pageCls = (p) => `${PAGE} ${p === currentPage ? PAGE_ON : PAGE_OFF}`;
  const edgeCls = (p) => `${EDGE} ${p === currentPage ? PAGE_ON : PAGE_OFF}`;

  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        Showing {pagination.startItem}-{pagination.endItem} of {pagination.totalItems} recordings
      </div>

      <div className="flex items-center gap-1">
        {/* First page — hybrid: 1 « */}
        <button className={edgeCls(1)} onClick={() => goToPage(1)} title="First Page">
          <span>1</span><span className="text-xs opacity-60">«</span>
        </button>

        {/* Previous — hidden on first page */}
        {currentPage > 1 && (
          <button className={NAV} onClick={() => goToPage(currentPage - 1)}
                  title="Previous Page">‹</button>
        )}

        {/* Middle pages */}
        {middle.map((p) => (
          <button key={p} className={pageCls(p)} onClick={() => goToPage(p)}
                  title={`Page ${p}`}>{p}</button>
        ))}

        {/* Next — hidden on last page */}
        {currentPage < totalPages && (
          <button className={NAV} onClick={() => goToPage(currentPage + 1)}
                  title="Next Page">›</button>
        )}

        {/* Last page — hybrid: » N */}
        {totalPages > 1 && (
          <button className={edgeCls(totalPages)} onClick={() => goToPage(totalPages)}
                  title="Last Page">
            <span className="text-xs opacity-60">»</span><span>{totalPages}</span>
          </button>
        )}
      </div>
    </div>
  );
}
