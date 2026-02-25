/**
 * GridPicker — a popover grid-cell selector for choosing columns × rows.
 * Hover over cells to preview the grid size; click to confirm.
 */

import { useState, useEffect, useRef } from 'preact/hooks';

const MAX_COLS = 8;
const MAX_ROWS = 6;

/**
 * Return the best [cols, rows] to display N streams in a near-square grid.
 * @param {number} n
 * @returns {[number, number]}
 */
export function computeOptimalGrid(n) {
  if (n <= 1)  return [1, 1];
  if (n <= 2)  return [2, 1];
  if (n <= 4)  return [2, 2];
  if (n <= 6)  return [3, 2];
  if (n <= 9)  return [3, 3];
  if (n <= 12) return [4, 3];
  if (n <= 16) return [4, 4];
  if (n <= 20) return [5, 4];
  // Fallback: roughly square
  const c = Math.ceil(Math.sqrt(n));
  return [c, Math.ceil(n / c)];
}

/**
 * GridPicker component.
 *
 * @param {object}   props
 * @param {number}   props.cols      Current column count
 * @param {number}   props.rows      Current row count
 * @param {Function} props.onSelect  Called with (cols, rows) on cell click
 */
export function GridPicker({ cols, rows, onSelect }) {
  const [hover, setHover] = useState({ c: cols, r: rows });
  const [open, setOpen]   = useState(false);
  const containerRef      = useRef(null);

  // Keep hover in sync when external selection changes (e.g. auto-grid on load)
  useEffect(() => { setHover({ c: cols, r: rows }); }, [cols, rows]);

  // Close the popover on outside click
  useEffect(() => {
    if (!open) return;
    const onDown = (e) => {
      if (containerRef.current && !containerRef.current.contains(e.target)) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', onDown);
    return () => document.removeEventListener('mousedown', onDown);
  }, [open]);

  return (
    <div ref={containerRef} className="relative">
      {/* Trigger button */}
      <button
        type="button"
        className="px-3 py-2 border border-border rounded-md bg-background text-foreground text-sm flex items-center gap-1.5 hover:bg-muted focus:outline-none focus:ring-2 focus:ring-primary transition-colors"
        onClick={() => setOpen(o => !o)}
        title="Select grid layout"
        aria-label={`Grid layout: ${cols} columns × ${rows} rows`}
      >
        {/* 2×2 grid icon */}
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24"
             fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
          <rect x="3"  y="3"  width="7" height="7" rx="1"/>
          <rect x="14" y="3"  width="7" height="7" rx="1"/>
          <rect x="3"  y="14" width="7" height="7" rx="1"/>
          <rect x="14" y="14" width="7" height="7" rx="1"/>
        </svg>
        <span className="font-medium tabular-nums">{cols}×{rows}</span>
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24"
             fill="none" stroke="currentColor" strokeWidth="2.5">
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </button>

      {/* Popover */}
      {open && (
        <div
          className="absolute right-0 top-full mt-1.5 z-50 bg-card border border-border rounded-lg shadow-xl p-3 select-none"
          onMouseLeave={() => setHover({ c: cols, r: rows })}
        >
          <p className="text-xs font-semibold text-center text-foreground mb-2 tabular-nums">
            {hover.c} × {hover.r}
          </p>
          <div
            style={{
              display: 'grid',
              gridTemplateColumns: `repeat(${MAX_COLS}, 20px)`,
              gap: '3px',
            }}
          >
            {Array.from({ length: MAX_ROWS }, (_, r) =>
              Array.from({ length: MAX_COLS }, (_, c) => {
                const highlighted = r < hover.r && c < hover.c;
                const isCurrent   = r < rows   && c < cols;
                return (
                  <div
                    key={`${r}-${c}`}
                    style={{ width: '20px', height: '20px' }}
                    className={`rounded-sm border cursor-pointer transition-colors ${
                      highlighted
                        ? 'bg-primary border-primary'
                        : isCurrent
                          ? 'bg-primary/25 border-primary/40'
                          : 'bg-muted border-border hover:border-primary/50 hover:bg-muted/80'
                    }`}
                    onMouseEnter={() => setHover({ c: c + 1, r: r + 1 })}
                    onClick={() => { onSelect(c + 1, r + 1); setOpen(false); }}
                  />
                );
              })
            )}
          </div>
          <p className="text-xs text-center text-muted-foreground mt-2">
            up to {MAX_COLS}×{MAX_ROWS}
          </p>
        </div>
      )}
    </div>
  );
}

