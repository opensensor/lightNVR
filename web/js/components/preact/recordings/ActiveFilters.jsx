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
         className="active-filters mb-4 p-3 bg-blue-50 dark:bg-blue-900/30 rounded-lg flex flex-wrap gap-2">
      {activeFiltersDisplay.map((filter, index) => (
        <div key={index} className="filter-tag inline-flex items-center px-3 py-1 rounded-full text-sm bg-blue-100 text-blue-800 dark:bg-blue-800 dark:text-blue-200">
          <span>{filter.label}</span>
          <button className="ml-2 text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 focus:outline-none"
                  onClick={() => removeFilter(filter.key)}>
            ×
          </button>
        </div>
      ))}
    </div>
  );
}
