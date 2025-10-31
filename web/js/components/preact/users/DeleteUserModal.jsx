/**
 * Delete User Modal Component
 */

import { h } from 'preact';

/**
 * Delete User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user being deleted
 * @param {Function} props.handleDeleteUser - Function to handle user deletion
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} Delete user modal
 */
export function DeleteUserModal({ currentUser, handleDeleteUser, onClose }) {
  // Direct delete handler
  const handleDeleteClick = (e) => {
    e.stopPropagation(); // Stop event from bubbling up
    handleDeleteUser();
  };

  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-white rounded-lg p-6 max-w-md w-full dark:bg-gray-800 dark:text-white" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">Delete User</h2>

        <p className="mb-6">
          Are you sure you want to delete the user "{currentUser.username}"? This action cannot be undone.
        </p>

        <div className="flex justify-end">
          <button
            className="px-4 py-2 bg-gray-300 text-gray-800 dark:bg-gray-600 dark:text-white rounded hover:bg-gray-400 dark:hover:bg-gray-500 mr-2"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            className="btn-danger"
            onClick={handleDeleteClick}
          >
            Delete User
          </button>
        </div>
      </div>
    </div>
  );
}
