/**
 * CalendarPicker - A modern, theme-aware calendar date picker for Preact.
 *
 * Props:
 *   value    - YYYY-MM-DD string (selected date)
 *   onChange - (dateString: string) => void   called with YYYY-MM-DD
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';

/* ── helpers ────────────────────────────────────────────────── */

const DAYS = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];
const MONTHS = [
  'January','February','March','April','May','June',
  'July','August','September','October','November','December'
];

/** Pad single-digit numbers to 2 chars */
const pad = n => String(n).padStart(2, '0');

/** YYYY-MM-DD from a Date (local) */
const fmt = d => `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;

/** Parse YYYY-MM-DD into local Date at midnight */
function parseLocal(str) {
  const [y, m, d] = str.split('-').map(Number);
  return new Date(y, m - 1, d);
}

/** Get calendar grid rows for a given year / month (0-indexed month). */
function buildGrid(year, month) {
  const first = new Date(year, month, 1);
  const startDay = first.getDay(); // 0=Sun
  const daysInMonth = new Date(year, month + 1, 0).getDate();

  const cells = [];
  // leading blanks
  for (let i = 0; i < startDay; i++) cells.push(null);
  for (let d = 1; d <= daysInMonth; d++) cells.push(d);
  // trailing blanks to fill last row
  while (cells.length % 7 !== 0) cells.push(null);
  return cells;
}

/* ── component ──────────────────────────────────────────────── */

export function CalendarPicker({ value, onChange }) {
  const today = fmt(new Date());
  const selected = value || today;

  // The month being *viewed* in the popup (not necessarily the selected date's month)
  const selDate = parseLocal(selected);
  const [viewYear, setViewYear] = useState(selDate.getFullYear());
  const [viewMonth, setViewMonth] = useState(selDate.getMonth());
  const [open, setOpen] = useState(false);

  const wrapperRef = useRef(null);

  // Sync viewed month when value changes externally
  useEffect(() => {
    const d = parseLocal(selected);
    setViewYear(d.getFullYear());
    setViewMonth(d.getMonth());
  }, [selected]);

  // Close on outside click
  useEffect(() => {
    if (!open) return;
    const handler = (e) => {
      if (wrapperRef.current && !wrapperRef.current.contains(e.target)) setOpen(false);
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [open]);

  /* navigation */
  const prevMonth = () => {
    if (viewMonth === 0) { setViewMonth(11); setViewYear(y => y - 1); }
    else setViewMonth(m => m - 1);
  };
  const nextMonth = () => {
    if (viewMonth === 11) { setViewMonth(0); setViewYear(y => y + 1); }
    else setViewMonth(m => m + 1);
  };
  const goToday = () => {
    const t = new Date();
    setViewYear(t.getFullYear());
    setViewMonth(t.getMonth());
    pick(today);
  };

  const prevDay = () => { const d = parseLocal(selected); d.setDate(d.getDate() - 1); pick(fmt(d)); };
  const nextDay = () => { const d = parseLocal(selected); d.setDate(d.getDate() + 1); pick(fmt(d)); };

  const pick = useCallback((dateStr) => {
    onChange(dateStr);
    setOpen(false);
  }, [onChange]);

  const grid = buildGrid(viewYear, viewMonth);

  /* ── display helpers ── */
  const displayDate = parseLocal(selected);
  const displayStr = `${MONTHS[displayDate.getMonth()]} ${displayDate.getDate()}, ${displayDate.getFullYear()}`;

  return (
    <div className="relative" ref={wrapperRef}>
      {/* Trigger row: ◀  date-button  ▶ */}
      <div className="flex items-center gap-1">
        <button
          type="button"
          onClick={prevDay}
          className="p-2 rounded border border-border bg-secondary text-secondary-foreground hover:bg-accent transition-colors"
          title="Previous day"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
          </svg>
        </button>

        <button
          type="button"
          onClick={() => setOpen(o => !o)}
          className="flex-1 flex items-center justify-between gap-2 p-2 rounded border border-border bg-background text-foreground hover:bg-accent transition-colors text-sm"
        >
          <span>{displayStr}</span>
          {/* calendar icon */}
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
            <line x1="16" y1="2" x2="16" y2="6" /><line x1="8" y1="2" x2="8" y2="6" />
            <line x1="3" y1="10" x2="21" y2="10" />
          </svg>
        </button>

        <button
          type="button"
          onClick={nextDay}
          className="p-2 rounded border border-border bg-secondary text-secondary-foreground hover:bg-accent transition-colors"
          title="Next day"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
            <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      </div>

      {/* Dropdown calendar */}
      {open && (
        <div
          className="absolute z-50 mt-1 rounded-lg border border-border bg-card text-card-foreground shadow-lg p-3 w-72"
          style={{ left: 0 }}
        >
          {/* Month / Year header */}
          <div className="flex items-center justify-between mb-2">
            <button type="button" onClick={prevMonth}
              className="p-1 rounded hover:bg-accent transition-colors"
              title="Previous month"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                <path strokeLinecap="round" strokeLinejoin="round" d="M15 19l-7-7 7-7" />
              </svg>
            </button>

            <span className="font-semibold text-sm select-none">
              {MONTHS[viewMonth]} {viewYear}
            </span>

            <button type="button" onClick={nextMonth}
              className="p-1 rounded hover:bg-accent transition-colors"
              title="Next month"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2">
                <path strokeLinecap="round" strokeLinejoin="round" d="M9 5l7 7-7 7" />
              </svg>
            </button>
          </div>

          {/* Day-of-week headers */}
          <div className="grid grid-cols-7 text-center text-xs text-muted-foreground mb-1">
            {DAYS.map(d => <span key={d} className="py-1">{d}</span>)}
          </div>

          {/* Day cells */}
          <div className="grid grid-cols-7 text-center text-sm">
            {grid.map((day, i) => {
              if (day === null) return <span key={`blank-${i}`} />;
              const cellDate = fmt(new Date(viewYear, viewMonth, day));
              const isToday = cellDate === today;
              const isSel = cellDate === selected;

              return (
                <button
                  key={cellDate}
                  type="button"
                  onClick={() => pick(cellDate)}
                  className={[
                    'py-1 rounded-md transition-colors',
                    isSel
                      ? 'bg-primary text-primary-foreground font-bold'
                      : isToday
                        ? 'bg-accent text-accent-foreground font-medium'
                        : 'hover:bg-accent/60 text-foreground',
                  ].join(' ')}
                >
                  {day}
                </button>
              );
            })}
          </div>

          {/* Footer: Today shortcut */}
          <div className="mt-2 pt-2 border-t border-border flex justify-center">
            <button
              type="button"
              onClick={goToday}
              className="text-xs px-3 py-1 rounded bg-secondary text-secondary-foreground hover:bg-accent transition-colors"
            >
              Today
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

