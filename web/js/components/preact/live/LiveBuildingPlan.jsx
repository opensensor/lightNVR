import { useEffect, useId, useMemo, useRef, useState } from 'preact/hooks';
import { fetchJSON, useQuery, useQueryClient } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog, TextInputDialog } from '../common/ModalDialog.jsx';
import { useI18n } from '../../../i18n.js';
import {
  clusterFloorPlanCameras,
  coverageWedgePath,
  floorPlanCanvasFromImage,
  floorPlanPayload,
  normalizeCoverage,
} from './liveOperator.js';

const BACKGROUND_MAX_BYTES = 768 * 1024;
const BACKGROUND_TYPES = ['image/png', 'image/jpeg'];

function clamp(value, minimum, maximum) {
  return Math.max(minimum, Math.min(maximum, value));
}

function clampCenter(coordinate, zoom) {
  return {
    x: clamp(coordinate.x, 0.5 / zoom, 1 - 0.5 / zoom),
    y: clamp(coordinate.y, 0.5 / zoom, 1 - 0.5 / zoom),
  };
}

function parseCanvasDimension(value, minimum, maximum) {
  const number = Number(value);
  return Number.isInteger(number) && number >= minimum && number <= maximum
    ? number : null;
}

function planBackgroundUrl(plan, localVersion) {
  const version = `${plan.updated_at || plan.revision}-${localVersion}`;
  return `/api/live/plans/${plan.uuid}/background?v=${version}`;
}

function loadImageSize(file) {
  return new Promise((resolve, reject) => {
    const url = URL.createObjectURL(file);
    const image = new Image();
    image.onload = () => {
      URL.revokeObjectURL(url);
      resolve({ width: image.naturalWidth, height: image.naturalHeight });
    };
    image.onerror = () => {
      URL.revokeObjectURL(url);
      reject(new Error('Could not read the image dimensions'));
    };
    image.src = url;
  });
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

function coverageValueLabel(value) {
  return `${Number(value.toFixed(3))}`;
}

function PlanCoverageControl({ id, label, value, minimum, maximum, onChange }) {
  const normalized = clamp(value, minimum, maximum);
  const [draft, setDraft] = useState(coverageValueLabel(normalized));

  useEffect(() => {
    setDraft(coverageValueLabel(normalized));
  }, [id, normalized]);

  const updateDraft = (candidate) => {
    setDraft(candidate);
    if (candidate.trim() === '') return;
    const parsed = Number(candidate);
    if (Number.isFinite(parsed) && parsed >= minimum && parsed <= maximum) {
      onChange(parsed);
    }
  };
  const commitDraft = () => {
    const parsed = draft.trim() === '' ? NaN : Number(draft);
    const committed = Number.isFinite(parsed)
      ? clamp(parsed, minimum, maximum) : normalized;
    setDraft(coverageValueLabel(committed));
    if (committed !== normalized) onChange(committed);
  };
  const draftNumber = draft.trim() === '' ? NaN : Number(draft);
  const draftValid = Number.isFinite(draftNumber) &&
    draftNumber >= minimum && draftNumber <= maximum;

  return (
    <div className="live-plan-coverage-field">
      <label htmlFor={`${id}-range`}>{label}</label>
      <div className="live-plan-coverage-row">
        <input id={`${id}-range`} type="range"
          min={minimum} max={maximum} step="1"
          value={normalized}
          aria-valuetext={`${coverageValueLabel(normalized)} degrees`}
          onInput={(event) => onChange(Number(event.currentTarget.value))} />
        <input id={`${id}-number`} type="number"
          min={minimum} max={maximum} step="1"
          aria-label={`${label} in degrees`}
          aria-invalid={!draftValid}
          value={draft}
          onInput={(event) => updateDraft(event.currentTarget.value)}
          onBlur={commitDraft}
          onKeyDown={(event) => {
            if (event.key === 'Enter') event.currentTarget.blur();
            if (event.key === 'Escape') {
              event.preventDefault();
              setDraft(coverageValueLabel(normalized));
            }
          }} />
      </div>
    </div>
  );
}

export function LiveBuildingPlan({
  streams,
  selectedPlanUuid,
  onSelectPlan,
  onOpenCamera,
}) {
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const panelUid = useId();
  const svgRef = useRef(null);
  const fileInputRef = useRef(null);
  const interactionRef = useRef(null);
  const suppressClickRef = useRef(false);
  const zoomRef = useRef(1);
  const centerRef = useRef({ x: 0.5, y: 0.5 });
  const canvasSizeRef = useRef({ width: 1200, height: 800 });
  const [editing, setEditing] = useState(false);
  const [draftCameras, setDraftCameras] = useState([]);
  const [draftSize, setDraftSize] = useState(null);
  const [draftRevision, setDraftRevision] = useState(null);
  const [selectedCameraUuid, setSelectedCameraUuid] = useState('');
  const [placementCameraUuid, setPlacementCameraUuid] = useState('');
  const [zoom, setZoom] = useState(1);
  const [center, setCenter] = useState({ x: 0.5, y: 0.5 });
  const [backgroundVersion, setBackgroundVersion] = useState(0);
  const [panning, setPanning] = useState(false);
  const [busy, setBusy] = useState(false);
  const [cameraSearch, setCameraSearch] = useState('');
  const [createPlanOpen, setCreatePlanOpen] = useState(false);
  const [renameOpen, setRenameOpen] = useState(false);
  const [deleteOpen, setDeleteOpen] = useState(false);

  const plansQuery = useQuery({
    queryKey: ['operator-floor-plans'],
    queryFn: ({ signal }) => fetchJSON('/api/live/plans', { signal }),
    staleTime: 15000,
  });
  const plans = plansQuery.data?.plans || [];
  const plan = plans.find((item) => item.uuid === selectedPlanUuid) || plans[0] || null;
  const canModify = Boolean(plansQuery.data?.can_modify);
  const streamsByUuid = useMemo(() => new Map(
    (streams || []).map((stream) => [stream.camera_uuid, stream]),
  ), [streams]);

  useEffect(() => {
    if (plan && plan.uuid !== selectedPlanUuid) onSelectPlan(plan.uuid);
  }, [plan?.uuid, selectedPlanUuid, onSelectPlan]);

  useEffect(() => {
    setEditing(false);
    setDraftCameras([]);
    setDraftSize(null);
    setDraftRevision(null);
    setSelectedCameraUuid('');
    setPlacementCameraUuid('');
    interactionRef.current = null;
    setPanning(false);
    setZoom(1);
    setCenter({ x: 0.5, y: 0.5 });
    setBackgroundVersion(0);
  }, [plan?.uuid]);

  useEffect(() => { zoomRef.current = zoom; }, [zoom]);
  useEffect(() => { centerRef.current = center; }, [center]);

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
  const selectedCoverage = selectedPlacement ? normalizeCoverage(selectedPlacement) : null;
  const indexedCameras = useMemo(() => {
    const term = cameraSearch.trim().toLocaleLowerCase();
    return cameras.map((camera) => ({
      placement: camera,
      stream: streamsByUuid.get(camera.camera_uuid),
    })).filter(({ stream }) => !term ||
      (stream?.name || '').toLocaleLowerCase().includes(term))
      .sort((a, b) => (a.stream?.name || '').localeCompare(b.stream?.name || ''));
  }, [cameras, streamsByUuid, cameraSearch]);

  useEffect(() => {
    const svg = svgRef.current;
    if (!svg) return undefined;
    const onWheel = (event) => {
      const matrix = svg.getScreenCTM();
      const canvasSize = canvasSizeRef.current;
      if (!matrix || !canvasSize.width || !canvasSize.height) return;
      const currentZoom = zoomRef.current;
      const nextZoom = clamp(
        currentZoom * Math.exp(-event.deltaY * 0.0015), 1, 4);
      if (nextZoom === currentZoom) return;
      event.preventDefault();
      const focus = clampCenter(centerRef.current, currentZoom);
      const point = svg.createSVGPoint();
      point.x = event.clientX;
      point.y = event.clientY;
      const transformed = point.matrixTransform(matrix.inverse());
      const cursor = {
        x: transformed.x / canvasSize.width,
        y: transformed.y / canvasSize.height,
      };
      const nextCenter = clampCenter({
        x: cursor.x - (cursor.x - focus.x) * currentZoom / nextZoom,
        y: cursor.y - (cursor.y - focus.y) * currentZoom / nextZoom,
      }, nextZoom);
      zoomRef.current = nextZoom;
      centerRef.current = nextCenter;
      setZoom(nextZoom);
      setCenter(nextCenter);
    };
    svg.addEventListener('wheel', onWheel, { passive: false });
    return () => svg.removeEventListener('wheel', onWheel);
  }, [plan?.uuid]);

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

  const replacePlanInCache = (updated) => {
    queryClient.setQueryData(['operator-floor-plans'], (current) => ({
      ...(current || {}),
      plans: (current?.plans || []).map((item) =>
        item.uuid === updated.uuid ? updated : item),
    }));
  };

  const renamePlan = async (name) => {
    setBusy(true);
    try {
      const updated = await fetchJSON(`/api/live/plans/${plan.uuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(floorPlanPayload({ ...plan, name }, plan.cameras)),
        timeout: 15000,
        retries: 0,
      });
      replacePlanInCache(updated);
      showStatusMessage(`Renamed to ${updated.name}`, 'success', 2500);
      return true;
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
      return false;
    } finally {
      setBusy(false);
    }
  };

  const deletePlan = async () => {
    setBusy(true);
    try {
      await fetchJSON(`/api/live/plans/${plan.uuid}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ revision: plan.revision }),
        timeout: 15000,
        retries: 0,
      });
      setEditing(false);
      setDraftCameras([]);
      setDraftSize(null);
      setDraftRevision(null);
      setSelectedCameraUuid('');
      onSelectPlan('');
      await queryClient.invalidateQueries({ queryKey: ['operator-floor-plans'] });
      showStatusMessage(`Deleted ${plan.name}`, 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const uploadBackground = async (file) => {
    if (!file || busy) return;
    if (!BACKGROUND_TYPES.includes(file.type)) {
      showStatusMessage('Background must be a PNG or JPEG image', 'error', 5000);
      return;
    }
    if (file.size > BACKGROUND_MAX_BYTES) {
      showStatusMessage(
        'Background image too large; maximum is 768 KB', 'error', 5000);
      return;
    }
    setBusy(true);
    try {
      const imageSize = await loadImageSize(file);
      const canvasSize = floorPlanCanvasFromImage(imageSize.width, imageSize.height);
      const uploaded = await fetchJSON(`/api/live/plans/${plan.uuid}/background`, {
        method: 'PUT',
        headers: { 'Content-Type': file.type },
        body: file,
        timeout: 30000,
        retries: 0,
      });
      replacePlanInCache(uploaded);
      setBackgroundVersion((current) => current + 1);
      if (!canvasSize) {
        showStatusMessage(
          'Uploaded, but the image aspect ratio does not fit any valid canvas size',
          'warning', 5000);
        return;
      }
      if (editing) {
        setDraftSize(canvasSize);
        showStatusMessage(
          `Background updated for ${uploaded.name}; save the layout to apply its canvas size`,
          'success', 3500);
        return;
      }
      if (canvasSize.canvas_width !== uploaded.canvas_width ||
        canvasSize.canvas_height !== uploaded.canvas_height) {
        try {
          const resized = await fetchJSON(`/api/live/plans/${plan.uuid}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(floorPlanPayload(
              { ...uploaded, ...canvasSize }, uploaded.cameras)),
            timeout: 15000,
            retries: 0,
          });
          replacePlanInCache(resized);
        } catch (resizeError) {
          showStatusMessage(
            `Background uploaded, but the canvas was not resized: ${resizeError.message}`,
            'warning', 6000);
          return;
        }
      }
      showStatusMessage(`Background updated for ${uploaded.name}`, 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const removeBackground = async () => {
    setBusy(true);
    try {
      const updated = await fetchJSON(`/api/live/plans/${plan.uuid}/background`, {
        method: 'DELETE',
        timeout: 15000,
        retries: 0,
      });
      replacePlanInCache(updated);
      showStatusMessage('Background removed', 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
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
      description="Give this building or floor a clear name. Upload a floor plan image and place cameras after it is created."
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
        <h3>{t('live.plan.emptyTitle')}</h3>
        <p>{t('live.plan.emptyDescription')}</p>
        {canModify && (
          <button type="button" className="btn-primary" disabled={busy}
            onClick={() => setCreatePlanOpen(true)}>Create building plan</button>
        )}
      </section>
      {createPlanDialog}
    </>;
  }

  const persistedWidth = plan.canvas_width || 1200;
  const persistedHeight = plan.canvas_height || 800;
  const draftWidthValue = draftSize?.canvas_width ?? persistedWidth;
  const draftHeightValue = draftSize?.canvas_height ?? persistedHeight;
  const draftWidth = parseCanvasDimension(draftWidthValue, 400, 4000);
  const draftHeight = parseCanvasDimension(draftHeightValue, 300, 4000);
  const canvasSizeValid = draftWidth !== null && draftHeight !== null;
  const width = editing ? (draftWidth ?? persistedWidth) : persistedWidth;
  const height = editing ? (draftHeight ?? persistedHeight) : persistedHeight;
  canvasSizeRef.current = { width, height };
  const viewWidth = width / zoom;
  const viewHeight = height / zoom;
  const safeCenter = clampCenter(center, zoom);
  const viewBox = [
    safeCenter.x * width - viewWidth / 2,
    safeCenter.y * height - viewHeight / 2,
    viewWidth,
    viewHeight,
  ].join(' ');
  const markerScale = 1 / zoom;
  const sizeForSave = {
    canvas_width: draftWidth ?? persistedWidth,
    canvas_height: draftHeight ?? persistedHeight,
  };

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
    setDraftSize(null);
    setDraftRevision(plan.revision);
    setEditing(true);
  };
  const cancelEditing = () => {
    setEditing(false);
    setDraftCameras([]);
    setDraftSize(null);
    setDraftRevision(null);
    setPlacementCameraUuid('');
  };
  const savePlan = async () => {
    setBusy(true);
    try {
      const updated = await fetchJSON(`/api/live/plans/${plan.uuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ...floorPlanPayload({
            ...plan,
            ...sizeForSave,
            revision: draftRevision ?? plan.revision,
          }, draftCameras),
        }),
        timeout: 15000,
        retries: 0,
      });
      replacePlanInCache(updated);
      setEditing(false);
      setDraftSize(null);
      setDraftRevision(null);
      setPlacementCameraUuid('');
      showStatusMessage(`Saved ${updated.name}`, 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const pan = (dx, dy) => setCenter((current) => ({
    x: clamp(current.x + dx / zoom, 0.5 / zoom, 1 - 0.5 / zoom),
    y: clamp(current.y + dy / zoom, 0.5 / zoom, 1 - 0.5 / zoom),
  }));
  const setPlanZoom = (nextZoom, focus = center) => {
    const clampedZoom = clamp(nextZoom, 1, 4);
    const clampedCenter = clampCenter(focus, clampedZoom);
    zoomRef.current = clampedZoom;
    centerRef.current = clampedCenter;
    setZoom(clampedZoom);
    setCenter(clampedCenter);
  };

  const handleSurfacePointerDown = (event) => {
    suppressClickRef.current = false;
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (editing && placementCameraUuid) return;
    const matrix = svgRef.current?.getScreenCTM();
    if (!matrix) return;
    const screenPlanWidth = Math.hypot(matrix.a, matrix.b) * width;
    const screenPlanHeight = Math.hypot(matrix.c, matrix.d) * height;
    if (!screenPlanWidth || !screenPlanHeight) return;
    interactionRef.current = {
      mode: 'pan',
      pointerId: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      origin: clampCenter(centerRef.current, zoomRef.current),
      zoom: zoomRef.current,
      screenPlanWidth,
      screenPlanHeight,
      moved: false,
    };
  };

  const handleCameraPointerDown = (event, cameraUuid) => {
    if (!editing) return;
    event.stopPropagation();
    suppressClickRef.current = false;
    interactionRef.current = {
      mode: 'camera',
      pointerId: event.pointerId,
      cameraUuid,
      startX: event.clientX,
      startY: event.clientY,
      moved: false,
    };
    setSelectedCameraUuid(cameraUuid);
    setPlacementCameraUuid(cameraUuid);
  };

  /**
   * Capture the pointer only once a real drag starts; capturing on plain
   * pointerdown would retarget click events away from camera markers.
   */
  const beginDrag = (interaction, event) => {
    interaction.moved = true;
    suppressClickRef.current = true;
    try {
      svgRef.current?.setPointerCapture(event.pointerId);
    } catch {
      // Pointer may already be gone on some browsers.
    }
    if (interaction.mode === 'pan') setPanning(true);
  };

  const handleSurfacePointerMove = (event) => {
    const interaction = interactionRef.current;
    if (!interaction || event.pointerId !== interaction.pointerId) return;
    const dx = event.clientX - interaction.startX;
    const dy = event.clientY - interaction.startY;
    if (!interaction.moved && Math.hypot(dx, dy) < 4) return;
    if (!interaction.moved) beginDrag(interaction, event);
    if (interaction.mode === 'camera') {
      const point = pointFromEvent(event);
      if (point) {
        setDraftCameras((current) => current.map((camera) =>
          camera.camera_uuid === interaction.cameraUuid
            ? { ...camera, ...point } : camera));
      }
      return;
    }
    setCenter(clampCenter({
      x: interaction.origin.x - dx / interaction.screenPlanWidth,
      y: interaction.origin.y - dy / interaction.screenPlanHeight,
    }, interaction.zoom));
  };

  const handleSurfacePointerEnd = (event) => {
    if (!interactionRef.current ||
      event.pointerId !== interactionRef.current.pointerId) return;
    interactionRef.current = null;
    setPanning(false);
    try {
      svgRef.current?.releasePointerCapture(event.pointerId);
    } catch {
      // Capture may already be released when the pointer left the window.
    }
  };

  const openBackgroundPicker = () => fileInputRef.current?.click();

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
              setPlanZoom(1, { x: 0.5, y: 0.5 });
            }} title="Fit plan">Fit</button>
          </div>
          {editing ? (
            <>
              {canModify && (
                <button type="button" onClick={openBackgroundPicker}
                  disabled={busy}>Background…</button>
              )}
              <button type="button" onClick={cancelEditing} disabled={busy}>Cancel</button>
              <button type="button" className="btn-primary" onClick={savePlan}
                disabled={busy || !canvasSizeValid}>Save plan</button>
            </>
          ) : canModify && (
            <>
              <button type="button" onClick={openBackgroundPicker}
                disabled={busy}
                title="Upload a PNG or JPEG image as the plan background">
                {plan.background_mime ? 'Replace background' : 'Upload background'}
              </button>
              {plan.background_mime && (
                <button type="button" onClick={removeBackground} disabled={busy}>
                  Remove background
                </button>
              )}
              <button type="button" onClick={() => setRenameOpen(true)} disabled={busy}>
                Rename
              </button>
              <button type="button" onClick={() => setDeleteOpen(true)} disabled={busy}>
                Delete
              </button>
              <button type="button" onClick={() => setCreatePlanOpen(true)} disabled={busy}>New plan</button>
              <button type="button" className="btn-primary" onClick={startEditing}
                disabled={busy}>Edit layout</button>
            </>
          )}
        </div>
      </header>

      <input ref={fileInputRef} type="file" accept="image/png,image/jpeg"
        className="live-plan-file-input" aria-hidden="true" tabIndex="-1"
        onChange={(event) => {
          const file = event.currentTarget.files?.[0];
          event.currentTarget.value = '';
          if (file) uploadBackground(file);
        }} />

      <div className="live-plan-workspace">
        <div className="live-plan-canvas-wrap">
          {editing && placementCameraUuid && (
            <div className="live-plan-placement-hint">Click the plan to position {streamsByUuid.get(placementCameraUuid)?.name}</div>
          )}
          <svg ref={svgRef} className={`live-plan-canvas ${panning ? 'is-panning' : ''}`}
            viewBox={viewBox} role="img" aria-label={`${plan.name} camera plan`}
            onPointerDown={handleSurfacePointerDown}
            onPointerMove={handleSurfacePointerMove}
            onPointerUp={handleSurfacePointerEnd}
            onPointerCancel={handleSurfacePointerEnd}
            onClick={(event) => {
              if (suppressClickRef.current) return;
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
            {plan.background_mime ? (
              <image href={planBackgroundUrl(plan, backgroundVersion)} x="0" y="0"
                width={width} height={height}
                preserveAspectRatio="none" className="live-plan-background" />
            ) : (
              <>
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
              </>
            )}

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
                      if (suppressClickRef.current) return;
                      setPlanZoom(zoom + 1, { x: marker.x, y: marker.y });
                    }}>
                    <circle r="27" /><text textAnchor="middle" dy="6">{marker.count}</text>
                  </g>
                );
              }
              const stream = streamsByUuid.get(marker.camera_uuid);
              const selected = marker.camera_uuid === selectedCameraUuid;
              const coverage = normalizeCoverage(marker);
              return (
                <g key={marker.camera_uuid}
                  className={`live-plan-camera is-${stream?.availability || 'never_connected'} ${selected ? 'is-selected' : ''}`}
                  transform={`translate(${markerX} ${markerY}) scale(${markerScale})`}
                  tabIndex="0" role="button"
                  aria-label={`${stream?.name || 'Camera'}; ${availabilityLabel(stream, t)}; facing ${coverageValueLabel(coverage.rotation)}° with a ${coverageValueLabel(coverage.fov)}° view`}
                  onPointerDown={(event) => handleCameraPointerDown(event, marker.camera_uuid)}
                  onClick={(event) => {
                    event.stopPropagation();
                    if (suppressClickRef.current) return;
                    setSelectedCameraUuid(marker.camera_uuid);
                    if (editing) setPlacementCameraUuid(marker.camera_uuid);
                  }}
                  onDblClick={(event) => {
                    event.stopPropagation();
                    if (!editing && stream) onOpenCamera(stream);
                  }}>
                  <path d={coverageWedgePath(coverage.fov)}
                    transform={`rotate(${coverage.rotation})`}
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
          {!plan.background_mime && !editing && (
            <div className="live-plan-background-hint">
              <strong>No floor plan image yet</strong>
              <p>The rooms shown here are only a placeholder. {canModify
                ? 'Upload a scanned plan, site drawing, or PNG/JPEG export (up to 768 KB) to map the real building.'
                : 'An administrator can upload a plan of the real building.'}</p>
              {canModify && (
                <button type="button" className="btn-primary" disabled={busy}
                  onClick={openBackgroundPicker}>Upload floor plan image</button>
              )}
            </div>
          )}
          <div className="live-plan-legend">
            <span><i className="is-live" />Live</span>
            <span><i className="is-offline" />Offline</span>
            <span><i className="is-never_connected" />Never connected</span>
            <span><i className="is-disabled" />Disabled</span>
          </div>
        </div>

        <aside className="live-plan-details" aria-live="polite">
          {editing && (
            <div className="live-plan-settings">
              <div><strong>Plan canvas</strong></div>
              <label>Width
                <input type="number" min="400" max="4000" step="10"
                  value={draftWidthValue} aria-invalid={draftWidth === null}
                  onInput={(event) => setDraftSize({
                    canvas_width: event.currentTarget.value,
                    canvas_height: draftHeightValue,
                  })} />
              </label>
              <label>Height
                <input type="number" min="300" max="4000" step="10"
                  value={draftHeightValue} aria-invalid={draftHeight === null}
                  onInput={(event) => setDraftSize({
                    canvas_width: draftWidthValue,
                    canvas_height: event.currentTarget.value,
                  })} />
              </label>
              <small className={canvasSizeValid ? '' : 'is-error'}>
                {canvasSizeValid
                  ? 'Uploading a background image resizes the canvas to match it.'
                  : 'Width must be 400–4000 and height must be 300–4000.'}
              </small>
            </div>
          )}
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
              {editing && selectedCoverage && (
                <div className="live-plan-placement-controls">
                  <PlanCoverageControl
                    key={`rotation-${selectedCameraUuid}`}
                    id={`${panelUid}-rotation`}
                    label="Camera direction"
                    value={selectedCoverage.rotation}
                    minimum={-180}
                    maximum={180}
                    onChange={(rotation) => updateSelectedPlacement({ rotation })} />
                  <PlanCoverageControl
                    key={`fov-${selectedCameraUuid}`}
                    id={`${panelUid}-fov`}
                    label="Field of view"
                    value={selectedCoverage.fov}
                    minimum={1}
                    maximum={180}
                    onChange={(fov) => updateSelectedPlacement({ fov })} />
                  <small>Wedges show direction and angle only; distance on the plan is not calibrated.</small>
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
              <span>{editing
                ? 'Drag placed markers to fine-tune. Inspect status and open one live view on demand.'
                : 'Inspect status and open one live view on demand.'}</span>
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
    <TextInputDialog
      isOpen={renameOpen}
      onClose={() => setRenameOpen(false)}
      onSubmit={renamePlan}
      title="Rename plan"
      description="Plans are shared with every operator on this site."
      inputLabel="Plan name"
      initialValue={plan.name}
      maxLength={127}
      confirmLabel="Rename plan"
      cancelLabel={t('common.cancel')}
    />
    <ConfirmDialog
      isOpen={deleteOpen}
      onClose={() => setDeleteOpen(false)}
      onConfirm={deletePlan}
      title="Delete plan"
      message={`Delete “${plan.name}” and its uploaded background? Camera placements on this plan are removed; the cameras themselves are untouched.`}
      confirmLabel="Delete plan"
      cancelLabel={t('common.cancel')}
      variant="danger"
    />
  </>;
}
