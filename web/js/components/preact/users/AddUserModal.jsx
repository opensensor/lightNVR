/**
 * Add User Modal Component
 */

import { USER_ROLES } from './UserRoles.js';
import { useEffect, useRef } from 'preact/hooks';

const FOCUSABLE_SELECTORS = [
  'a[href]',
  'area[href]',
  'button:not([disabled])',
  'input:not([disabled]):not([type="hidden"])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])'
];

const FOCUSABLE_SELECTOR_QUERY = FOCUSABLE_SELECTORS.join(',');
const ALLOWED_LOGIN_CIDRS_PLACEHOLDER = `e.g.
192.168.1.0/24
2001:db8::/32
(leave blank for unrestricted)`;

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
  const dialogRef = useRef(null);
  const firstFieldRef = useRef(null);

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

  // Handle keyboard interactions within the dialog (Escape to close, Tab to trap focus)
  const handleDialogKeyDown = (e) => {
    if (e.key === 'Escape' || e.key === 'Esc') {
      e.stopPropagation();
      onClose();
      return;
    }

    if (e.key === 'Tab' && dialogRef.current) {
      const focusableElements = Array.from(
        dialogRef.current.querySelectorAll(FOCUSABLE_SELECTOR_QUERY)
      ).filter(
        (el) => el.getAttribute('aria-hidden') !== 'true'
      );

      if (focusableElements.length === 0) {
        return;
      }

      const firstEl = focusableElements[0];
      const lastEl = focusableElements[focusableElements.length - 1];
      const current = document.activeElement;

      if (!e.shiftKey && current === lastEl) {
        e.preventDefault();
        firstEl.focus();
      } else if (e.shiftKey && current === firstEl) {
        e.preventDefault();
        lastEl.focus();
      }
    }
  };

  // Move focus into the modal when it opens
  useEffect(() => {
    if (firstFieldRef.current && typeof firstFieldRef.current.focus === 'function') {
      firstFieldRef.current.focus();
    } else if (dialogRef.current && typeof dialogRef.current.focus === 'function') {
      dialogRef.current.focus();
    }
  }, []);

  return (
    <div
      className="fixed inset-0 z-50 overflow-y-auto bg-black/50 p-4"
      onClick={onClose}
    >
      <div className="flex min-h-full items-center justify-center">
        <div
          role="dialog"
          aria-modal="true"
          aria-labelledby="add-user-modal-title"
          className="bg-card text-card-foreground my-8 max-h-[calc(100vh-2rem)] w-full max-w-md overflow-y-auto rounded-lg p-6"
          onClick={stopPropagation}
          onKeyDown={handleDialogKeyDown}
          ref={dialogRef}
          tabIndex={-1}
        >
          <h2 id="add-user-modal-title" className="text-xl font-bold mb-4">Add New User</h2>

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
                aria-required="true"
                autoComplete="username"
                ref={firstFieldRef}
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
                aria-required="true"
                autoComplete="new-password"
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
                autoComplete="email"
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
              <label className="flex items-center" htmlFor="is_active">
                <input
                  type="checkbox"
                  id="is_active"
                  name="is_active"
                  checked={formData.is_active}
                  onChange={handleInputChange}
                  className="mr-2"
                />
                <span className="text-sm font-bold">Active</span>
              </label>
            </div>

            <div className="mb-6">
              <label className="flex items-center" htmlFor="password_change_locked">
                <input
                  id="password_change_locked"
                  type="checkbox"
                  name="password_change_locked"
                  checked={formData.password_change_locked}
                  onChange={handleInputChange}
                  className="mr-2"
                  aria-describedby="password-change-locked-description"
                />
                <span className="text-sm font-bold">Lock Password Changes</span>
              </label>
              <p
                id="password-change-locked-description"
                className="text-xs text-muted-foreground mt-1 ml-6"
              >
                When locked, this user cannot change their own password
              </p>
            </div>

            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="allowed_tags">
                Allowed Stream Tags <span className="font-normal text-muted-foreground">(RBAC)</span>
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
                aria-describedby="allowed-tags-description"
              />
              <p id="allowed-tags-description" className="text-xs text-muted-foreground mt-1">
                Comma-separated stream tags. When set, this user can only see streams that share at least one matching stream tag. Leave blank to allow access to all streams.
              </p>
              {(() => {
                const parsedAllowedTags = (formData.allowed_tags || '')
                  .split(',')
                  .map(t => t.trim())
                  .filter(Boolean);
                return parsedAllowedTags.length > 0 ? (
                  <ul
                    className="mt-2 flex flex-wrap gap-1"
                    role="list"
                    aria-label="Current allowed stream tags"
                  >
                    {parsedAllowedTags.map(tag => (
                      <li key={tag} className="badge-info">
                        #{tag}
                      </li>
                    ))}
                  </ul>
                ) : null;
              })()}
            </div>

            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="allowed_login_cidrs">
                Allowed Login IP Ranges <span className="font-normal text-muted-foreground">(CIDR)</span>
              </label>
              <textarea
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                id="allowed_login_cidrs"
                name="allowed_login_cidrs"
                value={formData.allowed_login_cidrs || ''}
                onChange={handleInputChange}
                placeholder={ALLOWED_LOGIN_CIDRS_PLACEHOLDER}
                rows={4}
                maxLength={1023}
                aria-describedby="allowed-login-cidrs-description"
              />
              <p
                id="allowed-login-cidrs-description"
                className="text-xs text-muted-foreground mt-1"
              >
                One IPv4 or IPv6 CIDR per line. Comma-separated values are also accepted.
              </p>
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
    </div>
  );
}
