import { administrationJSON, saveAdminUser } from '../../api/administration.ts';
/**
 * LightNVR Web Interface Users View Component
 * Preact component for the user management page
 */

import { useState, useCallback, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.tsx';
import { useQuery, useMutation } from '../../query-client.ts';
import { useI18n } from '../../i18n.ts';

// Import user components

import { UsersTable } from './users/UsersTable.tsx';
import { AddUserModal } from './users/AddUserModal.tsx';
import { EditUserModal } from './users/EditUserModal.tsx';
import { DeleteUserModal } from './users/DeleteUserModal.tsx';
import { ApiKeyModal } from './users/ApiKeyModal.tsx';
import { TotpSetupModal } from './users/TotpSetupModal.tsx';
import { AccessPolicyModal } from './users/AccessPolicyModal.tsx';
import { RoleManagerModal } from './users/RoleManagerModal.tsx';
import { AuditHistoryModal } from './users/AuditHistoryModal.tsx';
import type { ComponentChildren } from 'preact';
import type { User } from '../../types/core.ts';
import type {
  ApiRequestError,
  ClearLoginLockoutResponse,
  UserFormData,
  UsersResponse,
  UserWritePayload,
} from '../../types/users.ts';

type UsersModal = 'add' | 'edit' | 'delete' | 'apiKey' | 'totp' | 'access' | 'roles' | 'audit';

/*
 * selectedUser below is non-null wherever it carries `!`: each openXModal sets it
 * together with its activeModal value, and the 'roles' and 'audit' modals, which
 * clear it, do not read it.
 */

/**
 * UsersView component
 * @returns {JSX.Element} UsersView component
 */
export function UsersView() {
  const { t } = useI18n();

  // State for modal visibility
  const [activeModal, setActiveModal] = useState<UsersModal | null>(null); // add, edit, delete, apiKey, totp, access, roles, audit, or null

  // State for the user targeted by the active management modal.
  const [selectedUser, setSelectedUser] = useState<User | null>(null);

  // Form state for adding/editing users
  const [formData, setFormData] = useState<UserFormData>({
    username: '',
    password: '',
    email: '',
    role: 1,
    is_active: true,
    password_change_locked: false,
    allowed_login_cidrs: ''
  });

  /**
   * Get auth headers for requests
   * @returns {Object} Headers object with Authorization if available
   */
  const getAuthHeaders = useCallback((): Record<string, string> => {
    const auth = localStorage.getItem('auth');
    return auth ? { 'Authorization': 'Basic ' + auth } : {};
  }, []);

  // Fetch users using useQuery
  const {
    data: usersData,
    isLoading: loading,
    error,
    refetch: refetchUsers
  } = useQuery<UsersResponse, ApiRequestError>(
    {queryKey: ['users'], queryFn: ({signal}) => administrationJSON<UsersResponse>('/api/auth/users', {headers:getAuthHeaders(), signal}), retry: false}
  );

  // Extract users array from response
  const users = usersData?.users || [];

  const renderPageHeader = (actionButton: ComponentChildren = null) => (
    <div className="page-header flex justify-between items-center mb-4 p-4 bg-card text-card-foreground rounded-lg shadow">
      <h2 className="text-xl font-semibold">{t('users.userManagement')}</h2>
      {actionButton}
    </div>
  );

  // Handler for the add user button
  const handleAddUserClick = useCallback(() => {
    // Reset form data for new user
    setFormData({
      username: '',
      password: '',
      email: '',
      role: 1,
      is_active: true,
      password_change_locked: false,
      allowed_login_cidrs: ''
    });
    setActiveModal('add');
  }, []);

  /**
   * Handle form input changes
   * @param {Event} e - Input change event
   */
  const handleInputChange = useCallback((e: Event) => {
    const { name, value, type, checked } = e.target as HTMLInputElement;

    setFormData(prevData => ({
      ...prevData,
      [name]: type === 'checkbox' ? checked : (name === 'role' ? parseInt(value, 10) : value)
    }));
  }, []);

  // Add user mutation
  const addUserMutation = useMutation<unknown, Error, UserWritePayload>({
    mutationFn: async (userData) => {
      return await saveAdminUser('/api/auth/users', {...userData}, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        },
        timeout: 15000, // 15 second timeout
        retries: 0,     // Never repeat a mutation automatically
        retryDelay: 1000 // 1 second between retries
      });
    },
    onSuccess: () => {
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage(t('users.userAdded'), 'success', 5000);

      // Refresh users list
      refetchUsers();
    },
    onError: (error) => {
      refetchUsers();
      showStatusMessage(t('users.userAddError', { message: error.message }), 'error', 8000);
    }
  });

  // Edit user mutation
  const editUserMutation = useMutation<unknown, Error, { userId: number; userData: UserWritePayload }>({
    mutationFn: async ({ userId, userData }) => {
      return await saveAdminUser(`/api/auth/users/${userId}`, {...userData}, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        },
        timeout: 15000, // 15 second timeout
        retries: 0,     // Never repeat a mutation automatically
        retryDelay: 1000 // 1 second between retries
      });
    },
    onSuccess: () => {
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage(t('users.userUpdated'), 'success', 5000);

      // Refresh users list
      refetchUsers();
    },
    onError: (error) => {
      refetchUsers();
      showStatusMessage(t('users.userUpdateError', { message: error.message }), 'error', 8000);
    }
  });

  // Delete user mutation
  const deleteUserMutation = useMutation<unknown, Error, number>({
    mutationFn: async (userId) => {
      return await administrationJSON(`/api/auth/users/${userId}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
        timeout: 15000, // 15 second timeout
        retries: 0,     // Never repeat a mutation automatically
        retryDelay: 1000 // 1 second between retries
      });
    },
    onSuccess: () => {
      // Close modal and show success message
      setActiveModal(null);
      showStatusMessage(t('users.userDeleted'), 'success', 5000);

      // Refresh users list
      refetchUsers();
    },
    onError: (error) => {
      console.error('Error deleting user:', error);
      showStatusMessage(t('users.userDeleteError', { message: error.message }), 'error', 8000);
    }
  });

  const clearLoginLockoutInFlight = useRef(false);
  const clearLoginLockoutMutation = useMutation<ClearLoginLockoutResponse, Error, number>({
    mutationFn: async (userId) => {
      return await administrationJSON<ClearLoginLockoutResponse>(`/api/auth/users/${userId}/login-lockout/clear`, {
        method: 'POST',
        headers: getAuthHeaders(),
        timeout: 15000,
        retries: 0,
        retryDelay: 1000
      });
    },
    onSuccess: (data) => {
      showStatusMessage(data?.message || t('users.loginLockoutCleared'), 'success', 5000);
      refetchUsers();
    },
    onError: (error) => {
      console.error('Error clearing login lockout:', error);
      showStatusMessage(t('users.loginLockoutClearError', { message: error.message }), 'error', 8000);
    },
    onSettled: () => { clearLoginLockoutInFlight.current = false; }
  });

  const { mutateAsync: addUserMutateAsync } = addUserMutation;

  /**
   * Handle form submission for adding a user.
   *
   * Returns the mutation's promise so the AddUserModal's <AsyncButton>
   * can disable the submit button + show a spinner while the POST is in
   * flight, preventing the #399 rapid-tap duplicate-save bug.
   */
  const handleAddUser = useCallback((e: Event) => {
    if (e && typeof e.preventDefault === 'function') e.preventDefault();

    const userData: UserWritePayload = {
      ...formData,
      allowed_login_cidrs: formData.allowed_login_cidrs?.trim() || null
    };
    return addUserMutateAsync(userData);
  }, [formData, addUserMutateAsync]);

  /**
   * Handle form submission for editing a user.
   *
   * Returns the mutation's promise so the EditUserModal's <AsyncButton>
   * can lock the submit button while the PUT is in flight (#399).
   */
  const handleEditUser = useCallback((e: Event) => {
    if (e && typeof e.preventDefault === 'function') e.preventDefault();

    const userData: UserWritePayload = {
      ...formData,
      allowed_login_cidrs: formData.allowed_login_cidrs?.trim() || null
    };
    // Numeric legacy roles are never writable here; role grants have a separate editor.
    delete userData.role;
    return editUserMutation.mutateAsync({
      userId: selectedUser!.id,
      userData
    });
  }, [selectedUser, formData, editUserMutation]);

  /**
   * Handle user deletion. Returns the mutation's promise so the child
   * DeleteUserModal's <AsyncButton> can disable itself + spin while the
   * DELETE is in flight (#399 / PRD UXD_01 §5.1 / T1).
   */
  const handleDeleteUser = useCallback(() => {
    return deleteUserMutation.mutateAsync(selectedUser!.id);
  }, [selectedUser, deleteUserMutation]);

  const handleClearLoginLockout = useCallback(() => {
    if (!selectedUser?.id || clearLoginLockoutInFlight.current) return;
    // Gate repeat events before the observer's pending render reaches the button.
    clearLoginLockoutInFlight.current = true;

    clearLoginLockoutMutation.mutate(selectedUser.id);
  }, [selectedUser, clearLoginLockoutMutation]);

  /**
   * Open the edit modal for a user
   * @param {Object} user - User to edit
   */
  const openEditModal = useCallback((user: User) => {
    setSelectedUser(user);
    setFormData({
      username: user.username,
      password: '', // Don't include the password in the form
      email: user.email || '',
      role: user.role,
      is_active: user.is_active,
      password_change_locked: user.password_change_locked || false,
      allowed_login_cidrs: user.allowed_login_cidrs || ''
    });
    setActiveModal('edit');
  }, []);

  /**
   * Open the delete modal for a user
   * @param {Object} user - User to delete
   */
  const openDeleteModal = useCallback((user: User) => {
    setSelectedUser(user);
    setActiveModal('delete');
  }, []);

  /**
   * Open API access management for a user.
   * @param {Object} user - User whose tokens should be managed
   */
  const openApiKeyModal = useCallback((user: User) => {
    setSelectedUser(user);
    setActiveModal('apiKey');
  }, []);

  /**
   * Open the TOTP MFA modal for a user
   * @param {Object} user - User to configure TOTP for
   */
  const openTotpModal = useCallback((user: User) => {
    setSelectedUser(user);
    setActiveModal('totp');
  }, []);

  const openAccessModal = useCallback((user: User) => {
    setSelectedUser(user);
    setActiveModal('access');
  }, []);

  const openRoleManager = useCallback(() => {
    setSelectedUser(null);
    setActiveModal('roles');
  }, []);

  const openAuditHistory = useCallback(() => {
    setSelectedUser(null);
    setActiveModal('audit');
  }, []);

  const pageActions = (
    <div className="flex flex-wrap gap-2">
      <button className="btn-secondary" onClick={openAuditHistory}>{t('audit.manage')}</button>
      <button className="btn-secondary" onClick={openRoleManager}>{t('access.roles.manage')}</button>
      <button className="btn-primary" onClick={handleAddUserClick}>{t('users.addUser')}</button>
    </div>
  );

  /**
   * Close any open modal
   */
  const closeModal = useCallback(() => {
    setActiveModal(null);
  }, []);

  // Render loading state
  if (loading && users.length === 0) {
    return (
      <div className="flex justify-center items-center p-8">
        <div className="w-12 h-12 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        <span className="sr-only">{t('common.loading')}</span>
      </div>
    );
  }

  // Helper to check if error is auth-related
  const isAuthError = error && (error.status === 401 || error.status === 403);
  const isForbiddenError = error && error.status === 403;
  const isUnauthorizedError = error && error.status === 401;

  // Render error state for auth issues
  if (isAuthError && users.length === 0) {
    return (
      <div>
        {renderPageHeader()}

        <div className={`border px-4 py-3 rounded relative mb-4 ${isForbiddenError ? 'bg-yellow-100 border-yellow-400 text-yellow-700 dark:bg-yellow-900 dark:border-yellow-600 dark:text-yellow-200' : 'bg-red-100 border-red-400 text-red-700 dark:bg-red-900 dark:border-red-600 dark:text-red-200'}`}>
          <h4 className="font-bold mb-2">
              {isForbiddenError ? t('users.accessDenied') : t('auth.authenticationRequired')}
          </h4>
          <p>
            {isForbiddenError
              ? t('users.accessDeniedDescription')
              : t('auth.sessionExpiredLoginAgain')}
          </p>
          {isUnauthorizedError && (
            <button
              className="mt-3 btn-primary"
              onClick={() => window.location.href = '/login.html'}
            >
              {t('auth.goToLogin')}
            </button>
          )}
        </div>
      </div>
    );
  }

  // Render generic error state
  if (error && users.length === 0) {
    return (
      <div>
        {renderPageHeader(
          <button
            className="btn-primary"
            onClick={() => refetchUsers()}
          >
            {t('common.retry')}
          </button>
        )}

        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4 dark:bg-red-900 dark:border-red-600 dark:text-red-200">
          <h4 className="font-bold mb-2">{t('users.errorLoadingUsers')}</h4>
          <p>{error.message || t('users.errorLoadingUsersDescription')}</p>
        </div>
      </div>
    );
  }

  // Render empty state
  if (users.length === 0 && !loading) {
    return (
      <div>
        {renderPageHeader(pageActions)}

        <div className="badge-info border px-4 py-3 rounded relative mb-4">
          <h4 className="font-bold mb-2">{t('users.noUsersFound')}</h4>
          <p>{t('users.noUsersDescription')}</p>
        </div>
        {activeModal === 'add' && (
          <AddUserModal
            formData={formData}
            handleInputChange={handleInputChange}
            handleAddUser={handleAddUser}
            onClose={closeModal}
          />
        )}
        {activeModal === 'roles' && (
          <RoleManagerModal onClose={closeModal} getAuthHeaders={getAuthHeaders} />
        )}
        {activeModal === 'audit' && (
          <AuditHistoryModal users={users} onClose={closeModal} getAuthHeaders={getAuthHeaders} />
        )}
      </div>
    );
  }

  // Render users table with modals
  return (
    <div>
      {renderPageHeader(pageActions)}

      <UsersTable
        users={users}
        onEdit={openEditModal}
        onDelete={openDeleteModal}
        onApiKey={openApiKeyModal}
        onMfa={openTotpModal}
        onAccess={openAccessModal}
      />

      {activeModal === 'add' && (
        <AddUserModal
          formData={formData}
          handleInputChange={handleInputChange}
          handleAddUser={handleAddUser}
          onClose={closeModal}
        />
      )}

      {activeModal === 'edit' && (
        <EditUserModal
          currentUser={selectedUser!}
          formData={formData}
          handleInputChange={handleInputChange}
          handleEditUser={handleEditUser}
          handleClearLoginLockout={handleClearLoginLockout}
          isClearingLoginLockout={clearLoginLockoutMutation.isLoading}
          showRoleField={selectedUser!.authorization_mode !== 'policy'}
          onClose={closeModal}
        />
      )}

      {activeModal === 'delete' && (
        <DeleteUserModal
          currentUser={selectedUser!}
          handleDeleteUser={handleDeleteUser}
          onClose={closeModal}
        />
      )}

      {activeModal === 'apiKey' && (
        <ApiKeyModal
          currentUser={selectedUser!}
          getAuthHeaders={getAuthHeaders}
          onClose={closeModal}
        />
      )}

      {activeModal === 'totp' && (
        <TotpSetupModal
          user={selectedUser!}
          onClose={closeModal}
          onSuccess={refetchUsers}
          getAuthHeaders={getAuthHeaders}
        />
      )}

      {activeModal === 'access' && selectedUser && (
        <AccessPolicyModal
          user={selectedUser}
          onClose={closeModal}
          getAuthHeaders={getAuthHeaders}
        />
      )}

      {activeModal === 'roles' && (
        <RoleManagerModal
          onClose={closeModal}
          getAuthHeaders={getAuthHeaders}
        />
      )}

      {activeModal === 'audit' && (
        <AuditHistoryModal
          users={users}
          onClose={closeModal}
          getAuthHeaders={getAuthHeaders}
        />
      )}
    </div>
  );
}
