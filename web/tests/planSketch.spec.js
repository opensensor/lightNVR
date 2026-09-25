import {
  SKETCH_LIMITS,
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
} from '../js/components/preact/live/planSketch.js';

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
});
