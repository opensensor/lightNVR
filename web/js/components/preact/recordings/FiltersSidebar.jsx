/**
 * FiltersSidebar component for RecordingsView
 * Collapsible sidebar with accordion-style filter sections
 */

import { useState, useEffect, useRef } from 'preact/hooks';

/** Small reusable accordion section for filter groups */
function FilterSection({ title, badge, isExpanded, onToggle, children }) {
  return (
    <div className="border border-border rounded-lg mb-2">
      <button
        type="button"
        onClick={onToggle}
        className="w-full flex items-center justify-between px-3 py-2.5 text-left hover:bg-muted/50 transition-colors rounded-t-lg"
      >
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-foreground">{title}</span>
          {badge && (
            <span className="px-1.5 py-0.5 text-[10px] rounded-full bg-primary/10 text-primary font-medium">
              {badge}
            </span>
          )}
        </div>
        <svg
          className={`w-4 h-4 text-muted-foreground transition-transform ${isExpanded ? 'rotate-180' : ''}`}
          fill="none" stroke="currentColor" viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {isExpanded && (
        <div className="px-3 pb-3 pt-2 border-t border-border space-y-2">
          {children}
        </div>
      )}
    </div>
  );
}

const SECTION_KEYS = ['dateRange', 'stream', 'recordingType', 'detectionObject', 'display'];
const DEFAULT_SECTIONS = { dateRange: true, stream: true, recordingType: true, detectionObject: false, display: false };

/**
 * FiltersSidebar component
 */
export function FiltersSidebar({
  filters,
  setFilters,
  pagination,
  setPagination,
  streams,
  applyFilters,
  resetFilters,
  handleDateRangeChange,
  setDefaultDateRange
}) {
  // Sidebar collapsed state
  const [collapsed, setCollapsed] = useState(() => {
    try { return localStorage.getItem('recordings_filters_collapsed') === 'true'; }
    catch { return false; }
  });

  // Per-section expanded states
  const [sections, setSections] = useState(() => {
    try {
      const saved = localStorage.getItem('recordings_filters_sections');
      return saved ? { ...DEFAULT_SECTIONS, ...JSON.parse(saved) } : { ...DEFAULT_SECTIONS };
    } catch { return { ...DEFAULT_SECTIONS }; }
  });

  const toggleCollapsed = () => {
    setCollapsed(prev => {
      localStorage.setItem('recordings_filters_collapsed', String(!prev));
      return !prev;
    });
  };

  const toggleSection = (key) => {
    setSections(prev => {
      const next = { ...prev, [key]: !prev[key] };
      localStorage.setItem('recordings_filters_sections', JSON.stringify(next));
      return next;
    });
  };

  // Detection label debounce
  const [localDetectionLabel, setLocalDetectionLabel] = useState(filters.detectionLabel || '');
  const debounceTimerRef = useRef(null);

  useEffect(() => {
    setLocalDetectionLabel(filters.detectionLabel || '');
  }, [filters.detectionLabel]);

  const handleDetectionLabelChange = (e) => {
    const value = e.target.value;
    setLocalDetectionLabel(value);
    if (debounceTimerRef.current) clearTimeout(debounceTimerRef.current);
    debounceTimerRef.current = setTimeout(() => {
      setFilters(prev => ({ ...prev, detectionLabel: value }));
    }, 500);
  };

  useEffect(() => () => {
    if (debounceTimerRef.current) clearTimeout(debounceTimerRef.current);
  }, []);

  // Count active non-default filters for badges
  const dateRangeBadge = filters.dateRange !== 'today' ? filters.dateRange.replace('last', '').replace('days', 'd') : null;
  const streamBadge = filters.streamId !== 'all' ? filters.streamId : null;
  const typeBadge = filters.recordingType !== 'all' ? 'detection' : null;
  const detectionBadge = filters.detectionLabel ? filters.detectionLabel : null;

  return (
    <aside
      id="filters-sidebar"
      className={`filters-sidebar bg-card text-card-foreground rounded-lg shadow md:sticky md:top-4 md:self-start transition-all duration-300 ${
        collapsed ? 'w-full md:w-12' : 'w-full md:w-64'
      }`}
    >
      {/* Header bar with toggle */}
      <div className={`flex items-center ${collapsed ? 'justify-center p-2' : 'justify-between px-3 py-2.5'} border-b border-border`}>
        {!collapsed && (
          <h3 className="text-sm font-bold text-foreground uppercase tracking-wider">Filters</h3>
        )}
        <button
          type="button"
          onClick={toggleCollapsed}
          className="p-1.5 rounded hover:bg-muted/70 transition-colors"
          title={collapsed ? 'Show filters' : 'Hide filters'}
        >
          {/* Funnel / filter icon */}
          <svg className="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
              d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
          </svg>
        </button>
      </div>

      {!collapsed && (
        <div className="p-3 space-y-0">
          {/* Date Range */}
          <FilterSection title="Date Range" badge={dateRangeBadge} isExpanded={sections.dateRange} onToggle={() => toggleSection('dateRange')}>
            <select
              id="date-range-select"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.dateRange}
              onChange={handleDateRangeChange}
            >
              <option value="today">Today</option>
              <option value="yesterday">Yesterday</option>
              <option value="last7days">Last 7 Days</option>
              <option value="last30days">Last 30 Days</option>
              <option value="custom">Custom Range</option>
            </select>

            {filters.dateRange === 'custom' && (
              <div className="space-y-2 mt-2">
                <div>
                  <label htmlFor="start-date" className="block text-xs font-medium text-muted-foreground mb-1">Start</label>
                  <div className="flex gap-1.5">
                    <input type="date" id="start-date"
                      className="flex-1 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.startDate}
                      onChange={e => setFilters(prev => ({ ...prev, startDate: e.target.value }))} />
                    <input type="time" id="start-time"
                      className="w-24 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.startTime}
                      onChange={e => setFilters(prev => ({ ...prev, startTime: e.target.value }))} />
                  </div>
                </div>
                <div>
                  <label htmlFor="end-date" className="block text-xs font-medium text-muted-foreground mb-1">End</label>
                  <div className="flex gap-1.5">
                    <input type="date" id="end-date"
                      className="flex-1 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.endDate}
                      onChange={e => setFilters(prev => ({ ...prev, endDate: e.target.value }))} />
                    <input type="time" id="end-time"
                      className="w-24 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.endTime}
                      onChange={e => setFilters(prev => ({ ...prev, endTime: e.target.value }))} />
                  </div>
                </div>
              </div>
            )}
          </FilterSection>

          {/* Stream */}
          <FilterSection title="Stream" badge={streamBadge} isExpanded={sections.stream} onToggle={() => toggleSection('stream')}>
            <select
              id="stream-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.streamId}
              onChange={e => setFilters(prev => ({ ...prev, streamId: e.target.value }))}
            >
              <option value="all">All Streams</option>
              {streams.map(stream => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
          </FilterSection>

          {/* Recording Type */}
          <FilterSection title="Recording Type" badge={typeBadge} isExpanded={sections.recordingType} onToggle={() => toggleSection('recordingType')}>
            <select
              id="detection-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.recordingType}
              onChange={e => setFilters(prev => ({ ...prev, recordingType: e.target.value }))}
            >
              <option value="all">All Recordings</option>
              <option value="detection">Detection Events Only</option>
            </select>
          </FilterSection>

          {/* Detection Object */}
          <FilterSection title="Detection Object" badge={detectionBadge} isExpanded={sections.detectionObject} onToggle={() => toggleSection('detectionObject')}>
            <input
              type="text"
              id="detection-label-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              placeholder="e.g., car, person, bicycle"
              value={localDetectionLabel}
              onChange={handleDetectionLabelChange}
            />
            <p className="text-[11px] text-muted-foreground">Filter by detected object type</p>
          </FilterSection>

          {/* Display Options */}
          <FilterSection title="Display" isExpanded={sections.display} onToggle={() => toggleSection('display')}>
            <label htmlFor="page-size" className="block text-xs font-medium text-muted-foreground mb-1">Items per page</label>
            <select
              id="page-size"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={pagination.pageSize}
              onChange={e => setPagination(prev => ({ ...prev, pageSize: parseInt(e.target.value, 10) }))}
            >
              <option value="10">10</option>
              <option value="20">20</option>
              <option value="50">50</option>
              <option value="100">100</option>
            </select>
          </FilterSection>

          {/* Action buttons */}
          <div className="flex gap-2 pt-2">
            <button
              id="apply-filters-btn"
              className="btn-primary flex-1 text-sm py-2"
              onClick={applyFilters}
            >
              Apply
            </button>
            <button
              id="reset-filters-btn"
              className="btn-secondary flex-1 text-sm py-2"
              onClick={resetFilters}
            >
              Reset
            </button>
          </div>
        </div>
      )}
    </aside>
  );
}
