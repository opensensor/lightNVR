import { validateForcedPasswordChange } from '../js/components/preact/forcedPasswordChange.js';

describe('forced password change validation', () => {
  test('requires the current password', () => {
    expect(validateForcedPasswordChange('', 'replacement1', 'replacement1'))
      .toBe('currentRequired');
  });

  test('rejects short and default passwords', () => {
    expect(validateForcedPasswordChange('admin', 'short', 'short'))
      .toBe('tooShort');
    expect(validateForcedPasswordChange('admin', 'admin', 'admin'))
      .toBe('defaultPassword');
  });

  test('requires matching confirmation', () => {
    expect(validateForcedPasswordChange('admin', 'replacement1', 'replacement2'))
      .toBe('mismatch');
  });

  test('accepts a valid replacement', () => {
    expect(validateForcedPasswordChange('admin', 'replacement1', 'replacement1'))
      .toBeNull();
  });
});
