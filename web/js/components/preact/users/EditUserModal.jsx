/**
 * Edit User Modal Component
 */

import { h } from 'preact';
import { USER_ROLES } from './UserRoles.js';

/**
 * Edit User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user being edited
 * @param {Object} props.formData - Form data for editing a user
 * @param {Function} props.handleInputChange - Function to handle input changes
 * @param {Function} props.handleEditUser - Function to handle user editing
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} Edit user modal
 */
export function EditUserModal({ currentUser, formData, handleInputChange, handleEditUser, onClose }) {
  // Direct submit handler
  const handleSubmit = (e) => {
    e.preventDefault();
    e.stopPropagation(); // Stop event from bubbling up
    handleEditUser(e);
  };

  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">Edit User: {currentUser.username}</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="username">
              Username
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="username"
              type="text"
              name="username"
              value={formData.username}
              onChange={handleInputChange}
              required
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="password">
              Password (leave blank to keep current)
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="password"
              type="password"
              name="password"
              value={formData.password}
              onChange={handleInputChange}
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="email">
              Email
            </label>
            <input
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="email"
              type="email"
              name="email"
              value={formData.email}
              onChange={handleInputChange}
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="role">
              Role
            </label>
            <select
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-white dark:bg-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="role"
              name="role"
              value={formData.role}
              onChange={handleInputChange}
            >
              {Object.entries(USER_ROLES).map(([value, label]) => (
                <option key={value} value={value}>{label}</option>
              ))}
            </select>
          </div>

          <div className="mb-6">
            <label className="flex items-center">
              <input
                type="checkbox"
                name="is_active"
                checked={formData.is_active}
                onChange={handleInputChange}
                className="mr-2"
              />
              <span className="text-sm font-bold">Active</span>
            </label>
          </div>

          <div className="flex justify-end mt-6">
            <button
              type="button"
              className="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
              onClick={onClose}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              Update User
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
