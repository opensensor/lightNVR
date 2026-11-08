/**
 * DetectionClassifications Component
 * INT_01 L15: minimal display of an external classification (for example a
 * YA-WAMF species) next to the detector's own result. The server links each
 * detection to its observation in GET /api/detection/results/{name}
 * ("observation" member) and already prefers an owner decision over automated
 * ones, so this renders one line per observation and never issues requests.
 * The detector label and confidence stay visible; there is no taxonomy view.
 */

import type { ComponentChild } from 'preact';
import { useI18n } from '../../i18n.ts';
import type { DetectionObservation } from '../../types/core.ts';

type Translate = (key: string, params?: Record<string, string | number>) => string;

/** The detection fields this view reads (core Detection or TimelineDetection). */
export interface ClassifiableDetection {
  label: string;
  confidence: number;
  observation?: DetectionObservation | null;
}

export interface DetectionClassificationView {
  observationId: string;
  observationUrl: string;
  detectorLabel: string;
  detectorConfidence: number;
  classifiedLabel: string | null;
  classifiedConfidence: number | null;
  classifier: string | null;
  ownerConfirmed: boolean;
}

const percent = (value: unknown): number =>
  typeof value === 'number' && Number.isFinite(value) ? Math.round(Math.min(1, Math.max(0, value)) * 100) : 0;

/** Null unless the detection carries a well-formed observation link. */
export function detectionClassificationView(detection: ClassifiableDetection): DetectionClassificationView | null {
  const observation = detection?.observation;
  if (!observation || typeof observation.id !== 'string' || !observation.id) return null;
  const classification = observation.classification && typeof observation.classification.label === 'string'
    ? observation.classification : null;
  return {
    observationId: observation.id,
    observationUrl: `/api/observations/${encodeURIComponent(observation.id)}`,
    detectorLabel: detection.label,
    detectorConfidence: percent(detection.confidence),
    classifiedLabel: classification ? classification.label : null,
    classifiedConfidence: classification ? percent(classification.confidence) : null,
    classifier: classification ? classification.classifier : null,
    ownerConfirmed: classification ? classification.kind === 'owner_confirmed' : false,
  };
}

/** Canvas overlay text: the detector result, then the classification if any. */
export function detectionOverlayLabel(detection: ClassifiableDetection): string {
  const original = `${detection.label} (${percent(detection.confidence)}%)`;
  const view = detectionClassificationView(detection);
  if (!view || view.classifiedLabel === null) return original;
  return `${original} · ${view.classifiedLabel} (${view.classifiedConfidence}%)`;
}

/** One view per linked observation, in detection order. */
export function classifiedDetections(detections: readonly ClassifiableDetection[]): DetectionClassificationView[] {
  const seen = new Set<string>();
  const views: DetectionClassificationView[] = [];
  for (const detection of detections || []) {
    const view = detectionClassificationView(detection);
    if (!view || seen.has(view.observationId)) continue;
    seen.add(view.observationId);
    views.push(view);
  }
  return views;
}

/** Pure render (testable without a DOM); null when nothing is linked. */
export function renderDetectionClassifications(
  detections: readonly ClassifiableDetection[], t: Translate,
): ComponentChild {
  const views = classifiedDetections(detections);
  if (views.length === 0) return null;
  return (
    <div className="detection-classifications mt-2 text-xs" data-testid="detection-classifications">
      <div className="font-semibold mb-1">{t('detections.classification.title')}</div>
      <ul className="space-y-1">
        {views.map((view) => (
          <li key={view.observationId} className="flex flex-wrap items-center gap-x-2 gap-y-1"
            data-testid="detection-classification">
            <span className="text-muted-foreground">
              {t('detections.classification.detector', { label: view.detectorLabel, confidence: view.detectorConfidence })}
            </span>
            {view.classifiedLabel !== null ? (
              <>
                <span className="font-medium">
                  {t('detections.classification.classified', { label: view.classifiedLabel, confidence: view.classifiedConfidence ?? 0 })}
                </span>
                <span className="text-muted-foreground">
                  {t('detections.classification.source', { classifier: view.classifier ?? '' })}
                </span>
                {view.ownerConfirmed && (
                  <span className="badge badge-success badge-sm" data-testid="detection-classification-owner">
                    {t('detections.classification.ownerConfirmed')}
                  </span>
                )}
              </>
            ) : (
              <span className="text-muted-foreground">{t('detections.classification.unclassified')}</span>
            )}
            <a href={view.observationUrl} target="_blank" rel="noopener noreferrer" className="link link-primary">
              {t('detections.classification.observationLink')}
            </a>
          </li>
        ))}
      </ul>
    </div>
  );
}

export interface DetectionClassificationsProps {
  detections: readonly ClassifiableDetection[];
}

export function DetectionClassifications({ detections }: DetectionClassificationsProps) {
  const { t } = useI18n();
  return renderDetectionClassifications(detections, t);
}
