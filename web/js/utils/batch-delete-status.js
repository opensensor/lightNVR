/** Keep accepted-but-pending deletions distinct from completed cleanup and failures. */
import { t } from '../i18n.js';

export function batchDeleteStatus(result) {
  const succeeded = result.succeeded || 0;
  const failed = result.failed || 0;
  const pending = result.pending_deletions || 0;
  if (pending > 0) {
    return t('recordings.batchDeletePending', { succeeded, pending })
      + (failed > 0 ? t('recordings.batchDeletePendingFailures', { count: failed }) : '');
  }
  if (succeeded > 0) {
    return t(succeeded === 1 ? 'recordings.batchDeleteSucceededOne' : 'recordings.batchDeleteSucceededMany', { count: succeeded })
      + (failed > 0 ? t('recordings.batchDeleteSucceededFailures', { count: failed }) : '');
  }
  return t(failed === 1 ? 'recordings.batchDeleteFailedOne' : 'recordings.batchDeleteFailedMany', { count: failed });
}
