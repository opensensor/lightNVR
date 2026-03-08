/**
 * FiltersSidebar component for RecordingsView
 * Collapsible sidebar with accordion-style filter sections
 */

import { useState, useEffect } from 'preact/hooks';
import { recordingsAPI } from './recordingsAPI.jsx';
import { formatUtils } from './formatUtils.js';
import { urlUtils } from './urlUtils.js';

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

function SelectedValues({ values, onRemove, formatValue = (value) => value, emptyText }) {
  if (!values.length) {
    return <p className="text-[11px] text-muted-foreground">{emptyText}</p>;
  }

  return (
    <div className="flex flex-wrap gap-2">
      {values.map((value) => (
        <button
          key={value}
          type="button"
          className="filter-tag hover:opacity-90"
          onClick={() => onRemove(value)}
          title={`Remove ${formatValue(value)}`}
        >
          <span>{formatValue(value)}</span>
          <span className="ml-2">×</span>
        </button>
      ))}
    </div>
  );
}

const DEFAULT_SECTIONS = {
  dateRange: true,
  stream: true,
  recordingType: true,
  detectionObject: false,
  captureMethod: false,
  tag: false,
  protectedStatus: false,
  display: false
};

const CAPTURE_METHOD_OPTIONS = ['scheduled', 'detection', 'motion', 'manual'];

const getCountBadge = (values) => (values.length > 0 ? `${values.length} selected` : null);

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
  handleDateRangeChange
}) {
  const [collapsed, setCollapsed] = useState(() => {
    try { return localStorage.getItem('recordings_filters_collapsed') === 'true'; }
    catch { return false; }
  });

  const [sections, setSections] = useState(() => {
    try {
      const saved = localStorage.getItem('recordings_filters_sections');
      return saved ? { ...DEFAULT_SECTIONS, ...JSON.parse(saved) } : { ...DEFAULT_SECTIONS };
    } catch { return { ...DEFAULT_SECTIONS }; }
  });

  const [availableDetectionLabels, setAvailableDetectionLabels] = useState([]);
  const [availableTags, setAvailableTags] = useState([]);

  useEffect(() => {
    recordingsAPI.getAllRecordingTags().then(setAvailableTags);
    recordingsAPI.getAllDetectionLabels().then(setAvailableDetectionLabels);
  }, []);

  const toggleCollapsed = () => {
    setCollapsed((prev) => {
      localStorage.setItem('recordings_filters_collapsed', String(!prev));
      return !prev;
    });
  };

  const toggleSection = (key) => {
    setSections((prev) => {
      const next = { ...prev, [key]: !prev[key] };
      localStorage.setItem('recordings_filters_sections', JSON.stringify(next));
      return next;
    });
  };

  const addMultiFilterValue = (key, value) => {
    setFilters((prev) => ({
      ...prev,
      [key]: urlUtils.addMultiValue(prev[key], value)
    }));
  };

  const removeMultiFilterValue = (key, value) => {
    setFilters((prev) => ({
      ...prev,
      [key]: urlUtils.removeMultiValue(prev[key], value)
    }));
  };

  const dateRangeBadge = filters.dateRange !== 'last7days'
    ? filters.dateRange.replace('last', '').replace('days', 'd')
    : null;
  const typeBadge = filters.recordingType === 'detection'
    ? 'with detections'
    : filters.recordingType === 'no_detection'
      ? 'without detections'
      : null;
  const protectedBadge = filters.protectedStatus !== 'all' ? filters.protectedStatus : null;

  return (
    <aside
      id="filters-sidebar"
      className={`filters-sidebar bg-card text-card-foreground rounded-lg shadow md:sticky md:top-4 md:self-start transition-all duration-300 ${
        collapsed ? 'w-full md:w-12' : 'w-full md:w-64'
      }`}
    >
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
          <svg className="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
              d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
          </svg>
        </button>
      </div>

      {!collapsed && (
        <div className="p-3 space-y-0">
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
                      onChange={(e) => setFilters((prev) => ({ ...prev, startDate: e.target.value }))} />
                    <input type="time" id="start-time"
                      className="w-24 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.startTime}
                      onChange={(e) => setFilters((prev) => ({ ...prev, startTime: e.target.value }))} />
                  </div>
                </div>
                <div>
                  <label htmlFor="end-date" className="block text-xs font-medium text-muted-foreground mb-1">End</label>
                  <div className="flex gap-1.5">
                    <input type="date" id="end-date"
                      className="flex-1 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.endDate}
                      onChange={(e) => setFilters((prev) => ({ ...prev, endDate: e.target.value }))} />
                    <input type="time" id="end-time"
                      className="w-24 p-1.5 text-sm border border-input rounded-md bg-background text-foreground"
                      value={filters.endTime}
                      onChange={(e) => setFilters((prev) => ({ ...prev, endTime: e.target.value }))} />
                  </div>
                </div>
              </div>
            )}
          </FilterSection>

          <FilterSection title="Streams" badge={getCountBadge(filters.streamIds)} isExpanded={sections.stream} onToggle={() => toggleSection('stream')}>
            <select
              id="stream-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('streamIds', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">Add stream…</option>
              {streams.map((stream) => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.streamIds}
              onRemove={(value) => removeMultiFilterValue('streamIds', value)}
              emptyText="No stream filters selected"
            />
          </FilterSection>

          <FilterSection title="Detections" badge={typeBadge} isExpanded={sections.recordingType} onToggle={() => toggleSection('recordingType')}>
            <select
              id="detection-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.recordingType}
              onChange={(e) => setFilters((prev) => ({ ...prev, recordingType: e.target.value }))}
            >
              <option value="all">All Recordings</option>
              <option value="detection">With Detections</option>
              <option value="no_detection">Without Detections</option>
            </select>
          </FilterSection>

          <FilterSection title="Detection Objects" badge={getCountBadge(filters.detectionLabels)} isExpanded={sections.detectionObject} onToggle={() => toggleSection('detectionObject')}>
            <select
              id="detection-label-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('detectionLabels', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">Add detection object…</option>
              {availableDetectionLabels.map((label) => (
                <option key={label} value={label}>{label}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.detectionLabels}
              onRemove={(value) => removeMultiFilterValue('detectionLabels', value)}
              emptyText="No detection objects selected"
            />
          </FilterSection>

          <FilterSection title="Capture Method" badge={getCountBadge(filters.captureMethods)} isExpanded={sections.captureMethod} onToggle={() => toggleSection('captureMethod')}>
            <select
              id="capture-method-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('captureMethods', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">Add capture method…</option>
              {CAPTURE_METHOD_OPTIONS.map((value) => (
                <option key={value} value={value}>{formatUtils.formatCaptureMethod(value)}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.captureMethods}
              onRemove={(value) => removeMultiFilterValue('captureMethods', value)}
              formatValue={(value) => formatUtils.formatCaptureMethod(value)}
              emptyText="No capture methods selected"
            />
          </FilterSection>

          <FilterSection title="Recording Tags" badge={getCountBadge(filters.tags)} isExpanded={sections.tag} onToggle={() => toggleSection('tag')}>
            <select
              id="tag-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('tags', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">Add tag…</option>
              {availableTags.map((tag) => (
                <option key={tag} value={tag}>{tag}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.tags}
              onRemove={(value) => removeMultiFilterValue('tags', value)}
              emptyText="No tags selected"
            />
          </FilterSection>

          <FilterSection title="Protected" badge={protectedBadge} isExpanded={sections.protectedStatus} onToggle={() => toggleSection('protectedStatus')}>
            <select
              id="protected-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.protectedStatus}
              onChange={(e) => setFilters((prev) => ({ ...prev, protectedStatus: e.target.value }))}
            >
              <option value="all">All</option>
              <option value="yes">Protected</option>
              <option value="no">Not Protected</option>
            </select>
          </FilterSection>

          <FilterSection title="Display" isExpanded={sections.display} onToggle={() => toggleSection('display')}>
            <label htmlFor="page-size" className="block text-xs font-medium text-muted-foreground mb-1">Items per page</label>
            <select
              id="page-size"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={pagination.pageSize}
              onChange={(e) => setPagination((prev) => ({ ...prev, pageSize: parseInt(e.target.value, 10) }))}
            >
              <option value="10">10</option>
              <option value="20">20</option>
              <option value="50">50</option>
              <option value="100">100</option>
            </select>
          </FilterSection>

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
