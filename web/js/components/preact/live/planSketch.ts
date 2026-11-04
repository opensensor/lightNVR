/**
 * Pure helpers for the building-plan layout sketch: the vector rooms, walls,
 * areas and labels an operator draws on a plan instead of (or on top of) an
 * uploaded image. Everything here is DOM-free so Jest can cover it.
 *
 * Coordinates are plan fractions (0..1) like camera placements, so a sketch
 * survives canvas resizes and background swaps. The API re-validates and
 * canonicalizes every shape; these limits mirror its bounds.
 */

import type {
  PlanPoint,
  PlanRect,
  RawSketchShape,
  RectCorner,
  RectEdge,
  ShapeOrderDirection,
  SketchAreaShape,
  SketchBounds,
  SketchCanvas,
  SketchHistory,
  SketchLabelSize,
  SketchPoint,
  SketchRectShape,
  SketchShape,
  SketchTone,
  SketchWallShape,
  Translate,
} from '../../../types/live.ts';

type SketchPolylineShape = SketchWallShape | SketchAreaShape;

export const SKETCH_VERSION = 1;
export const SKETCH_TONES = Object.freeze([
  'slate', 'blue', 'green', 'amber', 'red', 'violet',
] as const);
export const SKETCH_LABEL_SIZES = Object.freeze(['sm', 'md', 'lg'] as const);
export const SKETCH_SHAPE_TYPES = Object.freeze(['rect', 'wall', 'area', 'label'] as const);
export const SKETCH_TOOLS = Object.freeze(['select', 'room', 'wall', 'area', 'label'] as const);
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

function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

function round4(value: number): number {
  return Math.round(value * 10000) / 10000;
}

function finiteUnit(value: unknown): value is number {
  return Number.isFinite(value) && (value as number) >= 0 && (value as number) <= 1;
}

function cleanText(value: unknown): string {
  if (typeof value !== 'string') return '';
  const trimmed = value.replace(/\s+/g, ' ').trim();
  if (!trimmed || CONTROL_CHARS.test(trimmed)) return '';
  return trimmed.slice(0, SKETCH_LIMITS.textMax);
}

export function createShapeId(existingIds: Iterable<string> = []): string {
  const taken = new Set(existingIds);
  for (let attempt = 0; attempt < 32; attempt++) {
    const candidate = `s${Date.now().toString(36)}${Math.floor(Math.random() * 46656)
      .toString(36).padStart(3, '0')}`;
    if (!taken.has(candidate)) return candidate;
  }
  return `s${Date.now().toString(36)}${taken.size}`;
}

function normalizePoints(points: unknown, minimum: number): SketchPoint[] | null {
  if (!Array.isArray(points) || points.length < minimum ||
    points.length > SKETCH_LIMITS.maxPoints) return null;
  const normalized: SketchPoint[] = [];
  for (const point of points) {
    const x: unknown = Array.isArray(point) ? point[0] : point?.x;
    const y: unknown = Array.isArray(point) ? point[1] : point?.y;
    if (!finiteUnit(x) || !finiteUnit(y)) return null;
    normalized.push([round4(x), round4(y)]);
  }
  return normalized;
}

/**
 * Validate one stored or drafted shape. Returns a clean copy with only the
 * known fields, or null when the shape cannot be rendered safely.
 */
export function normalizeShape(raw?: RawSketchShape | null): SketchShape | null {
  if (!raw || typeof raw !== 'object') return null;
  if (typeof raw.id !== 'string' || !ID_PATTERN.test(raw.id)) return null;
  const tone = SKETCH_TONES.includes(raw.tone as SketchTone) ? raw.tone as SketchTone : SKETCH_TONES[0];
  switch (raw.type) {
    case 'rect': {
      const { x, y, w, h } = raw as Record<'x' | 'y' | 'w' | 'h', number>;
      if (![x, y, w, h].every(Number.isFinite) || !finiteUnit(x) || !finiteUnit(y) ||
        w < SKETCH_LIMITS.minRectSide || h < SKETCH_LIMITS.minRectSide ||
        x + w > 1 + 1e-6 || y + h > 1 + 1e-6) return null;
      const shape: SketchRectShape = {
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
      const shape: SketchAreaShape = { id: raw.id, type: 'area', tone, points };
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
        size: SKETCH_LABEL_SIZES.includes(raw.size as SketchLabelSize) ? raw.size as SketchLabelSize : 'md',
      };
    }
    default:
      return null;
  }
}

/** Shapes from an API plan object; corrupt entries are dropped, not fatal. */
export function sketchShapesFromPlan(plan?: { sketch?: { shapes?: RawSketchShape[] } | null } | null): SketchShape[] {
  const shapes = plan?.sketch?.shapes;
  if (!Array.isArray(shapes)) return [];
  const seen = new Set<string>();
  const result: SketchShape[] = [];
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
export function sketchPayload(shapes?: readonly RawSketchShape[] | null) {
  const clean = ((shapes || []).map(normalizeShape).filter(Boolean) as SketchShape[])
    .slice(0, SKETCH_LIMITS.maxShapes);
  if (clean.length === 0) return null;
  return { version: SKETCH_VERSION, shapes: clean };
}

/**
 * Snap a plan-fraction point to the drawn grid. The grid is square in canvas
 * units, so the fraction step differs per axis on non-square canvases.
 */
export function snapPoint(point: PlanPoint, canvas?: SketchCanvas | null, enabled: boolean = true): PlanPoint {
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

export function gridStep(canvas?: SketchCanvas | null): PlanPoint {
  return {
    x: canvas?.width ? SKETCH_GRID / canvas.width : 0.02,
    y: canvas?.height ? SKETCH_GRID / canvas.height : 0.02,
  };
}

export function shapeBounds(shape: SketchShape): SketchBounds {
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

export function shapeCenter(shape: SketchShape): PlanPoint {
  const bounds = shapeBounds(shape);
  return { x: (bounds.x0 + bounds.x1) / 2, y: (bounds.y0 + bounds.y1) / 2 };
}

/** Move a shape, clamping so it stays entirely inside the plan. */
export function translateShape(shape: SketchShape, dx: number, dy: number): SketchShape {
  const bounds = shapeBounds(shape);
  const shiftX = Math.max(-bounds.x0, Math.min(1 - bounds.x1, dx));
  const shiftY = Math.max(-bounds.y0, Math.min(1 - bounds.y1, dy));
  if (shape.type === 'rect' || shape.type === 'label') {
    return { ...shape, x: round4(shape.x + shiftX), y: round4(shape.y + shiftY) };
  }
  return {
    ...shape,
    points: shape.points.map(([x, y]): SketchPoint => [round4(x + shiftX), round4(y + shiftY)]),
  };
}

export function moveShapeVertex(shape: SketchShape, index: number, point: PlanPoint): SketchShape {
  if (!Array.isArray(shape.points) || index < 0 || index >= shape.points.length) return shape;
  const points = shape.points.slice();
  points[index] = [round4(clamp01(point.x)), round4(clamp01(point.y))];
  return { ...shape as SketchPolylineShape, points };
}

/** Build a rectangle from two drag corners; null when it is too small. */
export function rectFromPoints(a: PlanPoint, b: PlanPoint): PlanRect | null {
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
export function resizeRectCorner(shape: SketchShape, corner: RectCorner, point: PlanPoint): SketchShape {
  if (shape.type !== 'rect' || !CORNERS.includes(corner)) return shape;
  const anchor = {
    x: corner.includes('w') ? shape.x + shape.w : shape.x,
    y: corner.includes('n') ? shape.y + shape.h : shape.y,
  };
  const rect = rectFromPoints(anchor, point);
  return rect ? { ...shape, ...rect } : shape;
}

export function rectCorners(shape: SketchRectShape): (PlanPoint & { corner: RectCorner })[] {
  return [
    { corner: 'nw', x: shape.x, y: shape.y },
    { corner: 'ne', x: shape.x + shape.w, y: shape.y },
    { corner: 'sw', x: shape.x, y: shape.y + shape.h },
    { corner: 'se', x: shape.x + shape.w, y: shape.y + shape.h },
  ];
}

const EDGES = Object.freeze(['n', 's', 'e', 'w']);

/** Drag one rect edge; the opposite edge and the other axis stay fixed. */
export function resizeRectEdge(shape: SketchShape, edge: RectEdge, point: PlanPoint): SketchShape {
  if (shape.type !== 'rect' || !EDGES.includes(edge)) return shape;
  const x0 = shape.x;
  const y0 = shape.y;
  const x1 = shape.x + shape.w;
  const y1 = shape.y + shape.h;
  const a = { x: x0, y: y0 };
  const b = { x: x1, y: y1 };
  if (edge === 'n') a.y = point.y;
  if (edge === 's') b.y = point.y;
  if (edge === 'w') a.x = point.x;
  if (edge === 'e') b.x = point.x;
  const rect = rectFromPoints(a, b);
  return rect ? { ...shape, ...rect } : shape;
}

/** Edge handles at the midpoint of each rect side. */
export function rectEdges(shape: SketchRectShape): (PlanPoint & { edge: RectEdge })[] {
  return [
    { edge: 'n', x: shape.x + shape.w / 2, y: shape.y },
    { edge: 's', x: shape.x + shape.w / 2, y: shape.y + shape.h },
    { edge: 'w', x: shape.x, y: shape.y + shape.h / 2 },
    { edge: 'e', x: shape.x + shape.w, y: shape.y + shape.h / 2 },
  ];
}

/**
 * Midpoints of each polyline segment (closing segment included for areas);
 * dragging one inserts a new vertex after `index`.
 */
export function segmentMidpoints(shape?: SketchShape | null): (PlanPoint & { index: number })[] {
  if (!Array.isArray(shape?.points) || shape.points.length >= SKETCH_LIMITS.maxPoints) return [];
  const points = shape.points;
  const count = shape.type === 'area' ? points.length : points.length - 1;
  const result: (PlanPoint & { index: number })[] = [];
  for (let index = 0; index < count; index++) {
    const [ax, ay] = points[index]!;
    const [bx, by] = points[(index + 1) % points.length]!;
    result.push({ index, x: (ax + bx) / 2, y: (ay + by) / 2 });
  }
  return result;
}

export function insertShapeVertex(shape: SketchShape, afterIndex: number, point: PlanPoint): SketchShape {
  if (!Array.isArray(shape.points) || shape.points.length >= SKETCH_LIMITS.maxPoints ||
    afterIndex < 0 || afterIndex >= shape.points.length) return shape;
  const points = shape.points.slice();
  points.splice(afterIndex + 1, 0, [round4(clamp01(point.x)), round4(clamp01(point.y))]);
  return { ...shape as SketchPolylineShape, points };
}

export function canRemoveVertex(shape?: SketchShape | null): boolean {
  if (!Array.isArray(shape?.points)) return false;
  return shape.points.length > (shape.type === 'area' ? 3 : 2);
}

export function removeShapeVertex(shape: SketchShape, index: number): SketchShape {
  if (!canRemoveVertex(shape) || index < 0 || index >= shape.points!.length) return shape;
  return { ...shape as SketchPolylineShape, points: shape.points!.filter((_, candidate) => candidate !== index) };
}

/** Move a shape within the draw order: 'front', 'back', 'forward', 'backward'. */
export function reorderShape(shapes: SketchShape[], id: string, direction: ShapeOrderDirection): SketchShape[] {
  const index = shapes.findIndex((shape) => shape.id === id);
  if (index < 0) return shapes;
  const next = shapes.slice();
  const [shape] = next.splice(index, 1);
  let target = index;
  if (direction === 'front') target = next.length;
  else if (direction === 'back') target = 0;
  else if (direction === 'forward') target = Math.min(next.length, index + 1);
  else if (direction === 'backward') target = Math.max(0, index - 1);
  if (target === index) return shapes;
  next.splice(target, 0, shape!);
  return next;
}

/**
 * Constrain `point` so the segment from `origin` runs at a multiple of 45°
 * in canvas space (the canvas is not square in plan fractions).
 */
export function constrainAngle(origin: PlanPoint | null | undefined, point: PlanPoint, canvas?: SketchCanvas | null): PlanPoint {
  if (!origin || !canvas?.width || !canvas?.height) return point;
  const dx = (point.x - origin.x) * canvas.width;
  const dy = (point.y - origin.y) * canvas.height;
  const length = Math.hypot(dx, dy);
  if (!length) return point;
  const step = Math.PI / 4;
  const angle = Math.round(Math.atan2(dy, dx) / step) * step;
  // Project onto the snapped direction so the cursor's travel along it is kept.
  const along = dx * Math.cos(angle) + dy * Math.sin(angle);
  return {
    x: round4(clamp01(origin.x + (Math.cos(angle) * along) / canvas.width)),
    y: round4(clamp01(origin.y + (Math.sin(angle) * along) / canvas.height)),
  };
}

/**
 * Camera heading (degrees, -180..180, 0 = east, clockwise positive because
 * SVG y grows downward) from a placement toward a plan point. Returns null
 * when the point is on top of the camera. `snapDegrees` rounds the heading.
 */
export function aimRotation(from: PlanPoint | null | undefined, toward: PlanPoint | null | undefined, canvas?: SketchCanvas | null, snapDegrees: number = 0): number | null {
  if (!from || !toward || !canvas?.width || !canvas?.height) return null;
  const dx = (toward.x - from.x) * canvas.width;
  const dy = (toward.y - from.y) * canvas.height;
  if (Math.hypot(dx, dy) < 1) return null;
  let degrees = (Math.atan2(dy, dx) * 180) / Math.PI;
  if (snapDegrees > 0) degrees = Math.round(degrees / snapDegrees) * snapDegrees;
  if (degrees > 180) degrees -= 360;
  if (degrees <= -180) degrees += 360;
  return Math.round(degrees * 10) / 10;
}

/** Rotate a heading by `delta` degrees, wrapping into -180..180. */
export function wrapRotation(rotation: number, delta: number): number {
  let next = (Number.isFinite(rotation) ? rotation : 0) + delta;
  while (next > 180) next -= 360;
  while (next <= -180) next += 360;
  return Math.round(next * 10) / 10;
}

/** Union bounds of several shapes, or null for an empty list. */
export function shapesBounds(shapes?: SketchShape[] | null): SketchBounds | null {
  if (!shapes || shapes.length === 0) return null;
  return shapes.map(shapeBounds).reduce((acc, bounds) => ({
    x0: Math.min(acc.x0, bounds.x0), y0: Math.min(acc.y0, bounds.y0),
    x1: Math.max(acc.x1, bounds.x1), y1: Math.max(acc.y1, bounds.y1),
  }));
}

/** Ids of shapes lying entirely inside a marquee rectangle (plan fractions). */
export function shapesInMarquee(shapes: SketchShape[] | null | undefined, a: PlanPoint, b: PlanPoint): string[] {
  const x0 = Math.min(a.x, b.x);
  const y0 = Math.min(a.y, b.y);
  const x1 = Math.max(a.x, b.x);
  const y1 = Math.max(a.y, b.y);
  return (shapes || []).filter((shape) => {
    const bounds = shapeBounds(shape);
    return bounds.x0 >= x0 && bounds.y0 >= y0 && bounds.x1 <= x1 && bounds.y1 <= y1;
  }).map((shape) => shape.id);
}

/**
 * Move several shapes by one shared delta, clamped so the whole group stays
 * on the plan (shapes keep their relative positions at the edges).
 */
export function translateShapeGroup(shapes: SketchShape[], ids: string[], dx: number, dy: number): SketchShape[] {
  const selected = new Set(ids);
  const bounds = shapesBounds(shapes.filter((shape) => selected.has(shape.id)));
  if (!bounds) return shapes;
  const shiftX = Math.max(-bounds.x0, Math.min(1 - bounds.x1, dx));
  const shiftY = Math.max(-bounds.y0, Math.min(1 - bounds.y1, dy));
  if (!shiftX && !shiftY) return shapes;
  return shapes.map((shape) => selected.has(shape.id)
    ? translateShape(shape, shiftX, shiftY) : shape);
}

/** Bring several shapes to the front or back, keeping their relative order. */
export function reorderShapeGroup(shapes: SketchShape[], ids: string[], direction: ShapeOrderDirection): SketchShape[] {
  const selected = new Set(ids);
  const picked = shapes.filter((shape) => selected.has(shape.id));
  if (picked.length === 0) return shapes;
  const rest = shapes.filter((shape) => !selected.has(shape.id));
  return direction === 'back' ? [...picked, ...rest] : [...rest, ...picked];
}

/**
 * Field of view (1..180°) from dragging a wedge edge: twice the angle
 * between the camera heading and the pointer heading.
 */
export function fovFromHeading(rotation: number, heading: number, snapDegrees: number = 0): number | null {
  if (!Number.isFinite(rotation) || !Number.isFinite(heading)) return null;
  const delta = Math.abs(wrapRotation(heading, -rotation));
  let fov = delta * 2;
  if (snapDegrees > 0) fov = Math.round(fov / snapDegrees) * snapDegrees;
  return Math.max(1, Math.min(180, Math.round(fov * 10) / 10));
}

/** True when a click lands close enough to the first point to close an area. */
export function closesPolygon(points: SketchPoint[] | null | undefined, point: PlanPoint, canvas?: SketchCanvas | null, toleranceUnits: number = 14): boolean {
  if (!points || points.length < 3 || !canvas?.width || !canvas?.height) return false;
  const [firstX, firstY] = points[0]!;
  const dx = (point.x - firstX) * canvas.width;
  const dy = (point.y - firstY) * canvas.height;
  return Math.hypot(dx, dy) <= toleranceUnits;
}

/**
 * Turn an in-progress polyline into a shape. Double-click finishing emits a
 * duplicate trailing point, so consecutive duplicates are collapsed first.
 */
export function finishPolyline(type: 'wall' | 'area', points: SketchPoint[] | null | undefined, id: string, tone: SketchTone = SKETCH_TONES[0]): SketchShape | null {
  const unique: SketchPoint[] = [];
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

export function appendPolylinePoint(points: SketchPoint[], point: PlanPoint): SketchPoint[] {
  const next: SketchPoint = [round4(clamp01(point.x)), round4(clamp01(point.y))];
  const previous = points[points.length - 1];
  if (previous && previous[0] === next[0] && previous[1] === next[1]) return points;
  if (points.length >= SKETCH_LIMITS.maxPoints) return points;
  return [...points, next];
}

/** Human summary for shape lists and accessible names. */
export function shapeSummary(shape: SketchShape, t: Translate = (key) => key): string {
  const kind = t(`live.plan.sketch.type.${shape.type}`);
  if (shape.type === 'label') return `${kind}: ${shape.text}`;
  if (shape.label) return `${kind}: ${shape.label}`;
  if (shape.type === 'rect') {
    return `${kind} ${Math.round(shape.w * 100)}×${Math.round(shape.h * 100)}%`;
  }
  return `${kind} · ${t('live.plan.sketch.pointCount', { count: shape.points.length })}`;
}

export function duplicateShape(shape: SketchShape, existingIds: Iterable<string>, canvas?: SketchCanvas | null): SketchShape {
  const step = gridStep(canvas);
  return translateShape({ ...shape, id: createShapeId(existingIds) }, step.x, step.y);
}

/* ---- undo / redo ------------------------------------------------------ */

export function createHistory<T>(present: T): SketchHistory<T> {
  return { past: [], present, future: [] };
}

/**
 * Record `next` as a new undo step. Consecutive pushes with the same
 * `coalesceKey` inside `windowMs` (typing a label, dragging a slider,
 * repeated arrow nudges) replace the present instead of stacking steps.
 */
export function pushHistory<T>(history: SketchHistory<T>, next: T, coalesceKey: string | null = null, now: number = Date.now(), windowMs: number = 1200): SketchHistory<T> {
  if (next === history.present) return history;
  if (coalesceKey && history.coalesceKey === coalesceKey &&
    now - history.coalesceAt! <= windowMs) {
    return { ...history, present: next, future: [], coalesceAt: now };
  }
  const past = [...history.past, history.present].slice(-SKETCH_LIMITS.historyDepth);
  return coalesceKey
    ? { past, present: next, future: [], coalesceKey, coalesceAt: now }
    : { past, present: next, future: [] };
}

export function undoHistory<T>(history: SketchHistory<T>): SketchHistory<T> {
  if (history.past.length === 0) return history;
  const past = history.past.slice(0, -1);
  const present = history.past[history.past.length - 1]!;
  return { past, present, future: [history.present, ...history.future] };
}

export function redoHistory<T>(history: SketchHistory<T>): SketchHistory<T> {
  if (history.future.length === 0) return history;
  const [present, ...future] = history.future as [T, ...T[]];
  return { past: [...history.past, history.present], present, future };
}
