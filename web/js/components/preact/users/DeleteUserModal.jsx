import { useI18n } from '../../../i18n.js';
import { AsyncButton } from '../AsyncButton.jsx';

/**
 * Delete User Modal Component
 */

/**
 * Delete User Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user being deleted
 * @param {Function} props.handleDeleteUser - Function to handle user deletion
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} Delete user modal
 */
export function DeleteUserModal({ currentUser, handleDeleteUser, onClose }) {
  const { t } = useI18n();

  // Direct delete handler — wrapped in Promise.resolve so AsyncButton can
  // observe pending state whether the parent uses mutate or mutateAsync.
  const handleDeleteClick = (e) => {
    if (e && typeof e.stopPropagation === 'function') {
      e.stopPropagation(); // Stop event from bubbling up
    }
    return Promise.resolve(handleDeleteUser());
  };

  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">{t('users.deleteUser')}</h2>

        <p className="mb-6">
          {t('users.deleteUserConfirmation', { username: currentUser.username })}
        </p>

        <div className="flex justify-end">
          <button
            className="btn-secondary mr-2"
            onClick={onClose}
          >
            {t('common.cancel')}
          </button>
          {/* AsyncButton locks the button + shows a spinner while the DELETE
              is in flight so the #399 rapid-tap double-submit can't fire
              more than one deletion. */}
          <AsyncButton
            className="btn-danger"
            onClick={handleDeleteClick}
          >
            {t('users.deleteUser')}
          </AsyncButton>
        </div>
      </div>
    </div>
  );
}
