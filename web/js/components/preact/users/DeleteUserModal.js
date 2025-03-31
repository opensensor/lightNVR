/**
 * Delete User Modal Component
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';

/**
 * Delete User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user being deleted
 * @param {Function} props.handleDeleteUser - Function to handle user deletion
 * @param {Function} props.onClose - Function to close the modal
 * @param {Function} props.onSuccess - Function called after successful user deletion
 * @returns {JSX.Element} Delete user modal
 */
export function DeleteUserModal({ currentUser, handleDeleteUser, onClose, onSuccess }) {
  // Add this function to prevent event propagation and handle success callback
  const handleDeleteClick = (e) => {
    e.stopPropagation(); // Stop event from bubbling up
    handleDeleteUser().then(() => {
      if (onSuccess) {
        setTimeout(onSuccess, 500); // Add a small delay to ensure API call completes
      }
    });
  };

  return html`
    <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50" onClick=${onClose}>
      <div class="bg-white rounded-lg p-6 max-w-md w-full" onClick=${e => e.stopPropagation()}>
        <h2 class="text-xl font-bold mb-4">Delete User</h2>
        
        <p class="mb-6">
          Are you sure you want to delete the user "${currentUser.username}"? This action cannot be undone.
        </p>
        
        <div class="flex justify-end">
          <button
            class="px-4 py-2 bg-gray-300 text-gray-800 rounded hover:bg-gray-400 mr-2"
            onClick=${onClose}
          >
            Cancel
          </button>
          <button
            class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700"
            onClick=${handleDeleteClick}
          >
            Delete User
          </button>
        </div>
      </div>
    </div>
  `;
}
