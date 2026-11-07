import {getWorkspaces} from '../../../api/system.ts';
import { fleetWriteJSON } from '../../../api/fleetWrites.ts';
import { evidenceJSON } from '../../../api/evidence.ts';
import { listFleetLocations } from '../../../api/fleet.ts';
import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { useQuery, useQueryClient } from '../../../query-client.ts';
import { showStatusMessage } from '../ToastContainer.tsx';
import { ConfirmDialog, TextInputDialog } from '../common/ModalDialog.tsx';
import { useI18n } from '../../../i18n.ts';
import {
  WORKSPACE_KEYS,
  workspaceIsVisible,
} from '../../../utils/workspace-preferences.ts';
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
} from './liveOperator.ts';
import type { StreamAvailabilityFilter, WorkspacesResponse } from '../../../types/core.ts';
import type {
  FloorPlan,
  FloorPlansResponse,
  LiveEvent,
  LiveEventsResponse,
  LiveLayout,
  LiveLayoutsResponse,
  LiveLocation,
  LiveLocationsResponse,
  LiveOperatorFilter,
  LiveOperatorLayout,
  LiveOperatorSurface,
  LiveStream,
} from '../../../types/live.ts';
import { inlineSideOfArrow } from '../../../utils/a11y.ts';

interface LiveEventThumbnailProps {
  event: LiveEvent;
}

export interface LiveOperatorNavigatorProps {
  streams: LiveStream[];
  orderedStreams: LiveStream[];
  columns: number;
  rows: number;
  operatorSurface: LiveOperatorSurface;
  selectedPlanUuid: string;
  onFilterChange: (filter: LiveOperatorFilter | null) => unknown;
  onApplyLayout: (layout: LiveOperatorLayout) => unknown;
  onSurfaceChange: (surface: LiveOperatorSurface) => unknown;
  onSelectPlan: (uuid: string) => unknown;
}

/** A building plan as a camera scope in the Navigator. */
interface PlanCameraGroup {
  uuid: string;
  name: string;
  locationName: string;
  cameraUuids: string[];
  live: number;
}

const EMPTY_LOCATIONS: readonly never[] = Object.freeze([]);

function LiveEventThumbnail({ event }: LiveEventThumbnailProps) {
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

function formatEventTime(timestamp?: number) {
  if (timestamp == null) return '';
  return new Intl.DateTimeFormat(undefined, {
    hour: 'numeric', minute: '2-digit', second: '2-digit',
  }).format(new Date(timestamp * 1000));
}

function locationIcon(type?: string) {
  if (type === 'building') return '▦';
  if (type === 'floor') return '≡';
  if (type === 'area') return '⌗';
  return '▸';
}

function availabilityKey(value?: string) {
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
}: LiveOperatorNavigatorProps) {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const [navigatorPreference] = useState(() => window.matchMedia('(max-width: 768px)').matches
    ? 'lightnvr-live-navigator-mobile-collapsed' : 'lightnvr-live-navigator-collapsed');
  const [collapsed, setCollapsed] = useState(() => {
    const stored = localStorage.getItem(navigatorPreference);
    return stored === null ? navigatorPreference.includes('-mobile-') : stored === 'true';
  });
  const [tab, setTab] = useState('cameras');
  const [selectedLocation, setSelectedLocation] = useState(() =>
    new URLSearchParams(window.location.search).get('location') || '');
  const [availability, setAvailability] = useState<string>(() => {
    const stored = localStorage.getItem('lightnvr-live-availability');
    return LIVE_AVAILABILITY_OPTIONS.includes(stored as StreamAvailabilityFilter) ? stored as string : 'live';
  });
  const [expanded, setExpanded] = useState<Set<string>>(() => new Set());
  const didAutoExpand = useRef(false);
  const [search, setSearch] = useState('');
  const [busy, setBusy] = useState(false);
  const [layoutCameraScope, setLayoutCameraScope] = useState<string[] | null>(null);
  const [selectedLayoutUuid, setSelectedLayoutUuid] = useState('');
  const [scopePlanUuid, setScopePlanUuid] = useState('');
  const [saveLayoutOpen, setSaveLayoutOpen] = useState(false);
  const [renameLayoutTarget, setRenameLayoutTarget] = useState<LiveLayout | null>(null);
  const [deleteLayoutTarget, setDeleteLayoutTarget] = useState<LiveLayout | null>(null);

  const workspaceQuery = useQuery<WorkspacesResponse>({
    queryKey: ['ui-workspaces'],
    queryFn: ({ signal }) => getWorkspaces({signal}),
    staleTime: 60000,
  });
  const locationsQuery = useQuery<LiveLocationsResponse>({
    queryKey: ['fleet-locations'],
    queryFn: ({ signal }) => listFleetLocations({signal}),
    staleTime: 60000,
  });
  const layoutsQuery = useQuery<LiveLayoutsResponse>({
    queryKey: ['live-layouts'],
    queryFn: ({ signal }) => fleetWriteJSON('/api/live/layouts', { signal }),
    staleTime: 15000,
  });
  const plansQuery = useQuery<FloorPlansResponse>({
    queryKey: ['operator-floor-plans'],
    queryFn: ({ signal }) => fleetWriteJSON('/api/live/plans', { signal }),
    staleTime: 15000,
  });

  const enabled = workspaceIsVisible(
    workspaceQuery.data, WORKSPACE_KEYS.LIVE_NAVIGATOR);
  const investigationVisible = workspaceIsVisible(
    workspaceQuery.data, WORKSPACE_KEYS.INVESTIGATION);
  const locations: readonly LiveLocation[] = locationsQuery.data?.locations || EMPTY_LOCATIONS;
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

  const plans: readonly FloorPlan[] = plansQuery.data?.plans || EMPTY_LOCATIONS;
  const locationNames = useMemo(() => new Map(
    locations.map((location) => [location.uuid, location.name]),
  ), [locations]);
  const streamsByUuid = useMemo(() => new Map(
    (streams || []).map((stream) => [stream.camera_uuid, stream]),
  ), [streams]);
  // Cameras placed on a building plan are grouped by plan too: a placement
  // is not a fleet location assignment, so the location tree alone showed
  // such cameras as "Unassigned".
  const planGroups = useMemo(() => plans.map((plan): PlanCameraGroup => {
    const cameraUuids = [...new Set((plan.cameras || [])
      .map((camera) => camera.camera_uuid)
      .filter((uuid) => streamsByUuid.has(uuid)))];
    return {
      uuid: plan.uuid,
      name: plan.name,
      locationName: locationNames.get(plan.location_uuid as string) || '',
      cameraUuids,
      live: cameraUuids.filter((uuid) => streamsByUuid.get(uuid)?.availability === 'live').length,
    };
  }), [plans, streamsByUuid, locationNames]);
  const plansByCamera = useMemo(() => {
    const result = new Map<string, string[]>();
    for (const group of planGroups) {
      for (const uuid of group.cameraUuids) {
        if (!result.has(uuid)) result.set(uuid, []);
        result.get(uuid)!.push(group.name);
      }
    }
    return result;
  }, [planGroups]);

  const eventsQuery = useQuery<{ results: LiveEvent[] }>({
    queryKey: ['live-recent-events', effectiveCameraKey],
    queryFn: async ({ signal }) => {
      const responses = await Promise.all(buildRecentEventsRequests(
        [...effectiveCameraUuids],
      ).map((request) => evidenceJSON<LiveEventsResponse>('/api/investigations/search', {
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
    // Surface failures promptly instead of a long silent retry.
    retry: false,
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
    // First load only, so collapsing the last open location stays collapsed.
    if (tree.length === 0 || didAutoExpand.current) return;
    didAutoExpand.current = true;
    setExpanded(new Set(tree.map((node) => node.uuid)));
  }, [tree]);

  useEffect(() => {
    if (operatorSurface === 'plan') setTab('plans');
  }, [operatorSurface]);

  if (workspaceQuery.isLoading || !enabled) return null;

  const toggleCollapsed = () => {
    const next = !collapsed;
    setCollapsed(next);
    localStorage.setItem(navigatorPreference, String(next));
  };
  const toggleExpanded = (uuid: string) => {
    setExpanded((current) => {
      const next = new Set(current);
      if (next.has(uuid)) next.delete(uuid);
      else next.add(uuid);
      return next;
    });
  };
  const selectLocation = (uuid: string) => {
    setSelectedLocation(uuid);
    setLayoutCameraScope(null);
    setSelectedLayoutUuid('');
    setScopePlanUuid('');
  };
  const loadLocation = (uuid: string) => {
    const cameraUuids = [...cameraUuidsForLocation(locations, streams, uuid)];
    setSelectedLocation(uuid);
    setAvailability('live');
    setLayoutCameraScope(null);
    setSelectedLayoutUuid('');
    setScopePlanUuid('');
    onApplyLayout({ cameraUuids, columns, rows });
  };
  const selectPlanScope = (group: PlanCameraGroup) => {
    setSelectedLocation('');
    setLayoutCameraScope(group.cameraUuids);
    setSelectedLayoutUuid('');
    setScopePlanUuid(group.uuid);
  };
  const loadPlanScope = (group: PlanCameraGroup) => {
    selectPlanScope(group);
    setAvailability('live');
    onApplyLayout({ cameraUuids: group.cameraUuids, columns, rows });
  };
  const applyLayout = (layout: LiveLayout) => {
    const cameraUuids = cameraUuidsForLiveLayout(layout);
    setSelectedLocation(layout.location_uuid || '');
    setAvailability(layout.availability || 'live');
    setLayoutCameraScope(cameraUuids);
    setSelectedLayoutUuid(layout.uuid);
    setScopePlanUuid('');
    onApplyLayout({
      cameraUuids,
      columns: layout.columns,
      rows: layout.rows,
    });
    showStatusMessage(t('live.navigator.layoutLoaded', { name: layout.name }), 'success', 2500);
  };
  const saveLayout = async (name: string, shareRequested: boolean) => {
    const isShared = layoutsQuery.data?.can_share ? shareRequested : false;
    setBusy(true);
    try {
      const layout = await fleetWriteJSON<LiveLayout>('/api/live/layouts', {
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
      showStatusMessage((error as Error).message, 'error', 5000);
      return false;
    } finally {
      setBusy(false);
    }
  };
  /** Overwrite a saved view with the current grid, or rename it. */
  const updateLayout = async (layout: LiveLayout, { name = layout.name, fromCurrentGrid = false }: { name?: string; fromCurrentGrid?: boolean } = {}) => {
    setBusy(true);
    try {
      const payload = fromCurrentGrid
        ? buildLiveLayoutPayload({
          name,
          isShared: layout.is_shared,
          locationUuid: selectedLocation,
          availability,
          columns,
          rows,
          streams: orderedStreams,
        })
        : {
          ...buildLiveLayoutPayload({
            name,
            isShared: layout.is_shared,
            locationUuid: layout.location_uuid || '',
            availability: layout.availability,
            columns: layout.columns,
            rows: layout.rows,
          }),
          camera_slots: layout.camera_slots || [],
        };
      const updated = await fleetWriteJSON<LiveLayout>(`/api/live/layouts/${layout.uuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ...payload, revision: layout.revision }),
        timeout: 15000,
        retries: 0,
      });
      await queryClient.invalidateQueries({ queryKey: ['live-layouts'] });
      if (fromCurrentGrid) {
        setLayoutCameraScope(cameraUuidsForLiveLayout(updated));
        setSelectedLayoutUuid(updated.uuid || layout.uuid);
        setScopePlanUuid('');
      }
      showStatusMessage(fromCurrentGrid
        ? t('live.navigator.layoutUpdated', { name })
        : t('live.navigator.layoutRenamed', { name }), 'success', 2500);
      return true;
    } catch (error) {
      showStatusMessage((error as Error).message, 'error', 5000);
      return false;
    } finally {
      setBusy(false);
    }
  };
  const deleteLayout = async (layout: LiveLayout) => {
    setBusy(true);
    try {
      await fleetWriteJSON(`/api/live/layouts/${layout.uuid}`, {
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
      showStatusMessage((error as Error).message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  if (collapsed) {
    return (
      <aside className="live-operator-navigator is-collapsed">
        <button type="button" className="live-navigator-expand" onClick={toggleCollapsed}
          title={t('live.navigator.expand')} aria-label={t('live.navigator.expand')} aria-expanded={false}>
          <span aria-hidden="true">☰</span>
          <span className="live-mobile-label">{t('live.browseCameras')}</span>
          <span className="live-mobile-label live-camera-count">{orderedStreams.length}</span>
        </button>
      </aside>
    );
  }

  const selectedCameraUuids = [...effectiveCameraUuids];
  return <>
    <aside className="live-operator-navigator" aria-label={t('live.navigator.title')}>
      <div className="live-navigator-heading">
        <div><strong>{t('live.navigator.title')}</strong><small>{t('live.navigator.subtitle')}</small></div>
        <button type="button" onClick={toggleCollapsed} title={t('live.navigator.collapse')} aria-label={t('live.navigator.collapse')} aria-expanded={true}><span dir="ltr" className="rtl-flip inline-block" aria-hidden="true">‹</span></button>
      </div>
      <div
        className="live-navigator-tabs"
        role="tablist"
        aria-label={t('live.navigator.title')}>
        {['cameras', 'plans', 'views', ...(investigationVisible ? ['events'] : [])].map((value) => (
          <button
            key={value}
            type="button"
            id={`live-navigator-tab-${value}`}
            role="tab"
            aria-selected={tab === value}
            aria-controls={`live-navigator-panel-${value}`}
            tabIndex={tab === value ? 0 : -1}
            className={tab === value ? 'is-active' : ''}
            onKeyDown={(event) => {
              const tabValues = ['cameras', 'plans', 'views', ...(investigationVisible ? ['events'] : [])];
              const currentIndex = tabValues.indexOf(tab);
              if (currentIndex < 0) return;
              let nextIndex = currentIndex;
              const side = inlineSideOfArrow(event.key);
              if (side === 'end') nextIndex = (currentIndex + 1) % tabValues.length;
              else if (side === 'start') nextIndex = (currentIndex - 1 + tabValues.length) % tabValues.length;
              else if (event.key === 'Home') nextIndex = 0;
              else if (event.key === 'End') nextIndex = tabValues.length - 1;
              else return;
              event.preventDefault();
              const nextTab = tabValues[nextIndex];
              if (!nextTab) return;
              setTab(nextTab);
              onSurfaceChange(nextTab === 'plans' ? 'plan' : 'grid');
              event.currentTarget.parentElement?.querySelectorAll<HTMLButtonElement>('[role="tab"]')[nextIndex]?.focus();
            }}
            onClick={() => {
              setTab(value);
              onSurfaceChange(value === 'plans' ? 'plan' : 'grid');
            }}>
            {t(`live.navigator.${value}`)}
          </button>
        ))}
      </div>

      <div className="live-navigator-panel" role="tabpanel" id="live-navigator-panel-cameras"
        aria-labelledby="live-navigator-tab-cameras" hidden={tab !== 'cameras'} tabIndex={0}>
        {tab === 'cameras' && <>

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
                aria-expanded={node.children.length > 0 ? expanded.has(node.uuid) : undefined}
                onKeyDown={(event) => {
                  if (node.children.length === 0) return;
                  const side = inlineSideOfArrow(event.key);
                  if ((side === 'end' && !expanded.has(node.uuid)) || (side === 'start' && expanded.has(node.uuid))) {
                    event.preventDefault();
                    toggleExpanded(node.uuid);
                  }
                }}
                onClick={() => {
                  selectLocation(node.uuid);
                  if (node.children.length > 0) toggleExpanded(node.uuid);
                }}
                onDblClick={() => loadLocation(node.uuid)}>
                <span className="live-location-name">
                  {node.children.length > 0 && (
                    <span aria-hidden="true">
                      {expanded.has(node.uuid) ? '▾' : <span dir="ltr" className="rtl-flip inline-block" aria-hidden="true">▸</span>}
                    </span>
                  )}
                  <span dir="ltr" className="rtl-flip inline-block">{locationIcon(node.type)}</span><span>{node.name}</span>
                </span>
                <small>{node.live}/{node.total}</small>
              </button>
            ))}
          </div>
          {planGroups.length > 0 && (
            <div className="live-location-tree live-plan-scope-list">
              <div className="live-camera-list-heading">
                <span>{t('live.navigator.onPlans')}</span><small>{planGroups.length}</small>
              </div>
              {planGroups.map((group) => (
                <button key={group.uuid} type="button"
                  className={`live-location-row ${scopePlanUuid === group.uuid ? 'is-selected' : ''}`}
                  title={t('live.navigator.planScopeHint')}
                  onClick={() => selectPlanScope(group)}
                  onDblClick={() => loadPlanScope(group)}>
                  <span className="live-location-name"><span>⌖</span><span>{group.name}</span>
                    {group.locationName && <small>{group.locationName}</small>}</span>
                  <small>{group.live}/{group.cameraUuids.length}</small>
                </button>
              ))}
            </div>
          )}
          <div className="live-camera-list" aria-live="polite">
            <div className="live-camera-list-heading">
              <span>{t('live.navigator.cameras')}</span><small>{visibleCameras.length}</small>
            </div>
            {visibleCameras.map((stream) => (
              <button key={stream.camera_uuid} type="button" draggable
                className="live-camera-row"
                title={`${t('live.navigator.cameras')}: ${stream.name}`}
                onDragStart={(event) => {
                  event.dataTransfer!.setData('application/x-lightnvr-camera', stream.camera_uuid);
                  event.dataTransfer!.effectAllowed = 'copy';
                }}
                onClick={() => {
                  setLayoutCameraScope([stream.camera_uuid]);
                  setSelectedLayoutUuid('');
                  setScopePlanUuid('');
                  onApplyLayout({
                    cameraUuids: [stream.camera_uuid], columns: 1, rows: 1,
                  });
                }}>
                <span className={`live-availability-dot is-${stream.availability}`} />
                <span><strong>{stream.name}</strong><small>{[
                  stream.location_path || t('live.navigator.unassigned'),
                  ...(plansByCamera.get(stream.camera_uuid) || []),
                ].join(' · ')}</small></span>
              </button>
            ))}
            {visibleCameras.length === 0 && <p className="live-navigator-empty">{t('live.navigator.noCameras')}</p>}
          </div>
        </>}
      </div>

      <div className="live-navigator-panel" role="tabpanel" id="live-navigator-panel-views"
        aria-labelledby="live-navigator-tab-views" hidden={tab !== 'views'} tabIndex={0}>
        {tab === 'views' && <>

          <p className="live-plan-navigator-help">{t('live.navigator.viewsHelp')}</p>
          <button type="button" className="btn-primary live-layout-save" disabled={busy || !layoutsQuery.data?.can_modify}
            onClick={() => setSaveLayoutOpen(true)}>{t('live.navigator.saveCurrent')}</button>
          {layoutsQuery.data && !layoutsQuery.data.can_modify && (
            <p className="live-navigator-empty">{t('live.navigator.viewsReadOnly')}</p>
          )}
          {layoutsQuery.isLoading && <p className="live-navigator-empty">{t('live.navigator.loadingLayouts')}</p>}
          {layoutsQuery.error && (
            <div className="live-navigator-error" role="alert">
              <p>{t('live.navigator.layoutsError', { error: layoutsQuery.error.message })}</p>
              <button type="button" onClick={() => layoutsQuery.refetch()}>{t('live.navigator.retry')}</button>
            </div>
          )}
          {(layoutsQuery.data?.layouts || []).map((layout) => {
            const cameraCount = cameraUuidsForLiveLayout(layout).length;
            const editable = layout.owned && layoutsQuery.data?.can_modify;
            const selected = selectedLayoutUuid === layout.uuid;
            return (
              <div key={layout.uuid}
                className={`live-layout-row ${selected ? 'is-selected' : ''}`}>
                <button type="button" aria-current={selected ? 'true' : undefined}
                  onClick={() => applyLayout(layout)}>
                  <strong>{layout.name}</strong>
                  <small>{layout.columns}×{layout.rows} · {t('live.navigator.layoutCameraCount', { count: cameraCount })} · {t(`availability.${availabilityKey(layout.availability)}`)}{layout.is_shared ? ` · ${t('live.navigator.shared')}` : ''}</small>
                </button>
                {editable && (
                  <div className="live-layout-actions">
                    <button type="button" disabled={busy}
                      title={t('live.navigator.updateLayoutTitle')}
                      aria-label={t('live.navigator.updateLayoutTitle')}
                      onClick={() => updateLayout(layout, { fromCurrentGrid: true })}>⟳</button>
                    <button type="button" disabled={busy}
                      title={t('live.plan.rename')} aria-label={t('live.plan.rename')}
                      onClick={() => setRenameLayoutTarget(layout)}>✎</button>
                    <button type="button" className="live-layout-delete" disabled={busy}
                      title={t('common.delete')} aria-label={t('common.delete')}
                      onClick={() => setDeleteLayoutTarget(layout)}>×</button>
                  </div>
                )}
              </div>
            );
          })}
          {!layoutsQuery.isLoading && !layoutsQuery.error && (layoutsQuery.data?.layouts || []).length === 0 &&
            <p className="live-navigator-empty">{t('live.navigator.noLayouts')}</p>}
        </>}
      </div>

      <div className="live-navigator-panel" role="tabpanel" id="live-navigator-panel-plans"
        aria-labelledby="live-navigator-tab-plans" hidden={tab !== 'plans'} tabIndex={0}>
        {tab === 'plans' && <>

          <p className="live-plan-navigator-help">{t('live.plan.navigatorHelp')}</p>
          {(plansQuery.data?.plans || []).map((plan) => {
            const liveCount = (plan.cameras || []).filter((camera) =>
              streamsByUuid.get(camera.camera_uuid)?.availability === 'live').length;
            return (
              <div key={plan.uuid}
                className={`live-layout-row ${selectedPlanUuid === plan.uuid ? 'is-selected' : ''}`}>
                <button type="button" aria-current={selectedPlanUuid === plan.uuid ? 'true' : undefined}
                  onClick={() => {
                    onSelectPlan(plan.uuid);
                    onSurfaceChange('plan');
                  }}>
                  <strong>{plan.name}</strong>
                  <small>{[
                    t('live.navigator.planCameraCounts', { live: liveCount, total: (plan.cameras || []).length }),
                    locationNames.get(plan.location_uuid as string),
                  ].filter(Boolean).join(' · ')}</small>
                </button>
              </div>
            );
          })}
          {!plansQuery.isLoading && (plansQuery.data?.plans || []).length === 0 &&
            <p className="live-navigator-empty">{t('live.plan.noPlans')}</p>}
        </>}
      </div>

      {investigationVisible && (
      <div className="live-navigator-panel live-event-list" role="tabpanel" id="live-navigator-panel-events"
        aria-labelledby="live-navigator-tab-events" hidden={tab !== 'events'} tabIndex={0}>
        {tab === 'events' && <>

          <div className="live-camera-list-heading">
            <span>{t('live.navigator.eventsScope', { count: effectiveCameraUuids.size })}</span>
            <button type="button" className="live-navigator-refresh"
              disabled={effectiveCameraUuids.size === 0 || eventsQuery.isFetching}
              title={t('live.navigator.refreshEvents')} aria-label={t('live.navigator.refreshEvents')}
              onClick={() => eventsQuery.refetch()}>↻</button>
          </div>
          {effectiveCameraUuids.size === 0 ? (
            // A disabled query never leaves the loading state in Query v4,
            // so an empty scope must be handled before reading isLoading.
            <p className="live-navigator-empty">{t('live.navigator.eventsNoScope')}</p>
          ) : eventsQuery.error ? (
            <div className="live-navigator-error" role="alert">
              <p>{t('live.navigator.eventsError', { error: eventsQuery.error.message })}</p>
              <button type="button" onClick={() => eventsQuery.refetch()}>{t('live.navigator.retry')}</button>
            </div>
          ) : !eventsQuery.data ? (
            <p className="live-navigator-empty">{t('live.navigator.loadingEvents')}</p>
          ) : (eventsQuery.data.results || []).length === 0 ? (
            <p className="live-navigator-empty">{t('live.navigator.noRecentEvents')}</p>
          ) : (
            eventsQuery.data.results.map((event) => (
              <a key={event.result_id} href={investigationHref(event, selectedCameraUuids)} className="live-event-row">
                <LiveEventThumbnail event={event} />
                <span><strong>{event.detection?.label || event.event_type}</strong>
                  <small>{event.camera?.name} · {formatEventTime(event.start_time)}</small></span>
              </a>
            ))
          )}
        </>}
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
    <TextInputDialog
      isOpen={Boolean(renameLayoutTarget)}
      onClose={() => setRenameLayoutTarget(null)}
      onSubmit={(name) => updateLayout(renameLayoutTarget!, { name })}
      title={t('live.navigator.renameLayout')}
      inputLabel={t('common.name')}
      initialValue={renameLayoutTarget?.name || ''}
      confirmLabel={t('live.plan.rename')}
      cancelLabel={t('common.cancel')}
    />
    <ConfirmDialog
      isOpen={Boolean(deleteLayoutTarget)}
      onClose={() => setDeleteLayoutTarget(null)}
      onConfirm={() => deleteLayout(deleteLayoutTarget!)}
      title={t('common.delete')}
      message={deleteLayoutTarget
        ? t('live.navigator.deleteLayoutConfirm', { name: deleteLayoutTarget.name }) : ''}
      confirmLabel={t('common.delete')}
      cancelLabel={t('common.cancel')}
      variant="danger"
    />
  </>;
}
