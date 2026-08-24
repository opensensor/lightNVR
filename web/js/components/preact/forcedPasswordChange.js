export function validateForcedPasswordChange(currentPassword, newPassword, confirmation) {
  if (!currentPassword) return 'currentRequired';
  if (newPassword === 'admin') return 'defaultPassword';
  if (newPassword.length < 8) return 'tooShort';
  if (newPassword !== confirmation) return 'mismatch';
  return null;
}
