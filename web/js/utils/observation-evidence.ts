/**
 * INT_01 L9: per-camera observation evidence settings for integrations
 * (docs/integrations/OBSERVATIONS_CONTRACT.md section 13). The server enforces
 * the same bounds; the form checks them first so the user gets a clear message.
 */

type FormNumber = number | string;

export interface ObservationFormFields {
  observationEvidenceEnabled: boolean;
  /** Comma-separated detector labels; empty means every object label. */
  observationLabels: string;
  observationMaxAgeDays: FormNumber;
  /** The byte budget in MiB (the wire value is bytes). */
  observationMaxMib: FormNumber;
}

export interface ObservationWireFields {
  observation_evidence_enabled?: boolean;
  observation_labels?: string;
  observation_max_age_days?: number;
  observation_max_bytes?: number;
}

const MIB = 1024 * 1024;
export const OBSERVATION_MAX_AGE_DAYS = { min: 1, max: 90 } as const;
export const OBSERVATION_MAX_MIB = { min: 64, max: 1024 * 1024 } as const;
export const OBSERVATION_LABELS_MAX = 16;
export const OBSERVATION_LABEL_MAX_BYTES = 31;

export const DEFAULT_OBSERVATION_FORM_FIELDS: Readonly<ObservationFormFields> = Object.freeze({
  observationEvidenceEnabled: false,
  observationLabels: '',
  observationMaxAgeDays: 7,
  observationMaxMib: 2048,
});

const toInteger = (value: FormNumber): number => {
  const parsed = typeof value === 'number' ? value : Number(String(value).trim());
  return Number.isInteger(parsed) ? parsed : Number.NaN;
};

/** Trimmed label entries, or null when the list is not acceptable to the server. */
export function parseObservationLabels(value: string): string[] | null {
  if (value.trim() === '') return [];
  const labels = value.split(',').map(label => label.trim());
  if (labels.length > OBSERVATION_LABELS_MAX) return null;
  for (const label of labels) {
    if (label.length === 0 || label.length > OBSERVATION_LABEL_MAX_BYTES) return null;
    if (!/^[\x20-\x7e]+$/.test(label)) return null;
  }
  return labels;
}

/** Map stream details to form fields; missing values take the contract defaults. */
export function observationFormFields(stream: ObservationWireFields): ObservationFormFields {
  const bytes = stream.observation_max_bytes;
  return {
    observationEvidenceEnabled: stream.observation_evidence_enabled === true,
    observationLabels: stream.observation_labels ?? '',
    observationMaxAgeDays: stream.observation_max_age_days ?? DEFAULT_OBSERVATION_FORM_FIELDS.observationMaxAgeDays,
    observationMaxMib: typeof bytes === 'number' && bytes > 0
      ? Math.round(bytes / MIB)
      : DEFAULT_OBSERVATION_FORM_FIELDS.observationMaxMib,
  };
}

/** The i18n key of the first invalid field, or null when the form is valid. */
export function validateObservationForm(form: ObservationFormFields): string | null {
  const days = toInteger(form.observationMaxAgeDays);
  if (!(days >= OBSERVATION_MAX_AGE_DAYS.min && days <= OBSERVATION_MAX_AGE_DAYS.max)) {
    return 'streamsConfig.observationMaxAgeInvalid';
  }
  const mib = toInteger(form.observationMaxMib);
  if (!(mib >= OBSERVATION_MAX_MIB.min && mib <= OBSERVATION_MAX_MIB.max)) {
    return 'streamsConfig.observationMaxSizeInvalid';
  }
  if (parseObservationLabels(form.observationLabels ?? '') === null) {
    return 'streamsConfig.observationLabelsInvalid';
  }
  return null;
}

/** Wire fields for POST/PUT. Call validateObservationForm first. */
export function observationPayloadFields(form: ObservationFormFields): Required<ObservationWireFields> {
  const days = toInteger(form.observationMaxAgeDays);
  const mib = toInteger(form.observationMaxMib);
  const labels = parseObservationLabels(form.observationLabels ?? '');
  return {
    observation_evidence_enabled: !!form.observationEvidenceEnabled,
    observation_labels: labels ? labels.join(',') : (form.observationLabels ?? ''),
    observation_max_age_days: Number.isNaN(days)
      ? Number(DEFAULT_OBSERVATION_FORM_FIELDS.observationMaxAgeDays) : days,
    observation_max_bytes: (Number.isNaN(mib) ? Number(DEFAULT_OBSERVATION_FORM_FIELDS.observationMaxMib) : mib) * MIB,
  };
}
