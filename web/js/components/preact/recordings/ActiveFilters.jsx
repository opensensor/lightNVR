/**
 * ActiveFilters component for RecordingsView
 */

import { h } from 'preact';

/**
 * ActiveFilters component
 * @param {Object} props Component props
 * @returns {JSX.Element} ActiveFilters component
 */
export function ActiveFilters({ activeFiltersDisplay, removeFilter, hasActiveFilters }) {
  if (!hasActiveFilters) {
    return null;
  }

  return (
    <div id="active-filters"
         className="active-filters mb-4 p-3 rounded-lg flex flex-wrap gap-2 bg-muted">
      {activeFiltersDisplay.map((filter, index) => (
        <div key={index} className="filter-tag">
          <span>{filter.label}</span>
          <button className="ml-2 text-secondary-foreground hover:text-foreground focus:outline-none"
                  onClick={() => removeFilter(filter.key)}>
            ×
          </button>
        </div>
      ))}
    </div>
  );
}
