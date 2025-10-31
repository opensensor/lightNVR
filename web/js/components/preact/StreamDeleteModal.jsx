/**
 * LightNVR Web Interface Stream Delete Modal Component
 * Preact component for the stream delete modal
 */

import { useState } from 'preact/hooks';

/**
 * StreamDeleteModal component
 * @param {Object} props Component props
 * @param {string} props.streamId ID of the stream to delete
 * @param {string} props.streamName Name of the stream to display
 * @param {Function} props.onClose Function to call when the modal is closed
 * @param {Function} props.onDisable Function to call when the disable button is clicked
 * @param {Function} props.onDelete Function to call when the delete button is clicked
 * @returns {JSX.Element} StreamDeleteModal component
 */
export function StreamDeleteModal({ streamId, streamName, onClose, onDisable, onDelete }) {
  const [isConfirmDelete, setIsConfirmDelete] = useState(false);

  // Show delete confirmation step
  const showDeleteConfirmation = () => {
    setIsConfirmDelete(true);
  };

  // Handle disable stream
  const handleDisable = () => {
    onDisable(streamId);
    onClose();
  };

  // Handle delete stream
  const handleDelete = () => {
    onDelete(streamId);
    onClose();
  };

  return (
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
      <div class="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">
        <div class="flex justify-between items-center p-4 border-b border-border">
          <h3 class="text-lg font-medium">{isConfirmDelete ? 'Confirm Permanent Deletion' : 'Stream Actions'}</h3>
          <button type="button" class="text-2xl cursor-pointer border-none bg-transparent" onClick={onClose}>×</button>
        </div>

        <div class="p-6">
          {!isConfirmDelete ? (
            <div class="mb-6">
              <h4 class="text-lg font-medium mb-2">What would you like to do with "{streamName}"?</h4>
              <p class="text-muted-foreground mb-4">
                Please choose one of the following options:
              </p>

              <div class="space-y-4">
                <div class="p-4 border rounded-lg" style={{borderColor: 'hsl(var(--warning-muted))', backgroundColor: 'hsl(var(--warning-muted) / 0.3)'}}>
                  <h5 class="font-medium mb-2" style={{color: 'hsl(var(--warning))'}}>Disable Stream (Soft Delete)</h5>
                  <p class="text-muted-foreground mb-2">
                    This option will disable the stream but keep its configuration in the database. You can re-enable it later.
                  </p>
                  <ul class="list-disc list-inside text-sm text-muted-foreground mb-3">
                    <li>Stream will stop processing</li>
                    <li>Live streaming will be disabled</li>
                    <li>Recording will be disabled</li>
                    <li>Audio recording will be disabled</li>
                    <li>Detection-based recording will be disabled</li>
                    <li>Configuration is preserved</li>
                    <li>Existing recordings are kept</li>
                    <li>Can be re-enabled later</li>
                  </ul>
                  <button
                    class="w-full px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                    onClick={handleDisable}
                  >
                    Disable Stream
                  </button>
                </div>

                <div class="p-4 border rounded-lg" style={{borderColor: 'hsl(var(--danger-muted))', backgroundColor: 'hsl(var(--danger-muted) / 0.3)'}}>
                  <h5 class="font-medium mb-2" style={{color: 'hsl(var(--danger))'}}>Delete Stream (Permanent)</h5>
                  <p class="text-muted-foreground mb-2">
                    This option will permanently delete the stream configuration from the database. This action cannot be undone.
                  </p>
                  <ul class="list-disc list-inside text-sm text-muted-foreground mb-3">
                    <li>Stream will be completely removed</li>
                    <li>Configuration is deleted</li>
                    <li>Recordings remain accessible</li>
                    <li>Cannot be recovered</li>
                  </ul>
                  <button
                    class="w-full px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                    onClick={showDeleteConfirmation}
                  >
                    Delete Stream
                  </button>
                </div>
              </div>
            </div>
          ) : (<>
            <div class="mb-6">
              <div class="flex items-center justify-center mb-4 text-red-600 dark:text-red-500">
                <svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                </svg>
              </div>
              <h4 class="text-lg font-medium mb-2 text-center">Are you sure you want to permanently delete "{streamName}"?</h4>
              <p class="text-muted-foreground mb-4 text-center">
                This action cannot be undone. The stream configuration will be permanently removed from the database.
              </p>
            </div>

            <div class="flex justify-center space-x-3">
              <button
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                onClick={() => setIsConfirmDelete(false)}
              >
                Cancel
              </button>
              <button
                class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
                onClick={handleDelete}
              >
                Yes, Delete Permanently
              </button>
            </div></>
          )}
        </div>
      </div>
    </div>
  );
}
