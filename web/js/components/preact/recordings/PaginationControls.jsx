/**
 * PaginationControls component for RecordingsView
 * Uses daisyUI join + btn for a clean, theme-aware button group.
 */

/** Interior page numbers (excluding first & last), ±2 of current. */
function getMiddlePages(current, total) {
  if (total <= 2) return [];
  const pages = [];
  for (let i = current - 2; i <= current + 2; i++) {
    if (i > 1 && i < total) pages.push(i);
  }
  return pages;
}

/** Shared base for every button in the group */
const BTN = "join-item btn btn-sm";
/** Inactive: ghost-style using existing theme tokens */
const INACTIVE = `${BTN} bg-secondary text-secondary-foreground border-border hover:bg-accent hover:text-accent-foreground`;
/** Active: highlight using primary theme token */
const ACTIVE = `${BTN} bg-primary text-primary-foreground border-primary`;

/**
 * PaginationControls component
 */
export function PaginationControls({ pagination, goToPage }) {
  const { currentPage, totalPages } = pagination;
  const middle = getMiddlePages(currentPage, totalPages);

  const cls = (p) => (p === currentPage ? ACTIVE : INACTIVE);

  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        Showing {pagination.startItem}–{pagination.endItem} of {pagination.totalItems} recordings
      </div>

      <div className="join">
        {/* First page */}
        <button className={cls(1)} onClick={() => goToPage(1)} title="First page"
                disabled={currentPage === 1}>
          «
        </button>

        {/* Previous */}
        <button className={INACTIVE} onClick={() => goToPage(currentPage - 1)}
                title="Previous page" disabled={currentPage === 1}>
          ‹
        </button>

        {/* Middle page numbers */}
        {middle.map((p) => (
          <button key={p} className={cls(p)} onClick={() => goToPage(p)}
                  title={`Page ${p}`}>
            {p}
          </button>
        ))}

        {/* Next */}
        <button className={INACTIVE} onClick={() => goToPage(currentPage + 1)}
                title="Next page" disabled={currentPage === totalPages}>
          ›
        </button>

        {/* Last page */}
        <button className={cls(totalPages)} onClick={() => goToPage(totalPages)}
                title="Last page" disabled={currentPage === totalPages}>
          »
        </button>
      </div>
    </div>
  );
}
