import fs from 'node:fs';
import path from 'node:path';
import {
  DEFAULT_OBSERVATION_FORM_FIELDS,
  observationFormFields,
  observationPayloadFields,
  validateObservationForm,
} from '../js/utils/observation-evidence.ts';

/* INT_01 L9: per-camera observation evidence opt-in (contract section 13). */
describe('observation evidence settings', () => {
  test('defaults to off, every label, 7 days and 2 GiB', () => {
    expect(DEFAULT_OBSERVATION_FORM_FIELDS).toEqual({
      observationEvidenceEnabled: false,
      observationLabels: '',
      observationMaxAgeDays: 7,
      observationMaxMib: 2048,
    });
    // An older server that omits the fields maps to the same defaults.
    expect(observationFormFields({})).toEqual(DEFAULT_OBSERVATION_FORM_FIELDS);
    expect(observationPayloadFields(DEFAULT_OBSERVATION_FORM_FIELDS)).toEqual({
      observation_evidence_enabled: false,
      observation_labels: '',
      observation_max_age_days: 7,
      observation_max_bytes: 2147483648,
    });
  });

  test('an enabled toggle and budgets persist through the save payload', () => {
    const form = observationFormFields({
      observation_evidence_enabled: true,
      observation_labels: 'bird,squirrel',
      observation_max_age_days: 14,
      observation_max_bytes: 134217728,
    });
    expect(form).toEqual({
      observationEvidenceEnabled: true,
      observationLabels: 'bird,squirrel',
      observationMaxAgeDays: 14,
      observationMaxMib: 128,
    });
    expect(observationPayloadFields({ ...form, observationLabels: ' bird , squirrel ' })).toEqual({
      observation_evidence_enabled: true,
      observation_labels: 'bird,squirrel',
      observation_max_age_days: 14,
      observation_max_bytes: 134217728,
    });
    // Edited numeric inputs hold strings.
    expect(observationPayloadFields({ ...form, observationMaxAgeDays: '90', observationMaxMib: '1048576' }))
      .toMatchObject({ observation_max_age_days: 90, observation_max_bytes: 1099511627776 });
  });

  test('validates the server bounds before saving', () => {
    expect(validateObservationForm(DEFAULT_OBSERVATION_FORM_FIELDS)).toBeNull();
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationMaxAgeDays: 0 }))
      .toBe('streamsConfig.observationMaxAgeInvalid');
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationMaxAgeDays: 91 }))
      .toBe('streamsConfig.observationMaxAgeInvalid');
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationMaxMib: 63 }))
      .toBe('streamsConfig.observationMaxSizeInvalid');
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationMaxMib: 1048577 }))
      .toBe('streamsConfig.observationMaxSizeInvalid');
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationLabels: 'bird,,cat' }))
      .toBe('streamsConfig.observationLabelsInvalid');
    expect(validateObservationForm({
      ...DEFAULT_OBSERVATION_FORM_FIELDS,
      observationLabels: Array.from({ length: 17 }, (_, i) => `l${i}`).join(','),
    })).toBe('streamsConfig.observationLabelsInvalid');
    expect(validateObservationForm({ ...DEFAULT_OBSERVATION_FORM_FIELDS, observationLabels: 'x'.repeat(32) }))
      .toBe('streamsConfig.observationLabelsInvalid');
  });

  test('the stream editor wires the section, defaults and payload', () => {
    const read = file => fs.readFileSync(path.resolve(__dirname, '../js/components/preact', file), 'utf8');
    const view = read('StreamsView.tsx');
    expect(view).toContain('...observationFormFields(stream)');
    expect(view).toContain('...observationPayloadFields(currentStream)');
    expect(view.match(/\.\.\.DEFAULT_OBSERVATION_FORM_FIELDS/g)?.length).toBeGreaterThanOrEqual(2);
    const modal = read('StreamConfigModal.tsx');
    expect(modal).toContain("t('streamsConfig.observationEvidence')");
    expect(modal).toContain("t('streamsConfig.observationEvidenceDetectorRequired')");
    expect(modal).toContain('name="observationEvidenceEnabled"');
    expect(modal).toContain('checked={!!currentStream.observationEvidenceEnabled}');
  });

  test('the English catalog carries the section strings', () => {
    const en = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../public/locales/en.json'), 'utf8'));
    for (const key of ['observationEvidence', 'observationEvidenceHelp', 'observationEvidenceDetectorRequired',
      'observationEvidenceEnable', 'observationLabels', 'observationLabelsHelp', 'observationMaxAgeDays',
      'observationMaxSizeMib', 'observationBudgetHelp', 'observationMaxAgeInvalid',
      'observationMaxSizeInvalid', 'observationLabelsInvalid']) {
      expect(typeof en[`streamsConfig.${key}`]).toBe('string');
    }
  });
});
