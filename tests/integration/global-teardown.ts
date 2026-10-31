/** Cleanup only the executable/configuration/start-time owned by this test run. */
import { stopOwnedProcess } from './fixtures/run-ownership';

export default async function globalTeardown(): Promise<void> {
  await stopOwnedProcess();
  // Retain disposable run data for diagnosis. No raw PID-file fallback or rm -rf.
}
