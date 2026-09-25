import { rectCorners, rectFromPoints, shapeCenter, shapeSummary } from './planSketch.js';

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

function ShapeHandles({ shape, width, height, markerScale, onHandlePointerDown }) {
  const radius = 7 * markerScale;
  const handles = shape.type === 'rect'
    ? rectCorners(shape).map((corner) => ({
      key: corner.corner, x: corner.x, y: corner.y,
      handle: { kind: 'corner', corner: corner.corner },
    }))
    : (shape.points || []).map(([x, y], index) => ({
      key: `v${index}`, x, y, handle: { kind: 'vertex', index },
    }));
  return handles.map(({ key, x, y, handle }) => (
    <circle key={key} cx={x * width} cy={y * height} r={radius}
      className="live-plan-shape-handle"
      onPointerDown={(event) => {
        event.stopPropagation();
        onHandlePointerDown(event, shape.id, handle);
      }} />
  ));
}

function SketchShape({
  shape, width, height, editing, selected, markerScale, t,
  onPointerDown, onClick, onDblClick,
}) {
  const interactive = editing;
  const groupProps = interactive ? {
    tabIndex: 0,
    role: 'button',
    'aria-label': shapeSummary(shape, t),
    'aria-pressed': selected ? 'true' : 'false',
    onPointerDown: (event) => onPointerDown(event, shape.id, { kind: 'move' }),
    onClick: (event) => onClick(event, shape.id),
    onDblClick: (event) => onDblClick(event, shape.id),
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
      {editing && selected && (
        <ShapeHandles shape={shape} width={width} height={height}
          markerScale={markerScale} onHandlePointerDown={onPointerDown} />
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
 * mode they become focusable, draggable targets with resize handles.
 */
export function PlanSketchLayer({
  shapes,
  width,
  height,
  editing,
  selectedShapeId,
  markerScale,
  drawing,
  drawingClosable,
  t,
  onShapePointerDown,
  onShapeClick,
  onShapeDblClick,
}) {
  return (
    <g className={`live-plan-sketch ${editing ? 'is-editing' : ''}`}>
      {(shapes || []).map((shape) => (
        <SketchShape key={shape.id} shape={shape} width={width} height={height}
          editing={editing} selected={shape.id === selectedShapeId}
          markerScale={markerScale} t={t}
          onPointerDown={onShapePointerDown} onClick={onShapeClick}
          onDblClick={onShapeDblClick} />
      ))}
      <DrawingPreview drawing={drawing} width={width} height={height}
        markerScale={markerScale} closable={drawingClosable} />
    </g>
  );
}
