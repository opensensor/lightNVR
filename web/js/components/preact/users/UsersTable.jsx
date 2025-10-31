/**
 * Users Table Component
 */

import { h } from 'preact';
import { USER_ROLES } from './UserRoles.js';

/**
 * Users Table Component
 * @param {Object} props - Component props
 * @param {Array} props.users - List of users to display
 * @param {Function} props.onEdit - Function to handle edit action
 * @param {Function} props.onDelete - Function to handle delete action
 * @param {Function} props.onApiKey - Function to handle API key action
 * @returns {JSX.Element} Users table
 */
export function UsersTable({ users, onEdit, onDelete, onApiKey }) {
  // Create direct handlers for each button
  const handleEdit = (user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onEdit(user);
  };

  const handleDelete = (user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onDelete(user);
  };

  const handleApiKey = (user, e) => {
    e.preventDefault();
    e.stopPropagation();
    onApiKey(user);
  };

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead className="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th className="py-3 px-6 text-left font-semibold">ID</th>
            <th className="py-3 px-6 text-left font-semibold">Username</th>
            <th className="py-3 px-6 text-left font-semibold">Email</th>
            <th className="py-3 px-6 text-left font-semibold">Role</th>
            <th className="py-3 px-6 text-left font-semibold">Status</th>
            <th className="py-3 px-6 text-left font-semibold">Last Login</th>
            <th className="py-3 px-6 text-left font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
          {users.map(user => (
            <tr key={user.id} className="hover:bg-gray-100 dark:hover:bg-gray-600">
              <td className="py-3 px-6 border-b border-border">{user.id}</td>
              <td className="py-3 px-6 border-b border-border">{user.username}</td>
              <td className="py-3 px-6 border-b border-border">{user.email || '-'}</td>
              <td className="py-3 px-6 border-b border-border">{USER_ROLES[user.role] || 'Unknown'}</td>
              <td className="py-3 px-6 border-b border-border">
                <span className={`inline-block px-2 py-1 text-xs font-semibold rounded-full ${user.is_active ? 'badge-success' : 'badge-danger'}`}>
                  {user.is_active ? 'Active' : 'Inactive'}
                </span>
              </td>
              <td className="py-3 px-6 border-b border-border">{user.last_login ? new Date(user.last_login * 1000).toLocaleString() : 'Never'}</td>
              <td className="py-3 px-6 border-b border-border">
                <div className="flex space-x-2">
                  <button
                    className="p-1 rounded transition-colors" style={{color: 'hsl(var(--primary))'}} onMouseOver={(e) => {e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'; e.currentTarget.style.color = 'hsl(var(--primary) / 0.7)'}} onMouseOut={(e) => {e.currentTarget.style.backgroundColor = 'transparent'; e.currentTarget.style.color = 'hsl(var(--primary))'}}
                    onClick={(e) => handleEdit(user, e)}
                    title="Edit User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    className="p-1 rounded transition-colors" style={{color: 'hsl(var(--danger))'}} onMouseOver={(e) => {e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'; e.currentTarget.style.color = 'hsl(var(--danger) / 0.7)'}} onMouseOut={(e) => {e.currentTarget.style.backgroundColor = 'transparent'; e.currentTarget.style.color = 'hsl(var(--danger))'}}
                    onClick={(e) => handleDelete(user, e)}
                    title="Delete User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button
                    className="p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors"
                    onClick={(e) => handleApiKey(user, e)}
                    title="Manage API Key"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
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
