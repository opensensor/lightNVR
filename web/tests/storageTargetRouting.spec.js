import {
  archiveTargetWithoutTrigger,
  policyHasArchiveTrigger,
  referencedTargetUuids,
  targetIsRouted,
} from '../js/components/preact/settings/storageTargetRouting.js';

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

  test('primary, fallback and archive targets of enabled policies count', () => {
    const referenced = referencedTargetUuids([
      { enabled: true, primary_target_uuid: 'usb', migration_target_uuid: 'nas' },
    ], []);
    expect(targetIsRouted(targets.usb, referenced)).toBe(true);
    expect(targetIsRouted(targets.nas, referenced)).toBe(true);
    expect(targetIsRouted(targets.pooled, referenced)).toBe(false);
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

  test('an archive target needs a trigger, mirroring the lifecycle scheduler', () => {
    const base = { migration_target_uuid: 'nas', migration_after_days: '0', archive_after_seconds: '-1' };
    expect(policyHasArchiveTrigger(base)).toBe(false);
    expect(archiveTargetWithoutTrigger(base)).toBe(true);
    expect(archiveTargetWithoutTrigger({ ...base, migration_after_days: '7' })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_after_seconds: '0' })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_protected: true })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, archive_on_pressure: true })).toBe(false);
    expect(archiveTargetWithoutTrigger({ ...base, migration_target_uuid: '' })).toBe(false);
  });
});
