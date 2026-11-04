import {
  SKETCH_LIMITS,
  aimRotation,
  fovFromHeading,
  reorderShapeGroup,
  shapesBounds,
  shapesInMarquee,
  translateShapeGroup,
  canRemoveVertex,
  constrainAngle,
  insertShapeVertex,
  rectEdges,
  removeShapeVertex,
  reorderShape,
  resizeRectEdge,
  segmentMidpoints,
  wrapRotation,
  appendPolylinePoint,
  closesPolygon,
  createHistory,
  finishPolyline,
  normalizeShape,
  pushHistory,
  rectFromPoints,
  redoHistory,
  resizeRectCorner,
  sketchPayload,
  sketchShapesFromPlan,
  snapPoint,
  translateShape,
  undoHistory,
} from '../js/components/preact/live/planSketch.ts';

const canvas = { width: 1200, height: 800 };

describe('plan sketch geometry', () => {
  test('drops corrupt stored shapes and keeps valid ones', () => {
    const shapes = sketchShapesFromPlan({ sketch: { shapes: [
      { id: 'ok', type: 'rect', x: 0.1, y: 0.1, w: 0.2, h: 0.2, label: 'Lobby', tone: 'blue' },
      { id: 'bad id', type: 'rect', x: 0.1, y: 0.1, w: 0.2, h: 0.2 },
      { id: 'overflow', type: 'rect', x: 0.9, y: 0.1, w: 0.3, h: 0.2 },
      { id: 'wall', type: 'wall', points: [[0, 0], [1, 1]], tone: '#fff' },
      { id: 'short', type: 'area', points: [[0, 0], [1, 1]] },
      { id: 'label', type: 'label', x: 0.5, y: 0.5, text: '  North \n entrance ', size: 'huge' },
      { id: 'ok', type: 'wall', points: [[0, 0], [1, 1]] },
    ] } });
    expect(shapes.map((shape) => shape.id)).toEqual(['ok', 'wall', 'label']);
    expect(shapes[1].tone).toBe('slate');
    expect(shapes[2]).toMatchObject({ text: 'North entrance', size: 'md' });
    expect(sketchShapesFromPlan({ sketch: null })).toEqual([]);
  });

  test('payload is null when empty and rounds coordinates', () => {
    expect(sketchPayload([])).toBeNull();
    const payload = sketchPayload([{ id: 'a', type: 'wall', points: [[0.123456, 0.5], [1, 1]] }]);
    expect(payload).toEqual({ version: 1, shapes: [
      { id: 'a', type: 'wall', tone: 'slate', points: [[0.1235, 0.5], [1, 1]] },
    ] });
  });

  test('snaps to the 40 unit grid per axis and clamps to the plan', () => {
    expect(snapPoint({ x: 0.0301, y: 0.0499 }, canvas)).toEqual({ x: 0.0333, y: 0.05 });
    expect(snapPoint({ x: -0.2, y: 1.4 }, canvas)).toEqual({ x: 0, y: 1 });
    expect(snapPoint({ x: 0.0301, y: 0.0499 }, canvas, false)).toEqual({ x: 0.0301, y: 0.0499 });
  });

  test('translation keeps shapes inside the plan', () => {
    const rect = normalizeShape({ id: 'r', type: 'rect', x: 0.7, y: 0.7, w: 0.2, h: 0.2 });
    expect(translateShape(rect, 0.5, -1)).toMatchObject({ x: 0.8, y: 0 });
    const wall = normalizeShape({ id: 'w', type: 'wall', points: [[0.1, 0.1], [0.4, 0.1]] });
    expect(translateShape(wall, -0.5, 0).points).toEqual([[0, 0.1], [0.3, 0.1]]);
  });

  test('rect drawing enforces a minimum size and corner resizing keeps the anchor', () => {
    expect(rectFromPoints({ x: 0.5, y: 0.5 }, { x: 0.501, y: 0.9 })).toBeNull();
    expect(rectFromPoints({ x: 0.6, y: 0.6 }, { x: 0.2, y: 0.3 })).toEqual({ x: 0.2, y: 0.3, w: 0.4, h: 0.3 });
    const rect = normalizeShape({ id: 'r', type: 'rect', x: 0.2, y: 0.2, w: 0.2, h: 0.2 });
    expect(resizeRectCorner(rect, 'se', { x: 0.6, y: 0.5 })).toMatchObject({ x: 0.2, y: 0.2, w: 0.4, h: 0.3 });
    expect(resizeRectCorner(rect, 'nw', { x: 0.1, y: 0.1 })).toMatchObject({ x: 0.1, y: 0.1, w: 0.3, h: 0.3 });
  });

  test('polylines collapse double-click duplicates and require enough points', () => {
    let points = appendPolylinePoint([], { x: 0, y: 0 });
    points = appendPolylinePoint(points, { x: 0.5, y: 0 });
    points = appendPolylinePoint(points, { x: 0.5, y: 0 });
    expect(points).toHaveLength(2);
    expect(finishPolyline('area', points, 'a')).toBeNull();
    expect(finishPolyline('wall', [...points, [0.5, 0]], 'w', 'green')).toMatchObject({
      type: 'wall', tone: 'green', points: [[0, 0], [0.5, 0]],
    });
    expect(closesPolygon([[0.5, 0.5], [0.7, 0.5], [0.7, 0.7]], { x: 0.505, y: 0.505 }, canvas)).toBe(true);
    expect(closesPolygon([[0.5, 0.5], [0.7, 0.5], [0.7, 0.7]], { x: 0.55, y: 0.5 }, canvas)).toBe(false);
    expect(closesPolygon([[0.5, 0.5], [0.7, 0.5]], { x: 0.5, y: 0.5 }, canvas)).toBe(false);
  });

  test('history supports undo and redo with a bounded depth', () => {
    let history = createHistory([]);
    history = pushHistory(history, ['a']);
    history = pushHistory(history, ['a', 'b']);
    expect(undoHistory(history).present).toEqual(['a']);
    expect(redoHistory(undoHistory(history)).present).toEqual(['a', 'b']);
    expect(pushHistory(undoHistory(history), ['c']).future).toEqual([]);
    for (let index = 0; index < SKETCH_LIMITS.historyDepth + 20; index++) {
      history = pushHistory(history, [String(index)]);
    }
    expect(history.past.length).toBe(SKETCH_LIMITS.historyDepth);
  });

  test('history coalesces repeated edits with the same key inside the window', () => {
    let history = createHistory(['a']);
    history = pushHistory(history, ['b'], 'shape:1', 1000);
    history = pushHistory(history, ['c'], 'shape:1', 1500);
    expect(history.past).toEqual([['a']]);
    expect(history.present).toEqual(['c']);
    history = pushHistory(history, ['d'], 'shape:1', 5000);
    expect(history.past).toEqual([['a'], ['c']]);
    history = pushHistory(history, ['e'], 'shape:2', 5100);
    expect(history.past.length).toBe(3);
    // Undo breaks the coalescing chain so the next edit is its own step.
    history = pushHistory(undoHistory(history), ['f'], 'shape:2', 5200);
    expect(history.past.length).toBe(3);
    expect(undoHistory(history).present).toEqual(['d']);
  });

  test('rect edges resize one side and keep the opposite side fixed', () => {
    const rect = { id: 'r', type: 'rect', x: 0.2, y: 0.2, w: 0.4, h: 0.3 };
    expect(resizeRectEdge(rect, 'e', { x: 0.8, y: 0.9 })).toMatchObject({ x: 0.2, y: 0.2, w: 0.6, h: 0.3 });
    expect(resizeRectEdge(rect, 'n', { x: 0, y: 0.1 })).toMatchObject({ x: 0.2, y: 0.1, w: 0.4, h: 0.4 });
    // Dragging past the opposite edge flips instead of collapsing.
    expect(resizeRectEdge(rect, 'w', { x: 0.7, y: 0 })).toMatchObject({ x: 0.6, w: 0.1 });
    expect(resizeRectEdge(rect, 's', { x: 0, y: 0.2 })).toBe(rect);
    expect(rectEdges(rect).map((edge) => edge.edge)).toEqual(['n', 's', 'w', 'e']);
  });

  test('segment midpoints insert vertices and vertices respect minimum counts', () => {
    const wall = { id: 'w', type: 'wall', points: [[0, 0], [0.5, 0]] };
    const area = { id: 'a', type: 'area', points: [[0, 0], [0.5, 0], [0.5, 0.5]] };
    expect(segmentMidpoints(wall)).toEqual([{ index: 0, x: 0.25, y: 0 }]);
    expect(segmentMidpoints(area)).toHaveLength(3);
    expect(insertShapeVertex(wall, 0, { x: 0.25, y: 0.1 }).points)
      .toEqual([[0, 0], [0.25, 0.1], [0.5, 0]]);
    expect(canRemoveVertex(wall)).toBe(false);
    expect(removeShapeVertex(wall, 0)).toBe(wall);
    const square = insertShapeVertex(area, 2, { x: 0, y: 0.5 });
    expect(removeShapeVertex(square, 1).points).toEqual([[0, 0], [0.5, 0.5], [0, 0.5]]);
  });

  test('reorder moves shapes through the draw order', () => {
    const shapes = [{ id: 'a' }, { id: 'b' }, { id: 'c' }];
    const ids = (list) => list.map((shape) => shape.id).join('');
    expect(ids(reorderShape(shapes, 'a', 'front'))).toBe('bca');
    expect(ids(reorderShape(shapes, 'c', 'back'))).toBe('cab');
    expect(ids(reorderShape(shapes, 'a', 'forward'))).toBe('bac');
    expect(reorderShape(shapes, 'a', 'backward')).toBe(shapes);
    expect(reorderShape(shapes, 'missing', 'front')).toBe(shapes);
  });

  test('angle constraint locks segments to 45 degree steps in canvas space', () => {
    const origin = { x: 0.5, y: 0.5 };
    expect(constrainAngle(origin, { x: 0.7, y: 0.51 }, canvas)).toEqual({ x: 0.7, y: 0.5 });
    const diagonal = constrainAngle(origin, { x: 0.6, y: 0.62 }, canvas);
    expect(Math.abs((diagonal.x - 0.5) * canvas.width - (diagonal.y - 0.5) * canvas.height))
      .toBeLessThan(1);
  });

  test('camera aim follows the pointer with optional snapping and wraps rotations', () => {
    const camera = { x: 0.5, y: 0.5 };
    expect(aimRotation(camera, { x: 0.9, y: 0.5 }, canvas)).toBe(0);
    expect(aimRotation(camera, { x: 0.5, y: 0.9 }, canvas)).toBe(90);
    expect(aimRotation(camera, { x: 0.1, y: 0.5 }, canvas)).toBe(180);
    expect(aimRotation(camera, { x: 0.5, y: 0.1 }, canvas)).toBe(-90);
    expect(aimRotation(camera, { x: 0.7, y: 0.52 }, canvas, 45)).toBe(0);
    expect(aimRotation(camera, camera, canvas)).toBeNull();
    expect(wrapRotation(170, 15)).toBe(-175);
    expect(wrapRotation(-170, -15)).toBe(175);
    expect(wrapRotation(0, 180)).toBe(180);
  });

  test('marquee selects only fully enclosed shapes and groups move as one', () => {
    const shapes = [
      { id: 'a', type: 'rect', x: 0.1, y: 0.1, w: 0.1, h: 0.1 },
      { id: 'b', type: 'rect', x: 0.3, y: 0.1, w: 0.1, h: 0.1 },
      { id: 'c', type: 'wall', points: [[0.1, 0.5], [0.9, 0.5]] },
    ];
    expect(shapesInMarquee(shapes, { x: 0.45, y: 0.25 }, { x: 0.05, y: 0.05 })).toEqual(['a', 'b']);
    expect(shapesInMarquee(shapes, { x: 0, y: 0 }, { x: 0.25, y: 0.6 })).toEqual(['a']);
    expect(shapesBounds(shapes.slice(0, 2))).toEqual({ x0: 0.1, y0: 0.1, x1: 0.4, y1: 0.2 });
    // The group clamps at the plan edge together, keeping its spacing.
    const moved = translateShapeGroup(shapes, ['a', 'b'], -0.5, 0.05);
    expect(moved[0]).toMatchObject({ x: 0, y: 0.15 });
    expect(moved[1]).toMatchObject({ x: 0.2, y: 0.15 });
    expect(moved[2]).toBe(shapes[2]);
    expect(translateShapeGroup(shapes, [], 0.1, 0.1)).toBe(shapes);
  });

  test('group reorder keeps relative order', () => {
    const shapes = ['a', 'b', 'c', 'd'].map((id) => ({ id }));
    const ids = (list) => list.map((shape) => shape.id).join('');
    expect(ids(reorderShapeGroup(shapes, ['c', 'a'], 'front'))).toBe('bdac');
    expect(ids(reorderShapeGroup(shapes, ['d', 'b'], 'back'))).toBe('bdac');
    expect(reorderShapeGroup(shapes, ['zz'], 'front')).toBe(shapes);
  });

  test('field of view follows the wedge edge drag', () => {
    expect(fovFromHeading(0, 30)).toBe(60);
    expect(fovFromHeading(0, -30)).toBe(60);
    expect(fovFromHeading(170, -170)).toBe(40);
    expect(fovFromHeading(0, 120)).toBe(180);
    expect(fovFromHeading(0, 0)).toBe(1);
    expect(fovFromHeading(90, 113, 5)).toBe(45);
    expect(fovFromHeading(NaN, 10)).toBeNull();
  });
});
