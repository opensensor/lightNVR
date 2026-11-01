import { useEffect, useId, useMemo, useRef, useState } from 'preact/hooks';
import { fetchJSON, useQuery, useQueryClient } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog, TextInputDialog } from '../common/ModalDialog.jsx';
import { t as translate, useI18n } from '../../../i18n.js';
import {
  buildLocationTree,
  clusterFloorPlanCameras,
  flattenLocationTree,
  coverageWedgePath,
  floorPlanCanvasFromImage,
  floorPlanPayload,
  normalizeCoverage,
} from './liveOperator.js';
import {
  SKETCH_LIMITS,
  SKETCH_TONES,
  aimRotation,
  fovFromHeading,
  reorderShapeGroup,
  shapesBounds,
  shapesInMarquee,
  translateShapeGroup,
  appendPolylinePoint,
  canRemoveVertex,
  closesPolygon,
  constrainAngle,
  createHistory,
  createShapeId,
  duplicateShape,
  finishPolyline,
  gridStep,
  insertShapeVertex,
  moveShapeVertex,
  normalizeShape,
  pushHistory,
  rectFromPoints,
  redoHistory,
  removeShapeVertex,
  reorderShape,
  resizeRectCorner,
  resizeRectEdge,
  shapeBounds,
  sketchPayload,
  sketchShapesFromPlan,
  snapPoint,
  translateShape,
  undoHistory,
  wrapRotation,
} from './planSketch.js';
import { PlanSketchLayer } from './PlanSketchLayer.jsx';
import { PlanSketchInspector, PlanSketchTools } from './PlanSketchControls.jsx';
import { PlanContextMenu } from './PlanContextMenu.jsx';
import { applyBulkOrganization } from '../fleet/fleetOrganization.js';

const TOOL_SHORTCUTS = { v: 'select', r: 'room', w: 'wall', a: 'area', l: 'label' };
/** Draft edited as one undo history so Ctrl+Z covers shapes and cameras. */
const EMPTY_DRAFT = Object.freeze({ shapes: [], cameras: [] });
const CAMERA_ROTATE_STEP = 15;

function isFormField(target) {
  const tag = target?.tagName;
  return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' ||
    Boolean(target?.isContentEditable);
}

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
      reject(new Error(translate('live.plan.imageDimensionsError')));
    };
    image.src = url;
  });
}

function PlanCameraPreview({ stream }) {
  const { t } = useI18n();
  const [failed, setFailed] = useState(false);
  const url = useMemo(() => stream?.availability === 'live'
    ? `/go2rtc/api/frame.jpeg?src=${encodeURIComponent(stream.name)}&cache=5s&t=${Date.now()}`
    : null, [stream?.availability, stream?.name]);
  useEffect(() => setFailed(false), [stream?.camera_uuid]);
  if (!url || failed) {
    return <div className="live-plan-preview-placeholder">{t('live.plan.previewUnavailable')}</div>;
  }
  return <img src={url} alt={t('live.plan.previewAlt', { name: stream.name })}
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
  const { t } = useI18n();
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
          aria-valuetext={t('live.plan.degreesValue', { value: coverageValueLabel(normalized) })}
          onInput={(event) => onChange(Number(event.currentTarget.value))} />
        <input id={`${id}-number`} type="number"
          min={minimum} max={maximum} step="1"
          aria-label={t('live.plan.inDegrees', { label })}
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
  const sectionRef = useRef(null);
  const clipboardRef = useRef(null);
  const spaceHeldRef = useRef(false);
  const editStartDraftRef = useRef(EMPTY_DRAFT);
  const [editing, setEditing] = useState(false);
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
  const [sketchTool, setSketchTool] = useState('select');
  const [sketchTone, setSketchTone] = useState(SKETCH_TONES[0]);
  const [draftHistory, setDraftHistory] = useState(() => createHistory(EMPTY_DRAFT));
  const [selectedShapeIds, setSelectedShapeIds] = useState([]);
  const setSelectedShapeId = (shapeId) => setSelectedShapeIds(shapeId ? [shapeId] : []);
  const [marquee, setMarquee] = useState(null);
  const [draftLocationUuid, setDraftLocationUuid] = useState('');
  const [drawing, setDrawing] = useState(null);
  const [snap, setSnap] = useState(true);
  const [toolLock, setToolLock] = useState(false);
  const [spaceHeld, setSpaceHeld] = useState(false);
  const [contextMenu, setContextMenu] = useState(null);
  const [discardOpen, setDiscardOpen] = useState(false);
  const drawingRef = useRef(null);
  // Pointer handlers read the in-progress drawing synchronously; an effect
  // only syncs after paint, so a fast drag or click used to see stale state.
  const setDrawingNow = (next) => {
    const value = typeof next === 'function' ? next(drawingRef.current) : next;
    drawingRef.current = value;
    setDrawing(value);
  };
  const draftHistoryRef = useRef(draftHistory);
  draftHistoryRef.current = draftHistory;
  const keyHandlerRef = useRef(null);

  const plansQuery = useQuery({
    queryKey: ['operator-floor-plans'],
    queryFn: ({ signal }) => fetchJSON('/api/live/plans', { signal }),
    staleTime: 15000,
  });
  // Shared with the Navigator (same key), used to link a plan to a location.
  const locationsQuery = useQuery({
    queryKey: ['fleet-locations'],
    queryFn: ({ signal }) => fetchJSON('/api/locations', { signal }),
    staleTime: 60000,
  });
  const locationOptions = useMemo(() => {
    const locations = locationsQuery.data?.locations || [];
    const tree = buildLocationTree(locations, []);
    return flattenLocationTree(tree, new Set(locations.map((location) => location.uuid)));
  }, [locationsQuery.data]);
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
    setDraftSize(null);
    setDraftRevision(null);
    setSelectedCameraUuid('');
    setPlacementCameraUuid('');
    interactionRef.current = null;
    setPanning(false);
    setZoom(1);
    setCenter({ x: 0.5, y: 0.5 });
    setBackgroundVersion(0);
    setSketchTool('select');
    setDraftHistory(createHistory(EMPTY_DRAFT));
    setSelectedShapeId('');
    setDrawingNow(null);
    setContextMenu(null);
  }, [plan?.uuid]);

  useEffect(() => { zoomRef.current = zoom; }, [zoom]);
  useEffect(() => { centerRef.current = center; }, [center]);

  const draftCameras = draftHistory.present.cameras;
  const draftSketch = draftHistory.present.shapes;
  const cameras = editing ? draftCameras : (plan?.cameras || []);
  const viewSketch = useMemo(() => sketchShapesFromPlan(plan), [plan]);
  const sketchShapes = editing ? draftSketch : viewSketch;
  const selectedIdSet = useMemo(() => new Set(selectedShapeIds), [selectedShapeIds]);
  // Selection in draw order; a single selection drives handles and inspector.
  const selectedShapes = editing
    ? draftSketch.filter((shape) => selectedIdSet.has(shape.id)) : [];
  const selectedShape = selectedShapes.length === 1 ? selectedShapes[0] : null;
  const hasShapeSelection = selectedShapes.length > 0;
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

  // Shortcuts listen on the document while editing: focus regularly lands on
  // <body> (a deleted shape, a button that just became disabled), and a
  // section-scoped onKeyDown silently dropped Ctrl+Z in those cases.
  useEffect(() => {
    if (!editing) return undefined;
    const onKeyDown = (event) => keyHandlerRef.current?.(event);
    const releaseSpace = () => {
      if (!spaceHeldRef.current) return;
      spaceHeldRef.current = false;
      setSpaceHeld(false);
    };
    const onKeyUp = (event) => {
      if (event.key === ' ') releaseSpace();
    };
    document.addEventListener('keydown', onKeyDown);
    document.addEventListener('keyup', onKeyUp);
    window.addEventListener('blur', releaseSpace);
    return () => {
      document.removeEventListener('keydown', onKeyDown);
      document.removeEventListener('keyup', onKeyUp);
      window.removeEventListener('blur', releaseSpace);
      releaseSpace();
    };
  }, [editing]);

  if (plansQuery.isLoading) {
    return <div className="live-plan-state">{t('live.plan.loading')}</div>;
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
      showStatusMessage(t('live.plan.created', { name }), 'success', 2500);
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
      showStatusMessage(t('live.plan.renamed', { name: updated.name }), 'success', 2500);
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
      setDraftHistory(createHistory(EMPTY_DRAFT));
      setDraftSize(null);
      setDraftRevision(null);
      setSelectedCameraUuid('');
      onSelectPlan('');
      await queryClient.invalidateQueries({ queryKey: ['operator-floor-plans'] });
      showStatusMessage(t('live.plan.deleted', { name: plan.name }), 'success', 2500);
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const uploadBackground = async (file) => {
    if (!file || busy) return;
    if (!BACKGROUND_TYPES.includes(file.type)) {
      showStatusMessage(t('live.plan.backgroundType'), 'error', 5000);
      return;
    }
    if (file.size > BACKGROUND_MAX_BYTES) {
      showStatusMessage(
        t('live.plan.backgroundTooLarge'), 'error', 5000);
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
          t('live.plan.backgroundAspect'),
          'warning', 5000);
        return;
      }
      if (editing) {
        setDraftSize(canvasSize);
        showStatusMessage(
          t('live.plan.backgroundUpdatedSave', { name: uploaded.name }),
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
            t('live.plan.backgroundNotResized', { error: resizeError.message }),
            'warning', 6000);
          return;
        }
      }
      showStatusMessage(t('live.plan.backgroundUpdated', { name: uploaded.name }), 'success', 2500);
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
      showStatusMessage(t('live.plan.backgroundRemoved'), 'success', 2500);
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
      title={t('live.plan.createTitle')}
      description={t('live.plan.createDescription')}
      inputLabel={t('live.plan.nameLabel')}
      placeholder={t('live.plan.namePlaceholder')}
      confirmLabel={t('live.plan.createConfirm')}
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
            onClick={() => setCreatePlanOpen(true)}>{t('live.plan.createTitle')}</button>
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

  // Sketch geometry uses the full plan so walls can run along its edges;
  // grid snapping is the default and Alt bypasses it for one gesture.
  const sketchPointFromEvent = (event) => {
    const svg = svgRef.current;
    const matrix = svg?.getScreenCTM();
    if (!svg || !matrix) return null;
    const point = svg.createSVGPoint();
    point.x = event.clientX;
    point.y = event.clientY;
    const transformed = point.matrixTransform(matrix.inverse());
    return snapPoint({
      x: transformed.x / width,
      y: transformed.y / height,
    }, { width, height }, snap && !event.altKey);
  };

  const canvasDims = { width, height };
  const commitDraft = (update, coalesceKey = null) => setDraftHistory(
    (history) => pushHistory(history, update(history.present), coalesceKey));
  // Live drag frames update the present without recording a step; the
  // gesture is recorded once on release by finishGesture.
  const previewDraft = (update) => setDraftHistory(
    (history) => ({ ...history, present: update(history.present) }));
  const finishGesture = (before) => setDraftHistory(
    (history) => pushHistory({ ...history, present: before }, history.present));
  const commitSketch = (next, coalesceKey = null) =>
    commitDraft((draft) => ({ ...draft, shapes: next }), coalesceKey);
  const updateCameras = (update, coalesceKey = null) =>
    commitDraft((draft) => ({ ...draft, cameras: update(draft.cameras) }), coalesceKey);
  const updateShapeById = (shapeId, update, coalesceKey = null) =>
    commitDraft((draft) => ({
      ...draft,
      shapes: draft.shapes.map((shape) => shape.id === shapeId
        ? (normalizeShape(update(shape)) || shape) : shape),
    }), coalesceKey);
  const shapeIds = () => draftSketch.map((shape) => shape.id);
  const selectShape = (shapeId) => {
    setSelectedShapeId(shapeId);
    if (shapeId) {
      setSelectedCameraUuid('');
      setPlacementCameraUuid('');
    }
  };
  const selectShapes = (ids) => {
    setSelectedShapeIds(ids);
    if (ids.length > 0) {
      setSelectedCameraUuid('');
      setPlacementCameraUuid('');
    }
  };
  const toggleShapeSelection = (shapeId) => {
    const next = selectedIdSet.has(shapeId)
      ? selectedShapeIds.filter((id) => id !== shapeId)
      : [...selectedShapeIds, shapeId];
    selectShapes(next);
    return next.includes(shapeId);
  };
  // New shapes come back in Select with handles showing so mistakes can be
  // fixed straight away; the tool lock (Q) keeps drawing the same kind.
  const addShape = (shape, nextTool = toolLock ? sketchTool : 'select') => {
    if (!shape) return;
    commitSketch([...draftSketch, shape]);
    selectShape(shape.id);
    setSketchTool(nextTool);
  };
  const addShapes = (shapes) => {
    const room = SKETCH_LIMITS.maxShapes - draftSketch.length;
    const added = shapes.slice(0, Math.max(0, room));
    if (added.length === 0) return;
    commitSketch([...draftSketch, ...added]);
    selectShapes(added.map((shape) => shape.id));
    setSketchTool('select');
  };
  const replaceShape = (next) => {
    commitSketch(draftSketch.map((shape) => shape.id === next.id ? next : shape),
      `shape:${next.id}`);
    if (next.tone) setSketchTone(next.tone);
  };
  const deleteShapesById = (ids) => {
    const doomed = new Set(ids);
    commitDraft((draft) => ({
      ...draft, shapes: draft.shapes.filter((shape) => !doomed.has(shape.id)),
    }));
    setSelectedShapeIds((current) => current.filter((id) => !doomed.has(id)));
  };
  const deleteShapeById = (shapeId) => deleteShapesById([shapeId]);
  const deleteSelectedShape = () => {
    if (hasShapeSelection) deleteShapesById(selectedShapeIds);
  };
  /** Copies of `shapes` with fresh ids, offset one grid step. */
  const cloneShapes = (shapes) => {
    const taken = shapeIds();
    return shapes.map((shape) => {
      const copy = duplicateShape(shape, taken, canvasDims);
      taken.push(copy.id);
      return copy;
    });
  };
  const duplicateSelectedShape = () => {
    if (hasShapeSelection) addShapes(cloneShapes(selectedShapes));
  };
  const copyShapes = (shapes) => {
    if (shapes?.length) clipboardRef.current = shapes;
  };
  const copyShape = (shape) => copyShapes(shape ? [shape] : []);
  const pasteShape = (at = null) => {
    const source = clipboardRef.current;
    if (!source?.length || draftSketch.length >= SKETCH_LIMITS.maxShapes) return;
    const taken = shapeIds();
    let copies = source.map((shape) => {
      const id = createShapeId(taken);
      taken.push(id);
      return { ...shape, id };
    });
    const bounds = shapesBounds(copies);
    const step = gridStep(canvasDims);
    const dx = at ? at.x - (bounds.x0 + bounds.x1) / 2 : step.x;
    const dy = at ? at.y - (bounds.y0 + bounds.y1) / 2 : step.y;
    copies = translateShapeGroup(copies, copies.map((shape) => shape.id), dx, dy);
    // Repeated pastes cascade instead of stacking on one spot.
    clipboardRef.current = copies;
    addShapes(copies);
  };
  const reorderShapeById = (shapeId, direction) => commitDraft((draft) => ({
    ...draft, shapes: reorderShape(draft.shapes, shapeId, direction),
  }));
  const reorderShapesById = (ids, direction) => commitDraft((draft) => ({
    ...draft, shapes: reorderShapeGroup(draft.shapes, ids, direction),
  }));
  const recolorShapes = (ids, tone) => {
    const targets = new Set(ids);
    commitDraft((draft) => ({
      ...draft,
      shapes: draft.shapes.map((shape) => targets.has(shape.id) ? { ...shape, tone } : shape),
    }));
    setSketchTone(tone);
  };
  const nudgeSelectedShape = (stepsX, stepsY) => {
    if (!hasShapeSelection) return;
    const step = gridStep(canvasDims);
    commitSketch(translateShapeGroup(draftSketch, selectedShapeIds,
      stepsX * step.x, stepsY * step.y), `nudge:${selectedShapeIds.join(',')}`);
  };
  const focusShapeField = (shape) => {
    if (!shape) return;
    const fieldId = `${panelUid}-sketch-${shape.type === 'label' ? 'text' : 'label'}`;
    requestAnimationFrame(() => {
      const field = document.getElementById(fieldId);
      if (field) {
        field.focus();
        field.select?.();
      }
    });
  };
  const selectTool = (tool) => {
    setSketchTool(tool);
    setDrawingNow(null);
    if (tool !== 'select') {
      setPlacementCameraUuid('');
      setSelectedShapeId('');
    }
  };
  const removeLastDrawingPoint = () => {
    const current = drawingRef.current;
    if (!current || current.tool === 'room') return false;
    if ((current.points || []).length <= 1) {
      setDrawingNow(null);
      return true;
    }
    setDrawingNow({ ...current, points: current.points.slice(0, -1) });
    return true;
  };
  // While a wall or area is in progress, undo steps back one point instead of
  // throwing away the previous shape.
  const undoSketch = () => {
    if (removeLastDrawingPoint()) return;
    setDraftHistory((history) => undoHistory(history));
  };
  const redoSketch = () => {
    if (drawingRef.current) return;
    setDraftHistory((history) => redoHistory(history));
  };
  const finishDrawing = () => {
    const current = drawingRef.current;
    if (!current || current.tool === 'room') return;
    setDrawingNow(null);
    addShape(finishPolyline(current.tool, current.points, createShapeId(shapeIds()), sketchTone));
  };
  const cancelDrawing = () => {
    if (!drawingRef.current) return;
    setDrawingNow(null);
    showStatusMessage(t('live.plan.sketch.discardDrawing'), 'info', 1500);
  };
  const drawingClosable = Boolean(drawing && drawing.tool === 'area' && drawing.cursor &&
    closesPolygon(drawing.points, drawing.cursor, canvasDims));
  const drawingFinishable = Boolean(drawing && drawing.tool !== 'room' &&
    (drawing.points || []).length >= (drawing.tool === 'area' ? 3 : 2));

  /**
   * Add or move a camera. `awaitPosition` keeps it armed so the next press on
   * the plan positions it, and dragging from that press aims it.
   */
  const placeCamera = (cameraUuid, point = { x: 0.5, y: 0.5 }, awaitPosition = false) => {
    updateCameras((current) => {
      const existing = current.find((camera) => camera.camera_uuid === cameraUuid);
      if (existing) return current.map((camera) => camera.camera_uuid === cameraUuid
        ? { ...camera, ...point } : camera);
      return [...current, {
        camera_uuid: cameraUuid, ...point, rotation: 0, fov: 65,
      }];
    });
    setSelectedCameraUuid(cameraUuid);
    setPlacementCameraUuid(awaitPosition ? cameraUuid : '');
    setSelectedShapeId('');
    setSketchTool('select');
    setDrawingNow(null);
  };

  const updateCameraById = (cameraUuid, changes, coalesceKey = `camera:${cameraUuid}`) =>
    updateCameras((current) => current.map((camera) =>
      camera.camera_uuid === cameraUuid ? { ...camera, ...changes } : camera), coalesceKey);
  const updateSelectedPlacement = (changes) => updateCameraById(selectedCameraUuid, changes);
  const rotateCamera = (cameraUuid, delta) => {
    const placement = draftCameras.find((camera) => camera.camera_uuid === cameraUuid);
    if (!placement) return;
    updateCameraById(cameraUuid, {
      rotation: wrapRotation(normalizeCoverage(placement).rotation, delta),
    });
  };
  const nudgeCamera = (cameraUuid, stepsX, stepsY) => {
    const placement = draftCameras.find((camera) => camera.camera_uuid === cameraUuid);
    if (!placement) return;
    const step = gridStep(canvasDims);
    updateCameraById(cameraUuid, {
      x: clamp(placement.x + stepsX * step.x, 0.025, 0.975),
      y: clamp(placement.y + stepsY * step.y, 0.025, 0.975),
    });
  };
  const removeCamera = (cameraUuid) => {
    updateCameras((current) => current.filter((camera) => camera.camera_uuid !== cameraUuid));
    setSelectedCameraUuid((current) => current === cameraUuid ? '' : current);
    setPlacementCameraUuid((current) => current === cameraUuid ? '' : current);
  };

  const planLocation = plan.location_uuid
    ? (locationsQuery.data?.locations || []).find((location) => location.uuid === plan.location_uuid)
    : null;
  // Placing a camera on a plan is not a location assignment; offer to make
  // the Navigator's location tree agree with the plan in one step.
  const unassignedPlanStreams = planLocation
    ? (plan.cameras || []).map((camera) => streamsByUuid.get(camera.camera_uuid))
      .filter((stream) => stream && stream.location_uuid !== planLocation.uuid)
    : [];
  const assignPlanCamerasToLocation = async () => {
    if (!planLocation || unassignedPlanStreams.length === 0) return;
    setBusy(true);
    try {
      const { succeeded, failed } = await applyBulkOrganization(
        unassignedPlanStreams, { locationUuid: planLocation.uuid }, fetchJSON);
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: ['streams'] }),
        queryClient.invalidateQueries({ queryKey: ['fleet-locations'] }),
      ]);
      if (failed.length > 0) {
        showStatusMessage(t('live.plan.assignLocationPartial', {
          done: succeeded.length, failed: failed.length,
          error: failed[0].error,
        }), 'warning', 6000);
      } else {
        showStatusMessage(t('live.plan.assignLocationDone', {
          count: succeeded.length, location: planLocation.name,
        }), 'success', 3000);
      }
    } catch (error) {
      showStatusMessage(error.message, 'error', 5000);
    } finally {
      setBusy(false);
    }
  };

  const resetEditorState = () => {
    setEditing(false);
    setDraftHistory(createHistory(EMPTY_DRAFT));
    setDraftSize(null);
    setDraftRevision(null);
    setPlacementCameraUuid('');
    setSelectedShapeId('');
    setDrawingNow(null);
    setSketchTool('select');
    setContextMenu(null);
    interactionRef.current = null;
  };
  const startEditing = (tool = 'select') => {
    const initial = {
      shapes: viewSketch,
      cameras: (plan.cameras || []).map((camera) => ({ ...camera })),
    };
    editStartDraftRef.current = initial;
    setDraftHistory(createHistory(initial));
    setDraftLocationUuid(plan.location_uuid || '');
    setDraftSize(null);
    setDraftRevision(plan.revision);
    setSelectedShapeId('');
    setDrawingNow(null);
    setSketchTool(tool);
    setEditing(true);
  };
  const cancelEditing = () => resetEditorState();
  const hasDraftChanges = editing &&
    (draftHistory.present !== editStartDraftRef.current || draftSize !== null ||
      draftLocationUuid !== (plan.location_uuid || ''));
  const requestCancelEditing = () => {
    if (hasDraftChanges) setDiscardOpen(true);
    else cancelEditing();
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
            location_uuid: draftLocationUuid || null,
            revision: draftRevision ?? plan.revision,
          }, draftCameras),
          sketch: sketchPayload(draftSketch),
        }),
        timeout: 15000,
        retries: 0,
      });
      replacePlanInCache(updated);
      resetEditorState();
      showStatusMessage(t('live.plan.saved', { name: updated.name }), 'success', 2500);
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

  const isPanGesture = (event) => event.button === 1 || spaceHeldRef.current;
  const focusCanvas = () => {
    try {
      svgRef.current?.focus({ preventScroll: true });
    } catch {
      // Focus is a keyboard convenience only.
    }
  };
  const startPan = (event) => {
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
  const aimSnapDegrees = (event) => {
    if (event.shiftKey) return 45;
    return snap && !event.altKey ? 5 : 0;
  };
  const rawPlanPoint = (event) => {
    const svg = svgRef.current;
    const matrix = svg?.getScreenCTM();
    if (!svg || !matrix) return null;
    const point = svg.createSVGPoint();
    point.x = event.clientX;
    point.y = event.clientY;
    const transformed = point.matrixTransform(matrix.inverse());
    return { x: transformed.x / width, y: transformed.y / height };
  };
  /** Wall/area point; Shift locks the new segment to 45° steps. */
  const polylinePointFromEvent = (event, current) => {
    const point = sketchPointFromEvent(event);
    const last = current?.points?.[current.points.length - 1];
    if (!point || !event.shiftKey || !last) return point;
    return constrainAngle({ x: last[0], y: last[1] }, point, canvasDims);
  };

  const handleSurfacePointerDown = (event) => {
    suppressClickRef.current = false;
    if (event.pointerType === 'mouse' && event.button !== 0 && event.button !== 1) return;
    if (editing) focusCanvas();
    // Middle-drag or Space+drag pans with any tool active.
    if (isPanGesture(event)) {
      event.preventDefault();
      startPan(event);
      return;
    }
    if (editing && placementCameraUuid) {
      // Press places the armed camera; dragging from there aims it.
      const point = pointFromEvent(event);
      if (!point) return;
      const before = draftHistoryRef.current.present;
      const cameraUuid = placementCameraUuid;
      previewDraft((draft) => {
        const existing = draft.cameras.some((camera) => camera.camera_uuid === cameraUuid);
        return {
          ...draft,
          cameras: existing
            ? draft.cameras.map((camera) => camera.camera_uuid === cameraUuid
              ? { ...camera, ...point } : camera)
            : [...draft.cameras, { camera_uuid: cameraUuid, ...point, rotation: 0, fov: 65 }],
        };
      });
      suppressClickRef.current = true;
      interactionRef.current = {
        mode: 'aim', placing: true, pointerId: event.pointerId, cameraUuid,
        origin: point, before, startX: event.clientX, startY: event.clientY, moved: false,
      };
      return;
    }
    if (editing && sketchTool === 'room') {
      const start = sketchPointFromEvent(event);
      if (!start) return;
      interactionRef.current = {
        mode: 'draw-rect', pointerId: event.pointerId,
        startX: event.clientX, startY: event.clientY, start, moved: false,
      };
      setDrawingNow({ tool: 'room', start, current: start });
      return;
    }
    // Point-by-point tools add on click; a press must not start a pan.
    if (editing && sketchTool !== 'select') return;
    if (editing && event.pointerType !== 'touch') {
      // Dragging empty canvas box-selects; Shift/Ctrl adds to the selection.
      const start = rawPlanPoint(event);
      if (!start) return;
      interactionRef.current = {
        mode: 'marquee', pointerId: event.pointerId, start,
        base: event.shiftKey || event.ctrlKey || event.metaKey ? selectedShapeIds : [],
        startX: event.clientX, startY: event.clientY, moved: false,
      };
      return;
    }
    startPan(event);
  };

  const handleCameraPointerDown = (event, cameraUuid) => {
    if (!editing) return;
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (isPanGesture(event)) return;
    event.stopPropagation();
    suppressClickRef.current = false;
    interactionRef.current = {
      mode: 'camera',
      pointerId: event.pointerId,
      cameraUuid,
      before: draftHistoryRef.current.present,
      startX: event.clientX,
      startY: event.clientY,
      moved: false,
    };
    setSelectedCameraUuid(cameraUuid);
    setPlacementCameraUuid('');
    setSelectedShapeId('');
  };

  const handleAimPointerDown = (event, marker) => {
    if (!editing) return;
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (isPanGesture(event)) return;
    event.stopPropagation();
    suppressClickRef.current = false;
    interactionRef.current = {
      mode: 'aim',
      pointerId: event.pointerId,
      cameraUuid: marker.camera_uuid,
      origin: { x: marker.x, y: marker.y },
      before: draftHistoryRef.current.present,
      startX: event.clientX,
      startY: event.clientY,
      moved: false,
    };
  };

  const handleFovPointerDown = (event, marker) => {
    if (!editing) return;
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (isPanGesture(event)) return;
    event.stopPropagation();
    suppressClickRef.current = false;
    interactionRef.current = {
      mode: 'fov',
      pointerId: event.pointerId,
      cameraUuid: marker.camera_uuid,
      origin: { x: marker.x, y: marker.y },
      rotation: normalizeCoverage(marker).rotation,
      before: draftHistoryRef.current.present,
      startX: event.clientX,
      startY: event.clientY,
      moved: false,
    };
  };

  const handleShapePointerDown = (event, shapeId, handle) => {
    if (!editing || sketchTool !== 'select' || placementCameraUuid) return;
    if (event.pointerType === 'mouse' && event.button !== 0) return;
    if (isPanGesture(event)) return;
    event.stopPropagation();
    suppressClickRef.current = false;
    focusCanvas();
    const matrix = svgRef.current?.getScreenCTM();
    const shape = draftSketch.find((candidate) => candidate.id === shapeId);
    if (!matrix || !shape) return;
    let group = null;
    if (handle.kind === 'move') {
      if (event.shiftKey || event.ctrlKey || event.metaKey) {
        // Modifier-press toggles membership; the click must not undo it.
        const included = toggleShapeSelection(shapeId);
        suppressClickRef.current = true;
        if (!included) return;
        group = [...new Set([...selectedShapeIds, shapeId])];
      } else if (selectedIdSet.has(shapeId) && selectedShapeIds.length > 1) {
        group = selectedShapeIds;
      }
    }
    interactionRef.current = {
      mode: 'shape',
      pointerId: event.pointerId,
      shapeId,
      handle,
      group,
      original: shape,
      before: draftHistoryRef.current.present,
      startX: event.clientX,
      startY: event.clientY,
      screenPlanWidth: Math.hypot(matrix.a, matrix.b) * width,
      screenPlanHeight: Math.hypot(matrix.c, matrix.d) * height,
      moved: false,
    };
    if (!group) selectShape(shapeId);
  };

  const handleShapeClick = (event, shapeId) => {
    if (!editing || sketchTool !== 'select' || placementCameraUuid) return;
    event.stopPropagation();
    if (suppressClickRef.current) return;
    // A plain click inside a multi-selection narrows it to this shape.
    selectShape(shapeId);
  };

  // Double-click selects the shape and jumps to its text field in the
  // inspector (label text for label shapes, the optional caption otherwise).
  const handleShapeDblClick = (event, shapeId) => {
    if (!editing || sketchTool !== 'select' || placementCameraUuid) return;
    event.stopPropagation();
    const shape = draftSketch.find((candidate) => candidate.id === shapeId);
    if (!shape) return;
    selectShape(shapeId);
    focusShapeField(shape);
  };

  const applyShapeDrag = (interaction, event) => {
    let next = interaction.original;
    const kind = interaction.handle.kind;
    if (kind === 'move' && interaction.group) {
      let dx = (event.clientX - interaction.startX) / interaction.screenPlanWidth;
      let dy = (event.clientY - interaction.startY) / interaction.screenPlanHeight;
      const beforeShapes = interaction.before.shapes;
      if (snap && !event.altKey) {
        const bounds = shapesBounds(beforeShapes.filter((shape) =>
          interaction.group.includes(shape.id)));
        const snapped = snapPoint({ x: bounds.x0 + dx, y: bounds.y0 + dy }, canvasDims);
        dx = snapped.x - bounds.x0;
        dy = snapped.y - bounds.y0;
      }
      const moved = translateShapeGroup(beforeShapes, interaction.group, dx, dy);
      const byId = new Map(moved.map((shape) => [shape.id, shape]));
      const groupIds = new Set(interaction.group);
      previewDraft((draft) => ({
        ...draft,
        shapes: draft.shapes.map((shape) => groupIds.has(shape.id) ? byId.get(shape.id) || shape : shape),
      }));
      return;
    }
    if (kind === 'move') {
      const dx = (event.clientX - interaction.startX) / interaction.screenPlanWidth;
      const dy = (event.clientY - interaction.startY) / interaction.screenPlanHeight;
      next = translateShape(interaction.original, dx, dy);
      if (snap && !event.altKey) {
        const bounds = shapeBounds(next);
        const snapped = snapPoint({ x: bounds.x0, y: bounds.y0 }, canvasDims);
        next = translateShape(next, snapped.x - bounds.x0, snapped.y - bounds.y0);
      }
    } else {
      const point = sketchPointFromEvent(event);
      if (!point) return;
      if (kind === 'corner') {
        next = resizeRectCorner(interaction.original, interaction.handle.corner, point);
      } else if (kind === 'edge') {
        next = resizeRectEdge(interaction.original, interaction.handle.edge, point);
      } else if (kind === 'insert') {
        // Dragging a segment midpoint splits the segment with a new vertex.
        next = insertShapeVertex(interaction.original, interaction.handle.index, point);
      } else {
        next = moveShapeVertex(interaction.original, interaction.handle.index, point);
      }
    }
    previewDraft((draft) => ({
      ...draft,
      shapes: draft.shapes.map((shape) => shape.id === next.id ? next : shape),
    }));
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
    if (!interaction) {
      const current = drawingRef.current;
      if (current && current.tool !== 'room') {
        const cursor = polylinePointFromEvent(event, current);
        if (cursor) setDrawingNow((state) => state ? { ...state, cursor } : state);
      }
      return;
    }
    if (event.pointerId !== interaction.pointerId) return;
    const dx = event.clientX - interaction.startX;
    const dy = event.clientY - interaction.startY;
    if (!interaction.moved && Math.hypot(dx, dy) < 4) return;
    if (!interaction.moved) beginDrag(interaction, event);
    if (interaction.mode === 'draw-rect') {
      const current = sketchPointFromEvent(event);
      if (current) setDrawingNow({ tool: 'room', start: interaction.start, current });
      return;
    }
    if (interaction.mode === 'shape') {
      applyShapeDrag(interaction, event);
      return;
    }
    if (interaction.mode === 'marquee') {
      const current = rawPlanPoint(event);
      if (!current) return;
      setMarquee({ start: interaction.start, current });
      const hits = shapesInMarquee(draftSketch, interaction.start, current);
      selectShapes([...new Set([...interaction.base, ...hits])]);
      return;
    }
    if (interaction.mode === 'fov') {
      const heading = aimRotation(interaction.origin, rawPlanPoint(event), canvasDims);
      const fov = heading === null ? null : fovFromHeading(interaction.rotation, heading,
        event.shiftKey ? 15 : (snap && !event.altKey ? 5 : 0));
      if (fov !== null) {
        previewDraft((draft) => ({
          ...draft,
          cameras: draft.cameras.map((camera) => camera.camera_uuid === interaction.cameraUuid
            ? { ...camera, fov } : camera),
        }));
      }
      return;
    }
    if (interaction.mode === 'aim') {
      const rotation = aimRotation(interaction.origin, rawPlanPoint(event),
        canvasDims, aimSnapDegrees(event));
      if (rotation !== null) {
        previewDraft((draft) => ({
          ...draft,
          cameras: draft.cameras.map((camera) => camera.camera_uuid === interaction.cameraUuid
            ? { ...camera, rotation } : camera),
        }));
      }
      return;
    }
    if (interaction.mode === 'camera') {
      const point = pointFromEvent(event);
      if (point) {
        previewDraft((draft) => ({
          ...draft,
          cameras: draft.cameras.map((camera) =>
            camera.camera_uuid === interaction.cameraUuid
              ? { ...camera, ...point } : camera),
        }));
      }
      return;
    }
    setCenter(clampCenter({
      x: interaction.origin.x - dx / interaction.screenPlanWidth,
      y: interaction.origin.y - dy / interaction.screenPlanHeight,
    }, interaction.zoom));
  };

  const handleSurfacePointerEnd = (event) => {
    const interaction = interactionRef.current;
    if (!interaction || event.pointerId !== interaction.pointerId) return;
    interactionRef.current = null;
    setPanning(false);
    if (interaction.mode === 'draw-rect') {
      const current = drawingRef.current;
      setDrawingNow(null);
      const rect = current && interaction.moved
        ? rectFromPoints(current.start, current.current) : null;
      if (rect && event.type !== 'pointercancel') {
        addShape({
          id: createShapeId(shapeIds()), type: 'rect', tone: sketchTone,
          ...rect, filled: true,
        });
      }
    } else if (interaction.mode === 'aim' && interaction.placing) {
      finishGesture(interaction.before);
      setPlacementCameraUuid('');
      setSelectedCameraUuid(interaction.cameraUuid);
    } else if (interaction.mode === 'marquee') {
      setMarquee(null);
      // A press without a drag falls through to the canvas click (deselect).
    } else if ((interaction.mode === 'shape' || interaction.mode === 'camera' ||
      interaction.mode === 'aim' || interaction.mode === 'fov') && interaction.moved) {
      if (event.type === 'pointercancel') setDraft(interaction.before);
      else finishGesture(interaction.before);
    }
    try {
      svgRef.current?.releasePointerCapture(event.pointerId);
    } catch {
      // Capture may already be released when the pointer left the window.
    }
  };

  const openBackgroundPicker = () => fileInputRef.current?.click();

  /* ---- right-click menus ------------------------------------------------ */

  const closeContextMenu = () => setContextMenu(null);
  const openMenu = (event, items) => {
    event.preventDefault();
    event.stopPropagation();
    if (items.length === 0) return;
    setContextMenu({ clientX: event.clientX, clientY: event.clientY, items });
  };
  const separator = { separator: true };

  const drawingMenuItems = () => [
    {
      id: 'finish', label: t('live.plan.menu.finishDrawing'), shortcut: 'Enter',
      disabled: !drawingFinishable, onSelect: finishDrawing,
    },
    {
      id: 'remove-point', label: t('live.plan.menu.removeLastPoint'), shortcut: 'Backspace',
      onSelect: removeLastDrawingPoint,
    },
    { id: 'cancel', label: t('live.plan.menu.cancelDrawing'), shortcut: 'Esc', onSelect: cancelDrawing },
  ];

  const toneSwatches = (ids, activeTone) => ({
    id: 'tone', label: t('live.plan.sketch.tone'),
    swatches: SKETCH_TONES.map((tone) => ({
      id: tone, className: `tone-${tone}`, label: t(`live.plan.sketch.tone.${tone}`),
      active: activeTone === tone,
      onSelect: () => recolorShapes(ids, tone),
    })),
  });
  const groupMenuItems = (shapes) => {
    const ids = shapes.map((shape) => shape.id);
    const tones = new Set(shapes.map((shape) => shape.tone));
    return [
      {
        id: 'duplicate', label: t('live.plan.sketch.duplicate'), shortcut: 'Ctrl+D',
        disabled: draftSketch.length + ids.length > SKETCH_LIMITS.maxShapes,
        onSelect: () => addShapes(cloneShapes(shapes)),
      },
      { id: 'copy', label: t('live.plan.menu.copy'), shortcut: 'Ctrl+C', onSelect: () => copyShapes(shapes) },
      {
        id: 'cut', label: t('live.plan.menu.cut'), shortcut: 'Ctrl+X',
        onSelect: () => { copyShapes(shapes); deleteShapesById(ids); },
      },
      separator,
      { id: 'front', label: t('live.plan.menu.bringToFront'), onSelect: () => reorderShapesById(ids, 'front') },
      { id: 'back', label: t('live.plan.menu.sendToBack'), onSelect: () => reorderShapesById(ids, 'back') },
      separator,
      toneSwatches(ids, tones.size === 1 ? shapes[0].tone : null),
      separator,
      {
        id: 'delete', label: t('live.plan.menu.deleteShapes', { count: ids.length }),
        shortcut: 'Del', danger: true, onSelect: () => deleteShapesById(ids),
      },
    ];
  };

  const handleShapeContextMenu = (event, shapeId, handle) => {
    if (!editing) return;
    if (drawingRef.current) {
      openMenu(event, drawingMenuItems());
      return;
    }
    const shape = draftSketch.find((candidate) => candidate.id === shapeId);
    if (!shape) return;
    if (sketchTool !== 'select') {
      setSketchTool('select');
      setDrawingNow(null);
    }
    if (selectedIdSet.has(shapeId) && selectedShapes.length > 1) {
      openMenu(event, groupMenuItems(selectedShapes));
      return;
    }
    selectShape(shapeId);
    const items = [];
    if (handle?.kind === 'vertex' && canRemoveVertex(shape)) {
      items.push({
        id: 'remove-vertex', label: t('live.plan.menu.deletePoint'),
        onSelect: () => updateShapeById(shapeId,
          (current) => removeShapeVertex(current, handle.index)),
      }, separator);
    }
    items.push(
      {
        id: 'edit-text',
        label: shape.type === 'label' ? t('live.plan.menu.editText') : t('live.plan.menu.editLabel'),
        onSelect: () => focusShapeField(shape),
      },
      {
        id: 'duplicate', label: t('live.plan.sketch.duplicate'), shortcut: 'Ctrl+D',
        disabled: draftSketch.length >= SKETCH_LIMITS.maxShapes,
        onSelect: () => addShape(duplicateShape(shape, shapeIds(), canvasDims), 'select'),
      },
      { id: 'copy', label: t('live.plan.menu.copy'), shortcut: 'Ctrl+C', onSelect: () => copyShape(shape) },
      {
        id: 'cut', label: t('live.plan.menu.cut'), shortcut: 'Ctrl+X',
        onSelect: () => { copyShape(shape); deleteShapeById(shapeId); },
      },
      separator,
      { id: 'front', label: t('live.plan.menu.bringToFront'), onSelect: () => reorderShapeById(shapeId, 'front') },
      { id: 'forward', label: t('live.plan.menu.bringForward'), onSelect: () => reorderShapeById(shapeId, 'forward') },
      { id: 'backward', label: t('live.plan.menu.sendBackward'), onSelect: () => reorderShapeById(shapeId, 'backward') },
      { id: 'back', label: t('live.plan.menu.sendToBack'), onSelect: () => reorderShapeById(shapeId, 'back') },
      separator,
      toneSwatches([shapeId], shape.tone),
    );
    if (shape.type === 'rect') {
      items.push({
        id: 'fill',
        label: shape.filled ? t('live.plan.menu.removeFill') : t('live.plan.menu.addFill'),
        onSelect: () => updateShapeById(shapeId, (current) => ({ ...current, filled: !current.filled })),
      });
    }
    items.push(separator, {
      id: 'delete', label: t('live.plan.sketch.delete'), shortcut: 'Del', danger: true,
      onSelect: () => deleteShapeById(shapeId),
    });
    openMenu(event, items);
  };

  const handleCameraContextMenu = (event, cameraUuid, stream) => {
    if (!editing) {
      if (stream?.availability === 'live') {
        openMenu(event, [{
          id: 'open', label: t('live.plan.openLive'), onSelect: () => onOpenCamera(stream),
        }]);
      }
      return;
    }
    setSelectedCameraUuid(cameraUuid);
    setPlacementCameraUuid('');
    setSelectedShapeId('');
    openMenu(event, [
      {
        id: 'rotate-left', label: t('live.plan.menu.rotateLeft', { degrees: CAMERA_ROTATE_STEP }),
        shortcut: '[', onSelect: () => rotateCamera(cameraUuid, -CAMERA_ROTATE_STEP),
      },
      {
        id: 'rotate-right', label: t('live.plan.menu.rotateRight', { degrees: CAMERA_ROTATE_STEP }),
        shortcut: ']', onSelect: () => rotateCamera(cameraUuid, CAMERA_ROTATE_STEP),
      },
      { id: 'turn', label: t('live.plan.menu.turnAround'), onSelect: () => rotateCamera(cameraUuid, 180) },
      {
        id: 'reposition', label: t('live.plan.menu.reposition'),
        onSelect: () => setPlacementCameraUuid(cameraUuid),
      },
      separator,
      {
        id: 'remove', label: t('live.plan.removeCamera'), shortcut: 'Del', danger: true,
        onSelect: () => removeCamera(cameraUuid),
      },
    ]);
  };

  const handleCanvasContextMenu = (event) => {
    if (!editing) {
      if (canModify) {
        openMenu(event, [{
          id: 'edit', label: t('live.plan.editLayout'), onSelect: () => startEditing(),
        }]);
      }
      return;
    }
    const interaction = interactionRef.current;
    if (interaction?.mode === 'draw-rect') {
      // Right-click mid-drag abandons the room being drawn.
      event.preventDefault();
      interactionRef.current = null;
      setDrawingNow(null);
      return;
    }
    if (drawingRef.current) {
      openMenu(event, drawingMenuItems());
      return;
    }
    const point = sketchPointFromEvent(event);
    const items = [];
    if (placementCameraUuid) {
      items.push({
        id: 'cancel-placement', label: t('live.plan.menu.cancelPlacement'), shortcut: 'Esc',
        onSelect: () => setPlacementCameraUuid(''),
      }, separator);
    }
    items.push(
      {
        id: 'paste', label: t('live.plan.menu.pasteHere'), shortcut: 'Ctrl+V',
        disabled: !clipboardRef.current || draftSketch.length >= SKETCH_LIMITS.maxShapes,
        onSelect: () => pasteShape(point),
      },
      {
        id: 'label', label: t('live.plan.menu.addLabelHere'),
        disabled: !point || draftSketch.length >= SKETCH_LIMITS.maxShapes,
        onSelect: () => {
          const shape = {
            id: createShapeId(shapeIds()), type: 'label', tone: sketchTone,
            x: point.x, y: point.y, text: t('live.plan.sketch.defaultLabel'), size: 'md',
          };
          addShape(shape, 'select');
          focusShapeField(shape);
        },
      },
      separator,
      { id: 'room', label: t('live.plan.sketch.tool.room'), shortcut: 'R', onSelect: () => selectTool('room') },
      { id: 'wall', label: t('live.plan.sketch.tool.wall'), shortcut: 'W', onSelect: () => selectTool('wall') },
      { id: 'area', label: t('live.plan.sketch.tool.area'), shortcut: 'A', onSelect: () => selectTool('area') },
      separator,
      {
        id: 'undo', label: t('live.plan.sketch.undo'), shortcut: 'Ctrl+Z',
        disabled: draftHistory.past.length === 0, onSelect: undoSketch,
      },
      {
        id: 'redo', label: t('live.plan.sketch.redo'), shortcut: 'Ctrl+Shift+Z',
        disabled: draftHistory.future.length === 0, onSelect: redoSketch,
      },
      {
        id: 'snap', label: snap ? t('live.plan.menu.snapOff') : t('live.plan.menu.snapOn'),
        shortcut: 'G', onSelect: () => setSnap(!snap),
      },
    );
    openMenu(event, items);
  };

  /* ---- keyboard --------------------------------------------------------- */

  const handleKeyDown = (event) => {
    if (!editing || event.defaultPrevented) return;
    if (document.querySelector('[role="dialog"][aria-modal="true"]')) return;
    const target = event.target;
    const inScope = target === document.body || target === document.documentElement ||
      Boolean(sectionRef.current?.contains(target));
    if (!inScope) return;
    const selectedCamera = !hasShapeSelection && selectedCameraUuid
      ? draftCameras.find((camera) => camera.camera_uuid === selectedCameraUuid) : null;
    if (event.key === 'Escape') {
      if (contextMenu) closeContextMenu();
      else if (interactionRef.current?.mode === 'draw-rect') {
        interactionRef.current = null;
        setDrawingNow(null);
      } else if (drawingRef.current) cancelDrawing();
      else if (placementCameraUuid) setPlacementCameraUuid('');
      else if (hasShapeSelection) setSelectedShapeIds([]);
      else if (selectedCameraUuid) setSelectedCameraUuid('');
      else if (sketchTool !== 'select') selectTool('select');
      else return;
      event.preventDefault();
      return;
    }
    if (isFormField(target)) return;
    const key = event.key.toLowerCase();
    const mod = event.ctrlKey || event.metaKey;
    if (mod) {
      if (key === 'z') {
        event.preventDefault();
        if (event.shiftKey) redoSketch();
        else undoSketch();
      } else if (key === 'y') {
        event.preventDefault();
        redoSketch();
      } else if (key === 'a' && !drawingRef.current) {
        event.preventDefault();
        if (sketchTool !== 'select') selectTool('select');
        selectShapes(draftSketch.map((shape) => shape.id));
      } else if (key === 'c' && hasShapeSelection) {
        event.preventDefault();
        copyShapes(selectedShapes);
      } else if (key === 'x' && hasShapeSelection) {
        event.preventDefault();
        copyShapes(selectedShapes);
        deleteSelectedShape();
      } else if (key === 'v' && clipboardRef.current) {
        event.preventDefault();
        pasteShape();
      } else if (key === 'd' && hasShapeSelection) {
        event.preventDefault();
        duplicateSelectedShape();
      }
      return;
    }
    if (event.key === ' ' && target?.tagName !== 'BUTTON' && target?.tagName !== 'A') {
      event.preventDefault();
      if (!spaceHeldRef.current) {
        spaceHeldRef.current = true;
        setSpaceHeld(true);
      }
      return;
    }
    if (event.key === 'Enter' && drawingRef.current &&
      target?.tagName !== 'BUTTON' && target?.tagName !== 'A') {
      event.preventDefault();
      finishDrawing();
      return;
    }
    if (event.key === 'Delete' || event.key === 'Backspace') {
      if (drawingRef.current) {
        event.preventDefault();
        removeLastDrawingPoint();
      } else if (hasShapeSelection) {
        event.preventDefault();
        deleteSelectedShape();
      } else if (selectedCamera) {
        event.preventDefault();
        removeCamera(selectedCamera.camera_uuid);
      }
      return;
    }
    if (event.key.startsWith('Arrow') && (hasShapeSelection || selectedCamera)) {
      event.preventDefault();
      const steps = event.shiftKey ? 5 : 1;
      const delta = {
        ArrowLeft: [-steps, 0], ArrowRight: [steps, 0],
        ArrowUp: [0, -steps], ArrowDown: [0, steps],
      }[event.key];
      if (!delta) return;
      if (hasShapeSelection) nudgeSelectedShape(delta[0], delta[1]);
      else nudgeCamera(selectedCamera.camera_uuid, delta[0], delta[1]);
      return;
    }
    if (selectedCamera && ['[', ']', '{', '}'].includes(event.key)) {
      event.preventDefault();
      const step = event.shiftKey ? 45 : CAMERA_ROTATE_STEP;
      rotateCamera(selectedCamera.camera_uuid,
        event.key === '[' || event.key === '{' ? -step : step);
      return;
    }
    if (selectedCamera && ['-', '=', '+', '_'].includes(event.key)) {
      event.preventDefault();
      const step = event.shiftKey ? 15 : 5;
      const current = normalizeCoverage(selectedCamera).fov;
      const widen = event.key === '=' || event.key === '+';
      updateCameraById(selectedCamera.camera_uuid, {
        fov: clamp(current + (widen ? step : -step), 1, 180),
      });
      return;
    }
    if (event.altKey) return;
    if (key === 'q') {
      event.preventDefault();
      setToolLock(!toolLock);
      return;
    }
    if (key === 'g') {
      event.preventDefault();
      setSnap(!snap);
      return;
    }
    if (TOOL_SHORTCUTS[key]) {
      event.preventDefault();
      selectTool(TOOL_SHORTCUTS[key]);
    }
  };
  keyHandlerRef.current = handleKeyDown;

  return <>
    <section ref={sectionRef} className={`live-building-plan ${editing ? 'is-editing' : ''}`}>
      <header className="live-plan-toolbar">
        <div>
          <span className="live-plan-eyebrow">{t('live.plan.eyebrow')}</span>
          <h3>{plan.name}</h3>
          <small>{t('live.plan.placedSummary', { 
            placed: cameras.length,
            live: cameras.filter((camera) =>
              streamsByUuid.get(camera.camera_uuid)?.availability === 'live').length,
           })}</small>
        </div>
        <div className="live-plan-toolbar-actions">
          <div className="live-plan-zoom" aria-label={t('live.plan.zoomAndPan')}>
            <button type="button" onClick={() => pan(-0.12, 0)} title={t('live.plan.panLeft')}>←</button>
            <button type="button" onClick={() => pan(0, -0.12)} title={t('live.plan.panUp')}>↑</button>
            <button type="button" onClick={() => pan(0, 0.12)} title={t('live.plan.panDown')}>↓</button>
            <button type="button" onClick={() => pan(0.12, 0)} title={t('live.plan.panRight')}>→</button>
            <button type="button" onClick={() => setPlanZoom(zoom - 0.5)}
              disabled={zoom <= 1} title={t('live.plan.zoomOut')}>−</button>
            <span>{Math.round(zoom * 100)}%</span>
            <button type="button" onClick={() => setPlanZoom(zoom + 0.5)}
              disabled={zoom >= 4} title={t('live.plan.zoomIn')}>+</button>
            <button type="button" onClick={() => {
              setPlanZoom(1, { x: 0.5, y: 0.5 });
            }} title={t('live.plan.fitTitle')}>{t('live.plan.fit')}</button>
          </div>
          {editing ? (
            <>
              {canModify && (
                <button type="button" onClick={openBackgroundPicker}
                  disabled={busy}>{t('live.plan.backgroundMenu')}</button>
              )}
              <button type="button" onClick={requestCancelEditing} disabled={busy}>{t('common.cancel')}</button>
              <button type="button" className="btn-primary" onClick={savePlan}
                disabled={busy || !canvasSizeValid}>{t('live.plan.save')}</button>
            </>
          ) : canModify && (
            <>
              <button type="button" onClick={openBackgroundPicker}
                disabled={busy}
                title={t('live.plan.uploadBackgroundTitle')}>
                {plan.background_mime ? t('live.plan.replaceBackground') : t('live.plan.uploadBackground')}
              </button>
              {plan.background_mime && (
                <button type="button" onClick={removeBackground} disabled={busy}>
                  {t('live.plan.removeBackground')}
                </button>
              )}
              <button type="button" onClick={() => setRenameOpen(true)} disabled={busy}>
                {t('live.plan.rename')}
              </button>
              <button type="button" onClick={() => setDeleteOpen(true)} disabled={busy}>
                {t('common.delete')}
              </button>
              <button type="button" onClick={() => setCreatePlanOpen(true)} disabled={busy}>{t('live.plan.newPlan')}</button>
              <button type="button" className="btn-primary" onClick={() => startEditing()}
                disabled={busy}>{t('live.plan.editLayout')}</button>
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
            <div className="live-plan-placement-hint">{t('live.plan.placementHint', { name: streamsByUuid.get(placementCameraUuid)?.name })}</div>
          )}
          {editing && (
            <PlanSketchTools tool={sketchTool} onToolChange={selectTool}
              canUndo={draftHistory.past.length > 0 || Boolean(drawing && drawing.tool !== 'room')}
              canRedo={draftHistory.future.length > 0 && !drawing}
              onUndo={undoSketch} onRedo={redoSketch}
              snap={snap} onSnapChange={setSnap}
              toolLock={toolLock} onToolLockChange={setToolLock}
              shapeCount={draftSketch.length} t={t} />
          )}
          <svg ref={svgRef} tabIndex="-1"
            className={`live-plan-canvas ${panning ? 'is-panning' : ''} ${spaceHeld && !panning ? 'is-pan-ready' : ''} ${editing && sketchTool !== 'select' ? 'is-drawing' : ''} ${editing ? `is-tool-${sketchTool}` : ''} ${placementCameraUuid ? 'is-placing' : ''}`}
            viewBox={viewBox} role="img" aria-label={`${plan.name} camera plan`}
            onPointerDown={handleSurfacePointerDown}
            onPointerMove={handleSurfacePointerMove}
            onPointerUp={handleSurfacePointerEnd}
            onPointerCancel={handleSurfacePointerEnd}
            onContextMenu={handleCanvasContextMenu}
            onClick={(event) => {
              if (suppressClickRef.current) return;
              if (!editing) return;
              if (placementCameraUuid) return;
              if (sketchTool === 'wall' || sketchTool === 'area') {
                const current = drawingRef.current;
                const point = polylinePointFromEvent(event,
                  current && current.tool === sketchTool ? current : null);
                if (!point) return;
                if (current && current.tool === sketchTool) {
                  if (sketchTool === 'area' &&
                    closesPolygon(current.points, point, { width, height })) {
                    finishDrawing();
                    return;
                  }
                  setDrawingNow({ ...current, points: appendPolylinePoint(current.points, point), cursor: point });
                } else {
                  setDrawingNow({ tool: sketchTool, points: [[point.x, point.y]], cursor: point });
                }
                return;
              }
              if (sketchTool === 'label') {
                const point = sketchPointFromEvent(event);
                if (!point) return;
                addShape({
                  id: createShapeId(shapeIds()), type: 'label', tone: sketchTone,
                  x: point.x, y: point.y, text: t('live.plan.sketch.defaultLabel'), size: 'md',
                });
                return;
              }
              if (sketchTool === 'select' && !event.shiftKey && !event.ctrlKey && !event.metaKey) {
                if (hasShapeSelection) setSelectedShapeIds([]);
                if (selectedCameraUuid) setSelectedCameraUuid('');
              }
            }}
            onDblClick={(event) => {
              if (!editing || !drawingRef.current) return;
              event.preventDefault();
              finishDrawing();
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
            {plan.background_mime && (
              <image href={planBackgroundUrl(plan, backgroundVersion)} x="0" y="0"
                width={width} height={height}
                preserveAspectRatio="none" className="live-plan-background" />
            )}
            <PlanSketchLayer shapes={sketchShapes} width={width} height={height}
              editing={editing} selectedShapeIds={selectedIdSet}
              markerScale={markerScale} drawing={drawing}
              drawingClosable={drawingClosable} t={t}
              onShapePointerDown={handleShapePointerDown}
              onShapeClick={handleShapeClick}
              onShapeDblClick={handleShapeDblClick}
              onShapeContextMenu={handleShapeContextMenu} />
            {marquee && (() => {
              const box = rectFromPoints(marquee.start, marquee.current);
              return box ? (
                <rect x={box.x * width} y={box.y * height}
                  width={box.w * width} height={box.h * height}
                  className="live-plan-marquee" />
              ) : null;
            })()}

            {renderedMarkers.map((marker) => {
              const markerX = marker.x * width;
              const markerY = marker.y * height;
              if (marker.kind === 'cluster') {
                return (
                  <g key={marker.camera_uuids.join(':')}
                    className="live-plan-cluster" tabIndex="0" role="button"
                    aria-label={t('live.plan.clusterLabel', { count: marker.count })}
                    transform={`translate(${markerX} ${markerY}) scale(${markerScale})`}
                    onClick={(event) => {
                      event.stopPropagation();
                      if (suppressClickRef.current) return;
                      setPlanZoom(zoom + 1, { x: marker.x, y: marker.y });
                    }}
                    onKeyDown={(event) => {
                      if (event.key !== 'Enter' && event.key !== ' ') return;
                      if (event.key === ' ') event.preventDefault();
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
                  aria-label={t('live.plan.markerLabel', { 
                    name: stream?.name || t('live.plan.cameraFallback'),
                    availability: availabilityLabel(stream, t),
                    rotation: coverageValueLabel(coverage.rotation),
                    fov: coverageValueLabel(coverage.fov),
                   })}
                  onPointerDown={(event) => handleCameraPointerDown(event, marker.camera_uuid)}
                  onContextMenu={(event) => handleCameraContextMenu(event, marker.camera_uuid, stream)}
                  onKeyDown={(event) => {
                    if (event.key !== 'Enter' && event.key !== ' ') return;
                    event.stopPropagation();
                    if (event.key === ' ') event.preventDefault();
                    setSelectedCameraUuid(marker.camera_uuid);
                    if (editing) {
                      setSelectedShapeId('');
                    } else if (event.key === 'Enter' && stream) {
                      onOpenCamera(stream);
                    }
                  }}
                  onClick={(event) => {
                    event.stopPropagation();
                    if (suppressClickRef.current) return;
                    setSelectedCameraUuid(marker.camera_uuid);
                    if (editing) setSelectedShapeId('');
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
                  {editing && selected && (
                    // Drag the knob to aim the camera; Shift snaps to 45°.
                    <g className="live-plan-aim" transform={`rotate(${coverage.rotation})`}>
                      <line x1="19" y1="0" x2="63" y2="0" className="live-plan-aim-line" />
                      <circle cx="72" cy="0" r="9" className="live-plan-aim-handle"
                        onPointerDown={(event) => handleAimPointerDown(event, marker)}>
                        <title>{t('live.plan.aimHandle')}</title>
                      </circle>
                      {/* Wedge-edge handles widen or narrow the field of view. */}
                      {[-1, 1].map((side) => {
                        const half = (coverage.fov * Math.PI) / 360;
                        return (
                          <circle key={side} cx={55 * Math.cos(half)} cy={side * 55 * Math.sin(half)} r="7"
                            className="live-plan-fov-handle"
                            onPointerDown={(event) => handleFovPointerDown(event, marker)}>
                            <title>{t('live.plan.fovHandle')}</title>
                          </circle>
                        );
                      })}
                    </g>
                  )}
                </g>
              );
            })}
          </svg>
          {!plan.background_mime && !editing && viewSketch.length === 0 && (
            <div className="live-plan-background-hint">
              <strong>{t('live.plan.sketch.emptyTitle')}</strong>
              <p>{canModify
                ? t('live.plan.sketch.emptyBody')
                : t('live.plan.sketch.emptyBodyReadOnly')}</p>
              {canModify && (
                <div className="live-plan-hint-actions">
                  <button type="button" className="btn-primary" disabled={busy}
                    onClick={() => startEditing('room')}>{t('live.plan.sketch.drawLayout')}</button>
                  <button type="button" disabled={busy}
                    onClick={openBackgroundPicker}>{t('live.plan.sketch.uploadImage')}</button>
                </div>
              )}
            </div>
          )}
          <div className="live-plan-legend">
            <span><i className="is-live" />{t('live.plan.legendLive')}</span>
            <span><i className="is-offline" />{t('common.offline')}</span>
            <span><i className="is-never_connected" />{t('availability.neverConnected')}</span>
            <span><i className="is-disabled" />{t('availability.disabled')}</span>
          </div>
        </div>

        <aside className="live-plan-details" aria-live="polite">
          {editing && (
            <div className="live-plan-settings">
              <div><strong>{t('live.plan.canvas')}</strong></div>
              <label>{t('live.plan.width')}
                <input type="number" min="400" max="4000" step="10"
                  value={draftWidthValue} aria-invalid={draftWidth === null}
                  onInput={(event) => setDraftSize({
                    canvas_width: event.currentTarget.value,
                    canvas_height: draftHeightValue,
                  })} />
              </label>
              <label>{t('live.plan.height')}
                <input type="number" min="300" max="4000" step="10"
                  value={draftHeightValue} aria-invalid={draftHeight === null}
                  onInput={(event) => setDraftSize({
                    canvas_width: draftWidthValue,
                    canvas_height: event.currentTarget.value,
                  })} />
              </label>
              <small className={canvasSizeValid ? '' : 'is-error'}>
                {canvasSizeValid
                  ? t('live.plan.canvasHint')
                  : t('live.plan.canvasInvalid')}
              </small>
              <label className="live-plan-location-field">{t('live.plan.location')}
                <select value={draftLocationUuid}
                  onChange={(event) => setDraftLocationUuid(event.currentTarget.value)}>
                  <option value="">{t('live.plan.noLocation')}</option>
                  {locationOptions.map((location) => (
                    <option key={location.uuid} value={location.uuid}>
                      {`${'\u00a0\u00a0'.repeat(location.depth)}${location.name}`}
                    </option>
                  ))}
                </select>
              </label>
              <small>{t('live.plan.locationHint')}</small>
            </div>
          )}
          {!editing && canModify && planLocation && unassignedPlanStreams.length > 0 && (
            <div className="live-plan-location-callout">
              <p>{t('live.plan.assignLocationPrompt', {
                count: unassignedPlanStreams.length, location: planLocation.name,
              })}</p>
              <button type="button" className="btn-primary" disabled={busy}
                onClick={assignPlanCamerasToLocation}>
                {t('live.plan.assignLocation', { location: planLocation.name })}
              </button>
            </div>
          )}
          {editing && (
            <PlanSketchInspector uid={`${panelUid}-sketch`} shapes={draftSketch}
              selectedShape={selectedShape} selectedIds={selectedIdSet}
              selectionCount={selectedShapes.length} t={t}
              onSelect={(shapeId, additive) => (additive
                ? toggleShapeSelection(shapeId) : selectShape(shapeId))}
              onChange={replaceShape}
              onRecolorSelection={(tone) => recolorShapes(selectedShapeIds, tone)}
              onDelete={deleteSelectedShape} onDuplicate={duplicateSelectedShape}
              onDeselect={() => setSelectedShapeIds([])} />
          )}
          {hasShapeSelection ? null : selectedStream && selectedPlacement ? (
            <>
              <PlanCameraPreview stream={selectedStream} />
              <div className="live-plan-camera-summary">
                <span className={`live-availability-dot is-${selectedStream.availability}`} />
                <div><strong>{selectedStream.name}</strong>
                  <small>{availabilityLabel(selectedStream, t)}</small></div>
              </div>
              {!editing && selectedStream.availability === 'live' && (
                <button type="button" className="btn-primary"
                  onClick={() => onOpenCamera(selectedStream)}>{t('live.plan.openLive')}</button>
              )}
              {editing && selectedCoverage && (
                <div className="live-plan-placement-controls">
                  <PlanCoverageControl
                    key={`rotation-${selectedCameraUuid}`}
                    id={`${panelUid}-rotation`}
                    label={t('live.plan.direction')}
                    value={selectedCoverage.rotation}
                    minimum={-180}
                    maximum={180}
                    onChange={(rotation) => updateSelectedPlacement({ rotation })} />
                  <PlanCoverageControl
                    key={`fov-${selectedCameraUuid}`}
                    id={`${panelUid}-fov`}
                    label={t('live.plan.fov')}
                    value={selectedCoverage.fov}
                    minimum={1}
                    maximum={180}
                    onChange={(fov) => updateSelectedPlacement({ fov })} />
                  <small>{t('live.plan.coverageNote')}</small>
                  <button type="button" className="live-plan-remove-camera"
                    onClick={() => removeCamera(selectedCameraUuid)}>{t('live.plan.removeCamera')}</button>
                </div>
              )}
            </>
          ) : (
            <div className="live-plan-details-empty">
              <strong>{t('live.plan.selectCamera')}</strong>
              <span>{editing
                ? t('live.plan.selectHintEditing')
                : t('live.plan.selectHint')}</span>
            </div>
          )}

          {!editing && cameras.length > 0 && (
            <div className="live-plan-camera-index">
              <div><strong>{t('live.plan.cameras')}</strong><small>{indexedCameras.length}</small></div>
              <input type="search" value={cameraSearch} placeholder={t('live.plan.findPlaceholder')}
                aria-label={t('live.plan.findLabel')}
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
              <div><strong>{t('live.plan.unplaced')}</strong><small>{unplacedStreams.length}</small></div>
              <p>{t('live.plan.unplacedHint')}</p>
              <div>
                {unplacedStreams.map((stream) => (
                  <button key={stream.camera_uuid} type="button" draggable
                    className={placementCameraUuid === stream.camera_uuid ? 'is-selected' : ''}
                    onDragStart={(event) => {
                      event.dataTransfer.setData('application/x-lightnvr-camera', stream.camera_uuid);
                      event.dataTransfer.effectAllowed = 'copy';
                    }}
                    onClick={() => placeCamera(stream.camera_uuid, undefined, true)}>
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
      title={t('live.plan.renameTitle')}
      description={t('live.plan.renameDescription')}
      inputLabel={t('live.plan.nameLabel')}
      initialValue={plan.name}
      maxLength={127}
      confirmLabel={t('live.plan.renameTitle')}
      cancelLabel={t('common.cancel')}
    />
    <PlanContextMenu menu={contextMenu} label={t('live.plan.menu.label')}
      onClose={closeContextMenu} />
    <ConfirmDialog
      isOpen={discardOpen}
      onClose={() => setDiscardOpen(false)}
      onConfirm={() => {
        setDiscardOpen(false);
        cancelEditing();
      }}
      title={t('live.plan.discardTitle')}
      message={t('live.plan.discardMessage')}
      confirmLabel={t('live.plan.discardConfirm')}
      cancelLabel={t('live.plan.keepEditing')}
      variant="danger"
    />
    <ConfirmDialog
      isOpen={deleteOpen}
      onClose={() => setDeleteOpen(false)}
      onConfirm={deletePlan}
      title={t('live.plan.deleteTitle')}
      message={t('live.plan.deleteMessage', { name: plan.name })}
      confirmLabel={t('live.plan.deleteTitle')}
      cancelLabel={t('common.cancel')}
      variant="danger"
    />
  </>;
}
