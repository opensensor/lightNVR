/** Only stable operation names enter error messages; never URLs or wire data. */
export type StreamOperation = 'listStreams' | 'listStreamSummaries' | 'getStreamFull' | 'updateStream';

import type { EventOperation } from './events.ts';
import type { SystemOperation } from './system.ts';
import type { FleetOperation } from './fleet.ts';
import type { AuthenticationOperation } from './authentication.ts';
import type { AdministrationOperation } from './administration.ts';
import type { FleetWorkspaceOperation } from './fleetWrites.ts';

type ContractOperation = EventOperation | SystemOperation | FleetOperation | AuthenticationOperation | StreamOperation | 'listAuthSessions' | 'listTrustedDevices' | 'revokeAuthSession' | 'revokeTrustedDevice' | AdministrationOperation | FleetWorkspaceOperation;

export class ContractValidationError extends Error {
  readonly name = 'ContractValidationError';
  readonly mayHaveApplied: boolean;

  constructor(
    readonly operation: ContractOperation,
    readonly phase: 'request' | 'response',
    readonly schema: string,
    readonly status?: number,
    readonly issue: 'shape' | 'json' | 'status' = 'shape',
  ) {
    super(`Invalid ${operation} ${phase}: ${schema} (${issue}).`);
    this.mayHaveApplied = ['createEventDestination', 'updateEventDestination', 'deleteEventDestination', 'createEventRoute', 'updateEventRoute', 'deleteEventRoute', 'triggerMotion', 'updateSystemSettings', 'updateWorkspaces', 'updateSetupStatus', 'clearSystemLogs', 'createSystemBackup', 'restartSystem', 'shutdownSystem', 'updateStream', 'revokeAuthSession', 'revokeTrustedDevice', 'login', 'logout', 'completeTotpLogin', 'setupTotp', 'verifyTotp', 'disableTotp', 'changePassword', 'createFleetLocation', 'updateFleetLocation', 'deleteFleetLocation', 'createFleetTag', 'updateFleetTag', 'deleteFleetTag', 'createCameraCollection', 'updateCameraCollection', 'deleteCameraCollection', 'setCameraLocation', 'setCameraTags', 'mergeFleetTag', 'setCameraCollectionMembers', 'createFleetView', 'updateFleetView', 'deleteFleetView', 'createLiveLayout', 'updateLiveLayout', 'deleteLiveLayout', 'addCameraFavorite', 'deleteCameraFavorite', 'createFloorPlan', 'updateFloorPlan', 'deleteFloorPlan', 'setFloorPlanBackground', 'deleteFloorPlanBackground', 'createAdminUser', 'updateAdminUser', 'deleteAdminUser', 'createLegacyApiKey', 'setUserPasswordLock', 'clearUserLoginLockout', 'createAuthorizationRole', 'updateAuthorizationRole', 'deleteAuthorizationRole', 'setUserAuthorizationPolicy', 'updateAuditSettings'].includes(operation) && phase === 'response';
  }
}

/** Server-provided messages and causes are deliberately excluded. */
export class StreamRequestError extends Error {
  readonly name = 'StreamRequestError';

  constructor(
    readonly operation: StreamOperation,
    readonly status: number | undefined,
    readonly kind: 'http' | 'network' | 'timeout',
    /** Failed writes can have changed state. Refresh before offering a retry. */
    readonly mayHaveApplied = false,
    privacy = false,
  ) {
    super(privacy && status === 502
      ? 'Privacy mode may have been saved; refresh stream state before retrying.'
      : `${operation} failed${status === undefined ? '' : ` (HTTP ${status})`}${kind === 'timeout' ? ': request timed out' : ''}.`);
  }
}
