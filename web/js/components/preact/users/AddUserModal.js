/**
 * Add User Modal Component
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { USER_ROLES } from './UserRoles.js';

/**
 * Add User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.formData - Form data for adding a user
 * @param {Function} props.handleInputChange - Function to handle input changes
 * @param {Function} props.handleAddUser - Function to handle user addition
 * @param {Function} props.onClose - Function to close the modal
 * @param {Function} props.onSuccess - Function called after successful user addition
 * @returns {JSX.Element} Add user modal
 */
export function AddUserModal({ formData, handleInputChange, handleAddUser, onClose, onSuccess }) {
  // Add this function to prevent event propagation and handle success callback
  const handleSubmit = (e) => {
    e.preventDefault();
    e.stopPropagation(); // Stop event from bubbling up
    handleAddUser(e).then(() => {
      if (onSuccess) {
        setTimeout(onSuccess, 500); // Add a small delay to ensure API call completes
      }
    });
  };

  return html`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${onClose}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full" onClick=${e => e.stopPropagation()}>
        <h2 class="text-xl font-bold mb-4">Add New User</h2>
        
        <form onSubmit=${handleSubmit}>
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="username">
              Username
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              name="username"
              value=${formData.username}
              onChange=${handleInputChange}
              required
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
              Password
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              type="password"
              name="password"
              value=${formData.password}
              onChange=${handleInputChange}
              required
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="email">
              Email
            </label>
            <input
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="email"
              type="email"
              name="email"
              value=${formData.email}
              onChange=${handleInputChange}
            />
          </div>
          
          <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="role">
              Role
            </label>
            <select
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="role"
              name="role"
              value=${formData.role}
              onChange=${handleInputChange}
            >
              ${Object.entries(USER_ROLES).map(([value, label]) => html`
                <option value=${value}>${label}</option>
              `)}
            </select>
          </div>
          
          <div class="mb-6">
            <label class="flex items-center">
              <input
                type="checkbox"
                name="is_active"
                checked=${formData.is_active}
                onChange=${handleInputChange}
                class="mr-2"
              />
              <span class="text-gray-700 text-sm font-bold">Active</span>
            </label>
          </div>
          
          <div class="flex justify-end mt-6">
            <button
              type="button"
              class="px-4 py-2 bg-gray-300 text-gray-800 rounded hover:bg-gray-400 mr-2"
              onClick=${onClose}
            >
              Cancel
            </button>
            <button
              type="submit"
              class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Add User
            </button>
          </div>
        </form>
      </div>
    </div>
  `;
}
