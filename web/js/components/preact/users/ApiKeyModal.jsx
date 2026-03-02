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
  // Stop click propagation on modal content
  const stopPropagation = (e) => {
    e.stopPropagation();
  };

  // Log the API key for debugging
  console.log('API Key Modal - newApiKey:', newApiKey);

  // Create a custom close handler that prevents closing if an API key is displayed
  const handleClose = (e) => {
    // If we have an API key, prevent closing when clicking outside
    if (newApiKey && newApiKey !== 'Generating...') {
      // Only allow closing via the close button
      return;
    }
    // Otherwise, proceed with normal close
    onClose(e);
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={handleClose}>
      <div className="bg-card text-card-foreground rounded-lg p-6 max-w-md w-full" onClick={stopPropagation}>
        <h2 className="text-xl font-bold mb-4">API Key for {currentUser.username}</h2>

        <div className="mb-6">
          {newApiKey ? (
            <div className="mb-4">
              <label className="block text-sm font-bold mb-2">
                API Key
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
                  Copy
                </button>
              </div>
              <p className="text-sm text-muted-foreground mt-2">
                This key will only be shown once. Save it securely.
              </p>
            </div>
          ) : (
            <>
              <p className="mb-4">
                Generate a new API key for this user. This will invalidate any existing API key.
              </p>
              <button
                className="btn-primary w-full font-bold py-2 px-4 rounded mb-4"
                onClick={handleGenerateApiKey}
              >
                Generate New API Key
              </button>
            </>
          )}
        </div>

        <div className="flex justify-end">
          <button
            className={newApiKey && newApiKey !== 'Generating...' ? 'btn-primary' : 'btn-secondary'}
            onClick={onClose}
          >
            {newApiKey && newApiKey !== 'Generating...' ? 'Done' : 'Close'}
          </button>
        </div>
      </div>
    </div>
  );
}
