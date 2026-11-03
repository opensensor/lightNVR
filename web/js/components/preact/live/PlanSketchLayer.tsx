// @ts-nocheck
import {
  rectCorners, rectEdges, rectFromPoints, segmentMidpoints, shapeCenter, shapeSummary,
} from './planSketch.ts';

const LABEL_FONT_SIZES = { sm: 14, md: 20, lg: 28 };

function toPointsAttribute(points, width, height) {
  return points.map(([x, y]) => `${x * width},${y * height}`).join(' ');
}

function ShapeLabel({ text, x, y, className = '' }) {
  if (!text) return null;
  return (
    <text x={x} y={y} textAnchor="middle" dominantBaseline="middle"
      className={`live-plan-shape-label ${className}`}>{text}</text>
  );
}

function ShapeHandles({
  shape, width, height, markerScale, onHandlePointerDown, onContextMenu,
}) {
  const radius = 7 * markerScale;
  const handles = [];
  if (shape.type === 'rect') {
    for (const edge of rectEdges(shape)) {
      handles.push({
        key: `e${edge.edge}`, x: edge.x, y: edge.y, edge: true,
        className: `is-edge is-${edge.edge}`,
        handle: { kind: 'edge', edge: edge.edge },
      });
    }
    for (const corner of rectCorners(shape)) {
      handles.push({
        key: corner.corner, x: corner.x, y: corner.y,
        className: `is-corner is-${corner.corner}`,
        handle: { kind: 'corner', corner: corner.corner },
      });
    }
  } else if (shape.type !== 'label') {
    for (const middle of segmentMidpoints(shape)) {
      handles.push({
        key: `m${middle.index}`, x: middle.x, y: middle.y, insert: true,
        className: 'is-insert',
        handle: { kind: 'insert', index: middle.index },
      });
    }
    (shape.points || []).forEach(([x, y], index) => handles.push({
      key: `v${index}`, x, y, className: 'is-vertex',
      handle: { kind: 'vertex', index },
    }));
  }
  return handles.map(({ key, x, y, handle, className, insert, edge }) => {
    const size = insert || edge ? radius * 0.75 : radius;
    const shared = {
      className: `live-plan-shape-handle ${className}`,
      onPointerDown: (event) => {
        // The handler stops propagation only when it takes the gesture, so
        // drawing tools and pans still reach the canvas.
        onHandlePointerDown(event, shape.id, handle);
      },
      onContextMenu: (event) => onContextMenu(event, shape.id, handle),
    };
    if (edge) {
      return (
        <rect key={key} x={x * width - size} y={y * height - size}
          width={size * 2} height={size * 2} rx={size * 0.3} {...shared} />
      );
    }
    return <circle key={key} cx={x * width} cy={y * height} r={size} {...shared} />;
  });
}

/**
 * Invisible thick strokes along each room side so an edge can be grabbed
 * and resized directly, without selecting the room first.
 */
function RectEdgeZones({ shape, width, height, markerScale, onHandlePointerDown }) {
  const x0 = shape.x * width;
  const y0 = shape.y * height;
  const x1 = (shape.x + shape.w) * width;
  const y1 = (shape.y + shape.h) * height;
  const edges = [
    { edge: 'n', x1: x0, y1: y0, x2: x1, y2: y0 },
    { edge: 's', x1: x0, y1, x2: x1, y2: y1 },
    { edge: 'w', x1: x0, y1: y0, x2: x0, y2: y1 },
    { edge: 'e', x1, y1: y0, x2: x1, y2: y1 },
  ];
  return edges.map((edge) => (
    <line key={edge.edge} x1={edge.x1} y1={edge.y1} x2={edge.x2} y2={edge.y2}
      strokeWidth={14 * markerScale}
      className={`live-plan-shape-edge is-${edge.edge}`}
      onPointerDown={(event) => {
        onHandlePointerDown(event, shape.id, { kind: 'edge', edge: edge.edge });
      }} />
  ));
}

function SketchShape({
  shape, width, height, editing, selected, showHandles, markerScale, t,
  onPointerDown, onClick, onDblClick, onContextMenu,
}) {
  const interactive = editing;
  const groupProps = interactive ? {
    // Lowercase: SVG attribute names are case-sensitive and `tabIndex` is ignored.
    tabindex: 0,
    role: 'button',
    'aria-label': shapeSummary(shape, t),
    'aria-pressed': selected ? 'true' : 'false',
    onPointerDown: (event) => onPointerDown(event, shape.id, { kind: 'move' }),
    onClick: (event) => onClick(event, shape.id),
    onDblClick: (event) => onDblClick(event, shape.id),
    onContextMenu: (event) => onContextMenu(event, shape.id, null),
  } : {};
  let body = null;
  if (shape.type === 'rect') {
    const x = shape.x * width;
    const y = shape.y * height;
    const w = shape.w * width;
    const h = shape.h * height;
    body = <>
      <rect x={x} y={y} width={w} height={h} rx="4"
        className={`live-plan-shape-rect ${shape.filled ? 'is-filled' : ''}`} />
      <ShapeLabel text={shape.label} x={x + w / 2} y={y + h / 2} />
      {editing && (
        <RectEdgeZones shape={shape} width={width} height={height}
          markerScale={markerScale} onHandlePointerDown={onPointerDown} />
      )}
    </>;
  } else if (shape.type === 'wall') {
    const points = toPointsAttribute(shape.points, width, height);
    body = <>
      <polyline points={points} className="live-plan-shape-hit" />
      <polyline points={points} className="live-plan-shape-wall" />
    </>;
  } else if (shape.type === 'area') {
    const center = shapeCenter(shape);
    body = <>
      <polygon points={toPointsAttribute(shape.points, width, height)}
        className="live-plan-shape-area" />
      <ShapeLabel text={shape.label} x={center.x * width} y={center.y * height} />
    </>;
  } else if (shape.type === 'label') {
    body = (
      <text x={shape.x * width} y={shape.y * height} textAnchor="middle"
        dominantBaseline="middle"
        className={`live-plan-shape-text is-size-${shape.size}`}
        fontSize={LABEL_FONT_SIZES[shape.size] || LABEL_FONT_SIZES.md}>
        {shape.text}
      </text>
    );
  }
  return (
    <g className={`live-plan-shape is-${shape.type} tone-${shape.tone} ${selected ? 'is-selected' : ''}`}
      {...groupProps}>
      {body}
      {editing && selected && showHandles && (
        <ShapeHandles shape={shape} width={width} height={height}
          markerScale={markerScale} onHandlePointerDown={onPointerDown}
          onContextMenu={onContextMenu} />
      )}
    </g>
  );
}

function DrawingPreview({ drawing, width, height, markerScale, closable }) {
  if (!drawing) return null;
  if (drawing.tool === 'room') {
    const rect = rectFromPoints(drawing.start, drawing.current);
    if (!rect) return null;
    return (
      <rect x={rect.x * width} y={rect.y * height}
        width={rect.w * width} height={rect.h * height} rx="4"
        className="live-plan-drawing-rect" />
    );
  }
  const committed = drawing.points || [];
  if (committed.length === 0) return null;
  const preview = drawing.cursor ? [...committed, [drawing.cursor.x, drawing.cursor.y]] : committed;
  const radius = 6 * markerScale;
  return (
    <g className={`live-plan-drawing is-${drawing.tool}`}>
      {drawing.tool === 'area' && committed.length >= 3 && (
        <polygon points={toPointsAttribute(preview, width, height)}
          className="live-plan-drawing-area" />
      )}
      <polyline points={toPointsAttribute(preview, width, height)}
        className="live-plan-drawing-line" />
      {committed.map(([x, y], index) => (
        <circle key={index} cx={x * width} cy={y * height}
          r={index === 0 && closable ? radius * 1.6 : radius}
          className={`live-plan-drawing-point ${index === 0 && closable ? 'is-closable' : ''}`} />
      ))}
    </g>
  );
}

/**
 * Renders a plan's layout sketch beneath the camera markers. In view mode
 * shapes are inert so pans and marker clicks pass straight through; in edit
 * mode they become focusable, draggable targets: room edges resize
 * directly, and the selected shape shows corner/edge or vertex handles plus
 * segment midpoints that insert a new vertex when dragged.
 */
export function PlanSketchLayer({
  shapes,
  width,
  height,
  editing,
  selectedShapeIds,
  markerScale,
  drawing,
  drawingClosable,
  t,
  onShapePointerDown,
  onShapeClick,
  onShapeDblClick,
  onShapeContextMenu,
}) {
  return (
    <g className={`live-plan-sketch ${editing ? 'is-editing' : ''}`}>
      {(shapes || []).map((shape) => (
        <SketchShape key={shape.id} shape={shape} width={width} height={height}
          editing={editing} selected={Boolean(selectedShapeIds?.has(shape.id))}
          showHandles={selectedShapeIds?.size === 1}
          markerScale={markerScale} t={t}
          onPointerDown={onShapePointerDown} onClick={onShapeClick}
          onDblClick={onShapeDblClick} onContextMenu={onShapeContextMenu} />
      ))}
      <DrawingPreview drawing={drawing} width={width} height={height}
        markerScale={markerScale} closable={drawingClosable} />
    </g>
  );
}
