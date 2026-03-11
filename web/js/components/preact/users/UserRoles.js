/**
 * User role definitions
 */
export const USER_ROLE_KEYS = {
  0: 'users.role.admin',
  1: 'users.role.user',
  2: 'users.role.viewer',
  3: 'users.role.api'
};

export function getUserRoleLabel(t, role) {
  const key = USER_ROLE_KEYS[role];
  return key ? t(key) : t('common.unknown');
}
