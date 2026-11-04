/**
 * Summaries of recording storage across tiers (hot disks, warm mounts, cold
 * buckets) for the System page. Pure functions over /api/storage-targets so
 * they are unit-testable.
 *
 * The legacy "LightNVR storage" figure divides every recording byte by the
 * local recording filesystem's capacity, which is wrong once recordings are
 * archived to another tier: cold-bucket bytes inflate the local bar. The tier
 * summary reports each target against its own capacity (disks) or archive
 * budget (buckets).
 */
import type {
  StorageClass,
  StorageMeter,
  StorageTierRow,
  StorageTierSummary,
  StorageTierTarget,
  StorageTierTargetHealth,
} from '../../../types/system.ts';

const CLASS_ORDER = Object.freeze({ hot: 0, warm: 1, cold: 2 });
export const STORAGE_CLASSES: readonly StorageClass[] = Object.freeze<StorageClass[]>(['hot', 'warm', 'cold']);

function finite(value: unknown) {
  const number = Number(value);
  return Number.isFinite(number) && number >= 0 ? number : 0;
}

/** One display row per enabled target. */
export function storageTierRow(target: StorageTierTarget | null | undefined): StorageTierRow {
  const isBucket = target?.target_type === 's3';
  const health: StorageTierTargetHealth = target?.health || {};
  const recordingBytes = finite(target?.recording_bytes);
  const replicaBytes = finite(target?.replica_bytes);
  const storedBytes = recordingBytes + replicaBytes;
  const capacityBytes = finite(health.capacity_bytes);
  const budgetBytes = finite(target?.archive_budget_bytes);
  const storageClass = STORAGE_CLASSES.includes(target?.storage_class as StorageClass)
    ? target!.storage_class as StorageClass : (isBucket ? 'cold' : 'hot');
  let meter: StorageMeter | null = null;
  if (!isBucket && health.capacity_known !== false && capacityBytes > 0) {
    // A disk fills from everything on it, not only recordings.
    meter = { kind: 'filesystem', used: finite(health.used_bytes), total: capacityBytes };
  } else if (isBucket && budgetBytes > 0) {
    meter = { kind: 'budget', used: storedBytes, total: budgetBytes };
  }
  return {
    uuid: target?.uuid || '',
    name: target?.name || '',
    kind: isBucket ? 'bucket' : 'disk',
    storageClass,
    isDefault: Boolean(target?.is_default),
    recordingCount: finite(target?.recording_count),
    recordingBytes,
    replicaBytes,
    storedBytes,
    meter,
    pressure: health.pressure || 'unavailable',
    status: health.status || 'unknown',
    duplicateFilesystem: Boolean(health.duplicate_filesystem),
  };
}

/**
 * Summarize enabled targets. `tiered` is true when recordings can live in
 * more than one place, i.e. when the single legacy bar is misleading.
 */
export function summarizeStorageTiers(targets: StorageTierTarget[] | null | undefined): StorageTierSummary {
  const rows = (Array.isArray(targets) ? targets : [])
    .filter((target) => target && target.enabled !== false)
    .map(storageTierRow)
    .sort((a, b) => (CLASS_ORDER[a.storageClass] - CLASS_ORDER[b.storageClass]) ||
      (Number(b.isDefault) - Number(a.isDefault)) || a.name.localeCompare(b.name));
  const byClass = { hot: 0, warm: 0, cold: 0 };
  for (const row of rows) byClass[row.storageClass] += row.storedBytes;
  const storedBytes = rows.reduce((sum, row) => sum + row.storedBytes, 0);
  const tiered = rows.length > 1 || rows.some((row) => row.kind === 'bucket');
  return { rows, byClass, storedBytes, tiered };
}

export function meterPercent(meter: StorageMeter | null | undefined) {
  if (!meter || !(meter.total > 0)) return null;
  return Math.max(0, Math.min(100, (meter.used / meter.total) * 100));
}
