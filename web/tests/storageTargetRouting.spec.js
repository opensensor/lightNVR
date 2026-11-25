import {
  archiveTargetWithoutTrigger,
  availableFallbackTargets,
  policyHasArchiveTrigger,
  primaryPoolMemberIds,
  primaryTargetForPool,
  referencedTargetUuids,
  targetIsRouted,
} from '../js/components/preact/settings/storageTargetRouting.ts';

describe('storage target routing helpers (#621)', () => {
  const targets = {
    def: { uuid: 'def', is_default: true },
    usb: { uuid: 'usb', is_default: false },
    nas: { uuid: 'nas', is_default: false },
    pooled: { uuid: 'pooled', is_default: false },
  };

  test('a target with no policy is not routed, the default always is', () => {
    const referenced = referencedTargetUuids([], []);
    expect(targetIsRouted(targets.def, referenced)).toBe(true);
    expect(targetIsRouted(targets.usb, referenced)).toBe(false);
  });

  test('archive targets count only when an enabled policy has a move trigger', () => {
    const referenced = referencedTargetUuids([
      { enabled: true, primary_target_uuid: 'usb', migration_target_uuid: 'nas' },
    ], []);
    expect(targetIsRouted(targets.usb, referenced)).toBe(true);
    expect(targetIsRouted(targets.nas, referenced)).toBe(false);
    expect(targetIsRouted(targets.pooled, referenced)).toBe(false);
    const triggered = referencedTargetUuids([
      { enabled: true, primary_target_uuid: 'usb', migration_target_uuid: 'nas', migration_after_days: 7 },
    ], []);
    expect(targetIsRouted(targets.nas, triggered)).toBe(true);
  });

  test('a disabled policy routes nothing', () => {
    const referenced = referencedTargetUuids([
      { enabled: false, primary_target_uuid: 'usb' },
    ], []);
    expect(targetIsRouted(targets.usb, referenced)).toBe(false);
  });

  test('pool members are routed through the policy that uses the pool', () => {
    const pools = [{ uuid: 'pool-1', members: [{ target_uuid: 'pooled' }] }];
    expect(targetIsRouted(targets.pooled, referencedTargetUuids([
      { enabled: true, primary_pool_uuid: 'pool-1' },
    ], pools))).toBe(true);
    expect(targetIsRouted(targets.pooled, referencedTargetUuids([
      { enabled: true, primary_target_uuid: 'usb' },
    ], pools))).toBe(false);
  });

  test('pool routing can fall back to the old exact target only when it is outside the pool', () => {
    const targets = [
      { uuid: 'default', target_type: 'filesystem' },
      { uuid: 'pool-a', target_type: 'filesystem' },
      { uuid: 'pool-b', target_type: 'filesystem' },
      { uuid: 'archive', target_type: 's3' },
    ];
    const pools = [{ uuid: 'primary', members: [{ target_uuid: 'pool-a' }, { target_uuid: 'pool-b' }] }];
    expect(primaryPoolMemberIds('primary', pools, targets)).toEqual(['pool-a', 'pool-b']);
    expect(primaryTargetForPool('default', 'primary', pools, targets)).toBe('pool-a');
    expect(availableFallbackTargets('default', 'primary', pools, targets).map((target) => target.uuid))
      .toEqual(['default']);
    expect(availableFallbackTargets('default', '', pools, targets).map((target) => target.uuid))
      .toEqual(['pool-a', 'pool-b']);
    const poolContainingDefault = [{ uuid: 'primary', members: [{ target_uuid: 'default' }, { target_uuid: 'pool-a' }] }];
    expect(availableFallbackTargets('default', 'primary', poolContainingDefault, targets).map((target) => target.uuid))
      .toEqual(['pool-b']);
  });

  test('an archive target needs a trigger, mirroring the lifecycle scheduler', () => {
    const base = { migration_target_uuid: 'nas', migration_after_days: '0', archive_after_seconds: '-1' };
    expect(policyHasArchiveTrigger(base)).toBe(false);
    expect(policyHasArchiveTrigger({ ...base, archive_after_seconds: '' })).toBe(false);
    expect(archiveTargetWithoutTrigger(base)).toBe(true);
    expect(archiveTargetWithoutTrigger({ ...base, migration_after_days: '7' })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_after_seconds: '0' })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_protected: true })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_on_pressure: true })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, migration_target_uuid: '' })).toBe(false);
  });
});
