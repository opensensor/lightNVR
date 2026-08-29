import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON, useQuery, useQueryClient } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog, TextInputDialog } from '../common/ModalDialog.jsx';
import { useI18n } from '../../../i18n.js';
import {
  WORKSPACE_KEYS,
  workspaceIsVisible,
} from '../../../utils/workspace-preferences.js';
import {
  LIVE_AVAILABILITY_OPTIONS,
  buildLiveLayoutPayload,
  buildLocationTree,
  buildRecentEventsRequests,
  cameraUuidsForLiveLayout,
  cameraUuidsForLocation,
  cameraUuidScopeKey,
  filterLiveOperatorStreams,
  flattenLocationTree,
  investigationHref,
} from './liveOperator.js';

const EMPTY_LOCATIONS = Object.freeze([]);

function LiveEventThumbnail({ event }) {
  const url = event.thumbnail?.status === 'available'
    ? event.thumbnail?.url : null;
  const [failed, setFailed] = useState(false);

  useEffect(() => setFailed(false), [url]);

  if (!url || failed) {
    return <span className="live-event-placeholder" aria-hidden="true">◎</span>;
  }
  return (
    <img src={url} alt="" loading="lazy"
      onError={() => setFailed(true)} />
  );
}

function formatEventTime(timestamp) {
  if (!timestamp) return '';
  return new Intl.DateTimeFormat(undefined, {
    hour: 'numeric', minute: '2-digit', second: '2-digit',
  }).format(new Date(timestamp * 1000));
}

function locationIcon(type) {
  if (type === 'building') return '▦';
  if (type === 'floor') return '≡';
  if (type === 'area') return '⌗';
  return '▸';
}

function availabilityKey(value) {
  return value === 'never_connected' ? 'neverConnected' : value;
}

export function LiveOperatorNavigator({
  streams,
  orderedStreams,
  columns,
  rows,
  operatorSurface,
  selectedPlanUuid,
  onFilterChange,
  onApplyLayout,
  onSurfaceChange,
  onSelectPlan,
}) {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const [collapsed, setCollapsed] = useState(() =>
    localStorage.getItem('lightnvr-live-navigator-collapsed') === 'true');
  const [tab, setTab] = useState('cameras');
  const [selectedLocation, setSelectedLocation] = useState(() =>
    new URLSearchParams(window.location.search).get('location') || '');
  const [availability, setAvailability] = useState(() => {
    const stored = localStorage.getItem('lightnvr-live-availability');
    return LIVE_AVAILABILITY_OPTIONS.includes(stored) ? stored : 'live';
  });
  const [expanded, setExpanded] = useState(() => new Set());
  const [search, setSearch] = useState('');
  const [busy, setBusy] = useState(false);
  const [layoutCameraScope, setLayoutCameraScope] = useState(null);
  const [selectedLayoutUuid, setSelectedLayoutUuid] = useState('');
  const [saveLayoutOpen, setSaveLayoutOpen] = useState(false);
  const [deleteLayoutTarget, setDeleteLayoutTarget] = useState(null);

  const workspaceQuery = useQuery({
    queryKey: ['ui-workspaces'],
    queryFn: ({ signal }) => fetchJSON('/api/ui/workspaces', { signal }),
    staleTime: 60000,
  });
  const locationsQuery = useQuery({
    queryKey: ['fleet-locations'],
    queryFn: ({ signal }) => fetchJSON('/api/locations', { signal }),
    staleTime: 60000,
  });
  const layoutsQuery = useQuery({
    queryKey: ['live-layouts'],
    queryFn: ({ signal }) => fetchJSON('/api/live/layouts', { signal }),
    staleTime: 15000,
  });
  const plansQuery = useQuery({
    queryKey: ['operator-floor-plans'],
    queryFn: ({ signal }) => fetchJSON('/api/live/plans', { signal }),
    staleTime: 15000,
  });

  const enabled = workspaceIsVisible(
    workspaceQuery.data, WORKSPACE_KEYS.LIVE_NAVIGATOR);
  const investigationVisible = workspaceIsVisible(
    workspaceQuery.data, WORKSPACE_KEYS.INVESTIGATION);
  const locations = locationsQuery.data?.locations || EMPTY_LOCATIONS;
  const tree = useMemo(() => buildLocationTree(locations, streams), [locations, streams]);
  const locationRows = useMemo(() => flattenLocationTree(tree, expanded), [tree, expanded]);
  const scopedCameraUuids = useMemo(() =>
    cameraUuidsForLocation(locations, streams, selectedLocation),
  [locations, streams, selectedLocation]);
  const scopedCameraKey = cameraUuidScopeKey(scopedCameraUuids);
  const layoutCameraKey = layoutCameraScope === null
    ? null : cameraUuidScopeKey(layoutCameraScope);
  const effectiveCameraKey = layoutCameraKey ?? scopedCameraKey;
  const effectiveCameraUuids = useMemo(() => new Set(
    effectiveCameraKey ? effectiveCameraKey.split(',') : [],
  ), [effectiveCameraKey]);
  const operatorFilter = useMemo(() => ({
    selectedLocation,
    availability,
    cameraUuids: effectiveCameraUuids,
  }), [selectedLocation, availability, effectiveCameraUuids]);
  const scopedStreams = useMemo(() => filterLiveOperatorStreams(
    streams, effectiveCameraUuids, availability),
  [streams, effectiveCameraUuids, availability]);
  const visibleCameras = useMemo(() => {
    const term = search.trim().toLocaleLowerCase();
    if (!term) return scopedStreams;
    return scopedStreams.filter((stream) =>
      stream.name.toLocaleLowerCase().includes(term) ||
      (stream.location_path || '').toLocaleLowerCase().includes(term));
  }, [scopedStreams, search]);

  const eventsQuery = useQuery({
    queryKey: ['live-recent-events', effectiveCameraKey],
    queryFn: async ({ signal }) => {
      const responses = await Promise.all(buildRecentEventsRequests(
        [...effectiveCameraUuids],
      ).map((request) => fetchJSON('/api/investigations/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
        signal,
        timeout: 20000,
        retries: 0,
      })));
      return {
        results: responses.flatMap((response) => response.results || [])
          .sort((a, b) => b.start_time - a.start_time)
          .slice(0, 20),
      };
    },
    enabled: enabled && investigationVisible && tab === 'events' && effectiveCameraUuids.size > 0,
    staleTime: 15000,
    refetchInterval: 30000,
  });

  useEffect(() => {
    if (!enabled) {
      onFilterChange(null);
      return;
    }
    onFilterChange(operatorFilter);
    localStorage.setItem('lightnvr-live-availability', availability);
    const url = new URL(window.location.href);
    if (selectedLocation) url.searchParams.set('location', selectedLocation);
    else url.searchParams.delete('location');
    window.history.replaceState({}, '', url);
  }, [enabled, selectedLocation, availability, operatorFilter, onFilterChange]);

  useEffect(() => {
    if (tree.length === 0 || expanded.size > 0) return;
    setExpanded(new Set(tree.map((node) => node.uuid)));
  }, [tree, expanded.size]);

  useEffect(() => {
    if (operatorSurface === 'plan') setTab('plans');
  }, [operatorSurface]);

  if (workspaceQuery.isLoading || !enabled) return null;

  const toggleCollapsed = () => {
    const next = !collapsed;
    setCollapsed(next);
    localStorage.setItem('lightnvr-live-navigator-collapsed', String(next));
  };
  const toggleExpanded = (uuid) => {
    setExpanded((current) => {
      const next = new Set(current);
      if (next.has(uuid)) next.delete(uuid);
      else next.add(uuid);
      return next;
    });
  };
  const selectLocation = (uuid) => {
    setSelectedLocation(uuid);
    setLayoutCameraScope(null);
    setSelectedLayoutUuid('');
  };
  const loadLocation = (uuid) => {
    const cameraUuids = [...cameraUuidsForLocation(locations, streams, uuid)];
    setSelectedLocation(uuid);
    setAvailability('live');
    setLayoutCameraScope(null);
    setSelectedLayoutUuid('');
    onApplyLayout({ cameraUuids, columns, rows });
  };
  const applyLayout = (layout) => {
    const cameraUuids = cameraUuidsForLiveLayout(layout);
    setSelectedLocation(layout.location_uuid || '');
    setAvailability(layout.availability || 'live');
    setLayoutCameraScope(cameraUuids);
    setSelectedLayoutUuid(layout.uuid);
    onApplyLayout({
      cameraUuids,
      columns: layout.columns,
      rows: layout.rows,
    });
    showStatusMessage(t('live.navigator.layoutLoaded', { name: layout.name }), 'success', 2500);
  };
  const saveLayout = async (name, shareRequested) => {
    const isShared = layoutsQuery.data?.can_share ? shareRequested : false;
    setBusy(true);
    try {
      const layout = await fetchJSON('/api/live/layouts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(buildLiveLayoutPayload({
          name,
          isShared,
          locationUuid: selectedLocation,
          availability,
          columns,
          rows,
          streams: orderedStreams,
        })),
        timeout: 15000,
        retries: 0,
      });
      await queryClient.invalidateQueries({ queryKey: ['live-layouts'] });
      setLayoutCameraScope(cameraUuidsForLiveLayout(layout));
      setSelectedLayoutUuid(layout.uuid || '');
      showStatusMessage(t('live.navigator.layoutSaved', { name }), 'success', 2500);
      return true;
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
      return false;
    } finally {
      setBusy(false);
    }
  };
  const deleteLayout = async (layout) => {
    setBusy(true);
    try {
      await fetchJSON(`/api/live/layouts/${layout.uuid}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ revision: layout.revision }),
        timeout: 15000,
        retries: 0,
      });
      await queryClient.invalidateQueries({ queryKey: ['live-layouts'] });
      if (selectedLayoutUuid === layout.uuid) {
        setLayoutCameraScope(null);
        setSelectedLayoutUuid('');
      }
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  if (collapsed) {
    return (
      <aside className="live-operator-navigator is-collapsed">
        <button type="button" className="live-navigator-expand" onClick={toggleCollapsed}
          title={t('live.navigator.expand')} aria-label={t('live.navigator.expand')}>☰</button>
      </aside>
    );
  }

  const selectedCameraUuids = [...effectiveCameraUuids];
  return <>
    <aside className="live-operator-navigator" aria-label={t('live.navigator.title')}>
      <div className="live-navigator-heading">
        <div><strong>{t('live.navigator.title')}</strong><small>{t('live.navigator.subtitle')}</small></div>
        <button type="button" onClick={toggleCollapsed} title={t('live.navigator.collapse')}>‹</button>
      </div>
      <div className="live-navigator-tabs" role="tablist">
        {['cameras', 'plans', 'views', ...(investigationVisible ? ['events'] : [])].map((value) => (
          <button key={value} type="button" role="tab" aria-selected={tab === value}
            className={tab === value ? 'is-active' : ''} onClick={() => {
              setTab(value);
              onSurfaceChange(value === 'plans' ? 'plan' : 'grid');
            }}>
            {t(`live.navigator.${value}`)}
          </button>
        ))}
      </div>

      {tab === 'cameras' && (
        <div className="live-navigator-panel">
          <label className="live-navigator-search">
            <span className="sr-only">{t('live.navigator.search')}</span>
            <input type="search" value={search} placeholder={t('live.navigator.search')}
              onInput={(event) => setSearch(event.currentTarget.value)} />
          </label>
          <select className="live-navigator-availability" value={availability}
            onChange={(event) => setAvailability(event.currentTarget.value)}>
            {LIVE_AVAILABILITY_OPTIONS.map((value) => (
              <option key={value} value={value}>{t(`availability.${availabilityKey(value)}`)}</option>
            ))}
          </select>
          <button type="button" className={`live-location-row ${selectedLocation === '' && layoutCameraScope === null ? 'is-selected' : ''}`}
            onClick={() => selectLocation('')} onDblClick={() => loadLocation('')}>
            <span>⌂ {t('live.navigator.allCameras')}</span>
            <small>{streams.filter((stream) => stream.availability === 'live').length}/{streams.length}</small>
          </button>
          <div className="live-location-tree">
            {locationRows.map((node) => (
              <button key={node.uuid} type="button"
                className={`live-location-row ${selectedLocation === node.uuid && layoutCameraScope === null ? 'is-selected' : ''}`}
                style={{ '--location-depth': node.depth }}
                onClick={() => selectLocation(node.uuid)}
                onDblClick={() => loadLocation(node.uuid)}>
                <span className="live-location-name">
                  {node.children.length > 0 && (
                    <span onClick={(event) => {
                      event.stopPropagation(); toggleExpanded(node.uuid);
                    }}>{expanded.has(node.uuid) ? '▾' : '▸'}</span>
                  )}
                  <span>{locationIcon(node.type)}</span><span>{node.name}</span>
                </span>
                <small>{node.live}/{node.total}</small>
              </button>
            ))}
          </div>
          <div className="live-camera-list" aria-live="polite">
            <div className="live-camera-list-heading">
              <span>{t('live.navigator.cameras')}</span><small>{visibleCameras.length}</small>
            </div>
            {visibleCameras.map((stream) => (
              <button key={stream.camera_uuid} type="button" draggable
                className="live-camera-row"
                title={`${t('live.navigator.cameras')}: ${stream.name}`}
                onDragStart={(event) => {
                  event.dataTransfer.setData('application/x-lightnvr-camera', stream.camera_uuid);
                  event.dataTransfer.effectAllowed = 'copy';
                }}
                onClick={() => {
                  setLayoutCameraScope([stream.camera_uuid]);
                  setSelectedLayoutUuid('');
                  onApplyLayout({
                    cameraUuids: [stream.camera_uuid], columns: 1, rows: 1,
                  });
                }}>
                <span className={`live-availability-dot is-${stream.availability}`} />
                <span><strong>{stream.name}</strong><small>{stream.location_path || t('live.navigator.unassigned')}</small></span>
              </button>
            ))}
            {visibleCameras.length === 0 && <p className="live-navigator-empty">{t('live.navigator.noCameras')}</p>}
          </div>
        </div>
      )}

      {tab === 'views' && (
        <div className="live-navigator-panel">
          <button type="button" className="btn-primary live-layout-save" disabled={busy || !layoutsQuery.data?.can_modify}
            onClick={() => setSaveLayoutOpen(true)}>{t('live.navigator.saveCurrent')}</button>
          {(layoutsQuery.data?.layouts || []).map((layout) => (
            <div key={layout.uuid}
              className={`live-layout-row ${selectedLayoutUuid === layout.uuid ? 'is-selected' : ''}`}>
              <button type="button" aria-current={selectedLayoutUuid === layout.uuid ? 'true' : undefined}
                onClick={() => applyLayout(layout)}>
                <strong>{layout.name}</strong>
                <small>{layout.columns}×{layout.rows} · {t(`availability.${availabilityKey(layout.availability)}`)}{layout.is_shared ? ` · ${t('live.navigator.shared')}` : ''}</small>
              </button>
              {layout.owned && layoutsQuery.data?.can_modify && (
                <button type="button" className="live-layout-delete" disabled={busy}
                  title={t('common.delete')} onClick={() => setDeleteLayoutTarget(layout)}>×</button>
              )}
            </div>
          ))}
          {!layoutsQuery.isLoading && (layoutsQuery.data?.layouts || []).length === 0 &&
            <p className="live-navigator-empty">{t('live.navigator.noLayouts')}</p>}
        </div>
      )}

      {tab === 'plans' && (
        <div className="live-navigator-panel">
          <p className="live-plan-navigator-help">Choose a building or floor. Camera previews open only when requested.</p>
          {(plansQuery.data?.plans || []).map((plan) => {
            const liveCount = (plan.cameras || []).filter((camera) =>
              streams.find((stream) => stream.camera_uuid === camera.camera_uuid)?.availability === 'live').length;
            return (
              <div key={plan.uuid}
                className={`live-layout-row ${selectedPlanUuid === plan.uuid ? 'is-selected' : ''}`}>
                <button type="button" aria-current={selectedPlanUuid === plan.uuid ? 'true' : undefined}
                  onClick={() => {
                    onSelectPlan(plan.uuid);
                    onSurfaceChange('plan');
                  }}>
                  <strong>{plan.name}</strong>
                  <small>{liveCount}/{(plan.cameras || []).length} live · {(plan.cameras || []).length} placed</small>
                </button>
              </div>
            );
          })}
          {!plansQuery.isLoading && (plansQuery.data?.plans || []).length === 0 &&
            <p className="live-navigator-empty">No building plans yet. Create one in the plan workspace.</p>}
        </div>
      )}

      {tab === 'events' && (
        <div className="live-navigator-panel live-event-list">
          {eventsQuery.isLoading && <p className="live-navigator-empty">{t('live.navigator.loadingEvents')}</p>}
          {(eventsQuery.data?.results || []).map((event) => (
            <a key={event.result_id} href={investigationHref(event, selectedCameraUuids)} className="live-event-row">
              <LiveEventThumbnail event={event} />
              <span><strong>{event.detection?.label || event.event_type}</strong>
                <small>{event.camera?.name} · {formatEventTime(event.start_time)}</small></span>
            </a>
          ))}
          {!eventsQuery.isLoading && (eventsQuery.data?.results || []).length === 0 &&
            <p className="live-navigator-empty">{t('live.navigator.noRecentEvents')}</p>}
        </div>
      )}
    </aside>
    <TextInputDialog
      isOpen={saveLayoutOpen}
      onClose={() => setSaveLayoutOpen(false)}
      onSubmit={saveLayout}
      title={t('live.navigator.layoutNamePrompt')}
      inputLabel={t('common.name')}
      confirmLabel={t('common.saveChanges')}
      cancelLabel={t('common.cancel')}
      checkboxLabel={layoutsQuery.data?.can_share
        ? t('live.navigator.shareLayoutPrompt') : undefined}
    />
    <ConfirmDialog
      isOpen={Boolean(deleteLayoutTarget)}
      onClose={() => setDeleteLayoutTarget(null)}
      onConfirm={() => deleteLayout(deleteLayoutTarget)}
      title={t('common.delete')}
      message={deleteLayoutTarget
        ? t('live.navigator.deleteLayoutConfirm', { name: deleteLayoutTarget.name }) : ''}
      confirmLabel={t('common.delete')}
      cancelLabel={t('common.cancel')}
      variant="danger"
    />
  </>;
}
