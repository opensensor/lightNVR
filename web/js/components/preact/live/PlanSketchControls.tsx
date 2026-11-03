import {
  SKETCH_LABEL_SIZES,
  SKETCH_LIMITS,
  SKETCH_TONES,
  SKETCH_TOOLS,
  normalizeShape,
  shapeSummary,
} from './planSketch.ts';
import type {
  RawSketchShape,
  SketchAreaShape,
  SketchRectShape,
  SketchShape,
  SketchTone,
  SketchTool,
  SketchWallShape,
  Translate,
} from '../../../types/live.ts';

export interface PlanSketchToolsProps {
  tool: SketchTool;
  onToolChange: (tool: SketchTool) => unknown;
  canUndo: boolean;
  canRedo: boolean;
  onUndo: () => unknown;
  onRedo: () => unknown;
  snap: boolean;
  onSnapChange: (snap: boolean) => unknown;
  toolLock: boolean;
  onToolLockChange: (toolLock: boolean) => unknown;
  shapeCount: number;
  t: Translate;
}

interface PercentFieldProps {
  id: string;
  label: string;
  value: number;
  onCommit: (value: number) => unknown;
}

export interface PlanSketchInspectorProps {
  uid: string;
  shapes: SketchShape[];
  selectedShape: SketchShape | null;
  selectedIds?: Set<string>;
  selectionCount?: number;
  t: Translate;
  onSelect: (shapeId: string, additive: boolean) => unknown;
  onChange: (shape: SketchShape) => unknown;
  onRecolorSelection: (tone: SketchTone) => unknown;
  onDelete: () => unknown;
  onDuplicate: () => unknown;
  onDeselect: () => unknown;
}

const TOOL_KEYS = { select: 'V', room: 'R', wall: 'W', area: 'A', label: 'L' };

/** Floating tool strip shown over the canvas while a layout is being edited. */
export function PlanSketchTools({
  tool, onToolChange, canUndo, canRedo, onUndo, onRedo,
  snap, onSnapChange, toolLock, onToolLockChange, shapeCount, t,
}: PlanSketchToolsProps) {
  const limitReached = shapeCount >= SKETCH_LIMITS.maxShapes;
  return (
    <div className="live-plan-sketch-tools" role="toolbar"
      aria-label={t('live.plan.sketch.tools')}>
      <div className="live-plan-sketch-tool-group">
        {SKETCH_TOOLS.map((candidate) => (
          <button key={candidate} type="button"
            className={tool === candidate ? 'is-active' : ''}
            aria-pressed={tool === candidate ? 'true' : 'false'}
            disabled={candidate !== 'select' && limitReached}
            title={`${t(`live.plan.sketch.tool.${candidate}`)} (${TOOL_KEYS[candidate]})`}
            onClick={() => onToolChange(candidate)}>
            {t(`live.plan.sketch.tool.${candidate}`)}
          </button>
        ))}
      </div>
      <div className="live-plan-sketch-tool-group">
        <button type="button" onClick={onUndo} disabled={!canUndo}
          title={`${t('live.plan.sketch.undo')} (Ctrl+Z)`}>↶</button>
        <button type="button" onClick={onRedo} disabled={!canRedo}
          title={`${t('live.plan.sketch.redo')} (Ctrl+Shift+Z)`}>↷</button>
        <button type="button" className={snap ? 'is-active' : ''}
          aria-pressed={snap ? 'true' : 'false'}
          title={`${t('live.plan.sketch.snap')} (G)`}
          onClick={() => onSnapChange(!snap)}>⌗</button>
        <button type="button" className={toolLock ? 'is-active' : ''}
          aria-pressed={toolLock ? 'true' : 'false'}
          aria-label={t('live.plan.sketch.toolLock')}
          title={`${t('live.plan.sketch.toolLock')} (Q)`}
          onClick={() => onToolLockChange(!toolLock)}>{toolLock ? '🔒' : '🔓'}</button>
      </div>
      <p className="live-plan-sketch-hint">
        {limitReached
          ? t('live.plan.sketch.limitReached', { max: SKETCH_LIMITS.maxShapes })
          : t(`live.plan.sketch.hint.${tool}`)}
      </p>
    </div>
  );
}

function percentValue(fraction: number) {
  return `${Math.round(fraction * 1000) / 10}`;
}

function parsePercent(value: string) {
  const number = Number(value);
  return Number.isFinite(number) ? Math.max(0, Math.min(1, number / 100)) : null;
}

function PercentField({ id, label, value, onCommit }: PercentFieldProps) {
  return (
    <label className="live-plan-sketch-field" htmlFor={id}>
      <span>{label}</span>
      <input id={id} type="number" min="0" max="100" step="0.1"
        value={percentValue(value)}
        onChange={(event) => {
          const parsed = parsePercent(event.currentTarget.value);
          if (parsed !== null) onCommit(parsed);
        }} />
    </label>
  );
}

function updateRect(shape: SketchRectShape, key: 'x' | 'y' | 'w' | 'h', value: number): SketchShape {
  const next = { ...shape };
  if (key === 'x') next.x = Math.min(value, 1 - shape.w);
  if (key === 'y') next.y = Math.min(value, 1 - shape.h);
  if (key === 'w') next.w = Math.max(SKETCH_LIMITS.minRectSide, Math.min(value, 1 - shape.x));
  if (key === 'h') next.h = Math.max(SKETCH_LIMITS.minRectSide, Math.min(value, 1 - shape.y));
  return normalizeShape(next) || shape;
}

function updatePoint(shape: SketchWallShape | SketchAreaShape, index: number, axis: 0 | 1, value: number): SketchShape {
  const points = shape.points.map((point, candidate) => candidate === index
    ? (axis === 0 ? [value, point[1]] : [point[0], value]) : point);
  return normalizeShape({ ...shape, points }) || shape;
}

/**
 * Keyboard-friendly editor for the selected shape: every drag operation on
 * the canvas has a numeric equivalent here, and the shape list lets a
 * keyboard user select shapes without pointing at them.
 */
export function PlanSketchInspector({
  uid, shapes, selectedShape, selectedIds, selectionCount = 0, t,
  onSelect, onChange, onRecolorSelection, onDelete, onDuplicate, onDeselect,
}: PlanSketchInspectorProps) {
  const shape = selectedShape;
  const commit = (next: RawSketchShape) => {
    const normalized = normalizeShape(next);
    if (normalized) onChange(normalized);
  };
  const multiple = selectionCount > 1;
  const selectedList = multiple ? shapes.filter((candidate) => selectedIds?.has(candidate.id)) : [];
  const commonTone = selectedList.length > 0 &&
    selectedList.every((candidate) => candidate.tone === selectedList[0]!.tone)
    ? selectedList[0]!.tone : null;
  return (
    <div className="live-plan-sketch-inspector">
      {multiple ? (
        <div className="live-plan-sketch-selected">
          <div className="live-plan-sketch-selected-head">
            <strong>{t('live.plan.sketch.multiSelected', { count: selectionCount })}</strong>
            <button type="button" onClick={onDeselect}>{t('live.plan.sketch.deselect')}</button>
          </div>
          <div className="live-plan-sketch-tones" role="group"
            aria-label={t('live.plan.sketch.tone')}>
            {SKETCH_TONES.map((tone) => (
              <button key={tone} type="button" className={`tone-${tone} ${commonTone === tone ? 'is-active' : ''}`}
                aria-pressed={commonTone === tone ? 'true' : 'false'}
                title={t(`live.plan.sketch.tone.${tone}`)}
                onClick={() => onRecolorSelection(tone)}>
                <i />
              </button>
            ))}
          </div>
          <p className="live-plan-sketch-multi-hint">{t('live.plan.sketch.multiHint')}</p>
          <div className="live-plan-sketch-actions">
            <button type="button" onClick={onDuplicate}
              disabled={shapes.length + selectionCount > SKETCH_LIMITS.maxShapes}>
              {t('live.plan.sketch.duplicate')}
            </button>
            <button type="button" className="is-danger" onClick={onDelete}>
              {t('live.plan.menu.deleteShapes', { count: selectionCount })}
            </button>
          </div>
        </div>
      ) : shape ? (
        <div className="live-plan-sketch-selected">
          <div className="live-plan-sketch-selected-head">
            <strong>{t(`live.plan.sketch.type.${shape.type}`)}</strong>
            <button type="button" onClick={onDeselect}>{t('live.plan.sketch.deselect')}</button>
          </div>
          <div className="live-plan-sketch-tones" role="group"
            aria-label={t('live.plan.sketch.tone')}>
            {SKETCH_TONES.map((tone) => (
              <button key={tone} type="button" className={`tone-${tone} ${shape.tone === tone ? 'is-active' : ''}`}
                aria-pressed={shape.tone === tone ? 'true' : 'false'}
                title={t(`live.plan.sketch.tone.${tone}`)}
                onClick={() => commit({ ...shape, tone })}>
                <i />
              </button>
            ))}
          </div>
          {(shape.type === 'rect' || shape.type === 'area') && (
            <label className="live-plan-sketch-field is-wide" htmlFor={`${uid}-label`}>
              <span>{t('live.plan.sketch.labelText')}</span>
              <input id={`${uid}-label`} type="text" maxLength={SKETCH_LIMITS.textMax}
                value={shape.label || ''}
                onInput={(event) => {
                  const label = event.currentTarget.value;
                  const next = { ...shape };
                  if (label.trim()) next.label = label;
                  else delete next.label;
                  onChange(normalizeShape(next) || { ...shape, label: undefined });
                }} />
            </label>
          )}
          {shape.type === 'label' && (
            <>
              <label className="live-plan-sketch-field is-wide" htmlFor={`${uid}-text`}>
                <span>{t('live.plan.sketch.text')}</span>
                <input id={`${uid}-text`} type="text" maxLength={SKETCH_LIMITS.textMax}
                  value={shape.text}
                  onInput={(event) => {
                    const text = event.currentTarget.value;
                    if (text.trim()) commit({ ...shape, text });
                  }} />
              </label>
              <label className="live-plan-sketch-field" htmlFor={`${uid}-size`}>
                <span>{t('live.plan.sketch.size')}</span>
                <select id={`${uid}-size`} value={shape.size}
                  onChange={(event) => commit({ ...shape, size: event.currentTarget.value })}>
                  {SKETCH_LABEL_SIZES.map((size) => (
                    <option key={size} value={size}>{t(`live.plan.sketch.size.${size}`)}</option>
                  ))}
                </select>
              </label>
            </>
          )}
          {shape.type === 'rect' && (
            <>
              <label className="live-plan-sketch-field is-check" htmlFor={`${uid}-filled`}>
                <input id={`${uid}-filled`} type="checkbox" checked={shape.filled}
                  onChange={(event) => commit({ ...shape, filled: event.currentTarget.checked })} />
                <span>{t('live.plan.sketch.filled')}</span>
              </label>
              <div className="live-plan-sketch-grid">
                <PercentField id={`${uid}-x`} label={t('live.plan.sketch.left')} value={shape.x}
                  onCommit={(value) => onChange(updateRect(shape, 'x', value))} />
                <PercentField id={`${uid}-y`} label={t('live.plan.sketch.top')} value={shape.y}
                  onCommit={(value) => onChange(updateRect(shape, 'y', value))} />
                <PercentField id={`${uid}-w`} label={t('live.plan.sketch.width')} value={shape.w}
                  onCommit={(value) => onChange(updateRect(shape, 'w', value))} />
                <PercentField id={`${uid}-h`} label={t('live.plan.sketch.height')} value={shape.h}
                  onCommit={(value) => onChange(updateRect(shape, 'h', value))} />
              </div>
            </>
          )}
          {shape.type === 'label' && (
            <div className="live-plan-sketch-grid">
              <PercentField id={`${uid}-x`} label={t('live.plan.sketch.left')} value={shape.x}
                onCommit={(value) => commit({ ...shape, x: value })} />
              <PercentField id={`${uid}-y`} label={t('live.plan.sketch.top')} value={shape.y}
                onCommit={(value) => commit({ ...shape, y: value })} />
            </div>
          )}
          {(shape.type === 'wall' || shape.type === 'area') && (
            <details className="live-plan-sketch-points">
              <summary>{t('live.plan.sketch.points')} · {shape.points.length}</summary>
              {shape.points.map((point, index) => (
                <div key={index} className="live-plan-sketch-grid">
                  <PercentField id={`${uid}-p${index}x`}
                    label={`${t('live.plan.sketch.point', { index: index + 1 })} X`}
                    value={point[0]}
                    onCommit={(value) => onChange(updatePoint(shape, index, 0, value))} />
                  <PercentField id={`${uid}-p${index}y`}
                    label={`${t('live.plan.sketch.point', { index: index + 1 })} Y`}
                    value={point[1]}
                    onCommit={(value) => onChange(updatePoint(shape, index, 1, value))} />
                </div>
              ))}
            </details>
          )}
          <div className="live-plan-sketch-actions">
            <button type="button" onClick={onDuplicate}
              disabled={shapes.length >= SKETCH_LIMITS.maxShapes}>
              {t('live.plan.sketch.duplicate')}
            </button>
            <button type="button" className="is-danger" onClick={onDelete}>
              {t('live.plan.sketch.delete')}
            </button>
          </div>
        </div>
      ) : null}
      <div className="live-plan-sketch-list">
        <div><strong>{t('live.plan.sketch.shapes')}</strong><small>{shapes.length}</small></div>
        {shapes.length === 0 ? (
          <p>{t('live.plan.sketch.noShapes')}</p>
        ) : (
          <div>
            {shapes.map((candidate) => {
              const isSelected = Boolean(selectedIds?.has(candidate.id));
              return (
              <button key={candidate.id} type="button"
                className={`tone-${candidate.tone} ${isSelected ? 'is-selected' : ''}`}
                aria-pressed={isSelected ? 'true' : 'false'}
                onClick={(event) => onSelect(candidate.id,
                  event.shiftKey || event.ctrlKey || event.metaKey)}>
                <i />
                <span>{shapeSummary(candidate, t)}</span>
              </button>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
