import { useI18n } from '../../../i18n.js';

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
  const showPager = totalPages > 1 && !pagination.showAll;
  const { t } = useI18n();

  const cls = (p) => (p === currentPage ? ACTIVE : INACTIVE);

  return (
    <div className="pagination-controls flex flex-col sm:flex-row justify-between items-center p-4 border-t border-border">
      <div className="pagination-info text-sm text-muted-foreground mb-2 sm:mb-0">
        {pagination.showAll && pagination.totalItems > 0
          ? t('recordings.showingAllRecordings', { count: pagination.totalItems })
          : t('recordings.showingRecordingsRange', { start: pagination.startItem, end: pagination.endItem, total: pagination.totalItems })}
      </div>

      {showPager && <div className="join">
        {/* First page */}
        <button className={cls(1)} onClick={() => goToPage(1)} title={t('pagination.firstPage')}
                disabled={currentPage === 1}>
          «
        </button>

        {/* Previous */}
        <button className={INACTIVE} onClick={() => goToPage(currentPage - 1)}
                title={t('pagination.previousPage')} disabled={currentPage === 1}>
          ‹
        </button>

        {/* Middle page numbers */}
        {middle.map((p) => (
          <button key={p} className={cls(p)} onClick={() => goToPage(p)}
                  title={t('pagination.page', { number: p })}>
            {p}
          </button>
        ))}

        {/* Next */}
        <button className={INACTIVE} onClick={() => goToPage(currentPage + 1)}
                title={t('pagination.nextPage')} disabled={currentPage === totalPages}>
          ›
        </button>

        {/* Last page */}
        <button className={cls(totalPages)} onClick={() => goToPage(totalPages)}
                title={t('pagination.lastPage')} disabled={currentPage === totalPages}>
          »
        </button>
      </div>}
    </div>
  );
}
