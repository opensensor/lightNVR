/**
 * LightNVR Web Interface Stream Delete Modal Component
 * Preact component for the stream delete modal
 */

import { useState } from 'preact/hooks';
import { useI18n } from '../../i18n.js';

/**
 * StreamDeleteModal component
 * @param {Object} props Component props
 * @param {string} props.streamId ID of the stream to delete
 * @param {string} props.streamName Name of the stream to display
 * @param {Function} props.onClose Function to call when the modal is closed
 * @param {Function} props.onDisable Function to call when the disable button is clicked
 * @param {Function} props.onDelete Function to call when the delete button is clicked
 * @param {boolean} props.isDeleting Whether a delete operation is in progress
 * @param {boolean} props.isDisabling Whether a disable operation is in progress
 * @returns {JSX.Element} StreamDeleteModal component
 */
export function StreamDeleteModal({ streamId, streamName, onClose, onDisable, onDelete, isDeleting = false, isDisabling = false }) {
  const { t } = useI18n();
  const [isConfirmDelete, setIsConfirmDelete] = useState(false);
  const isLoading = isDeleting || isDisabling;

  // Show delete confirmation step
  const showDeleteConfirmation = () => {
    setIsConfirmDelete(true);
  };

  // Handle disable stream - don't close modal, let parent handle it via onSuccess/onError
  const handleDisable = () => {
    onDisable(streamId);
  };

  // Handle delete stream - don't close modal, let parent handle it via onSuccess/onError
  const handleDelete = () => {
    onDelete(streamId);
  };

  return (
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 transition-opacity duration-300">
      <div class="bg-card text-card-foreground rounded-lg shadow-xl max-w-md w-full">
        <div class="flex justify-between items-center p-4 border-b border-border">
          <h3 class="text-lg font-medium">{isLoading
            ? (isDeleting ? t('streams.deletingStream') : t('streams.disablingStream'))
            : isConfirmDelete ? t('streams.confirmPermanentDeletion') : t('streams.streamActions')}</h3>
          {!isLoading && <button type="button" class="text-2xl cursor-pointer border-none bg-transparent" onClick={onClose}>×</button>}
        </div>

        <div class="p-6">
          {isLoading ? (
            <div class="flex flex-col items-center justify-center py-8">
              <div class="inline-block animate-spin rounded-full border-4 border-input border-t-primary w-10 h-10 mb-4"></div>
              <p class="text-muted-foreground">
                {isDeleting ? t('streams.permanentlyDeletingStream') : t('streams.disablingStreamProgress')}
              </p>
              <p class="text-sm text-muted-foreground mt-2">{t('streams.thisMayTakeAFewSeconds')}</p>
            </div>
          ) : !isConfirmDelete ? (
            <div class="mb-6">
              <h4 class="text-lg font-medium mb-2">{t('streams.whatWouldYouLikeToDoWith', { streamName })}</h4>
              <p class="text-muted-foreground mb-4">
                {t('streams.chooseFollowingOptions')}
              </p>

              <div class="space-y-4">
                <div class="p-4 border rounded-lg" style={{borderColor: 'hsl(var(--warning-muted))', backgroundColor: 'hsl(var(--warning-muted) / 0.3)'}}>
                  <h5 class="font-medium mb-2" style={{color: 'hsl(var(--warning))'}}>{t('streams.disableStreamSoftDelete')}</h5>
                  <p class="text-muted-foreground mb-2">
                    {t('streams.disableStreamExplanation')}
                  </p>
                  <ul class="list-disc list-inside text-sm text-muted-foreground mb-3">
                    <li>{t('streams.disableStreamBulletStopProcessing')}</li>
                    <li>{t('streams.disableStreamBulletLiveDisabled')}</li>
                    <li>{t('streams.disableStreamBulletRecordingDisabled')}</li>
                    <li>{t('streams.disableStreamBulletAudioDisabled')}</li>
                    <li>{t('streams.disableStreamBulletDetectionDisabled')}</li>
                    <li>{t('streams.disableStreamBulletConfigurationPreserved')}</li>
                    <li>{t('streams.disableStreamBulletRecordingsKept')}</li>
                    <li>{t('streams.disableStreamBulletCanReenable')}</li>
                  </ul>
                  <button
                    class="w-full px-4 py-2 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                    onClick={handleDisable}
                    disabled={isLoading}
                  >
                    {t('common.disable')}
                  </button>
                </div>

                <div class="p-4 border rounded-lg" style={{borderColor: 'hsl(var(--danger-muted))', backgroundColor: 'hsl(var(--danger-muted) / 0.3)'}}>
                  <h5 class="font-medium mb-2" style={{color: 'hsl(var(--danger))'}}>{t('streams.deleteStreamPermanent')}</h5>
                  <p class="text-muted-foreground mb-2">
                    {t('streams.deleteStreamExplanation')}
                  </p>
                  <ul class="list-disc list-inside text-sm text-muted-foreground mb-3">
                    <li>{t('streams.deleteStreamBulletRemoved')}</li>
                    <li>{t('streams.deleteStreamBulletConfigurationDeleted')}</li>
                    <li>{t('streams.deleteStreamBulletRecordingsAccessible')}</li>
                    <li>{t('streams.deleteStreamBulletCannotRecover')}</li>
                  </ul>
                  <button
                    class="w-full px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                    onClick={showDeleteConfirmation}
                    disabled={isLoading}
                  >
                    {t('common.delete')}
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
              <h4 class="text-lg font-medium mb-2 text-center">{t('streams.areYouSureDeletePermanent', { streamName })}</h4>
              <p class="text-muted-foreground mb-4 text-center">
                {t('streams.streamDeleteCannotBeUndone')}
              </p>
            </div>

            <div class="flex justify-center space-x-3">
              <button
                class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                onClick={() => setIsConfirmDelete(false)}
                disabled={isLoading}
              >
                {t('common.cancel')}
              </button>
              <button
                class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 disabled:opacity-50 disabled:cursor-not-allowed"
                onClick={handleDelete}
                disabled={isLoading}
              >
                {t('streams.yesDeletePermanently')}
              </button>
            </div></>
          )}
        </div>
      </div>
    </div>
  );
}
