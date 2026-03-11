import { useI18n } from '../../../i18n.js';

/**
 * API Key Modal Component
 */

/**
 * API Key Modal Component
 * @param {Object} props - Component props
 * @param {Object} props.currentUser - Current user for API key generation
 * @param {string} props.newApiKey - Newly generated API key
 * @param {Function} props.handleGenerateApiKey - Function to handle API key generation
 * @param {Function} props.copyApiKey - Function to copy API key to clipboard
 * @param {Function} props.onClose - Function to close the modal
 * @returns {JSX.Element} API key modal
 */
export function ApiKeyModal({ currentUser, newApiKey, handleGenerateApiKey, copyApiKey, onClose }) {
  const { t } = useI18n();

  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  // Log the API key for debugging
  console.log('API Key Modal - newApiKey:', newApiKey);

  // Create a custom close handler that prevents closing if an API key is displayed
  const handleClose = (e) => {
    // If we have an API key, prevent closing when clicking outside
    if (newApiKey && newApiKey !== t('users.generatingApiKey')) {
      // Only allow closing via the close button
      return;
    }
    // Otherwise, proceed with normal close
    onClose(e);
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={handleClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">{t('users.apiKeyFor', { username: currentUser.username })}</h2>

        <div className="mb-6">
          {newApiKey ? (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2">
                {t('users.apiKey')}
              </label>
              <div className="flex">
                <input
                  className="w-full px-3 py-2 border border-input rounded-l-md bg-background text-foreground focus:outline-none focus:ring-2 focus:ring-primary"
                  type="text"
                  value={newApiKey}
                  readOnly
                />
                <button
                  className="btn-primary font-bold py-2 px-4 rounded-r"
                  onClick={copyApiKey}
                >
                  {t('common.copy')}
                </button>
              </div>
              <p className="text-sm text-muted-foreground mt-2">
                {t('users.apiKeyShownOnce')}
              </p>
            </div>
          ) : (
            <>
              <p className="mb-4">
                {t('users.generateApiKeyDescription')}
              </p>
              <button
                className="btn-primary w-full font-bold py-2 px-4 rounded mb-4"
                onClick={handleGenerateApiKey}
              >
                {t('users.generateNewApiKey')}
              </button>
            </>
          )}
        </div>

        <div className="flex justify-end">
          <button
            className={newApiKey && newApiKey !== t('users.generatingApiKey') ? 'btn-primary' : 'btn-secondary'}
            onClick={onClose}
          >
            {newApiKey && newApiKey !== t('users.generatingApiKey') ? t('common.done') : t('common.close')}
          </button>
        </div>
      </div>
    </div>
  );
}
