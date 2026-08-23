import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON, useQuery } from '../../query-client.js';
import { useI18n } from '../../i18n.js';
import { LoadingIndicator } from './LoadingIndicator.jsx';
import { FleetFilters } from './fleet/FleetFilters.jsx';
import { FleetTable } from './fleet/FleetTable.jsx';
import {
  DEFAULT_FLEET_STATE,
  PAGE_SIZES,
  buildFleetQueryRequest,
  clampFleetPage,
  countFleetFilters,
  facetCount,
  readFleetUrlState,
  writeFleetUrlState,
} from './fleet/fleetQuery.js';

function useDebouncedValue(value, delay) {
  const [debouncedValue, setDebouncedValue] = useState(value);
  useEffect(() => {
    const timeout = setTimeout(() => setDebouncedValue(value), delay);
    return () => clearTimeout(timeout);
  }, [value, delay]);
  return debouncedValue;
}

function StatCard({ label, value, tone = 'default' }) {
  const tones = {
    default: 'border-border',
    success: 'border-[hsl(var(--success)/0.55)]',
    warning: 'border-[hsl(var(--warning)/0.65)]',
    danger: 'border-[hsl(var(--danger)/0.55)]',
  };
  return (
    <div className={`rounded-lg border-l-4 bg-card p-4 shadow-sm ${tones[tone]}`}>
      <div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{label}</div>
      <div className="mt-1 text-2xl font-bold tabular-nums">{value}</div>
    </div>
  );
}

function EmptyFleet({ filtered, onClear, t }) {
  return (
    <div className="flex flex-col items-center px-6 py-16 text-center">
      <svg className="mb-4 h-12 w-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M4 7h16M5 7l1 12h12l1-12M9 11v4m6-4v4M9 7V5h6v2" />
      </svg>
      <h2 className="text-lg font-semibold">{filtered ? t('fleet.emptyFilteredTitle') : t('fleet.emptyTitle')}</h2>
      <p className="mt-1 max-w-md text-sm text-muted-foreground">
        {filtered ? t('fleet.emptyFilteredDescription') : t('fleet.emptyDescription')}
      </p>
      {filtered && <button type="button" className="btn-secondary mt-4" onClick={onClear}>{t('fleet.clearFilters')}</button>}
    </div>
  );
}

function Pagination({ state, total, totalPages, onChange, t }) {
  const first = total === 0 ? 0 : ((state.page - 1) * state.pageSize) + 1;
  const last = Math.min(state.page * state.pageSize, total);
  return (
    <div className="flex flex-col gap-3 border-t border-border px-4 py-3 sm:flex-row sm:items-center sm:justify-between">
      <div className="text-sm text-muted-foreground">
        {t('fleet.paginationSummary', { first, last, total })}
      </div>
      <div className="flex flex-wrap items-center gap-2">
        <label className="text-sm text-muted-foreground" htmlFor="fleet-page-size">{t('fleet.perPage')}</label>
        <select
          id="fleet-page-size"
          className="rounded-md border border-input bg-background px-2 py-1.5 text-sm"
          value={state.pageSize}
          onChange={(event) => onChange({ pageSize: Number(event.currentTarget.value), page: 1 }, false)}
        >
          {PAGE_SIZES.map((size) => <option key={size} value={size}>{size}</option>)}
        </select>
        <button type="button" className="btn-secondary px-3 py-1.5" disabled={state.page <= 1} onClick={() => onChange({ page: state.page - 1 }, false)}>
          {t('common.previous')}
        </button>
        <span className="min-w-20 text-center text-sm tabular-nums">
          {t('fleet.pageOf', { page: totalPages === 0 ? 0 : state.page, pages: totalPages })}
        </span>
        <button type="button" className="btn-secondary px-3 py-1.5" disabled={totalPages === 0 || state.page >= totalPages} onClick={() => onChange({ page: state.page + 1 }, false)}>
          {t('common.next')}
        </button>
      </div>
    </div>
  );
}

export function FleetView() {
  const { t, locale } = useI18n();
  const [state, setState] = useState(() => readFleetUrlState(typeof window === 'undefined' ? '' : window.location.search));
  const debouncedSearch = useDebouncedValue(state.search, 300);
  const requestBody = useMemo(
    () => buildFleetQueryRequest(state, debouncedSearch),
    [state.locationUuid, state.tagUuids, state.health, state.enabled, state.recordingModes, state.page, state.pageSize, state.sortBy, state.sortOrder, debouncedSearch]
  );

  const { data, error, isLoading, isFetching, refetch } = useQuery({
    queryKey: ['fleet-cameras', requestBody],
    queryFn: ({ signal }) => fetchJSON('/api/fleet/cameras/query', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(requestBody),
      signal,
      timeout: 20000,
      retries: 1,
    }),
    staleTime: 15000,
    refetchInterval: 30000,
    placeholderData: (previousData) => previousData,
  });

  const updateState = useCallback((changes, resetPage = true) => {
    setState((current) => ({ ...current, ...changes, page: resetPage ? 1 : (changes.page ?? current.page) }));
  }, []);

  const clearFilters = useCallback(() => {
    setState((current) => ({
      ...DEFAULT_FLEET_STATE,
      pageSize: current.pageSize,
      sortBy: current.sortBy,
      sortOrder: current.sortOrder,
    }));
  }, []);

  const handleSort = useCallback((field) => {
    setState((current) => ({
      ...current,
      sortBy: field,
      sortOrder: current.sortBy === field && current.sortOrder === 'asc' ? 'desc' : 'asc',
      page: 1,
    }));
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    const nextUrl = writeFleetUrlState(window.location.href, state);
    window.history.replaceState({}, '', nextUrl);
  }, [state]);

  useEffect(() => {
    if (!data || state.page <= Math.max(data.total_pages, 1)) return;
    updateState({ page: clampFleetPage(state.page, data.total_pages) }, false);
  }, [data, state.page, updateState]);

  const cameras = data?.cameras || [];
  const facets = data?.facets || {};
  const filterCount = countFleetFilters(state);
  const hasFilter = filterCount > 0 || Boolean(state.search.trim());
  const total = data?.total || 0;

  return (
    <main className="flex-grow py-2" aria-labelledby="fleet-title">
      <div className="mb-5 flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[hsl(var(--primary))]">{t('fleet.eyebrow')}</div>
          <h1 id="fleet-title" className="mt-1 text-3xl font-bold">{t('fleet.title')}</h1>
          <p className="mt-1 max-w-3xl text-sm text-muted-foreground">{t('fleet.description')}</p>
        </div>
        <button type="button" className="btn-secondary self-start sm:self-auto" onClick={() => refetch()} disabled={isFetching}>
          <span className={isFetching ? 'inline-block animate-spin' : ''} aria-hidden="true">↻</span>
          <span className="ml-2">{isFetching ? t('fleet.refreshing') : t('common.refresh')}</span>
        </button>
      </div>

      <section className="mb-5 grid grid-cols-2 gap-3 lg:grid-cols-4" aria-label={t('fleet.summary')}>
        <StatCard label={t('fleet.totalCameras')} value={total} />
        <StatCard label={t('fleet.health.up')} value={facetCount(facets, 'health', 'up')} tone="success" />
        <StatCard label={t('fleet.needsAttention')} value={facetCount(facets, 'health', 'degraded')} tone="warning" />
        <StatCard label={t('fleet.health.down')} value={facetCount(facets, 'health', 'down')} tone="danger" />
      </section>

      <div className="mb-4 rounded-lg border border-border bg-card p-3 shadow-sm">
        <div className="flex flex-col gap-3 md:flex-row md:items-center">
          <div className="relative flex-1">
            <svg className="pointer-events-none absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="m21 21-4.35-4.35m1.35-5.65a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z" />
            </svg>
            <label className="sr-only" htmlFor="fleet-search">{t('fleet.searchLabel')}</label>
            <input
              id="fleet-search"
              type="search"
              className="w-full rounded-md border border-input bg-background py-2.5 pl-10 pr-3 text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]"
              value={state.search}
              placeholder={t('fleet.searchPlaceholder')}
              onInput={(event) => updateState({ search: event.currentTarget.value })}
            />
          </div>
          <div className="flex items-center justify-between gap-2 md:justify-end">
            <span className="text-sm text-muted-foreground">{t('fleet.resultCount', { count: total })}</span>
            {hasFilter && <button type="button" className="btn-secondary whitespace-nowrap" onClick={clearFilters}>{t('fleet.clearFilters')}</button>}
          </div>
        </div>

        <details className="mt-3 border-t border-border pt-3 lg:hidden">
          <summary className="cursor-pointer text-sm font-semibold">
            {t('fleet.filters')} {filterCount > 0 && <span className="ml-1 rounded-full bg-[hsl(var(--primary))] px-2 py-0.5 text-xs text-[hsl(var(--primary-foreground))]">{filterCount}</span>}
          </summary>
          <div className="mt-4"><FleetFilters state={state} facets={facets} onChange={updateState} t={t} idPrefix="fleet-mobile" /></div>
        </details>
      </div>

      <div className="grid gap-4 lg:grid-cols-[15rem_minmax(0,1fr)]">
        <aside className="hidden self-start rounded-lg border border-border bg-card p-4 shadow-sm lg:block" aria-label={t('fleet.filters')}>
          <div className="mb-4 flex items-center justify-between">
            <h2 className="font-semibold">{t('fleet.filters')}</h2>
            {filterCount > 0 && <span className="rounded-full bg-[hsl(var(--primary))] px-2 py-0.5 text-xs text-[hsl(var(--primary-foreground))]">{filterCount}</span>}
          </div>
          <FleetFilters state={state} facets={facets} onChange={updateState} t={t} idPrefix="fleet-desktop" />
        </aside>

        <section className="min-w-0 overflow-hidden rounded-lg border border-border bg-card shadow-sm" aria-live="polite" aria-busy={isLoading || isFetching}>
          {isLoading && !data ? (
            <LoadingIndicator message={t('fleet.loading')} />
          ) : error ? (
            <div className="flex flex-col items-center px-6 py-16 text-center">
              <h2 className="text-lg font-semibold text-[hsl(var(--danger))]">{t('fleet.loadError')}</h2>
              <p className="mt-1 max-w-lg text-sm text-muted-foreground">{error.message}</p>
              <button type="button" className="btn-secondary mt-4" onClick={() => refetch()}>{t('common.retry')}</button>
            </div>
          ) : cameras.length === 0 ? (
            <EmptyFleet filtered={hasFilter} onClear={clearFilters} t={t} />
          ) : (
            <>
              <FleetTable cameras={cameras} state={state} onSort={handleSort} locale={locale} t={t} />
              <Pagination state={state} total={total} totalPages={data.total_pages || 0} onChange={updateState} t={t} />
            </>
          )}
        </section>
      </div>
    </main>
  );
}
