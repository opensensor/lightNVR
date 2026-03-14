/**
 * Users Table Component
 */

import { useState, useMemo, useCallback } from 'preact/hooks';
import { getUserRoleLabel } from './UserRoles.js';
import { formatLocalDateTime } from '../../../utils/date-utils.js';
import { useI18n } from '../../../i18n.js';

/**
 * Users Table Component
 * @param {Object} props - Component props
 * @param {Array} props.users - List of users to display
 * @param {Function} props.onEdit - Function to handle edit action
 * @param {Function} props.onDelete - Function to handle delete action
 * @param {Function} props.onApiKey - Function to handle API key action
 * @param {Function} props.onMfa - Function to handle MFA setup action
 * @returns {JSX.Element} Users table
 */
export function UsersTable({ users, onEdit, onDelete, onApiKey, onMfa }) {
  const { t } = useI18n();

  const normalizedUsers = Array.isArray(users) ? users : [];
  const UNSORTED_COLUMN = '';

  // Sorting state
  const [sortColumn, setSortColumn] = useState(UNSORTED_COLUMN);
  const [sortDirection, setSortDirection] = useState('asc');

  const handleSort = (column) => {
    if (sortColumn === column) {
      setSortDirection(prev => prev === 'asc' ? 'desc' : 'asc');
    } else {
      setSortColumn(column);
      setSortDirection('asc');
    }
  };

  const sortedUsers = useMemo(() => {
    if (sortColumn === UNSORTED_COLUMN) return normalizedUsers;
    return [...normalizedUsers].sort((a, b) => {
      let aVal, bVal;
      if (sortColumn === 'id') {
        aVal = a.id || 0;
        bVal = b.id || 0;
      } else if (sortColumn === 'username') {
        aVal = (a.username || '').toLowerCase();
        bVal = (b.username || '').toLowerCase();
      } else if (sortColumn === 'email') {
        aVal = (a.email || '').toLowerCase();
        bVal = (b.email || '').toLowerCase();
      } else if (sortColumn === 'role') {
        aVal = a.role ?? 0;
        bVal = b.role ?? 0;
      } else if (sortColumn === 'status') {
        aVal = a.is_active ? 1 : 0;
        bVal = b.is_active ? 1 : 0;
      } else if (sortColumn === 'password') {
        aVal = a.password_change_locked ? 1 : 0;
        bVal = b.password_change_locked ? 1 : 0;
      } else if (sortColumn === 'mfa') {
        aVal = a.totp_enabled ? 1 : 0;
        bVal = b.totp_enabled ? 1 : 0;
      } else if (sortColumn === 'lastLogin') {
        aVal = a.last_login ? new Date(a.last_login).getTime() : 0;
        bVal = b.last_login ? new Date(b.last_login).getTime() : 0;
      } else {
        return 0;
      }
      if (aVal < bVal) return sortDirection === 'asc' ? -1 : 1;
      if (aVal > bVal) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  }, [normalizedUsers, sortColumn, sortDirection]);

  // Create memoized handlers for each button to maintain stable references
  const handleEdit = useCallback((user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onEdit(user);
  }, [onEdit]);

  const handleDelete = useCallback((user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onDelete(user);
  }, [onDelete]);

  const handleApiKey = useCallback((user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onApiKey(user);
  }, [onApiKey]);

  const handleMfa = useCallback((user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onMfa(user);
  }, [onMfa]);

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            {[
              { key: 'id',        label: 'ID' },
              { key: 'username',  label: t('fields.username') },
              { key: 'email',     label: t('fields.email') },
              { key: 'role',      label: t('fields.role') },
              { key: 'status',    label: t('users.status') },
              { key: 'password',  label: t('fields.password') },
              { key: 'mfa',       label: 'MFA' },
              { key: 'lastLogin', label: t('users.lastLogin') },
            ].map(({ key, label }) => (
              <th
                key={key}
                className="py-3 px-6 text-left font-semibold cursor-pointer select-none hover:text-foreground"
                onClick={() => handleSort(key)}
              >
                <span className="inline-flex items-center gap-1">
                  {label}
                  <span className="inline-flex flex-col leading-none text-[0.6rem]">
                    <span className={sortColumn === key && sortDirection === 'asc' ? 'opacity-100' : 'opacity-30'}>▲</span>
                    <span className={sortColumn === key && sortDirection === 'desc' ? 'opacity-100' : 'opacity-30'}>▼</span>
                  </span>
                </span>
              </th>
            ))}
            <th className="py-3 px-6 text-left font-semibold">{t('common.actions')}</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
          {sortedUsers.map((user, index) => (
            <tr key={user.id != null ? `user-${user.id}` : `user-index-${index}`} className="hover:bg-gray-100 dark:hover:bg-gray-600">
              <td className="py-3 px-6 border-b border-border">{user.id ?? '-'}</td>
              <td className="py-3 px-6 border-b border-border">{user.username ?? '-'}</td>
              <td className="py-3 px-6 border-b border-border">{user.email || '-'}</td>
              <td className="py-3 px-6 border-b border-border">{getUserRoleLabel(t, user.role)}</td>
              <td className="py-3 px-6 border-b border-border">
                <span className={`inline-block px-2 py-1 text-xs font-semibold rounded-full ${user.is_active ? 'badge-success' : 'badge-danger'}`}>
                  {user.is_active ? t('users.active') : t('users.inactive')}
                </span>
              </td>
              <td className="py-3 px-6 border-b border-border">
                <span className={`inline-block px-2 py-1 text-xs font-semibold rounded-full ${user.password_change_locked ? 'badge-warning' : 'badge-info'}`} title={user.password_change_locked ? t('users.passwordChangesLocked') : t('users.passwordChangesAllowed')}>
                  {user.password_change_locked ? t('users.locked') : t('users.unlocked')}
                </span>
              </td>
              <td className="py-3 px-6 border-b border-border">
                <span className={`inline-block px-2 py-1 text-xs font-semibold rounded-full ${user.totp_enabled ? 'badge-success' : 'badge-info'}`}>
                  {user.totp_enabled ? t('users.mfaEnabled') : t('users.mfaDisabled')}
                </span>
              </td>
              <td className="py-3 px-6 border-b border-border">{user.last_login ? formatLocalDateTime(user.last_login) : t('common.never')}</td>
              <td className="py-3 px-6 border-b border-border">
                <div className="flex space-x-2">
                  <button
                    className="p-1 rounded transition-colors text-[hsl(var(--primary))] hover:bg-[hsl(var(--primary)_/_0.1)] hover:text-[hsl(var(--primary)_/_0.7)]"
                    onClick={(e) => handleEdit(user, e)}
                    title={t('users.editUser')}
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    className="p-1 rounded transition-colors text-[hsl(var(--danger))] hover:bg-[hsl(var(--danger)_/_0.1)] hover:text-[hsl(var(--danger)_/_0.7)]"
                    onClick={(e) => handleDelete(user, e)}
                    title={t('users.deleteUser')}
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button
                    className="p-1 rounded transition-colors text-[hsl(var(--muted-foreground))] hover:text-[hsl(var(--muted-foreground)_/_0.8)] hover:bg-[hsl(var(--muted)_/_0.5)]"
                    onClick={(e) => handleApiKey(user, e)}
                    title={t('users.manageApiKey')}
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                    </svg>
                  </button>
                  <button
                    className="p-1 rounded transition-colors text-[hsl(var(--muted-foreground))] hover:text-[hsl(var(--muted-foreground)_/_0.8)] hover:bg-[hsl(var(--muted)_/_0.5)]"
                    onClick={(e) => handleMfa(user, e)}
                    title={t('users.manageMfa')}
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
