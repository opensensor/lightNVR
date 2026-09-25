/**
 * Helpers that explain whether a storage target can ever receive recordings.
 *
 * A target is inventory: new segments keep going to the default target until
 * an enabled placement policy names the target (directly or through a pool),
 * and a policy's archive target is only used when something triggers a move.
 * The settings page uses these to say so instead of letting a passing "Test
 * target" imply footage will land there (#621).
 */

const DIRECT_TARGET_KEYS = ['primary_target_uuid', 'fallback_target_uuid', 'migration_target_uuid'];
const POOL_KEYS = ['primary_pool_uuid', 'replication_pool_uuid'];

/** Set of target UUIDs that at least one enabled policy can route to. */
export function referencedTargetUuids(policies = [], pools = []) {
  const referenced = new Set();
  const poolMembers = new Map(
    (pools || []).map((pool) => [
      pool?.uuid,
      (pool?.members || []).map((member) => member?.target_uuid).filter(Boolean),
    ]),
  );
  for (const policy of policies || []) {
    if (!policy || policy.enabled === false) continue;
    for (const key of DIRECT_TARGET_KEYS) {
      if (policy[key]) referenced.add(policy[key]);
    }
    for (const key of POOL_KEYS) {
      const members = policy[key] ? poolMembers.get(policy[key]) : null;
      if (members) members.forEach((uuid) => referenced.add(uuid));
    }
  }
  return referenced;
}

/** The default target always receives recordings; others need a policy. */
export function targetIsRouted(target, referenced) {
  if (!target) return false;
  if (target.is_default) return true;
  return !!referenced && referenced.has(target.uuid);
}

/** Mirrors the lifecycle scheduler: a move needs one of these triggers. */
export function policyHasArchiveTrigger(values) {
  const after = Number(values?.migration_after_days);
  const archiveAfter = Number(values?.archive_after_seconds ?? -1);
  return (Number.isFinite(after) && after > 0)
    || (Number.isFinite(archiveAfter) && archiveAfter >= 0)
    || !!values?.archive_protected
    || !!values?.archive_on_pressure;
}

/** True when the editor names an archive target that nothing will ever move to. */
export function archiveTargetWithoutTrigger(values) {
  return !!values?.migration_target_uuid && !policyHasArchiveTrigger(values);
}
