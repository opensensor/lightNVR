/**
 * LightNVR Web Interface Users View Component
 * Preact component for the user management page
 */

import { h } from '../../preact.min.js';
import { useState, useEffect, useRef, useCallback } from '../../preact.hooks.module.js';
import { html } from '../../html-helper.js';
import { showStatusMessage } from './UI.js';
import { fetchJSON, enhancedFetch, createRequestController } from '../../fetch-utils.js';

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
  // State for users data and loading status
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  // State for modal visibility
  const [activeModal, setActiveModal] = useState(null); // 'add', 'edit', 'delete', 'apiKey', or null
  
  // State for selected user and API key
  const [selectedUser, setSelectedUser] = useState(null);
  const [apiKey, setApiKey] = useState('');
  
  // Form state for adding/editing users
  const [formData, setFormData] = useState({
    username: '',
    password: '',
    email: '',
    role: 1,
    is_active: true
  });

  // Request controller for cancelling requests on unmount
  const requestControllerRef = useRef(null);

  /**
   * Get auth headers for requests
   * @returns {Object} Headers object with Authorization if available
   */
  const getAuthHeaders = useCallback(() => {
    const auth = localStorage.getItem('auth');
    return auth ? { 'Authorization': 'Basic ' + auth } : {};
  }, []);

  /**
   * Fetch users from the API
   */
  const fetchUsers = useCallback(async () => {
    setLoading(true);
    try {
      const data = await fetchJSON('/api/auth/users', {
        headers: getAuthHeaders(),
        cache: 'no-store',
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 2,     // Retry twice
        retryDelay: 1000 // 1 second between retries
      });
      
      setUsers(data.users || []); // Extract users array from response
      setError(null);
    } catch (error) {
      // Only show error if the request wasn't cancelled
      if (error.message !== 'Request was cancelled') {
        console.error('Error fetching users:', error);
        setError('Failed to load users. Please try again.');
      }
    } finally {
      setLoading(false);
    }
  }, [getAuthHeaders]);

  // Fetch users on component mount
  useEffect(() => {
    // Create a new request controller
    requestControllerRef.current = createRequestController();
    
    fetchUsers();
    
    // Clean up and cancel pending requests on unmount
    return () => {
      if (requestControllerRef.current) {
        requestControllerRef.current.abort();
      }
    };
  }, [fetchUsers]);

  // Add event listener for the add user button
  useEffect(() => {
    const addUserBtn = document.getElementById('add-user-btn');
    if (addUserBtn) {
      const handleAddUserClick = () => {
        // Reset form data for new user
        setFormData({
          username: '',
          password: '',
          email: '',
          role: 1,
          is_active: true
        });
        setActiveModal('add');
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
   * Handle form input changes
   * @param {Event} e - Input change event
   */
  const handleInputChange = useCallback((e) => {
    const { name, value, type, checked } = e.target;
    
    setFormData(prevData => ({
      ...prevData,
      [name]: type === 'checkbox' ? checked : (name === 'role' ? parseInt(value, 10) : value)
    }));
  }, []);

  /**
   * Handle form submission for adding a user
   * @param {Event} e - Form submit event
   */
  const handleAddUser = useCallback(async (e) => {
    if (e) e.preventDefault();
    
    try {
      console.log('Adding user:', formData.username);
      
      await fetchJSON('/api/auth/users', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        },
        body: JSON.stringify(formData),
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage('User added successfully', 'success', 5000);
      
      // Refresh users list
      await fetchUsers();
    } catch (err) {
      console.error('Error adding user:', err);
      showStatusMessage(`Error adding user: ${err.message}`, 'error', 8000);
    }
  }, [formData, getAuthHeaders, fetchUsers]);

  /**
   * Handle form submission for editing a user
   * @param {Event} e - Form submit event
   */
  const handleEditUser = useCallback(async (e) => {
    if (e) e.preventDefault();
    
    try {
      console.log('Editing user:', selectedUser.id, selectedUser.username);
      
      await fetchJSON(`/api/auth/users/${selectedUser.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        },
        body: JSON.stringify(formData),
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage('User updated successfully', 'success', 5000);
      
      // Refresh users list
      await fetchUsers();
    } catch (err) {
      console.error('Error updating user:', err);
      showStatusMessage(`Error updating user: ${err.message}`, 'error', 8000);
    }
  }, [selectedUser, formData, getAuthHeaders, fetchUsers]);

  /**
   * Handle user deletion
   */
  const handleDeleteUser = useCallback(async () => {
    try {
      console.log('Deleting user:', selectedUser.id, selectedUser.username);
      
      await enhancedFetch(`/api/auth/users/${selectedUser.id}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
        signal: requestControllerRef.current?.signal,
        timeout: 15000, // 15 second timeout
        retries: 1,     // Retry once
        retryDelay: 1000 // 1 second between retries
      });
      
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage('User deleted successfully', 'success', 5000);
      
      // Refresh users list
      await fetchUsers();
    } catch (err) {
      console.error('Error deleting user:', err);
      showStatusMessage(`Error deleting user: ${err.message}`, 'error', 8000);
    }
  }, [selectedUser, getAuthHeaders, fetchUsers]);

  /**
   * Handle generating a new API key for a user
   */
  const handleGenerateApiKey = useCallback(async () => {
    // Show loading state
    setApiKey('Generating...');
    
    try {
      console.log('Generating API key for user:', selectedUser.id, selectedUser.username);
      
      const data = await fetchJSON(`/api/auth/users/${selectedUser.id}/api-key`, {
        method: 'POST',
        headers: getAuthHeaders(),
        signal: requestControllerRef.current?.signal,
        timeout: 20000, // 20 second timeout for key generation
        retries: 1,     // Retry once
        retryDelay: 2000 // 2 seconds between retries
      });
      
      setApiKey(data.api_key);
      showStatusMessage('API key generated successfully', 'success');
      
      // Refresh users list
      await fetchUsers();
    } catch (err) {
      console.error('Error generating API key:', err);
      setApiKey('');
      showStatusMessage(`Error generating API key: ${err.message}`, 'error');
    }
  }, [selectedUser, getAuthHeaders, fetchUsers]);

  /**
   * Copy API key to clipboard
   */
  const copyApiKey = useCallback(() => {
    navigator.clipboard.writeText(apiKey)
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
  }, [apiKey]);

  /**
   * Open the edit modal for a user
   * @param {Object} user - User to edit
   */
  const openEditModal = useCallback((user) => {
    setSelectedUser(user);
    setFormData({
      username: user.username,
      password: '', // Don't include the password in the form
      email: user.email || '',
      role: user.role,
      is_active: user.is_active
    });
    setActiveModal('edit');
  }, []);

  /**
   * Open the delete modal for a user
   * @param {Object} user - User to delete
   */
  const openDeleteModal = useCallback((user) => {
    setSelectedUser(user);
    setActiveModal('delete');
  }, []);

  /**
   * Open the API key modal for a user
   * @param {Object} user - User to generate API key for
   */
  const openApiKeyModal = useCallback((user) => {
    setSelectedUser(user);
    setApiKey('');
    setActiveModal('apiKey');
  }, []);

  /**
   * Close any open modal
   */
  const closeModal = useCallback(() => {
    setActiveModal(null);
  }, []);

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
  if (users.length === 0 && !loading) {
    return html`
      <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4">
        <h4 class="font-bold mb-2">No Users Found</h4>
        <p>Click the "Add User" button to create your first user.</p>
      </div>
      ${activeModal === 'add' && html`<${AddUserModal} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleAddUser=${handleAddUser} 
        onClose=${closeModal} 
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
      
      ${activeModal === 'add' && html`<${AddUserModal} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleAddUser=${handleAddUser} 
        onClose=${closeModal} 
      />`}
      
      ${activeModal === 'edit' && html`<${EditUserModal} 
        currentUser=${selectedUser} 
        formData=${formData} 
        handleInputChange=${handleInputChange} 
        handleEditUser=${handleEditUser} 
        onClose=${closeModal} 
      />`}
      
      ${activeModal === 'delete' && html`<${DeleteUserModal} 
        currentUser=${selectedUser} 
        handleDeleteUser=${handleDeleteUser} 
        onClose=${closeModal} 
      />`}
      
      ${activeModal === 'apiKey' && html`<${ApiKeyModal} 
        currentUser=${selectedUser} 
        newApiKey=${apiKey} 
        handleGenerateApiKey=${handleGenerateApiKey} 
        copyApiKey=${copyApiKey} 
        onClose=${closeModal} 
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
  
  import("../../preact.min.js").then(({ render }) => {
    render(html`<${UsersView} />`, container);
  });
}
