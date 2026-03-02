/**
 * Add User Modal Component
 */

import { USER_ROLES } from './UserRoles.js';

/**
 * Add User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.formData - Form data for adding a user
 * @param {Function} props.handleInputChange - Function to handle input changes
 * @param {Function} props.handleAddUser - Function to handle user addition
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} Add user modal
 */
export function AddUserModal({ formData, handleInputChange, handleAddUser, onClose }) {
  // Direct submit handler
  const handleSubmit = (e) => {
    e.preventDefault();
    e.stopPropagation(); // Stop event from bubbling up
    handleAddUser(e);
  };

  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">Add New User</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="username">
              Username
            </label>
            <input
              className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
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
              Password
            </label>
            <input
              className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
              id="password"
              type="password"
              name="password"
              value={formData.password}
              onChange={handleInputChange}
              required
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="email">
              Email
            </label>
            <input
              className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
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
              className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
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

          <div className="mb-4">
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

          <div className="mb-6">
            <label className="flex items-center">
              <input
                type="checkbox"
                name="password_change_locked"
                checked={formData.password_change_locked}
                onChange={handleInputChange}
                className="mr-2"
              />
              <span className="text-sm font-bold">Lock Password Changes</span>
            </label>
            <p className="text-xs text-muted-foreground mt-1 ml-6">
              When locked, this user cannot change their own password
            </p>
          </div>

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="allowed_tags">
              Allowed Tags <span className="font-normal text-muted-foreground">(RBAC)</span>
            </label>
            <input
              className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
              id="allowed_tags"
              type="text"
              name="allowed_tags"
              value={formData.allowed_tags || ''}
              onChange={handleInputChange}
              placeholder="e.g. outdoor,lobby (leave blank for unrestricted)"
              maxLength={255}
            />
            <p className="text-xs text-muted-foreground mt-1">
              Comma-separated tags. When set, this user can only see streams that share at least one matching tag. Leave blank to allow access to all streams.
            </p>
            {(formData.allowed_tags || '').split(',').filter(t => t.trim()).length > 0 && (
              <div className="mt-2 flex flex-wrap gap-1">
                {(formData.allowed_tags || '').split(',').filter(t => t.trim()).map(tag => (
                  <span key={tag.trim()} className="badge-info">
                    #{tag.trim()}
                  </span>
                ))}
              </div>
            )}
          </div>

          <div className="flex justify-end mt-6">
            <button
              type="button"
              className="btn-secondary mr-2"
              onClick={onClose}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              Add User
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
