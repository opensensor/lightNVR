/**
 * LightNVR Web Interface Stream Bulk Action Modal Component
 * Confirmation modal for bulk disable / bulk delete of streams.
 */

/**
 * StreamBulkActionModal component
 * @param {Object}   props
 * @param {'enable'|'disable'|'delete'} props.action  Which bulk action is being confirmed
 * @param {string[]} props.streamNames                 Names of the streams being acted on
 * @param {Function} props.onClose                    Called when the user cancels
 * @param {Function} props.onConfirm                  Called when the user confirms
 * @param {boolean}  props.isWorking                  Whether the operation is in progress
 */
export function StreamBulkActionModal({ action, streamNames, onClose, onConfirm, isWorking }) {
  const count = streamNames.length;
  const isDelete = action === 'delete';
  const isEnable = action === 'enable';

  // Show up to 5 names inline; collapse the rest into "+ N more"
  const MAX_LISTED = 5;
  const listedNames  = streamNames.slice(0, MAX_LISTED);
  const hiddenCount  = count - listedNames.length;

  const title = isWorking
    ? (isDelete ? 'Deleting Streams…' : isEnable ? 'Enabling Streams…' : 'Disabling Streams…')
    : (isDelete ? 'Confirm Permanent Deletion' : isEnable ? 'Confirm Enable' : 'Confirm Disable');

  return (
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
      <div class="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">

        {/* Header */}
        <div class="flex justify-between items-center p-4 border-b border-border">
          <h3 class="text-lg font-medium">{title}</h3>
          {!isWorking && (
            <button type="button" class="text-2xl cursor-pointer border-none bg-transparent" onClick={onClose}>×</button>
          )}
        </div>

        {/* Body */}
        <div class="p-6">
          {isWorking ? (
            <div class="flex flex-col items-center justify-center py-8">
              <div class="inline-block animate-spin rounded-full border-4 border-input border-t-primary w-10 h-10 mb-4"></div>
              <p class="text-muted-foreground">
                {isDelete ? 'Permanently deleting streams…' : isEnable ? 'Enabling streams…' : 'Disabling streams…'}
              </p>
              <p class="text-sm text-muted-foreground mt-2">This may take a few seconds.</p>
            </div>
          ) : (
            <>
              {/* Icon — green check for enable, warning triangle for disable/delete */}
              <div class={`flex items-center justify-center mb-4 ${
                isDelete  ? 'text-red-600 dark:text-red-500'
                : isEnable ? 'text-green-600 dark:text-green-400'
                :             'text-yellow-600 dark:text-yellow-400'
              }`}>
                {isEnable ? (
                  <svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                  </svg>
                ) : (
                  <svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                    <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                  </svg>
                )}
              </div>

              {/* Message */}
              <p class="text-center font-medium mb-2">
                {isDelete  ? `Permanently delete ${count} stream${count !== 1 ? 's' : ''}?`
                 : isEnable ? `Enable ${count} stream${count !== 1 ? 's' : ''}?`
                 :             `Disable ${count} stream${count !== 1 ? 's' : ''}?`}
              </p>
              <p class="text-sm text-muted-foreground text-center mb-4">
                {isDelete
                  ? 'This will permanently remove the stream configurations. This action cannot be undone.'
                  : isEnable
                    ? 'The selected streams will resume processing with their existing configurations.'
                    : 'The selected streams will stop processing. Their configurations are preserved and can be re-enabled later.'}
              </p>

              {/* Stream name list */}
              <ul class="text-sm text-muted-foreground mb-6 space-y-1 bg-muted/30 rounded-lg px-4 py-3 max-h-36 overflow-y-auto">
                {listedNames.map(name => (
                  <li key={name} class="truncate">• {name}</li>
                ))}
                {hiddenCount > 0 && (
                  <li class="text-muted-foreground/70 italic">+ {hiddenCount} more</li>
                )}
              </ul>

              {/* Action buttons */}
              <div class="flex justify-center space-x-3">
                <button
                  type="button"
                  class="px-4 py-2 bg-secondary text-secondary-foreground rounded hover:bg-secondary/80 transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                  onClick={onClose}
                >
                  Cancel
                </button>
                <button
                  type="button"
                  class={`px-4 py-2 text-white rounded transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                    isDelete  ? 'bg-red-600 hover:bg-red-700 focus:ring-red-500'
                    : isEnable ? 'bg-green-600 hover:bg-green-700 focus:ring-green-500'
                    :             'bg-yellow-600 hover:bg-yellow-700 focus:ring-yellow-500'
                  }`}
                  onClick={onConfirm}
                >
                  {isDelete ? 'Delete Permanently' : isEnable ? 'Enable Selected' : 'Disable Selected'}
                </button>
              </div>
            </>
          )}
        </div>

      </div>
    </div>
  );
}

