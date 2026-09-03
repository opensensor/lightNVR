import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON, useQuery, useQueryClient } from '../../query-client.js';
import { useI18n } from '../../i18n.js';
import { showStatusMessage } from './ToastContainer.jsx';
import { EventDestinationEditor } from './events/EventDestinationEditor.jsx';
import { EventRouteEditor } from './events/EventRouteEditor.jsx';
import { ConfirmDialog } from './common/ModalDialog.jsx';
import {
  groupCatalog,
  healthConditionsFromSettings,
  readEventSection,
  shortEventType,
  writeEventSection,
} from './events/eventRouting.js';

function SummaryCard({ label, value, detail }) {
  return <div className="rounded-lg border border-border bg-card p-4 shadow-sm"><div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{label}</div><div className="mt-1 text-2xl font-bold tabular-nums">{value}</div>{detail && <div className="mt-1 text-xs text-muted-foreground">{detail}</div>}</div>;
}

function EmptyState({ title, description, action, actionLabel }) {
  return <div className="flex flex-col items-center px-6 py-14 text-center"><div className="mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-muted text-2xl" aria-hidden="true">↗</div><h3 className="text-lg font-semibold">{title}</h3><p className="mt-1 max-w-lg text-sm text-muted-foreground">{description}</p>{action && <button type="button" className="btn-primary mt-4" onClick={action}>{actionLabel}</button>}</div>;
}

function LoadingOrError({ loading, error, onRetry, children, t }) {
  if (loading) return <div className="p-12 text-center text-sm text-muted-foreground">{t('common.loading')}</div>;
  if (error) return <div className="p-12 text-center"><h3 className="font-semibold text-[hsl(var(--danger))]">{t('events.loadError')}</h3><p className="mt-1 text-sm text-muted-foreground">{error.status === 403 ? t('events.accessDenied') : error.message}</p><button type="button" className="btn-secondary mt-4" onClick={onRetry}>{t('common.retry')}</button></div>;
  return children;
}

function RouteList({ routes, destinationNames, busyKey, onEdit, onToggle, onDelete, onAdd, t }) {
  if (routes.length === 0) return <EmptyState title={t('events.route.emptyTitle')} description={t('events.route.emptyDescription')} action={onAdd} actionLabel={t('events.route.add')} />;
  return (
    <div className="divide-y divide-border">
      {routes.map((route) => {
        const suppression = route.suppression || {};
        const suppressionValues = [suppression.debounce_seconds, suppression.cooldown_seconds, suppression.grouping_window_seconds, suppression.max_events_per_minute].filter((value) => Number(value) > 0);
        return (
          <article key={route.uuid} className="p-4 sm:p-5">
            <div className="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
              <div className="min-w-0 flex-1">
                <div className="flex flex-wrap items-center gap-2"><h3 className="text-base font-semibold">{route.name}</h3><span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${route.enabled ? 'badge-success' : 'bg-muted text-muted-foreground'}`}>{t(route.enabled ? 'common.enabled' : 'common.disabled')}</span></div>
                {route.description && <p className="mt-1 text-sm text-muted-foreground">{route.description}</p>}
                <dl className="mt-3 grid gap-3 text-sm sm:grid-cols-2 xl:grid-cols-4">
                  <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.route.destination')}</dt><dd className="mt-0.5 truncate">{destinationNames.get(route.destination) || route.destination}</dd></div>
                  <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.route.eventTypes')}</dt><dd className="mt-0.5">{(route.event_types || []).map(shortEventType).join(', ')}</dd></div>
                  <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.route.cameraScope')}</dt><dd className="mt-0.5">{t(route.camera_scope?.type === 'selector' ? 'events.route.dynamicScope' : 'events.route.allCameras')}</dd></div>
                  <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.route.deliveryPolicy')}</dt><dd className="mt-0.5">{suppressionValues.length ? t('events.route.suppressionActive') : t('events.route.everyMatch')}</dd></div>
                </dl>
              </div>
              <div className="flex flex-wrap gap-2 xl:justify-end">
                <button type="button" className="btn-secondary" onClick={() => onEdit(route)}>{t('common.edit')}</button>
                <button type="button" className="btn-secondary" disabled={busyKey === route.uuid} onClick={() => onToggle(route)}>{t(route.enabled ? 'common.disable' : 'events.enable')}</button>
                <button type="button" className="rounded-md border border-[hsl(var(--danger)/0.55)] px-3 py-2 text-sm text-[hsl(var(--danger))] hover:bg-[hsl(var(--danger)/0.08)]" disabled={busyKey === route.uuid} onClick={() => onDelete(route)}>{t('common.delete')}</button>
              </div>
            </div>
          </article>
        );
      })}
    </div>
  );
}

function DestinationList({ defaultDestination, destinations, routes, busyKey, onEdit, onToggle, onDelete, onAdd, t }) {
  const routeCounts = useMemo(() => {
    const counts = new Map();
    routes.forEach((route) => counts.set(route.destination, (counts.get(route.destination) || 0) + 1));
    return counts;
  }, [routes]);
  return (
    <div>
      <article className="border-b border-border bg-muted/10 p-4 sm:p-5">
        <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div><div className="flex flex-wrap items-center gap-2"><h3 className="font-semibold">{defaultDestination?.name || t('events.destination.default')}</h3><span className="rounded-full badge-info px-2 py-0.5 text-xs font-semibold">{t('events.destination.settingsManaged')}</span></div><p className="mt-1 text-sm text-muted-foreground">{t('events.destination.defaultDescription', { count: routeCounts.get('mqtt:default') || 0 })}</p></div>
          <a className="btn-secondary text-center" href="/settings.html#mqtt">{t('events.destination.openSettings')}</a>
        </div>
      </article>
      {destinations.length === 0 ? <EmptyState title={t('events.destination.emptyTitle')} description={t('events.destination.emptyDescription')} action={onAdd} actionLabel={t('events.destination.add')} /> : (
        <div className="divide-y divide-border">
          {destinations.map((destination) => (
            <article key={destination.uuid} className="p-4 sm:p-5">
              <div className="flex flex-col gap-4 xl:flex-row xl:items-start xl:justify-between">
                <div className="min-w-0 flex-1">
                  <div className="flex flex-wrap items-center gap-2"><h3 className="font-semibold">{destination.name}</h3><span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${destination.enabled ? 'badge-success' : 'bg-muted text-muted-foreground'}`}>{t(destination.enabled ? 'events.destination.active' : 'events.destination.paused')}</span></div>
                  {destination.description && <p className="mt-1 text-sm text-muted-foreground">{destination.description}</p>}
                  <dl className="mt-3 grid gap-3 text-sm sm:grid-cols-2 xl:grid-cols-4">
                    <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.destination.broker')}</dt><dd className="mt-0.5 font-mono text-xs break-all">{destination.broker?.host}:{destination.broker?.port}</dd></div>
                    <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.destination.topic')}</dt><dd className="mt-0.5 truncate font-mono text-xs" title={destination.broker?.topic_template}>{destination.broker?.topic_template}</dd></div>
                    <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.destination.presence')}</dt><dd className="mt-0.5 truncate font-mono text-xs" title={destination.broker?.status_topic_template || ''}>{destination.broker?.status_topic_template || t('common.disabled')}</dd></div>
                    <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.destination.security')}</dt><dd className="mt-0.5">{t(`events.destination.tls.${destination.tls?.mode || 'system'}`)} · QoS {destination.broker?.qos}</dd></div>
                    <div><dt className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('events.destination.routes')}</dt><dd className="mt-0.5">{routeCounts.get(destination.key) || 0}</dd></div>
                  </dl>
                </div>
                <div className="flex flex-wrap gap-2 xl:justify-end">
                  <button type="button" className="btn-secondary" onClick={() => onEdit(destination)}>{t('common.edit')}</button>
                  <button type="button" className="btn-secondary" disabled={busyKey === destination.uuid} onClick={() => onToggle(destination)}>{t(destination.enabled ? 'events.destination.pause' : 'events.enable')}</button>
                  <button type="button" className="rounded-md border border-[hsl(var(--danger)/0.55)] px-3 py-2 text-sm text-[hsl(var(--danger))] hover:bg-[hsl(var(--danger)/0.08)]" disabled={busyKey === destination.uuid || (routeCounts.get(destination.key) || 0) > 0} title={(routeCounts.get(destination.key) || 0) > 0 ? t('events.destination.deleteInUse') : ''} onClick={() => onDelete(destination)}>{t('common.delete')}</button>
                </div>
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}

function EventCatalog({ eventTypes, t }) {
  const groups = useMemo(() => groupCatalog(eventTypes), [eventTypes]);
  return <div className="space-y-5 p-4 sm:p-5">{groups.map((group) => <section key={group.family}><h3 className="text-sm font-semibold uppercase tracking-[0.14em] text-muted-foreground">{group.family}</h3><div className="mt-2 grid gap-3 md:grid-cols-2 xl:grid-cols-3">{group.types.map((eventType) => <article key={eventType.type} className="rounded-lg border border-border bg-background p-4"><div className="flex flex-wrap items-start justify-between gap-2"><h4 className="font-semibold">{shortEventType(eventType.type)}</h4><span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${eventType.severity === 'critical' || eventType.severity === 'error' ? 'badge-danger' : eventType.severity === 'warning' ? 'badge-warning' : 'badge-info'}`}>{eventType.severity}</span></div><p className="mt-2 text-sm text-muted-foreground">{eventType.description}</p><div className="mt-3 flex flex-wrap gap-2 text-xs text-muted-foreground"><span className="rounded bg-muted px-2 py-1">{eventType.expected_rate}</span><span className="rounded bg-muted px-2 py-1">{eventType.sensitivity}</span><span className="rounded bg-muted px-2 py-1">{eventType.subject_kind}</span></div><code className="mt-3 block break-all text-[10px] text-muted-foreground">{eventType.type}</code></article>)}</div></section>)}</div>;
}

export function EventRoutingView() {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const [section, setSection] = useState(() => readEventSection(typeof window === 'undefined' ? '' : window.location.search));
  const [routeEditor, setRouteEditor] = useState(null);
  const [destinationEditor, setDestinationEditor] = useState(null);
  const [busyKey, setBusyKey] = useState('');
  const [deleteCandidate, setDeleteCandidate] = useState(null);

  const routesQuery = useQuery(['event-routes'], '/api/event-routes', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 5000 });
  const destinationsQuery = useQuery(['event-destinations'], '/api/event-destinations', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 5000 });
  const catalogQuery = useQuery(['event-catalog'], '/api/events/catalog', { cache: 'no-store', timeout: 15000, retries: 1 }, { staleTime: 60000 });
  const healthRegistryQuery = useQuery(['settings-health-registry'], '/api/settings', { cache: 'no-store', timeout: 15000, retries: 0 }, { staleTime: 60000 });
  const locationsQuery = useQuery(['fleet-locations'], '/api/locations', {}, { staleTime: 60000 });
  const tagsQuery = useQuery(['fleet-tags'], '/api/camera-tags', {}, { staleTime: 60000 });

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.history.replaceState({}, '', writeEventSection(window.location.href, section));
  }, [section]);

  const routes = routesQuery.data?.routes || [];
  const destinations = destinationsQuery.data?.destinations || [];
  const defaultDestination = destinationsQuery.data?.default_destination;
  const eventTypes = catalogQuery.data?.event_types || [];
  const healthConditions = useMemo(
    () => healthConditionsFromSettings(healthRegistryQuery.data),
    [healthRegistryQuery.data]
  );
  const destinationOptions = useMemo(() => [
    defaultDestination || { key: 'mqtt:default', name: t('events.destination.default'), managed: false },
    ...destinations,
  ], [defaultDestination, destinations, t]);
  const destinationNames = useMemo(() => new Map(destinationOptions.map((destination) => [destination.key, destination.name])), [destinationOptions]);
  const activeRoutes = routes.filter((route) => route.enabled).length;
  const activeDestinations = destinations.filter((destination) => destination.enabled).length;
  const loading = routesQuery.isLoading || destinationsQuery.isLoading || catalogQuery.isLoading;
  const error = routesQuery.error || destinationsQuery.error || catalogQuery.error;

  const refresh = async () => Promise.all([routesQuery.refetch(), destinationsQuery.refetch(), catalogQuery.refetch()]);
  const staleAware = async (operation, refetch) => {
    try {
      return await operation();
    } catch (requestError) {
      showStatusMessage(requestError.message, 'error', 8000);
      if (requestError.status === 409 && refetch) await refetch();
      throw requestError;
    }
  };

  const saveRoute = async (payload) => {
    const editing = Boolean(routeEditor?.uuid);
    const response = await staleAware(() => fetchJSON(editing ? `/api/event-routes/${encodeURIComponent(routeEditor.uuid)}` : '/api/event-routes', { method: editing ? 'PUT' : 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 }), routesQuery.refetch);
    await queryClient.invalidateQueries({ queryKey: ['event-routes'] });
    setRouteEditor(null);
    showStatusMessage(t(editing ? 'events.route.updated' : 'events.route.created'), 'success');
    return response;
  };
  const previewRoute = (payload) => staleAware(() => fetchJSON('/api/event-routes/preview', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 }));
  const toggleRoute = async (route) => {
    setBusyKey(route.uuid);
    try { await staleAware(() => fetchJSON(`/api/event-routes/${encodeURIComponent(route.uuid)}`, { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ enabled: !route.enabled, revision: route.revision }), timeout: 15000, retries: 0 }), routesQuery.refetch); await routesQuery.refetch(); showStatusMessage(t(route.enabled ? 'events.route.disabled' : 'events.route.enabledMessage'), 'success'); } catch (_error) { /* surfaced by staleAware */ } finally { setBusyKey(''); }
  };
  const deleteRoute = async (route) => {
    setBusyKey(route.uuid);
    try { await staleAware(() => fetchJSON(`/api/event-routes/${encodeURIComponent(route.uuid)}?revision=${encodeURIComponent(route.revision)}`, { method: 'DELETE', timeout: 15000, retries: 0 }), routesQuery.refetch); await routesQuery.refetch(); showStatusMessage(t('events.route.deleted'), 'success'); } catch (_error) { /* surfaced by staleAware */ } finally { setBusyKey(''); }
  };

  const saveDestination = async (payload) => {
    const editing = Boolean(destinationEditor?.uuid);
    const response = await staleAware(() => fetchJSON(editing ? `/api/event-destinations/${encodeURIComponent(destinationEditor.uuid)}` : '/api/event-destinations', { method: editing ? 'PUT' : 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload), timeout: 20000, retries: 0 }), destinationsQuery.refetch);
    await queryClient.invalidateQueries({ queryKey: ['event-destinations'] });
    setDestinationEditor(null);
    showStatusMessage(t(editing ? 'events.destination.updated' : 'events.destination.created'), 'success');
    return response;
  };
  const toggleDestination = async (destination) => {
    setBusyKey(destination.uuid);
    try { await staleAware(() => fetchJSON(`/api/event-destinations/${encodeURIComponent(destination.uuid)}`, { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ enabled: !destination.enabled, revision: destination.revision }), timeout: 15000, retries: 0 }), destinationsQuery.refetch); await destinationsQuery.refetch(); showStatusMessage(t(destination.enabled ? 'events.destination.pausedMessage' : 'events.destination.enabledMessage'), 'success'); } catch (_error) { /* surfaced by staleAware */ } finally { setBusyKey(''); }
  };
  const deleteDestination = async (destination) => {
    setBusyKey(destination.uuid);
    try { await staleAware(() => fetchJSON(`/api/event-destinations/${encodeURIComponent(destination.uuid)}?revision=${encodeURIComponent(destination.revision)}`, { method: 'DELETE', timeout: 15000, retries: 0 }), destinationsQuery.refetch); await destinationsQuery.refetch(); showStatusMessage(t('events.destination.deleted'), 'success'); } catch (_error) { /* surfaced by staleAware */ } finally { setBusyKey(''); }
  };

  const sections = [
    ['routes', t('events.routes'), routes.length],
    ['destinations', t('events.destinations'), destinations.length + 1],
    ['catalog', t('events.catalog'), eventTypes.length],
  ];
  return (
    <div className="py-2" aria-labelledby="events-title">
      <div className="mb-5 flex flex-col gap-3 lg:flex-row lg:items-end lg:justify-between"><div><div className="text-xs font-semibold uppercase tracking-[0.18em] text-[hsl(var(--primary))]">{t('events.eyebrow')}</div><h1 id="events-title" className="mt-1 text-3xl font-bold">{t('events.title')}</h1><p className="mt-1 max-w-3xl text-sm text-muted-foreground">{t('events.description')}</p></div><div className="flex flex-wrap gap-2">{section === 'routes' && <button type="button" className="btn-primary" onClick={() => setRouteEditor({})}>{t('events.route.add')}</button>}{section === 'destinations' && <button type="button" className="btn-primary" onClick={() => setDestinationEditor({})}>{t('events.destination.add')}</button>}<button type="button" className="btn-secondary" onClick={refresh}>{t('common.refresh')}</button></div></div>
      <div className="mb-5 grid grid-cols-2 gap-3 lg:grid-cols-4"><SummaryCard label={t('events.summary.routes')} value={routes.length} detail={t('events.summary.enabled', { count: activeRoutes })} /><SummaryCard label={t('events.summary.destinations')} value={destinations.length + 1} detail={t('events.summary.managedActive', { count: activeDestinations })} /><SummaryCard label={t('events.summary.types')} value={eventTypes.length} detail={t('events.summary.registry')} /><SummaryCard label={t('events.summary.interface')} value="MQTT" detail={t('events.summary.interfaceHelp')} /></div>
      <div className="overflow-hidden rounded-lg border border-border bg-card shadow-sm"><div className="flex flex-wrap gap-1 border-b border-border bg-muted/15 p-2" role="tablist" aria-label={t('events.title')}>{sections.map(([key, label, count]) => <button key={key} type="button" role="tab" aria-selected={section === key} className={`rounded-md px-3 py-2 text-sm font-medium ${section === key ? 'bg-background text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}`} onClick={() => setSection(key)}>{label}<span className="ml-2 rounded-full bg-muted px-2 py-0.5 text-xs tabular-nums">{count}</span></button>)}</div><LoadingOrError loading={loading} error={error} onRetry={refresh} t={t}>{section === 'routes' ? <RouteList routes={routes} destinationNames={destinationNames} busyKey={busyKey} onEdit={setRouteEditor} onToggle={toggleRoute} onDelete={(route) => setDeleteCandidate({ kind: 'route', item: route })} onAdd={() => setRouteEditor({})} t={t} /> : section === 'destinations' ? <DestinationList defaultDestination={defaultDestination} destinations={destinations} routes={routes} busyKey={busyKey} onEdit={setDestinationEditor} onToggle={toggleDestination} onDelete={(destination) => setDeleteCandidate({ kind: 'destination', item: destination })} onAdd={() => setDestinationEditor({})} t={t} /> : <EventCatalog eventTypes={eventTypes} t={t} />}</LoadingOrError></div>
      {routeEditor && <EventRouteEditor key={routeEditor.uuid || 'new'} route={routeEditor.uuid ? routeEditor : null} catalog={eventTypes} healthConditions={healthConditions} healthRegistryUnavailable={Boolean(healthRegistryQuery.error)} destinations={destinationOptions} locations={locationsQuery.data?.locations || []} tags={tagsQuery.data?.tags || []} onPreview={previewRoute} onSave={saveRoute} onClose={() => setRouteEditor(null)} t={t} />}
      {destinationEditor && <EventDestinationEditor key={destinationEditor.uuid || 'new'} destination={destinationEditor.uuid ? destinationEditor : null} onSave={saveDestination} onClose={() => setDestinationEditor(null)} t={t} />}
      <ConfirmDialog
        isOpen={Boolean(deleteCandidate)}
        onClose={() => setDeleteCandidate(null)}
        onConfirm={() => deleteCandidate?.kind === 'route'
          ? deleteRoute(deleteCandidate.item) : deleteDestination(deleteCandidate.item)}
        title={t('common.delete')}
        message={deleteCandidate
          ? t(deleteCandidate.kind === 'route'
            ? 'events.route.deleteConfirm' : 'events.destination.deleteConfirm',
          { name: deleteCandidate.item.name })
          : ''}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </div>
  );
}
