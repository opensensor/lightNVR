import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON, useQuery } from '../../../query-client.js';

function useDebouncedValue(value, delay) {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay);
    return () => clearTimeout(timeout);
  }, [value, delay]);
  return debounced;
}

export function StaticCollectionMembers({ selectedUuids, onChange, t }) {
  const [search, setSearch] = useState('');
  const [page, setPage] = useState(1);
  const debouncedSearch = useDebouncedValue(search, 250);
  const selected = useMemo(() => new Set(selectedUuids), [selectedUuids]);
  const request = useMemo(() => ({
    selector: { version: 1, expression: { op: 'all' } },
    search: debouncedSearch.trim(),
    page,
    page_size: 25,
    sort_by: 'name',
    sort_order: 'asc',
    facets: false,
  }), [debouncedSearch, page]);
  const { data, error, isLoading } = useQuery({
    queryKey: ['collection-member-picker', request],
    queryFn: ({ signal }) => fetchJSON('/api/fleet/cameras/query', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request), signal, timeout: 20000, retries: 1,
    }),
    staleTime: 15000,
    placeholderData: (previous) => previous,
  });
  const cameras = data?.cameras || [];
  const selectedOnPage = cameras.filter((camera) => selected.has(camera.camera_uuid)).length;
  const allOnPage = cameras.length > 0 && selectedOnPage === cameras.length;

  useEffect(() => setPage(1), [debouncedSearch]);
  useEffect(() => {
    if (data?.total_pages > 0 && page > data.total_pages) setPage(data.total_pages);
  }, [data?.total_pages, page]);

  const toggleCamera = (uuid, checked) => {
    const next = new Set(selected);
    if (checked) next.add(uuid);
    else next.delete(uuid);
    onChange([...next]);
  };
  const togglePage = (checked) => {
    const next = new Set(selected);
    cameras.forEach((camera) => checked ? next.add(camera.camera_uuid) : next.delete(camera.camera_uuid));
    onChange([...next]);
  };

  return (
    <div className="rounded-lg border border-border p-3">
      <div className="mb-3 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h3 className="font-semibold">{t('collections.members.title')}</h3>
          <p className="text-xs text-muted-foreground">{t('collections.members.selected', { count: selected.size })}</p>
        </div>
        <input type="search" className="rounded-md border border-input bg-background px-3 py-2 text-sm" value={search} placeholder={t('collections.members.search')} onInput={(event) => { setSearch(event.currentTarget.value); setPage(1); }} />
      </div>
      <div className="max-h-72 overflow-y-auto rounded-md border border-border">
        <label className="sticky top-0 z-[1] flex items-center gap-2 border-b border-border bg-muted px-3 py-2 text-sm font-medium">
          <input type="checkbox" className="h-4 w-4" checked={allOnPage} ref={(element) => { if (element) element.indeterminate = selectedOnPage > 0 && !allOnPage; }} onChange={(event) => togglePage(event.currentTarget.checked)} />
          {t('collections.members.selectPage')}
        </label>
        {cameras.map((camera) => (
          <label key={camera.camera_uuid} className="flex cursor-pointer items-start gap-3 border-b border-border px-3 py-2 last:border-b-0 hover:bg-muted/30">
            <input type="checkbox" className="mt-1 h-4 w-4 flex-none" checked={selected.has(camera.camera_uuid)} onChange={(event) => toggleCamera(camera.camera_uuid, event.currentTarget.checked)} />
            <span className="min-w-0"><span className="block truncate text-sm font-medium">{camera.name}</span><span className="block truncate text-xs text-muted-foreground">{camera.location?.path || t('fleet.unassigned')}</span></span>
          </label>
        ))}
        {isLoading && !data && <p className="p-5 text-center text-sm text-muted-foreground">{t('common.loading')}</p>}
        {error && <p className="p-5 text-center text-sm text-[hsl(var(--danger))]">{error.message}</p>}
        {!isLoading && !error && cameras.length === 0 && <p className="p-5 text-center text-sm text-muted-foreground">{t('collections.members.empty')}</p>}
      </div>
      <div className="mt-3 flex items-center justify-between gap-3 text-sm">
        <span className="text-muted-foreground">{t('fleet.paginationSummary', { first: data?.total ? ((page - 1) * 25) + 1 : 0, last: Math.min(page * 25, data?.total || 0), total: data?.total || 0 })}</span>
        <div className="flex items-center gap-2">
          <button type="button" className="btn-secondary px-2 py-1" disabled={page <= 1} onClick={() => setPage((value) => value - 1)}>{t('common.previous')}</button>
          <span>{t('fleet.pageOf', { page: data?.total_pages ? page : 0, pages: data?.total_pages || 0 })}</span>
          <button type="button" className="btn-secondary px-2 py-1" disabled={!data?.total_pages || page >= data.total_pages} onClick={() => setPage((value) => value + 1)}>{t('common.next')}</button>
        </div>
      </div>
    </div>
  );
}
