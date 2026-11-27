import type { Go2rtcOverrideValidation } from '../../../types/settings.ts';

/** Keep prior diagnostics while making a preset conflict visible as an error. */
export function presetConflictValidation(
  previous: Go2rtcOverrideValidation | null,
  conflicts: string[],
): Go2rtcOverrideValidation {
  return {
    ...previous,
    valid: false,
    error: {
      message: `Preset insertion would duplicate top-level section(s): ${conflicts.join(', ')}. Merge into the existing section instead of inserting the full preset.`,
    },
  };
}
