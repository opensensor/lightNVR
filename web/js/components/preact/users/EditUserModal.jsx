/**
 * Edit User Modal Component
 */

import { USER_ROLE_KEYS } from './UserRoles.js';
import { useEffect, useRef } from 'preact/hooks';
import { useI18n } from '../../../i18n.js';

const FOCUSABLE_SELECTORS = [
  'a[href]',
  'button:not([disabled])',
  'textarea:not([disabled])',
  'input:not([disabled])',
  'select:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',');

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
export function EditUserModal({
  currentUser,
  formData,
  handleInputChange,
  handleEditUser,
  handleClearLoginLockout,
  isClearingLoginLockout = false,
  onClose,
  title,
  submitLabel,
  showPasswordField = true,
  showRoleField = true,
  showActiveField = true,
  showPasswordLockField = true,
  showAllowedTagsField = true,
  showAllowedLoginCidrsField = true,
  showClearLoginLockoutButton = true,
}) {
  const modalRef = useRef(null);
  const previousFocusedElementRef = useRef(null);
  const backdropPointerDownRef = useRef(false);
  const { t } = useI18n();

  const resolvedSubmitLabel = submitLabel || t('users.updateUser');
  const resolvedTitle = title || t('users.editUserNamed', { username: currentUser.username });
  const allowedLoginCidrsPlaceholder = `${t('common.example')}
192.168.1.25
192.168.1.0/24
2001:db8::1
${t('users.allowedLoginCidrsPlaceholderTail')}`;

  useEffect(() => {
    // Remember previously focused element
    previousFocusedElementRef.current = document.activeElement;

    const modalNode = modalRef.current;
    if (modalNode) {
      const focusableElements = modalNode.querySelectorAll(FOCUSABLE_SELECTORS);
      if (focusableElements.length > 0) {
        focusableElements[0].focus();
      } else {
        modalNode.setAttribute('tabindex', '-1');
        modalNode.focus();
      }
    }

    return () => {
      // Restore focus to previously focused element if possible
      const prev = previousFocusedElementRef.current;
      if (prev && typeof prev.focus === 'function') {
        prev.focus();
      }
    };
  }, []);

  const handleKeyDown = (e) => {
    if (e.key === 'Escape') {
      e.stopPropagation();
      onClose();
      return;
    }

    if (e.key === 'Tab') {
      const modalNode = modalRef.current;
      if (!modalNode) {
        return;
      }
      const focusable = Array.from(modalNode.querySelectorAll(FOCUSABLE_SELECTORS));
      if (focusable.length === 0) {
        return;
      }

      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      const current = document.activeElement;

      if (!e.shiftKey && current === last) {
        e.preventDefault();
        first.focus();
      } else if (e.shiftKey && current === first) {
        e.preventDefault();
        last.focus();
      }
    }
  };

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

  const handleBackdropMouseDown = (e) => {
    backdropPointerDownRef.current = e.target === e.currentTarget;
  };

  const handleBackdropClick = (e) => {
    if (backdropPointerDownRef.current && e.target === e.currentTarget) {
      onClose();
    }
    backdropPointerDownRef.current = false;
  };

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onMouseDown={handleBackdropMouseDown}
      onClick={handleBackdropClick}
    >
      <div
        ref={modalRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="edit-user-modal-title"
        className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full"
        onClick={stopPropagation}
        onKeyDown={handleKeyDown}
      >
        <h2 id="edit-user-modal-title" className="text-xl font-bold mb-4">
          {resolvedTitle}
        </h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="username">
              {t('fields.username')}
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

          {showPasswordField && (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="password">
                {t('users.passwordKeepCurrent')}
              </label>
              <input
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                id="password"
                type="password"
                name="password"
                value={formData.password}
                onChange={handleInputChange}
              />
            </div>
          )}

          <div className="mb-4">
            <label className="block text-sm font-bold mb-2" htmlFor="email">
              {t('fields.email')}
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

          {showRoleField && (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="role">
                {t('fields.role')}
              </label>
              <select
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                id="role"
                name="role"
                value={formData.role}
                onChange={handleInputChange}
              >
                {Object.entries(USER_ROLE_KEYS).map(([value, key]) => (
                  <option key={value} value={value}>{t(key)}</option>
                ))}
              </select>
            </div>
          )}

          {showActiveField && (
            <div className="mb-4">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  name="is_active"
                  checked={formData.is_active}
                  onChange={handleInputChange}
                  className="mr-2"
                />
                <span className="text-sm font-bold">{t('users.active')}</span>
              </label>
            </div>
          )}

          {showPasswordLockField && (
            <div className="mb-6">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  name="password_change_locked"
                  checked={formData.password_change_locked}
                  onChange={handleInputChange}
                  className="mr-2"
                />
                <span className="text-sm font-bold">{t('users.lockPasswordChanges')}</span>
              </label>
              <p className="text-xs text-muted-foreground mt-1 ml-6">
                {t('users.lockPasswordChangesHelp')}
              </p>
            </div>
          )}

          {showAllowedTagsField && (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="allowed_tags">
                {t('users.allowedStreamTags')} <span className="font-normal text-muted-foreground">(RBAC)</span>
              </label>
              <input
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                id="allowed_tags"
                type="text"
                name="allowed_tags"
                value={formData.allowed_tags || ''}
                onChange={handleInputChange}
                placeholder={t('users.allowedTagsPlaceholder')}
                maxLength={255}
              />
              <p className="text-xs text-muted-foreground mt-1">
                {t('users.allowedTagsHelp')}
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
          )}

          {showAllowedLoginCidrsField && (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2" htmlFor="allowed_login_cidrs">
                {t('users.allowedLoginIpRanges')} <span className="font-normal text-muted-foreground">(CIDR)</span>
              </label>
              <textarea
                className="w-full px-3 py-2 border border-input rounded-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                id="allowed_login_cidrs"
                name="allowed_login_cidrs"
                value={formData.allowed_login_cidrs || ''}
                onChange={handleInputChange}
                placeholder={allowedLoginCidrsPlaceholder}
                rows={4}
                maxLength={1023}
              />
              <p className="text-xs text-muted-foreground mt-1">
                {t('users.allowedLoginCidrsHelp')}
              </p>
            </div>
          )}

          <div className={`mt-6 flex items-center gap-3 ${showClearLoginLockoutButton ? 'justify-between' : 'justify-end'}`}>
            {showClearLoginLockoutButton && (
              <button
                type="button"
                className="btn-secondary"
                onClick={handleClearLoginLockout}
                disabled={isClearingLoginLockout}
              >
                {isClearingLoginLockout ? t('users.clearingLoginLockout') : t('users.clearLoginLockout')}
              </button>
            )}

            <div className="flex justify-end">
            <button
              type="button"
              className="btn-secondary mr-2"
              onClick={onClose}
            >
              {t('common.cancel')}
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              {resolvedSubmitLabel}
            </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  );
}
