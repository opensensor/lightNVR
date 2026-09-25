/**
 * Pure helpers for the building-plan layout sketch: the vector rooms, walls,
 * areas and labels an operator draws on a plan instead of (or on top of) an
 * uploaded image. Everything here is DOM-free so Jest can cover it.
 *
 * Coordinates are plan fractions (0..1) like camera placements, so a sketch
 * survives canvas resizes and background swaps. The API re-validates and
 * canonicalizes every shape; these limits mirror its bounds.
 */

export const SKETCH_VERSION = 1;
export const SKETCH_TONES = Object.freeze([
  'slate', 'blue', 'green', 'amber', 'red', 'violet',
]);
export const SKETCH_LABEL_SIZES = Object.freeze(['sm', 'md', 'lg']);
export const SKETCH_SHAPE_TYPES = Object.freeze(['rect', 'wall', 'area', 'label']);
export const SKETCH_TOOLS = Object.freeze(['select', 'room', 'wall', 'area', 'label']);
export const SKETCH_LIMITS = Object.freeze({
  maxShapes: 400,
  maxPoints: 64,
  minRectSide: 0.005,
  textMax: 96,
  idMax: 32,
  historyDepth: 100,
});
/** Canvas units between grid lines; matches the SVG grid pattern. */
export const SKETCH_GRID = 40;

const ID_PATTERN = /^[A-Za-z0-9_-]{1,32}$/;
const CONTROL_CHARS = /[\u0000-\u001f\u007f]/;

function clamp01(value) {
  return Math.max(0, Math.min(1, value));
}

function round4(value) {
  return Math.round(value * 10000) / 10000;
}

function finiteUnit(value) {
  return Number.isFinite(value) && value >= 0 && value <= 1;
}

function cleanText(value) {
  if (typeof value !== 'string') return '';
  const trimmed = value.replace(/\s+/g, ' ').trim();
  if (!trimmed || CONTROL_CHARS.test(trimmed)) return '';
  return trimmed.slice(0, SKETCH_LIMITS.textMax);
}

export function createShapeId(existingIds = []) {
  const taken = new Set(existingIds);
  for (let attempt = 0; attempt < 32; attempt++) {
    const candidate = `s${Date.now().toString(36)}${Math.floor(Math.random() * 46656)
      .toString(36).padStart(3, '0')}`;
    if (!taken.has(candidate)) return candidate;
  }
  return `s${Date.now().toString(36)}${taken.size}`;
}

function normalizePoints(points, minimum) {
  if (!Array.isArray(points) || points.length < minimum ||
    points.length > SKETCH_LIMITS.maxPoints) return null;
  const normalized = [];
  for (const point of points) {
    const x = Array.isArray(point) ? point[0] : point?.x;
    const y = Array.isArray(point) ? point[1] : point?.y;
    if (!finiteUnit(x) || !finiteUnit(y)) return null;
    normalized.push([round4(x), round4(y)]);
  }
  return normalized;
}

/**
 * Validate one stored or drafted shape. Returns a clean copy with only the
 * known fields, or null when the shape cannot be rendered safely.
 */
export function normalizeShape(raw) {
  if (!raw || typeof raw !== 'object') return null;
  if (typeof raw.id !== 'string' || !ID_PATTERN.test(raw.id)) return null;
  const tone = SKETCH_TONES.includes(raw.tone) ? raw.tone : SKETCH_TONES[0];
  switch (raw.type) {
    case 'rect': {
      const { x, y, w, h } = raw;
      if (![x, y, w, h].every(Number.isFinite) || !finiteUnit(x) || !finiteUnit(y) ||
        w < SKETCH_LIMITS.minRectSide || h < SKETCH_LIMITS.minRectSide ||
        x + w > 1 + 1e-6 || y + h > 1 + 1e-6) return null;
      const shape = {
        id: raw.id, type: 'rect', tone,
        x: round4(x), y: round4(y),
        w: round4(Math.min(w, 1 - x)), h: round4(Math.min(h, 1 - y)),
        filled: raw.filled !== false,
      };
      const label = cleanText(raw.label);
      if (label) shape.label = label;
      return shape;
    }
    case 'wall': {
      const points = normalizePoints(raw.points, 2);
      return points ? { id: raw.id, type: 'wall', tone, points } : null;
    }
    case 'area': {
      const points = normalizePoints(raw.points, 3);
      if (!points) return null;
      const shape = { id: raw.id, type: 'area', tone, points };
      const label = cleanText(raw.label);
      if (label) shape.label = label;
      return shape;
    }
    case 'label': {
      const text = cleanText(raw.text);
      if (!finiteUnit(raw.x) || !finiteUnit(raw.y) || !text) return null;
      return {
        id: raw.id, type: 'label', tone,
        x: round4(raw.x), y: round4(raw.y), text,
        size: SKETCH_LABEL_SIZES.includes(raw.size) ? raw.size : 'md',
      };
    }
    default:
      return null;
  }
}

/** Shapes from an API plan object; corrupt entries are dropped, not fatal. */
export function sketchShapesFromPlan(plan) {
  const shapes = plan?.sketch?.shapes;
  if (!Array.isArray(shapes)) return [];
  const seen = new Set();
  const result = [];
  for (const raw of shapes) {
    const shape = normalizeShape(raw);
    if (!shape || seen.has(shape.id)) continue;
    seen.add(shape.id);
    result.push(shape);
    if (result.length >= SKETCH_LIMITS.maxShapes) break;
  }
  return result;
}

/** API payload for a draft: null clears the stored sketch. */
export function sketchPayload(shapes) {
  const clean = (shapes || []).map(normalizeShape).filter(Boolean)
    .slice(0, SKETCH_LIMITS.maxShapes);
  if (clean.length === 0) return null;
  return { version: SKETCH_VERSION, shapes: clean };
}

/**
 * Snap a plan-fraction point to the drawn grid. The grid is square in canvas
 * units, so the fraction step differs per axis on non-square canvases.
 */
export function snapPoint(point, canvas, enabled = true) {
  const x = clamp01(point.x);
  const y = clamp01(point.y);
  if (!enabled || !canvas?.width || !canvas?.height) return { x: round4(x), y: round4(y) };
  const stepX = SKETCH_GRID / canvas.width;
  const stepY = SKETCH_GRID / canvas.height;
  return {
    x: round4(clamp01(Math.round(x / stepX) * stepX)),
    y: round4(clamp01(Math.round(y / stepY) * stepY)),
  };
}

export function gridStep(canvas) {
  return {
    x: canvas?.width ? SKETCH_GRID / canvas.width : 0.02,
    y: canvas?.height ? SKETCH_GRID / canvas.height : 0.02,
  };
}

export function shapeBounds(shape) {
  if (shape.type === 'rect') {
    return { x0: shape.x, y0: shape.y, x1: shape.x + shape.w, y1: shape.y + shape.h };
  }
  if (shape.type === 'label') {
    return { x0: shape.x, y0: shape.y, x1: shape.x, y1: shape.y };
  }
  const xs = shape.points.map((point) => point[0]);
  const ys = shape.points.map((point) => point[1]);
  return {
    x0: Math.min(...xs), y0: Math.min(...ys),
    x1: Math.max(...xs), y1: Math.max(...ys),
  };
}

export function shapeCenter(shape) {
  const bounds = shapeBounds(shape);
  return { x: (bounds.x0 + bounds.x1) / 2, y: (bounds.y0 + bounds.y1) / 2 };
}

/** Move a shape, clamping so it stays entirely inside the plan. */
export function translateShape(shape, dx, dy) {
  const bounds = shapeBounds(shape);
  const shiftX = Math.max(-bounds.x0, Math.min(1 - bounds.x1, dx));
  const shiftY = Math.max(-bounds.y0, Math.min(1 - bounds.y1, dy));
  if (shape.type === 'rect' || shape.type === 'label') {
    return { ...shape, x: round4(shape.x + shiftX), y: round4(shape.y + shiftY) };
  }
  return {
    ...shape,
    points: shape.points.map(([x, y]) => [round4(x + shiftX), round4(y + shiftY)]),
  };
}

export function moveShapeVertex(shape, index, point) {
  if (!Array.isArray(shape.points) || index < 0 || index >= shape.points.length) return shape;
  const points = shape.points.slice();
  points[index] = [round4(clamp01(point.x)), round4(clamp01(point.y))];
  return { ...shape, points };
}

/** Build a rectangle from two drag corners; null when it is too small. */
export function rectFromPoints(a, b) {
  const x0 = clamp01(Math.min(a.x, b.x));
  const y0 = clamp01(Math.min(a.y, b.y));
  const x1 = clamp01(Math.max(a.x, b.x));
  const y1 = clamp01(Math.max(a.y, b.y));
  const w = round4(x1 - x0);
  const h = round4(y1 - y0);
  if (w < SKETCH_LIMITS.minRectSide || h < SKETCH_LIMITS.minRectSide) return null;
  return { x: round4(x0), y: round4(y0), w, h };
}

const CORNERS = Object.freeze(['nw', 'ne', 'sw', 'se']);

/** Drag one rect corner; the opposite corner stays fixed. */
export function resizeRectCorner(shape, corner, point) {
  if (shape.type !== 'rect' || !CORNERS.includes(corner)) return shape;
  const anchor = {
    x: corner.includes('w') ? shape.x + shape.w : shape.x,
    y: corner.includes('n') ? shape.y + shape.h : shape.y,
  };
  const rect = rectFromPoints(anchor, point);
  return rect ? { ...shape, ...rect } : shape;
}

export function rectCorners(shape) {
  return [
    { corner: 'nw', x: shape.x, y: shape.y },
    { corner: 'ne', x: shape.x + shape.w, y: shape.y },
    { corner: 'sw', x: shape.x, y: shape.y + shape.h },
    { corner: 'se', x: shape.x + shape.w, y: shape.y + shape.h },
  ];
}

/** True when a click lands close enough to the first point to close an area. */
export function closesPolygon(points, point, canvas, toleranceUnits = 14) {
  if (!points || points.length < 3 || !canvas?.width || !canvas?.height) return false;
  const [firstX, firstY] = points[0];
  const dx = (point.x - firstX) * canvas.width;
  const dy = (point.y - firstY) * canvas.height;
  return Math.hypot(dx, dy) <= toleranceUnits;
}

/**
 * Turn an in-progress polyline into a shape. Double-click finishing emits a
 * duplicate trailing point, so consecutive duplicates are collapsed first.
 */
export function finishPolyline(type, points, id, tone = SKETCH_TONES[0]) {
  const unique = [];
  for (const point of points || []) {
    const previous = unique[unique.length - 1];
    if (previous && previous[0] === point[0] && previous[1] === point[1]) continue;
    unique.push([round4(point[0]), round4(point[1])]);
  }
  const minimum = type === 'area' ? 3 : 2;
  if (unique.length < minimum) return null;
  return normalizeShape({
    id, type, tone, points: unique.slice(0, SKETCH_LIMITS.maxPoints),
  });
}

export function appendPolylinePoint(points, point) {
  const next = [round4(clamp01(point.x)), round4(clamp01(point.y))];
  const previous = points[points.length - 1];
  if (previous && previous[0] === next[0] && previous[1] === next[1]) return points;
  if (points.length >= SKETCH_LIMITS.maxPoints) return points;
  return [...points, next];
}

/** Human summary for shape lists and accessible names. */
export function shapeSummary(shape, t = (key) => key) {
  const kind = t(`live.plan.sketch.type.${shape.type}`);
  if (shape.type === 'label') return `${kind}: ${shape.text}`;
  if (shape.label) return `${kind}: ${shape.label}`;
  if (shape.type === 'rect') {
    return `${kind} ${Math.round(shape.w * 100)}×${Math.round(shape.h * 100)}%`;
  }
  return `${kind} · ${t('live.plan.sketch.pointCount', { count: shape.points.length })}`;
}

export function duplicateShape(shape, existingIds, canvas) {
  const step = gridStep(canvas);
  return translateShape({ ...shape, id: createShapeId(existingIds) }, step.x, step.y);
}

/* ---- undo / redo ------------------------------------------------------ */

export function createHistory(present) {
  return { past: [], present, future: [] };
}

export function pushHistory(history, next) {
  if (next === history.present) return history;
  const past = [...history.past, history.present].slice(-SKETCH_LIMITS.historyDepth);
  return { past, present: next, future: [] };
}

export function undoHistory(history) {
  if (history.past.length === 0) return history;
  const past = history.past.slice(0, -1);
  const present = history.past[history.past.length - 1];
  return { past, present, future: [history.present, ...history.future] };
}

export function redoHistory(history) {
  if (history.future.length === 0) return history;
  const [present, ...future] = history.future;
  return { past: [...history.past, history.present], present, future };
}
