/** Keep accepted-but-pending deletions distinct from completed cleanup and failures. */
export function batchDeleteStatus(result) {
  const succeeded = result.succeeded || 0;
  const failed = result.failed || 0;
  const pending = result.pending_deletions || 0;
  if (pending > 0) {
    return `Accepted ${succeeded} deletions; ${pending} await storage cleanup${failed > 0 ? `; failed to delete ${failed}` : ''}`;
  }
  if (succeeded > 0) {
    return `Successfully deleted ${succeeded} recording${succeeded !== 1 ? 's' : ''}${failed > 0 ? `, but failed to delete ${failed}` : ''}`;
  }
  return `Failed to delete ${failed} recording${failed !== 1 ? 's' : ''}`;
}
