import {
  meterPercent,
  storageTierRow,
  summarizeStorageTiers,
} from '../js/components/preact/system/storageTiers.ts';

const GB = 1024 ** 3;

const hotDisk = {
  uuid: 'hot', name: 'NVMe', target_type: 'filesystem', storage_class: 'hot',
  enabled: true, is_default: true, recording_count: 1200, recording_bytes: 180 * GB,
  replica_bytes: 0, archive_budget_bytes: 0,
  health: {
    status: 'healthy', capacity_known: true, capacity_bytes: 500 * GB,
    used_bytes: 230 * GB, pressure: 'normal', duplicate_filesystem: false,
  },
};
const coldBucket = {
  uuid: 'cold', name: 'Archive', target_type: 's3', storage_class: 'cold',
  enabled: true, recording_count: 9000, recording_bytes: 2048 * GB,
  replica_bytes: 12 * GB, archive_budget_bytes: 4096 * GB,
  health: { status: 'healthy', capacity_known: false, capacity_bytes: 0, used_bytes: 0, pressure: 'normal' },
};

describe('recording storage tiers', () => {
  test('a single local disk keeps the legacy single bar', () => {
    const summary = summarizeStorageTiers([hotDisk]);
    expect(summary.tiered).toBe(false);
    expect(summary.rows).toHaveLength(1);
  });

  test('a hot disk plus a cold bucket reports each against its own limit', () => {
    const summary = summarizeStorageTiers([coldBucket, hotDisk]);
    expect(summary.tiered).toBe(true);
    expect(summary.rows.map((row) => row.uuid)).toEqual(['hot', 'cold']);
    const [hot, cold] = summary.rows;
    // Disks are measured against filesystem capacity (everything on it).
    expect(hot.meter).toEqual({ kind: 'filesystem', used: 230 * GB, total: 500 * GB });
    // Buckets against the archive budget, counting verified copies too.
    expect(cold.meter).toEqual({ kind: 'budget', used: 2060 * GB, total: 4096 * GB });
    expect(summary.byClass).toEqual({ hot: 180 * GB, warm: 0, cold: 2060 * GB });
    expect(summary.storedBytes).toBe(2240 * GB);
    expect(meterPercent(hot.meter)).toBeCloseTo(46, 0);
  });

  test('buckets without a budget and disabled targets', () => {
    const unbudgeted = storageTierRow({ ...coldBucket, archive_budget_bytes: 0 });
    expect(unbudgeted.meter).toBeNull();
    expect(unbudgeted.kind).toBe('bucket');
    const summary = summarizeStorageTiers([hotDisk, { ...coldBucket, enabled: false }]);
    expect(summary.tiered).toBe(false);
    // Even a single bucket target is tiered: its bytes are not on local disk.
    expect(summarizeStorageTiers([coldBucket]).tiered).toBe(true);
  });

  test('tolerates partial or malformed targets', () => {
    const row = storageTierRow({ uuid: 'x', name: 'Odd', storage_class: 'lukewarm', recording_bytes: -5 });
    expect(row).toMatchObject({ storageClass: 'hot', storedBytes: 0, meter: null, pressure: 'unavailable' });
    expect(summarizeStorageTiers(null)).toMatchObject({ rows: [], tiered: false, storedBytes: 0 });
    expect(meterPercent({ used: 10, total: 0 })).toBeNull();
    expect(meterPercent({ used: 20, total: 10 })).toBe(100);
  });
});
