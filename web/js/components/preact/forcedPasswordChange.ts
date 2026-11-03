export type ForcedPasswordChangeError = 'currentRequired' | 'defaultPassword' | 'tooShort' | 'mismatch';

export function validateForcedPasswordChange(
  currentPassword: string,
  newPassword: string,
  confirmation: string,
): ForcedPasswordChangeError | null {
  if (!currentPassword) return 'currentRequired';
  if (newPassword === 'admin') return 'defaultPassword';
  if (newPassword.length < 8) return 'tooShort';
  if (newPassword !== confirmation) return 'mismatch';
  return null;
}
