/**
 * Shapes for the user management, API token, role, policy, and audit screens.
 * Read from src/web/api_handlers_authorization.c, api_handlers_audit.c,
 * api_handlers_users_backend_agnostic.c, and api_handlers_totp.c. `User` itself
 * is in core.ts.
 *
 * Secrets: only the three one-time responses (`ApiTokenCreateResponse.secret`,
 * `LegacyApiKeyResponse.api_key`, `TotpSetupResponse.secret`) and the form
 * password carry one. `ApiToken`, `RevealedSecret.token`, and `AuditEvent` have
 * no secret field on purpose: reading one, or writing one in an object literal,
 * fails to compile. A spread of a wider object is not caught, as with any type.
 */
import type { t as translate } from '../i18n.ts';
import type { components } from '../../generated/api/schema';

export type Translate = typeof translate;

/** The modals call this for each request; see getAuthHeaders in UsersView. */
export type GetAuthHeaders = () => Record<string, string>;

/** Caught fetchJSON errors. Only HTTPError in fetch-utils.ts sets `status`. */
export type ApiRequestError = Error & { status?: number };

// ---------------------------------------------------------------------------
// Users
// ---------------------------------------------------------------------------

/** GET /api/auth/users */
export type UsersResponse = components['schemas']['AdminUsers'];

/** The add and edit form. `role` is the numeric role, as on User. */
export interface UserFormData {
  username: string;
  password: string;
  email: string;
  role: number;
  is_active: boolean;
  password_change_locked: boolean;
  allowed_login_cidrs: string;
}

/** Request body built from the form. `role` is dropped for policy-mode users. */
export interface UserWritePayload extends Omit<UserFormData, 'role' | 'allowed_login_cidrs'> {
  role?: number;
  allowed_login_cidrs: string | null;
}

/** POST /api/auth/users/:id/login-lockout/clear */
export type ClearLoginLockoutResponse = components['schemas']['LoginLockoutCleared'];

/** GET /api/auth/users/:id/totp/status */
export type TotpStatusResponse = components['schemas']['TotpStatusResponse'];

/** POST /api/auth/users/:id/totp/setup */
export type TotpSetupResponse = components['schemas']['TotpSetupResponse'];

// ---------------------------------------------------------------------------
// Authorization catalog, roles, and policy
// ---------------------------------------------------------------------------

/** Scope types offered by the forms. The validators still reject other values at runtime. */
export type AccessScopeType = 'all' | 'collection' | 'selector';

/**
 * The selector JSON the server stores. Opaque here: only CollectionSelectorBuilder
 * reads the expression tree, and FleetSelector must stay assignable to this.
 */
export interface CameraSelector {
  version: number;
  expression: { op: string };
}

/** Item of GET /api/authorization/actions. The flags may be absent on older servers. */
export type AuthorizationAction = components['schemas']['AdminActionCatalogEntry'];

export type AuthorizationActionsResponse = components['schemas']['AdminActionCatalog'];

export interface AuthorizationActionGroup {
  category: string;
  actions: AuthorizationAction[];
}

/** Item of GET /api/authorization/roles. */
export type AuthorizationRole = components['schemas']['AdminAuthorizationRole'];

export type AuthorizationRolesResponse = components['schemas']['AdminRoles'];

/** POST and PUT /api/authorization/roles. */
export type AuthorizationRoleMutationResponse = components['schemas']['AdminRoleWritten'];

/** RoleManagerModal's editable copy of a role. */
export type RoleDraft = Pick<AuthorizationRole, 'uuid' | 'name' | 'description' | 'builtin'> & {actions: string[]};

/** The `scope` object on grants and tokens. The server sends both nullable fields. */
export interface AccessScope {
  type: AccessScopeType;
  selector: CameraSelector | null;
  collection_uuid: string | null;
}

export type AuthorizationGrant = components['schemas']['AdminPolicyGrant'];

/** GET and PUT /api/authorization/users/:id */
export type UserPolicyResponse = components['schemas']['AdminUserPolicy'];

/** AccessPolicyModal's editable copy of a grant. */
export interface DraftGrant {
  draftId: string;
  roleUuid: string;
  scopeType: AccessScopeType;
  selector: CameraSelector | null;
  collectionUuid: string;
  selectorError: string;
  persistedUuid?: string;
}

export interface PolicyDraft {
  mode: string;
  policyVersion: number;
  grants: DraftGrant[];
}

/** POST /api/authorization/simulate. Only the fields the panel reads are required. */
export type AuthorizationSimulationResult = components['schemas']['AdminSimulation'];

/** Item of GET /api/camera-collections. */
export interface AccessCollection {
  uuid: string;
  name: string;
  description?: string;
  type?: string;
  shared: boolean;
  owner_user_id?: number | null;
  member_count?: number;
  effective_count: number;
}

export interface AccessCollectionsResponse {
  collections?: AccessCollection[];
}

/** Item of GET /api/locations. The handler omits `path`; buildLocationRows derives it. */
export interface AccessLocation {
  uuid: string;
  name: string;
  path?: string;
  parent_uuid?: string | null;
  type?: string;
  sort_order?: number;
  is_system?: boolean;
  metadata?: Record<string, unknown>;
  child_count?: number;
  camera_count?: number;
  created_at?: number;
  updated_at?: number;
}

export interface AccessLocationsResponse {
  locations?: AccessLocation[];
}

/** Item of GET /api/camera-tags. */
export interface AccessTag {
  uuid: string;
  label: string;
  color?: string;
  description?: string;
  camera_count?: number;
  created_at?: number;
  updated_at?: number;
}

export interface AccessTagsResponse {
  tags?: AccessTag[];
}

// ---------------------------------------------------------------------------
// API tokens
// ---------------------------------------------------------------------------

/** Item of GET /api/authorization/users/:id/tokens. Holds the prefix, never the secret. */
export interface ApiToken {
  uuid: string;
  description: string;
  prefix: string;
  expires_at: number;
  revoked_at: number | null;
  last_used_at: number | null;
  created_at: number;
  actions: string[];
  scope: AccessScope;
}

export interface ApiTokensResponse {
  user_id?: number;
  count?: number;
  tokens?: ApiToken[];
}

/** POST /api/authorization/users/:id/tokens. `secret` is returned only here. */
export interface ApiTokenCreateResponse {
  secret: string;
  token: ApiToken;
}

/** POST /api/auth/users/:id/api-key. `api_key` is the one-time secret. */
export type LegacyApiKeyResponse = components['schemas']['LegacyApiKeyCreated'];

/** The value on screen in ApiKeyModal's reveal step. Component state only. */
export interface RevealedSecret {
  kind: 'scoped' | 'legacy';
  secret: string;
  token?: ApiToken;
}

export type ApiTokenConfirmation = { kind: 'revoke'; token: ApiToken } | { kind: 'legacy' };

export interface TokenDraft {
  description: string;
  expiryDays: number;
  actionKeys: string[];
  scopeType: AccessScopeType;
  collectionUuid: string;
  selector: CameraSelector | null;
  selectorError: string;
}

// ---------------------------------------------------------------------------
// Audit history
// ---------------------------------------------------------------------------

/** Item of GET /api/audit/events. `details` is free-form JSON per event type. */
export type AuditEvent = components['schemas']['AuditEvent'];

export type AuditEventsPage = components['schemas']['AuditEventsPage'];

/** The filter form. Every value is text, including the datetime-local bounds. */
export interface AuditFilters {
  principalUserId: string;
  action: string;
  outcome: string;
  eventType: string;
  targetUuid: string;
  requestId: string;
  since: string;
  until: string;
}

/** What the query helpers accept: the form's text, or epoch seconds for the bounds. */
export type AuditQueryFilters = Partial<Omit<AuditFilters, 'since' | 'until'>> & {
  since?: string | number;
  until?: string | number;
};

/** Item of AuditSettings.allowed_decision_modes. */
export type AuditDecisionModeEntry = components['schemas']['AuditDecisionMode'];

export interface AuditDecisionModeGroup {
  category: string;
  actions: AuditDecisionModeEntry[];
}

/** GET and PUT /api/audit/settings. `pruned_events` follows a retention change. */
export type AuditSettings = components['schemas']['AuditSettings'];

export interface AuditSummaryDetails {
  count: number;
  firstAt: number;
  lastAt: number;
}
