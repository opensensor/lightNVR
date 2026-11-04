/**
 * User role definitions
 */
import type { Translate } from '../../../types/users.ts';

export const USER_ROLE_KEYS: Record<string, string> = {
  0: 'users.role.admin',
  1: 'users.role.user',
  2: 'users.role.viewer',
  3: 'users.role.api'
};

export function getUserRoleLabel(t: Translate, role: number) {
  const key = USER_ROLE_KEYS[role];
  return key ? t(key) : t('common.unknown');
}
