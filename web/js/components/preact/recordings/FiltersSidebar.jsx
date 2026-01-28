/**
 * FiltersSidebar component for RecordingsView
 */

import { h } from 'preact';

/**
 * FiltersSidebar component
 * @param {Object} props Component props
 * @returns {JSX.Element} FiltersSidebar component
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
  return (
    <aside id="filters-sidebar"
           className="filters-sidebar w-full md:w-64 bg-card text-card-foreground rounded-lg shadow p-4 md:sticky md:top-4 md:self-start transition-all duration-300">
      <div className="filter-group mb-4">
        <h3 className="text-lg font-medium mb-2 pb-1 border-b border-border">Date Range</h3>
        <div className="filter-option mb-2">
          <label htmlFor="date-range-select" className="block mb-1 text-sm font-medium">Quick Select:</label>
          <select id="date-range-select"
                  className="w-full p-2 border border-input rounded bg-background text-foreground"
                  value={filters.dateRange}
                  onChange={handleDateRangeChange}>
            <option value="today">Today</option>
            <option value="yesterday">Yesterday</option>
            <option value="last7days">Last 7 Days</option>
            <option value="last30days">Last 30 Days</option>
            <option value="custom">Custom Range</option>
          </select>
        </div>

        <div id="custom-date-range"
             className="filter-option space-y-3"
             style={{display: filters.dateRange === 'custom' ? 'block' : 'none'}}>
          <div className="space-y-1">
            <label htmlFor="start-date" className="block text-sm font-medium">Start Date:</label>
            <input type="date" id="start-date"
                   className="w-full p-2 border border-input rounded bg-background text-foreground"
                   value={filters.startDate}
                   onChange={e => setFilters(prev => ({ ...prev, startDate: e.target.value }))} />
            <div className="mt-1">
              <label htmlFor="start-time" className="block text-sm font-medium">Time:</label>
              <input type="time" id="start-time"
                     className="w-full p-2 border border-input rounded bg-background text-foreground"
                     value={filters.startTime}
                     onChange={e => setFilters(prev => ({ ...prev, startTime: e.target.value }))} />
            </div>
          </div>

          <div className="space-y-1">
            <label htmlFor="end-date" className="block text-sm font-medium">End Date:</label>
            <input type="date" id="end-date"
                   className="w-full p-2 border border-input rounded bg-background text-foreground"
                   value={filters.endDate}
                   onChange={e => setFilters(prev => ({ ...prev, endDate: e.target.value }))} />
            <div className="mt-1">
              <label htmlFor="end-time" className="block text-sm font-medium">Time:</label>
              <input type="time" id="end-time"
                     className="w-full p-2 border border-input rounded bg-background text-foreground"
                     value={filters.endTime}
                     onChange={e => setFilters(prev => ({ ...prev, endTime: e.target.value }))} />
            </div>
          </div>
        </div>
      </div>

      <div className="filter-group mb-4">
        <h3 className="text-lg font-medium mb-2 pb-1 border-b border-border">Stream</h3>
        <div className="filter-option">
          <select id="stream-filter"
                  className="w-full p-2 border border-input rounded bg-background text-foreground"
                  value={filters.streamId}
                  onChange={e => setFilters(prev => ({ ...prev, streamId: e.target.value }))}>
            <option value="all">All Streams</option>
            {streams.map(stream => (
              <option key={stream.name} value={stream.name}>{stream.name}</option>
            ))}
          </select>
        </div>
      </div>

      <div className="filter-group mb-4">
        <h3 className="text-lg font-medium mb-2 pb-1 border-b border-border">Recording Type</h3>
        <div className="filter-option">
          <select id="detection-filter"
                  className="w-full p-2 border border-input rounded bg-background text-foreground"
                  value={filters.recordingType}
                  onChange={e => setFilters(prev => ({ ...prev, recordingType: e.target.value }))}>
            <option value="all">All Recordings</option>
            <option value="detection">Detection Events Only</option>
          </select>
        </div>
      </div>

      <div className="filter-group mb-4">
        <h3 className="text-lg font-medium mb-2 pb-1 border-b border-border">Display Options</h3>
        <div className="filter-option">
          <label htmlFor="page-size" className="block mb-1 text-sm font-medium">Items per page:</label>
          <select id="page-size"
                  className="w-full p-2 border border-input rounded bg-background text-foreground"
                  value={pagination.pageSize}
                  onChange={e => setPagination(prev => ({ ...prev, pageSize: parseInt(e.target.value, 10) }))}>
            <option value="10">10</option>
            <option value="20">20</option>
            <option value="50">50</option>
            <option value="100">100</option>
          </select>
        </div>
      </div>

      <div className="filter-actions flex space-x-2">
        <button id="apply-filters-btn"
                className="btn-primary flex-1"
                onClick={applyFilters}>
          Apply Filters
        </button>
        <button id="reset-filters-btn"
                className="btn-secondary flex-1"
                onClick={resetFilters}>
          Reset
        </button>
      </div>
    </aside>
  );
}
