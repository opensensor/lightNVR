/**
 * LightNVR Web Interface Users View Component
 * Preact component for the user management page
 */

import { h } from '../../preact.min.js';
import { useState, useEffect } from '../../preact.hooks.module.js';
import { html } from '../../html-helper.js';
import { showStatusMessage } from './UI.js';

/**
 * User role names
 */
const USER_ROLES = {
  0: 'Admin',
  1: 'User',
  2: 'Viewer',
  3: 'API'
};

/**
 * UsersView component
 * @returns {JSX.Element} UsersView component
 */
export function UsersView() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [showAddModal, setShowAddModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [showApiKeyModal, setShowApiKeyModal] = useState(false);
  const [currentUser, setCurrentUser] = useState(null);
  const [newApiKey, setNewApiKey] = useState('');

  // Form state for adding/editing users
  const [formData, setFormData] = useState({
    username: '',
    password: '',
    email: '',
    role: 1,
    is_active: true
  });

  // Fetch users on component mount
  useEffect(() => {
    fetchUsers();
  }, []);

  // Add event listener for the add user button
  useEffect(() => {
    const addUserBtn = document.getElementById('add-user-btn');
    if (addUserBtn) {
      addUserBtn.addEventListener('click', () => {
        setFormData({
          username: '',
          password: '',
          email: '',
          role: 1,
          is_active: true
        });
        setShowAddModal(true);
      });
    }
    
    return () => {
      if (addUserBtn) {
        addUserBtn.removeEventListener('click', () => {
          setShowAddModal(true);
        });
      }
    };
  }, []);

  /**
   * Fetch users from the API
   */
  const fetchUsers = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const response = await fetch('/api/auth/users');
      
      if (!response.ok) {
        throw new Error(`Failed to fetch users: ${response.status} ${response.statusText}`);
      }
      
      const data = await response.json();
      setUsers(data.users || []);
    } catch (err) {
      console.error('Error fetching users:', err);
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  /**
   * Handle form input changes
   * @param {Event} e - Input change event
   */
  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    
    setFormData({
      ...formData,
      [name]: type === 'checkbox' ? checked : (name === 'role' ? parseInt(value, 10) : value)
    });
  };

  /**
   * Handle form submission for adding a user
   * @param {Event} e - Form submit event
   */
  const handleAddUser = async (e) => {
    e.preventDefault();
    
    try {
      const response = await fetch('/api/auth/users', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(formData)
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to add user: ${response.status} ${response.statusText}`);
      }
      
      // Close modal and refresh users
      setShowAddModal(false);
      showStatusMessage('User added successfully', 'success', 5000);
      fetchUsers();
    } catch (err) {
      console.error('Error adding user:', err);
      showStatusMessage(`Error adding user: ${err.message}`, 'error', 8000);
    }
  };

  /**
   * Handle form submission for editing a user
   * @param {Event} e - Form submit event
   */
  const handleEditUser = async (e) => {
    e.preventDefault();
    
    try {
      const response = await fetch(`/api/auth/users/${currentUser.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(formData)
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to update user: ${response.status} ${response.statusText}`);
      }
      
      // Close modal and refresh users
      setShowEditModal(false);
      showStatusMessage('User updated successfully', 'success', 5000);
      fetchUsers();
    } catch (err) {
      console.error('Error updating user:', err);
      showStatusMessage(`Error updating user: ${err.message}`, 'error', 8000);
    }
  };

  /**
   * Handle user deletion
   */
  const handleDeleteUser = async () => {
    try {
      const response = await fetch(`/api/auth/users/${currentUser.id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to delete user: ${response.status} ${response.statusText}`);
      }
      
      // Close modal and refresh users
      setShowDeleteModal(false);
      showStatusMessage('User deleted successfully', 'success', 5000);
      fetchUsers();
    } catch (err) {
      console.error('Error deleting user:', err);
      showStatusMessage(`Error deleting user: ${err.message}`, 'error', 8000);
    }
  };

  /**
   * Handle generating a new API key for a user
   */
  const handleGenerateApiKey = async () => {
    try {
      const response = await fetch(`/api/auth/users/${currentUser.id}/api-key`, {
        method: 'POST'
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to generate API key: ${response.status} ${response.statusText}`);
      }
      
      const data = await response.json();
      setNewApiKey(data.api_key);
      showStatusMessage('API key generated successfully', 'success', 5000);
      fetchUsers(); // Refresh users to get the updated API key
    } catch (err) {
      console.error('Error generating API key:', err);
      showStatusMessage(`Error generating API key: ${err.message}`, 'error', 8000);
    }
  };

  /**
   * Open the edit modal for a user
   * @param {Object} user - User to edit
   */
  const openEditModal = (user) => {
    setCurrentUser(user);
    setFormData({
      username: user.username,
      password: '', // Don't include the password in the form
      email: user.email || '',
      role: user.role,
      is_active: user.is_active
    });
    setShowEditModal(true);
  };

  /**
   * Open the delete modal for a user
   * @param {Object} user - User to delete
   */
  const openDeleteModal = (user) => {
    setCurrentUser(user);
    setShowDeleteModal(true);
  };

  /**
   * Open the API key modal for a user
   * @param {Object} user - User to generate API key for
   */
  const openApiKeyModal = (user) => {
    setCurrentUser(user);
    setNewApiKey('');
    setShowApiKeyModal(true);
  };

  /**
   * Copy API key to clipboard
   */
  const copyApiKey = () => {
    navigator.clipboard.writeText(newApiKey)
      .then(() => {
        showStatusMessage('API key copied to clipboard', 'success', 5000);
      })
      .catch((err) => {
        console.error('Error copying API key:', err);
        showStatusMessage('Failed to copy API key', 'error', 8000);
      });
  };

  /**
   * Render the add user modal
   * @returns {JSX.Element} Add user modal
   */
  const renderAddUserModal = () => {
    return html`
      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Add User</h3>
            <button 
              class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              onClick=${() => setShowAddModal(false)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <form onSubmit=${handleAddUser}>
            <div class="p-4">
              <div class="mb-4">
                <label for="username" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Username</label>
                <input 
                  type="text" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="username" 
                  name="username" 
                  value=${formData.username} 
                  onChange=${handleInputChange} 
                  required 
                  minlength="3" 
                  maxlength="32"
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Username must be between 3 and 32 characters</p>
              </div>
              <div class="mb-4">
                <label for="password" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Password</label>
                <input 
                  type="password" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="password" 
                  name="password" 
                  value=${formData.password} 
                  onChange=${handleInputChange} 
                  required 
                  minlength="8"
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Password must be at least 8 characters</p>
              </div>
              <div class="mb-4">
                <label for="email" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Email (Optional)</label>
                <input 
                  type="email" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="email" 
                  name="email" 
                  value=${formData.email} 
                  onChange=${handleInputChange}
                />
              </div>
              <div class="mb-4">
                <label for="role" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Role</label>
                <select 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="role" 
                  name="role" 
                  value=${formData.role} 
                  onChange=${handleInputChange}
                >
                  <option value="0">Admin</option>
                  <option value="1">User</option>
                  <option value="2">Viewer</option>
                  <option value="3">API</option>
                </select>
              </div>
              <div class="mb-4 flex items-center">
                <input 
                  type="checkbox" 
                  class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600" 
                  id="is_active" 
                  name="is_active" 
                  checked=${formData.is_active} 
                  onChange=${handleInputChange}
                />
                <label class="ml-2 text-gray-700 dark:text-gray-300" for="is_active">Active</label>
              </div>
            </div>
            <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
              <button 
                type="button" 
                class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${() => setShowAddModal(false)}
              >
                Cancel
              </button>
              <button 
                type="submit" 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
              >
                Add User
              </button>
            </div>
          </form>
        </div>
      </div>
    `;
  };

  /**
   * Render the edit user modal
   * @returns {JSX.Element} Edit user modal
   */
  const renderEditUserModal = () => {
    return html`
      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Edit User: ${currentUser.username}</h3>
            <button 
              class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              onClick=${() => setShowEditModal(false)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <form onSubmit=${handleEditUser}>
            <div class="p-4">
              <div class="mb-4">
                <label for="username" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Username</label>
                <input 
                  type="text" 
                  class="w-full p-2 border border-gray-300 rounded bg-gray-100 dark:bg-gray-600 dark:border-gray-600 dark:text-gray-300" 
                  id="username" 
                  value=${currentUser.username} 
                  disabled
                />
              </div>
              <div class="mb-4">
                <label for="password" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Password (leave blank to keep current)</label>
                <input 
                  type="password" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="password" 
                  name="password" 
                  value=${formData.password} 
                  onChange=${handleInputChange} 
                  minlength="8"
                />
                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Password must be at least 8 characters or leave blank to keep current</p>
              </div>
              <div class="mb-4">
                <label for="email" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Email (Optional)</label>
                <input 
                  type="email" 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="email" 
                  name="email" 
                  value=${formData.email} 
                  onChange=${handleInputChange}
                />
              </div>
              <div class="mb-4">
                <label for="role" class="block mb-1 font-medium text-gray-700 dark:text-gray-300">Role</label>
                <select 
                  class="w-full p-2 border border-gray-300 rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                  id="role" 
                  name="role" 
                  value=${formData.role} 
                  onChange=${handleInputChange}
                >
                  <option value="0">Admin</option>
                  <option value="1">User</option>
                  <option value="2">Viewer</option>
                  <option value="3">API</option>
                </select>
              </div>
              <div class="mb-4 flex items-center">
                <input 
                  type="checkbox" 
                  class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600" 
                  id="is_active" 
                  name="is_active" 
                  checked=${formData.is_active} 
                  onChange=${handleInputChange}
                />
                <label class="ml-2 text-gray-700 dark:text-gray-300" for="is_active">Active</label>
              </div>
            </div>
            <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
              <button 
                type="button" 
                class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
                onClick=${() => setShowEditModal(false)}
              >
                Cancel
              </button>
              <button 
                type="submit" 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
              >
                Save Changes
              </button>
            </div>
          </form>
        </div>
      </div>
    `;
  };

  /**
   * Render the delete user modal
   * @returns {JSX.Element} Delete user modal
   */
  const renderDeleteUserModal = () => {
    return html`
      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Delete User</h3>
            <button 
              class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              onClick=${() => setShowDeleteModal(false)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-4">
            <p class="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to delete the user <strong>${currentUser.username}</strong>?
            </p>
            <p class="mb-4 text-red-600 dark:text-red-400 font-medium">
              This action cannot be undone.
            </p>
          </div>
          <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
            <button 
              type="button" 
              class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
              onClick=${() => setShowDeleteModal(false)}
            >
              Cancel
            </button>
            <button 
              type="button" 
              class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
              onClick=${handleDeleteUser}
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    `;
  };

  /**
   * Render the API key modal
   * @returns {JSX.Element} API key modal
   */
  const renderApiKeyModal = () => {
    return html`
      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <div class="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">API Key for ${currentUser.username}</h3>
            <button 
              class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              onClick=${() => setShowApiKeyModal(false)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="p-4">
            ${newApiKey ? html`
              <div class="mb-4">
                <label class="block mb-1 font-medium text-gray-700 dark:text-gray-300">API Key</label>
                <div class="flex">
                  <input 
                    type="text" 
                    class="flex-grow p-2 border border-gray-300 rounded-l dark:bg-gray-700 dark:border-gray-600 dark:text-white" 
                    value=${newApiKey} 
                    readonly 
                  />
                  <button 
                    class="px-3 py-2 bg-gray-200 text-gray-700 border border-gray-300 rounded-r hover:bg-gray-300 transition-colors dark:bg-gray-600 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-500"
                    type="button" 
                    onClick=${copyApiKey}
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
                    </svg>
                  </button>
                </div>
                <p class="mt-2 text-sm text-yellow-600 dark:text-yellow-400 font-medium">
                  Save this key now. For security reasons, it will not be displayed again.
                </p>
              </div>
            ` : html`
              <p class="mb-2 text-gray-700 dark:text-gray-300">Generate a new API key for ${currentUser.username}?</p>
              <p class="mb-4 text-yellow-600 dark:text-yellow-400 font-medium">This will invalidate any existing API key for this user.</p>
            `}
          </div>
          <div class="p-4 border-t border-gray-200 dark:border-gray-700 flex justify-end space-x-3">
            <button 
              type="button" 
              class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
              onClick=${() => setShowApiKeyModal(false)}
            >
              Close
            </button>
            ${!newApiKey && html`
              <button 
                type="button" 
                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                onClick=${handleGenerateApiKey}
              >
                Generate Key
              </button>
            `}
          </div>
        </div>
      </div>
    `;
  };

  /**
   * Render the users table
   * @returns {JSX.Element} Users table
   */
  const renderUsersTable = () => {
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
                      onClick=${() => openEditModal(user)}
                      title="Edit User"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                      </svg>
                    </button>
                    <button 
                      class="p-1 text-red-600 hover:text-red-800 rounded hover:bg-red-100 transition-colors"
                      onClick=${() => openDeleteModal(user)}
                      title="Delete User"
                    >
                      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                    <button 
                      class="p-1 text-gray-600 hover:text-gray-800 rounded hover:bg-gray-100 transition-colors"
                      onClick=${() => openApiKeyModal(user)}
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
  };

  // Render loading state
  if (loading && users.length === 0) {
    return html`
      <div class="flex justify-center items-center p-8">
        <div class="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        <span class="sr-only">Loading...</span>
      </div>
    `;
  }

  // Render error state
  if (error) {
    return html`
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">Error</h4>
        <p class="mb-4">${error}</p>
        <hr class="border-red-300 my-4" />
        <button 
          class="px-4 py-2 border border-red-500 text-red-500 rounded hover:bg-red-500 hover:text-white transition-colors"
          onClick=${fetchUsers}
        >
          Retry
        </button>
      </div>
    `;
  }

  // Render empty state
  if (users.length === 0) {
    return html`
      <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">No Users Found</h4>
        <p>Click the "Add User" button to create your first user.</p>
      </div>
      ${showAddModal && renderAddUserModal()}
    `;
  }

  // Render users table
  return html`
    <div>
      ${renderUsersTable()}
      ${showAddModal && renderAddUserModal()}
      ${showEditModal && renderEditUserModal()}
      ${showDeleteModal && renderDeleteUserModal()}
      ${showApiKeyModal && renderApiKeyModal()}
    </div>
  `;
}

/**
 * Load the UsersView component
 */
export function loadUsersView() {
  const container = document.getElementById('users-container');
  if (!container) return;
  
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${UsersView} />`, container);
  });
}
