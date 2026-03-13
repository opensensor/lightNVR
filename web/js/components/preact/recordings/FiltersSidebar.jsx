/**
 * FiltersSidebar component for RecordingsView
 * Collapsible sidebar with accordion-style filter sections
 */

import { useState, useEffect } from 'preact/hooks';
import { recordingsAPI } from './recordingsAPI.jsx';
import { formatUtils } from './formatUtils.js';
import { urlUtils } from './urlUtils.js';
import { useI18n } from '../../../i18n.js';

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

function SelectedValues({ values, onRemove, formatValue = (value) => value, emptyText, t }) {
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
          title={t('recordings.removeFilterValue', { value: formatValue(value) })}
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
  toggleCollapsed,
  filters,
  setFilters,
  pagination,
  setPagination,
  streams,
  applyFilters,
  resetFilters,
  handleDateRangeChange
}) {
  const { t } = useI18n();

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
    ? t('recordings.withDetections')
    : filters.recordingType === 'no_detection'
      ? t('recordings.withoutDetections')
      : null;
  const protectedBadge = filters.protectedStatus !== 'all' ? filters.protectedStatus : null;

  return (
    <aside
      id="filters-sidebar"
      className="filters-sidebar bg-card text-card-foreground rounded-lg shadow md:sticky md:top-4 md:self-start transition-all duration-300 w-full md:w-64"
    >
      <div className="flex items-center justify-between px-3 py-2.5 border-b border-border">
        <h3 className="text-sm font-bold text-foreground uppercase tracking-wider">{t('recordings.filters')}</h3>
        <button
          type="button"
          onClick={toggleCollapsed}
          className="p-1.5 rounded hover:bg-muted/70 transition-colors"
          title={t('recordings.hideFilters')}
        >
          <svg className="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
              d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
          </svg>
        </button>
      </div>

      <div className="p-3 space-y-0">
          <FilterSection title={t('recordings.dateRange')} badge={dateRangeBadge} isExpanded={sections.dateRange} onToggle={() => toggleSection('dateRange')}>
            <select
              id="date-range-select"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.dateRange}
              onChange={handleDateRangeChange}
            >
              <option value="today">{t('recordings.today')}</option>
              <option value="yesterday">{t('recordings.yesterday')}</option>
              <option value="last7days">{t('recordings.last7Days')}</option>
              <option value="last30days">{t('recordings.last30Days')}</option>
              <option value="custom">{t('recordings.customRange')}</option>
            </select>

            {filters.dateRange === 'custom' && (
              <div className="space-y-2 mt-2">
                <div>
                  <label htmlFor="start-date" className="block text-xs font-medium text-muted-foreground mb-1">{t('recordings.start')}</label>
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
                  <label htmlFor="end-date" className="block text-xs font-medium text-muted-foreground mb-1">{t('recordings.end')}</label>
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

          <FilterSection title={t('nav.streams')} badge={getCountBadge(filters.streamIds)} isExpanded={sections.stream} onToggle={() => toggleSection('stream')}>
            <select
              id="stream-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('streamIds', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">{t('recordings.allStreams')}</option>
              {streams.map((stream) => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.streamIds}
              onRemove={(value) => removeMultiFilterValue('streamIds', value)}
              emptyText={t('recordings.noStreamFiltersSelected')}
              t={t}
            />
          </FilterSection>

          <FilterSection title={t('recordings.detections')} badge={typeBadge} isExpanded={sections.recordingType} onToggle={() => toggleSection('recordingType')}>
            <select
              id="detection-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.recordingType}
              onChange={(e) => setFilters((prev) => ({ ...prev, recordingType: e.target.value }))}
            >
              <option value="all">{t('recordings.allRecordings')}</option>
              <option value="detection">{t('recordings.withDetections')}</option>
              <option value="no_detection">{t('recordings.withoutDetections')}</option>
            </select>
          </FilterSection>

          <FilterSection title={t('recordings.detectionObjects')} badge={getCountBadge(filters.detectionLabels)} isExpanded={sections.detectionObject} onToggle={() => toggleSection('detectionObject')}>
            <select
              id="detection-label-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('detectionLabels', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">{t('recordings.addDetectionObject')}</option>
              {availableDetectionLabels.map((label) => (
                <option key={label} value={label}>{label}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.detectionLabels}
              onRemove={(value) => removeMultiFilterValue('detectionLabels', value)}
              emptyText={t('recordings.noDetectionObjectsSelected')}
              t={t}
            />
          </FilterSection>

          <FilterSection title={t('recordings.captureMethodLabel')} badge={getCountBadge(filters.captureMethods)} isExpanded={sections.captureMethod} onToggle={() => toggleSection('captureMethod')}>
            <select
              id="capture-method-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('captureMethods', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">{t('recordings.addCaptureMethod')}</option>
              {CAPTURE_METHOD_OPTIONS.map((value) => (
                <option key={value} value={value}>{formatUtils.formatCaptureMethod(value, t)}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.captureMethods}
              onRemove={(value) => removeMultiFilterValue('captureMethods', value)}
              formatValue={(value) => formatUtils.formatCaptureMethod(value, t)}
              emptyText={t('recordings.noCaptureMethodsSelected')}
              t={t}
            />
          </FilterSection>

          <FilterSection title={t('recordings.recordingTags')} badge={getCountBadge(filters.tags)} isExpanded={sections.tag} onToggle={() => toggleSection('tag')}>
            <select
              id="tag-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              defaultValue=""
              onChange={(e) => {
                if (e.target.value) addMultiFilterValue('tags', e.target.value);
                e.target.value = '';
              }}
            >
              <option value="">{t('recordings.addTag')}</option>
              {availableTags.map((tag) => (
                <option key={tag} value={tag}>{tag}</option>
              ))}
            </select>
            <SelectedValues
              values={filters.tags}
              onRemove={(value) => removeMultiFilterValue('tags', value)}
              emptyText={t('recordings.noTagsSelected')}
              t={t}
            />
          </FilterSection>

          <FilterSection title={t('recordings.protected')} badge={protectedBadge} isExpanded={sections.protectedStatus} onToggle={() => toggleSection('protectedStatus')}>
            <select
              id="protected-filter"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={filters.protectedStatus}
              onChange={(e) => setFilters((prev) => ({ ...prev, protectedStatus: e.target.value }))}
            >
              <option value="all">{t('common.all')}</option>
              <option value="yes">{t('recordings.protected')}</option>
              <option value="no">{t('recordings.notProtected')}</option>
            </select>
          </FilterSection>

          <FilterSection title={t('recordings.display')} isExpanded={sections.display} onToggle={() => toggleSection('display')}>
            <label htmlFor="page-size" className="block text-xs font-medium text-muted-foreground mb-1">{t('recordings.itemsPerPage')}</label>
            <select
              id="page-size"
              className="w-full p-2 text-sm border border-input rounded-md bg-background text-foreground"
              value={pagination.showAll ? 'all' : pagination.pageSize.toString()}
              onChange={(e) => {
                const paginationLimit = urlUtils.parsePaginationLimit(e.target.value);
                setPagination((prev) => ({
                  ...prev,
                  currentPage: 1,
                  pageSize: paginationLimit.pageSize,
                  showAll: paginationLimit.showAll
                }));
              }}
            >
              <option value="10">10</option>
              <option value="20">20</option>
              <option value="50">50</option>
              <option value="100">100</option>
              <option value="all">{t('common.all')}</option>
            </select>
          </FilterSection>

        <div className="flex gap-2 pt-2">
          <button
            id="apply-filters-btn"
            className="btn-primary flex-1 text-sm py-2"
            onClick={applyFilters}
          >
            {t('recordings.apply')}
          </button>
          <button
            id="reset-filters-btn"
            className="btn-secondary flex-1 text-sm py-2"
            onClick={resetFilters}
          >
            {t('recordings.reset')}
          </button>
        </div>
      </div>
    </aside>
  );
}
