/**
 * LightNVR Web Interface Users View Component
 * Preact component for the user management page
 */

import { h } from '../../preact.min.js';
import { useState, useEffect } from '../../preact.hooks.module.js';
import { html } from '../../html-helper.js';
import { showStatusMessage } from './UI.js';

// Import user components
import { USER_ROLES } from './users/UserRoles.js';
import { UsersTable } from './users/UsersTable.js';
import { AddUserModal } from './users/AddUserModal.js';
import { EditUserModal } from './users/EditUserModal.js';
import { DeleteUserModal } from './users/DeleteUserModal.js';
import { ApiKeyModal } from './users/ApiKeyModal.js';

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
      const handleAddUserClick = () => {
        setFormData({
          username: '',
          password: '',
          email: '',
          role: 1,
          is_active: true
        });
        setShowAddModal(true);
      };
      
      addUserBtn.addEventListener('click', handleAddUserClick);
      
      return () => {
        if (addUserBtn) {
          addUserBtn.removeEventListener('click', handleAddUserClick);
        }
      };
    }
  }, []);

  /**
   * Fetch users from the API
   */
  const fetchUsers = async () => {
    setLoading(true);
    try {
      // Get auth from localStorage
      const auth = localStorage.getItem('auth');
      
      const response = await fetch('/api/auth/users', {
        headers: {
          ...(auth ? { 'Authorization': 'Basic ' + auth } : {})
        },
        // Add cache-busting parameter
        cache: 'no-store'
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch users: ${response.status}`);
      }
      
      const data = await response.json();
      setUsers(data.users || []); // Extract users array from response
    } catch (error) {
      console.error('Error fetching users:', error);
      setError('Failed to load users. Please try again.');
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
    if (e) e.preventDefault();
    
    try {
      console.log('Adding user:', formData.username);
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
      
      const result = await response.json();
      console.log('User added successfully:', result);
      
      // Close modal first
      setShowAddModal(false);
      
      // Show success message
      showStatusMessage('User added successfully', 'success', 5000);
      
      // Wait a moment before refreshing to ensure the API operation has completed
      setTimeout(() => {
        console.log('Refreshing users after adding user');
        fetchUsers();
      }, 1000);
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
    if (e) e.preventDefault();
    
    try {
      console.log('Editing user:', currentUser.id, currentUser.username);
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
      
      const result = await response.json();
      console.log('User updated successfully:', result);
      
      // Close modal first
      setShowEditModal(false);
      
      // Show success message
      showStatusMessage('User updated successfully', 'success', 5000);
      
      // Wait a moment before refreshing to ensure the API operation has completed
      setTimeout(() => {
        console.log('Refreshing users after editing user');
        fetchUsers();
      }, 1000);
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
      console.log('Deleting user:', currentUser.id, currentUser.username);
      const response = await fetch(`/api/auth/users/${currentUser.id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to delete user: ${response.status} ${response.statusText}`);
      }
      
      console.log('User deleted successfully');
      
      // Close modal first
      setShowDeleteModal(false);
      
      // Show success message
      showStatusMessage('User deleted successfully', 'success', 5000);
      
      // Fetch users immediately after the operation completes
      await fetchUsers();
    } catch (err) {
      console.error('Error deleting user:', err);
      showStatusMessage(`Error deleting user: ${err.message}`, 'error', 8000);
    }
  };

  /**
   * Handle generating a new API key for a user
   */
  const handleGenerateApiKey = async () => {
    // Show loading state
    setNewApiKey('Generating...');
    
    try {
      console.log('Generating API key for user:', currentUser.id, currentUser.username);
      const response = await fetch(`/api/auth/users/${currentUser.id}/api-key`, {
        method: 'POST'
      });
      
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || `Failed to generate API key: ${response.status} ${response.statusText}`);
      }
      
      const data = await response.json();
      console.log('API key generated successfully');
      setNewApiKey(data.api_key);
      
      // Show success message
      showStatusMessage('API key generated successfully', 'success');
      
      // Wait a moment before refreshing to ensure the API operation has completed
      setTimeout(() => {
        console.log('Refreshing users after generating API key');
        fetchUsers(); // Refresh users to get the updated API key
      }, 1000);
    } catch (err) {
      console.error('Error generating API key:', err);
      
      // Reset the API key state
      setNewApiKey('');
      
      // Show error message
      showStatusMessage(`Error generating API key: ${err.message}`, 'error');
    }
  };

  /**
   * Copy API key to clipboard
   */
  const copyApiKey = () => {
    navigator.clipboard.writeText(newApiKey)
      .then(() => {
        // Use global toast function if available
        if (window.showSuccessToast) {
          window.showSuccessToast('API key copied to clipboard');
        } else {
          // Fallback to standard showStatusMessage
          showStatusMessage('API key copied to clipboard', 'success');
        }
      })
      .catch((err) => {
        console.error('Error copying API key:', err);
        
        // Use global toast function if available
        if (window.showErrorToast) {
          window.showErrorToast('Failed to copy API key');
        } else {
          // Fallback to standard showStatusMessage
          showStatusMessage('Failed to copy API key', 'error');
        }
      });
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
   * Close the add user modal
   */
  const closeAddModal = () => {
    setShowAddModal(false);
  };

  /**
   * Close the edit user modal
   */
  const closeEditModal = () => {
    setShowEditModal(false);
  };

  /**
   * Close the delete user modal
   */
  const closeDeleteModal = () => {
    setShowDeleteModal(false);
  };

  /**
   * Close the API key modal
   */
  const closeApiKeyModal = () => {
    setShowApiKeyModal(false);
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

  // Render empty state
  if (users.length === 0) {
    return html`
      <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">No Users Found</h4>
        <p>Click the "Add User" button to create your first user.</p>
      </div>
      ${showAddModal && html`<${AddUserModal} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleAddUser=${handleAddUser} 
        onClose=${closeAddModal} 
      />`}
    `;
  }

  // Render users table with modals
  return html`
    <div>
      <${UsersTable} 
        users=${users} 
        onEdit=${openEditModal} 
        onDelete=${openDeleteModal} 
        onApiKey=${openApiKeyModal} 
      />
      
      ${showAddModal && html`<${AddUserModal} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleAddUser=${handleAddUser} 
        onClose=${closeAddModal} 
      />`}
      
      ${showEditModal && html`<${EditUserModal} 
        currentUser=${currentUser} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleEditUser=${handleEditUser} 
        onClose=${closeEditModal} 
      />`}
      
      ${showDeleteModal && html`<${DeleteUserModal} 
        currentUser=${currentUser} 
        handleDeleteUser=${handleDeleteUser} 
        onClose=${closeDeleteModal} 
      />`}
      
      ${showApiKeyModal && html`<${ApiKeyModal} 
        currentUser=${currentUser} 
        newApiKey=${newApiKey} 
        handleGenerateApiKey=${handleGenerateApiKey} 
        copyApiKey=${copyApiKey} 
        onClose=${closeApiKeyModal} 
      />`}
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
