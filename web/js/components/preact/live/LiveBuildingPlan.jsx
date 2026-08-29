import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { fetchJSON, useQuery, useQueryClient } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { TextInputDialog } from '../common/ModalDialog.jsx';
import { useI18n } from '../../../i18n.js';
import { clusterFloorPlanCameras, floorPlanPayload } from './liveOperator.js';

function clamp(value, minimum, maximum) {
  return Math.max(minimum, Math.min(maximum, value));
}

function PlanCameraPreview({ stream }) {
  const [failed, setFailed] = useState(false);
  const url = stream?.availability === 'live'
    ? `/go2rtc/api/frame.jpeg?src=${encodeURIComponent(stream.name)}&t=${Date.now()}`
    : null;
  useEffect(() => setFailed(false), [stream?.camera_uuid]);
  if (!url || failed) {
    return <div className="live-plan-preview-placeholder">Camera preview unavailable</div>;
  }
  return <img src={url} alt={`Current view from ${stream.name}`}
    onError={() => setFailed(true)} />;
}

function availabilityLabel(stream, t) {
  if (!stream) return t('availability.neverConnected');
  if (stream.availability === 'never_connected') return t('availability.neverConnected');
  return t(`availability.${stream.availability || 'offline'}`);
}

export function LiveBuildingPlan({
  streams,
  selectedPlanUuid,
  onSelectPlan,
  onOpenCamera,
}) {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const svgRef = useRef(null);
  const [editing, setEditing] = useState(false);
  const [draftCameras, setDraftCameras] = useState([]);
  const [selectedCameraUuid, setSelectedCameraUuid] = useState('');
  const [placementCameraUuid, setPlacementCameraUuid] = useState('');
  const [zoom, setZoom] = useState(1);
  const [center, setCenter] = useState({ x: 0.5, y: 0.5 });
  const [busy, setBusy] = useState(false);
  const [cameraSearch, setCameraSearch] = useState('');
  const [createPlanOpen, setCreatePlanOpen] = useState(false);

  const plansQuery = useQuery({
    queryKey: ['operator-floor-plans'],
    queryFn: ({ signal }) => fetchJSON('/api/live/plans', { signal }),
    staleTime: 15000,
  });
  const plans = plansQuery.data?.plans || [];
  const plan = plans.find((item) => item.uuid === selectedPlanUuid) || plans[0] || null;
  const streamsByUuid = useMemo(() => new Map(
    (streams || []).map((stream) => [stream.camera_uuid, stream]),
  ), [streams]);

  useEffect(() => {
    if (plan && plan.uuid !== selectedPlanUuid) onSelectPlan(plan.uuid);
  }, [plan?.uuid, selectedPlanUuid, onSelectPlan]);

  useEffect(() => {
    setEditing(false);
    setDraftCameras([]);
    setSelectedCameraUuid('');
    setPlacementCameraUuid('');
    setZoom(1);
    setCenter({ x: 0.5, y: 0.5 });
  }, [plan?.uuid]);

  const cameras = editing ? draftCameras : (plan?.cameras || []);
  const placedUuids = useMemo(() => new Set(
    cameras.map((camera) => camera.camera_uuid),
  ), [cameras]);
  const unplacedStreams = useMemo(() => (streams || [])
    .filter((stream) => !placedUuids.has(stream.camera_uuid))
    .sort((a, b) => a.name.localeCompare(b.name)),
  [streams, placedUuids]);
  const renderedMarkers = useMemo(() => editing
    ? cameras.map((camera) => ({ kind: 'camera', ...camera }))
    : clusterFloorPlanCameras(cameras, zoom),
  [cameras, zoom, editing]);
  const selectedStream = streamsByUuid.get(selectedCameraUuid);
  const selectedPlacement = cameras.find(
    (camera) => camera.camera_uuid === selectedCameraUuid);
  const indexedCameras = useMemo(() => {
    const term = cameraSearch.trim().toLocaleLowerCase();
    return cameras.map((camera) => ({
      placement: camera,
      stream: streamsByUuid.get(camera.camera_uuid),
    })).filter(({ stream }) => !term ||
      (stream?.name || '').toLocaleLowerCase().includes(term))
      .sort((a, b) => (a.stream?.name || '').localeCompare(b.stream?.name || ''));
  }, [cameras, streamsByUuid, cameraSearch]);

  if (plansQuery.isLoading) {
    return <div className="live-plan-state">Loading building plans…</div>;
  }
  if (plansQuery.error) {
    return <div className="live-plan-state is-error">{plansQuery.error.message}</div>;
  }

  const createPlan = async (name) => {
    setBusy(true);
    try {
      const created = await fetchJSON('/api/live/plans', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name, canvas_width: 1200, canvas_height: 800, cameras: [],
        }),
        timeout: 15000,
        retries: 0,
      });
      await queryClient.invalidateQueries({ queryKey: ['operator-floor-plans'] });
      onSelectPlan(created.uuid);
      showStatusMessage(`Created ${name}`, 'success', 2500);
      return true;
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
      return false;
    } finally {
      setBusy(false);
    }
  };

  const createPlanDialog = (
    <TextInputDialog
      isOpen={createPlanOpen}
      onClose={() => setCreatePlanOpen(false)}
      onSubmit={createPlan}
      title="Create building plan"
      description="Give this building or floor a clear name. You can place cameras after it is created."
      inputLabel="Plan name"
      placeholder="Main building · First floor"
      confirmLabel="Create plan"
      cancelLabel={t('common.cancel')}
    />
  );

  if (!plan) {
    return <>
      <section className="live-plan-empty">
        <div className="live-plan-empty-icon">⌖</div>
        <h3>Build a spatial camera view</h3>
        <p>A plan shows status markers for the whole site and opens video only when an operator requests it.</p>
        {plansQuery.data?.can_modify && (
          <button type="button" className="btn-primary" disabled={busy}
            onClick={() => setCreatePlanOpen(true)}>Create building plan</button>
        )}
      </section>
      {createPlanDialog}
    </>;
  }

  const width = plan.canvas_width || 1200;
  const height = plan.canvas_height || 800;
  const viewWidth = width / zoom;
  const viewHeight = height / zoom;
  const halfX = 0.5 / zoom;
  const halfY = 0.5 / zoom;
  const safeCenter = {
    x: clamp(center.x, halfX, 1 - halfX),
    y: clamp(center.y, halfY, 1 - halfY),
  };
  const viewBox = [
    safeCenter.x * width - viewWidth / 2,
    safeCenter.y * height - viewHeight / 2,
    viewWidth,
    viewHeight,
  ].join(' ');
  const markerScale = 1 / zoom;

  const pointFromEvent = (event) => {
    const svg = svgRef.current;
    const matrix = svg?.getScreenCTM();
    if (!svg || !matrix) return null;
    const point = svg.createSVGPoint();
    point.x = event.clientX;
    point.y = event.clientY;
    const transformed = point.matrixTransform(matrix.inverse());
    return {
      x: clamp(transformed.x / width, 0.025, 0.975),
      y: clamp(transformed.y / height, 0.025, 0.975),
    };
  };

  const placeCamera = (cameraUuid, point = { x: 0.5, y: 0.5 }) => {
    setDraftCameras((current) => {
      const existing = current.find((camera) => camera.camera_uuid === cameraUuid);
      if (existing) return current.map((camera) => camera.camera_uuid === cameraUuid
        ? { ...camera, ...point } : camera);
      return [...current, {
        camera_uuid: cameraUuid, ...point, rotation: 0, fov: 65,
      }];
    });
    setSelectedCameraUuid(cameraUuid);
    setPlacementCameraUuid(cameraUuid);
  };

  const updateSelectedPlacement = (changes) => {
    setDraftCameras((current) => current.map((camera) =>
      camera.camera_uuid === selectedCameraUuid ? { ...camera, ...changes } : camera));
  };

  const startEditing = () => {
    setDraftCameras((plan.cameras || []).map((camera) => ({ ...camera })));
    setEditing(true);
  };
  const cancelEditing = () => {
    setEditing(false);
    setDraftCameras([]);
    setPlacementCameraUuid('');
  };
  const savePlan = async () => {
    setBusy(true);
    try {
      const updated = await fetchJSON(`/api/live/plans/${plan.uuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(floorPlanPayload(plan, draftCameras)),
        timeout: 15000,
        retries: 0,
      });
      queryClient.setQueryData(['operator-floor-plans'], (current) => ({
        ...(current || {}),
        plans: (current?.plans || []).map((item) =>
          item.uuid === updated.uuid ? updated : item),
      }));
      setEditing(false);
      setPlacementCameraUuid('');
      showStatusMessage(`Saved ${plan.name}`, 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const pan = (dx, dy) => setCenter((current) => ({
    x: clamp(current.x + dx / zoom, halfX, 1 - halfX),
    y: clamp(current.y + dy / zoom, halfY, 1 - halfY),
  }));
  const setPlanZoom = (nextZoom, focus = center) => {
    setZoom(clamp(nextZoom, 1, 4));
    setCenter(focus);
  };

  return <>
    <section className={`live-building-plan ${editing ? 'is-editing' : ''}`}>
      <header className="live-plan-toolbar">
        <div>
          <span className="live-plan-eyebrow">Building plan</span>
          <h3>{plan.name}</h3>
          <small>{cameras.length} placed · {cameras.filter((camera) =>
            streamsByUuid.get(camera.camera_uuid)?.availability === 'live').length} live now</small>
        </div>
        <div className="live-plan-toolbar-actions">
          <div className="live-plan-zoom" aria-label="Plan zoom and pan">
            <button type="button" onClick={() => pan(-0.12, 0)} title="Pan left">←</button>
            <button type="button" onClick={() => pan(0, -0.12)} title="Pan up">↑</button>
            <button type="button" onClick={() => pan(0, 0.12)} title="Pan down">↓</button>
            <button type="button" onClick={() => pan(0.12, 0)} title="Pan right">→</button>
            <button type="button" onClick={() => setPlanZoom(zoom - 0.5)}
              disabled={zoom <= 1} title="Zoom out">−</button>
            <span>{Math.round(zoom * 100)}%</span>
            <button type="button" onClick={() => setPlanZoom(zoom + 0.5)}
              disabled={zoom >= 4} title="Zoom in">+</button>
            <button type="button" onClick={() => {
              setZoom(1); setCenter({ x: 0.5, y: 0.5 });
            }} title="Fit plan">Fit</button>
          </div>
          {editing ? (
            <>
              <button type="button" onClick={cancelEditing} disabled={busy}>Cancel</button>
              <button type="button" className="btn-primary" onClick={savePlan}
                disabled={busy}>Save plan</button>
            </>
          ) : plansQuery.data?.can_modify && (
            <>
              <button type="button" onClick={() => setCreatePlanOpen(true)} disabled={busy}>New plan</button>
              <button type="button" className="btn-primary" onClick={startEditing}>Edit layout</button>
            </>
          )}
        </div>
      </header>

      <div className="live-plan-workspace">
        <div className="live-plan-canvas-wrap">
          {editing && placementCameraUuid && (
            <div className="live-plan-placement-hint">Click the plan to position {streamsByUuid.get(placementCameraUuid)?.name}</div>
          )}
          <svg ref={svgRef} className="live-plan-canvas"
            viewBox={viewBox} role="img" aria-label={`${plan.name} camera plan`}
            onClick={(event) => {
              if (!editing || !placementCameraUuid) return;
              const point = pointFromEvent(event);
              if (point) placeCamera(placementCameraUuid, point);
            }}
            onDragOver={(event) => {
              if (editing && event.dataTransfer?.types?.includes('application/x-lightnvr-camera')) {
                event.preventDefault();
              }
            }}
            onDrop={(event) => {
              if (!editing) return;
              const cameraUuid = event.dataTransfer?.getData('application/x-lightnvr-camera');
              const point = pointFromEvent(event);
              if (cameraUuid && point) {
                event.preventDefault();
                placeCamera(cameraUuid, point);
              }
            }}>
            <defs>
              <pattern id={`plan-grid-${plan.uuid}`} width="40" height="40"
                patternUnits="userSpaceOnUse">
                <path d="M 40 0 L 0 0 0 40" className="live-plan-grid-line" />
              </pattern>
            </defs>
            <rect width={width} height={height} className="live-plan-ground" />
            <rect width={width} height={height}
              fill={`url(#plan-grid-${plan.uuid})`} />
            <rect x={width * 0.07} y={height * 0.08}
              width={width * 0.86} height={height * 0.84}
              rx="8" className="live-plan-building-shell" />
            <path d={`M ${width * 0.38} ${height * 0.08} V ${height * 0.38}
              M ${width * 0.38} ${height * 0.62} V ${height * 0.92}
              M ${width * 0.68} ${height * 0.08} V ${height * 0.38}
              M ${width * 0.68} ${height * 0.62} V ${height * 0.92}
              M ${width * 0.07} ${height * 0.38} H ${width * 0.93}
              M ${width * 0.07} ${height * 0.62} H ${width * 0.93}`}
              className="live-plan-interior-walls" />
            <rect x={width * 0.44} y={height * 0.35}
              width={width * 0.12} height={height * 0.3}
              className="live-plan-corridor" />

            {renderedMarkers.map((marker) => {
              const markerX = marker.x * width;
              const markerY = marker.y * height;
              if (marker.kind === 'cluster') {
                return (
                  <g key={marker.camera_uuids.join(':')}
                    className="live-plan-cluster" tabIndex="0" role="button"
                    aria-label={`${marker.count} cameras; zoom in`}
                    transform={`translate(${markerX} ${markerY}) scale(${markerScale})`}
                    onClick={(event) => {
                      event.stopPropagation();
                      setPlanZoom(zoom + 1, { x: marker.x, y: marker.y });
                    }}>
                    <circle r="27" /><text textAnchor="middle" dy="6">{marker.count}</text>
                  </g>
                );
              }
              const stream = streamsByUuid.get(marker.camera_uuid);
              const selected = marker.camera_uuid === selectedCameraUuid;
              return (
                <g key={marker.camera_uuid}
                  className={`live-plan-camera is-${stream?.availability || 'never_connected'} ${selected ? 'is-selected' : ''}`}
                  transform={`translate(${markerX} ${markerY}) scale(${markerScale})`}
                  tabIndex="0" role="button"
                  aria-label={`${stream?.name || 'Camera'}; ${availabilityLabel(stream, t)}`}
                  onClick={(event) => {
                    event.stopPropagation();
                    setSelectedCameraUuid(marker.camera_uuid);
                    if (editing) setPlacementCameraUuid(marker.camera_uuid);
                  }}>
                  <path d="M 3 -10 L 55 -28 L 55 28 Z"
                    transform={`rotate(${marker.rotation || 0})`}
                    className="live-plan-fov" />
                  <circle r="17" className="live-plan-camera-ring" />
                  <path d="M -8 -6 H 5 A 3 3 0 0 1 8 -3 V 6 H -8 Z M 8 -3 L 14 -8 V 8 L 8 3 Z"
                    className="live-plan-camera-icon" />
                  <text x="24" y="5" className="live-plan-camera-label">
                    {stream?.name || marker.camera_uuid.slice(0, 8)}
                  </text>
                </g>
              );
            })}
          </svg>
          <div className="live-plan-legend">
            <span><i className="is-live" />Live</span>
            <span><i className="is-offline" />Offline</span>
            <span><i className="is-never_connected" />Never connected</span>
            <span><i className="is-disabled" />Disabled</span>
          </div>
        </div>

        <aside className="live-plan-details" aria-live="polite">
          {selectedStream && selectedPlacement ? (
            <>
              <PlanCameraPreview stream={selectedStream} />
              <div className="live-plan-camera-summary">
                <span className={`live-availability-dot is-${selectedStream.availability}`} />
                <div><strong>{selectedStream.name}</strong>
                  <small>{availabilityLabel(selectedStream, t)}</small></div>
              </div>
              {!editing && selectedStream.availability === 'live' && (
                <button type="button" className="btn-primary"
                  onClick={() => onOpenCamera(selectedStream)}>Open live camera</button>
              )}
              {editing && (
                <div className="live-plan-placement-controls">
                  <span>Camera direction</span>
                  <div>
                    <button type="button" onClick={() => updateSelectedPlacement({
                      rotation: clamp((selectedPlacement.rotation || 0) - 15, -180, 180),
                    })}>↺ 15°</button>
                    <strong>{Math.round(selectedPlacement.rotation || 0)}°</strong>
                    <button type="button" onClick={() => updateSelectedPlacement({
                      rotation: clamp((selectedPlacement.rotation || 0) + 15, -180, 180),
                    })}>15° ↻</button>
                  </div>
                  <button type="button" className="live-plan-remove-camera"
                    onClick={() => {
                      setDraftCameras((current) => current.filter((camera) =>
                        camera.camera_uuid !== selectedCameraUuid));
                      setSelectedCameraUuid('');
                      setPlacementCameraUuid('');
                    }}>Remove from plan</button>
                </div>
              )}
            </>
          ) : (
            <div className="live-plan-details-empty">
              <strong>Select a camera</strong>
              <span>Inspect status and open one live view on demand.</span>
            </div>
          )}

          {!editing && cameras.length > 0 && (
            <div className="live-plan-camera-index">
              <div><strong>Plan cameras</strong><small>{indexedCameras.length}</small></div>
              <input type="search" value={cameraSearch} placeholder="Find on this plan…"
                aria-label="Find a camera on this plan"
                onInput={(event) => setCameraSearch(event.currentTarget.value)} />
              <div>
                {indexedCameras.map(({ placement, stream }) => (
                  <button key={placement.camera_uuid} type="button"
                    className={selectedCameraUuid === placement.camera_uuid ? 'is-selected' : ''}
                    onClick={() => {
                      setSelectedCameraUuid(placement.camera_uuid);
                      setCenter({ x: placement.x, y: placement.y });
                    }}>
                    <span className={`live-availability-dot is-${stream?.availability || 'never_connected'}`} />
                    <span>{stream?.name || placement.camera_uuid.slice(0, 8)}</span>
                  </button>
                ))}
              </div>
            </div>
          )}

          {editing && (
            <div className="live-plan-unplaced">
              <div><strong>Unplaced cameras</strong><small>{unplacedStreams.length}</small></div>
              <p>Select a camera, then click its position on the plan.</p>
              <div>
                {unplacedStreams.map((stream) => (
                  <button key={stream.camera_uuid} type="button" draggable
                    className={placementCameraUuid === stream.camera_uuid ? 'is-selected' : ''}
                    onDragStart={(event) => {
                      event.dataTransfer.setData('application/x-lightnvr-camera', stream.camera_uuid);
                      event.dataTransfer.effectAllowed = 'copy';
                    }}
                    onClick={() => placeCamera(stream.camera_uuid)}>
                    <span className={`live-availability-dot is-${stream.availability}`} />
                    <span>{stream.name}</span><b>＋</b>
                  </button>
                ))}
              </div>
            </div>
          )}
        </aside>
      </div>
    </section>
    {createPlanDialog}
  </>;
}
