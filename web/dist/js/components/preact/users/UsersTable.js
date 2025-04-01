/**
 * Users Table Component
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
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
  
  return html`
    <div class="overflow-x-auto">
      <table class="w-full border-collapse">
        <thead class="bg-gray-50 dark:bg-gray-700">
          <tr>
            <th class="py-3 px-6 text-left font-semibold">ID</th>
            <th class="py-3 px-6 text-left font-semibold">Username</th>
            <th class="py-3 px-6 text-left font-semibold">Email</th>
            <th class="py-3 px-6 text-left font-semibold">Role</th>
            <th class="py-3 px-6 text-left font-semibold">Status</th>
            <th class="py-3 px-6 text-left font-semibold">Last Login</th>
            <th class="py-3 px-6 text-left font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
          ${users.map(user => html`
            <tr key=${user.id} class="hover:bg-gray-100 dark:hover:bg-gray-600">
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${user.id}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${user.username}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${user.email || '-'}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${USER_ROLES[user.role] || 'Unknown'}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <span class=${`inline-block px-2 py-1 text-xs font-semibold rounded-full ${user.is_active ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
                  ${user.is_active ? 'Active' : 'Inactive'}
                </span>
              </td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">${user.last_login ? new Date(user.last_login * 1000).toLocaleString() : 'Never'}</td>
              <td class="py-3 px-6 border-b border-gray-200 dark:border-gray-700">
                <div class="flex space-x-2">
                  <button 
                    class="p-1 text-blue-600 hover:text-blue-800 rounded hover:bg-blue-100 transition-colors"
                    onClick=${(e) => handleEdit(user, e)}
                    title="Edit User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-red-600 hover:text-red-800 rounded hover:bg-red-100 transition-colors"
                    onClick=${(e) => handleDelete(user, e)}
                    title="Delete User"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                  <button 
                    class="p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors"
                    onClick=${(e) => handleApiKey(user, e)}
                    title="Manage API Key"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          `)}
        </tbody>
      </table>
    </div>
  `;
}
