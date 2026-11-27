import { presetConflictValidation } from '../js/components/preact/settings/presetConflictValidation.ts';

describe('preset conflict validation', () => {
  it('shows a conflict even after a successful validation and retains warnings', () => {
    const previous = { valid: true, warnings: ['Existing server warning'] };
    const result = presetConflictValidation(previous, ['ffmpeg']);
    expect(result.valid).toBe(false);
    expect(result.warnings).toEqual(previous.warnings);
    expect(result.error?.message).toContain('ffmpeg');
  });

  it('shows a conflict before any server validation', () => {
    const result = presetConflictValidation(null, ['streams']);
    expect(result.valid).toBe(false);
    expect(result.error?.message).toContain('streams');
  });
});
