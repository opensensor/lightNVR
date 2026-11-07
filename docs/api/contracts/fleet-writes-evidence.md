# Fleet organization and live workspace contract evidence (F4)

Baseline: `84ed0174`; implemented in the isolated `codex/hardening-fleet-writes` worktree.
This slice accepts 31 previously residual registrations. Together with the existing
57 operations this isolated branch has 88 operations on 63 paths. The complete
API remains explicitly partial; coverage is not an authorization grant.

## Accepted route inventory

| Method | Native route | Operation | Successful payload |
|---|---|---|---|
| POST | `/api/locations` | `createFleetLocation` | 201 `FleetLocation` |
| PUT | `/api/locations/#` | `updateFleetLocation` | 200 `FleetLocation` |
| DELETE | `/api/locations/#` | `deleteFleetLocation` | 200 `FleetMutationAck` |
| POST | `/api/camera-tags` | `createFleetTag` | 201 `FleetTag` |
| PUT | `/api/camera-tags/#` | `updateFleetTag` | 200 `FleetTag` |
| DELETE | `/api/camera-tags/#` | `deleteFleetTag` | 200 `FleetMutationAck` |
| POST | `/api/camera-collections` | `createCameraCollection` | 201 `CameraCollection` |
| PUT | `/api/camera-collections/#` | `updateCameraCollection` | 200 `CameraCollection` |
| DELETE | `/api/camera-collections/#` | `deleteCameraCollection` | 200 `FleetMutationAck` |
| PUT | `/api/cameras/#/location` | `setCameraLocation` | 200 `CameraLocationAssignment` |
| PUT | `/api/cameras/#/tags` | `setCameraTags` | 200 `CameraTagAssignmentsResponse` |
| POST | `/api/camera-tags/#/merge` | `mergeFleetTag` | 200 `FleetTagMergeResult` |
| PUT | `/api/camera-collections/#/members` | `setCameraCollectionMembers` | 200 `CameraCollectionMembersResponse` |
| POST | `/api/fleet/views` | `createFleetView` | 201 `FleetSavedView` |
| PUT | `/api/fleet/views/#` | `updateFleetView` | 200 `FleetSavedView` |
| DELETE | `/api/fleet/views/#` | `deleteFleetView` | 200 `FleetMutationAck` |
| GET | `/api/live/layouts` | `listLiveLayouts` | 200 `LiveLayoutsResponse` |
| POST | `/api/live/layouts` | `createLiveLayout` | 201 `LiveLayout` |
| PUT | `/api/live/layouts/#` | `updateLiveLayout` | 200 `LiveLayout` |
| DELETE | `/api/live/layouts/#` | `deleteLiveLayout` | 200 `FleetMutationAck` |
| GET | `/api/camera-favorites` | `listCameraFavorites` | 200 `CameraFavoritesResponse` |
| PUT | `/api/camera-favorites/#` | `addCameraFavorite` | 200 `CameraFavorite` |
| DELETE | `/api/camera-favorites/#` | `deleteCameraFavorite` | 200 `CameraFavoriteRemoval` |
| GET | `/api/live/plans` | `listFloorPlans` | 200 `FloorPlansResponse` |
| GET | `/api/live/plans/#` | `getFloorPlan` | 200 `FloorPlan` |
| POST | `/api/live/plans` | `createFloorPlan` | 201 `FloorPlan` |
| PUT | `/api/live/plans/#` | `updateFloorPlan` | 200 `FloorPlan` |
| DELETE | `/api/live/plans/#` | `deleteFloorPlan` | 200 `FloorPlanDeletion` |
| GET | `/api/live/plans/#/background` | `getFloorPlanBackground` | 200 `PNG/JPEG bytes` |
| PUT | `/api/live/plans/#/background` | `setFloorPlanBackground` | 200 `FloorPlan` |
| DELETE | `/api/live/plans/#/background` | `deleteFloorPlanBackground` | 200 `FloorPlan` |

Every operation is attached to its existing native registration in
`docs/api/contracts/coverage.json`. Request schemas describe partial dictionary
updates, full revisioned workspace updates, tag/member sets and binary uploads
separately. Response objects are closed and reuse the accepted F2 location, tag,
collection and saved-view schemas. Floor-plan sketches use discriminated shapes;
placements, UUIDs, bounds, flags and revisions are validated. Existing native byte
length, reference and cross-field checks remain authoritative; documentation calls
out capacity, ownership, sharing and optimistic revision constraints.

## Authority and compatibility

- Location/tag dictionaries and collections require global `camera.configure`.
  Camera location assignment checks that camera's configuration permission; tag
  assignment additionally needs that camera's `live.view`. Denied tag/member
  replacements leave the previous sets intact. Missing referenced tags fail the
  entire replacement. Occupied locations cannot be deleted.
- Shared collection members/counts and floor-plan placements remain scoped to
  visible cameras. Static-to-smart transitions remove the explicit-members API;
  it returns 409 rather than silently replacing a selector.
- Personal saved views and layouts retain owner-only updates/deletion. Shared
  visibility does not grant mutation. Private foreign views return 404. Stale
  revisions fail with 409 and do not delete the current object. Only admins share.
- **Compatibility retained:** saved views still allow a scoped token to manage its
  owner's personal saved views, as the existing handler does. Layout/favorite
  writes reject scoped tokens; favorites reads do not borrow their owner's list.
  Favorite deletion remains possible after camera visibility is lost.
- Floor-plan mutations require both global `camera.configure` and a non-scoped
  identity. The new `non_scoped_global`/`non_scoped_camera` predicates combine
  those restrictions in discovery instead of advertising a forbidden operation.
  Basic authentication, browser sessions and legacy-key behavior are preserved.
  Demo mutation restrictions remain enforced by handlers; this receipt does not
  claim an independent demo-mode deployment qualification.

## Real consumer adoption

`web/js/api/fleetWrites.ts` is a closed route/operation boundary. It validates JSON
requests and actual responses using generated guards, checks expected status and
JSON content type, rejects unknown routes, sanitizes transport failures and
preserves cancellation. Raw uploads must be bounded PNG/JPEG blobs. Mutations
never retry automatically, even when an older caller supplies `retries: 1`.
Response contract errors retain `mayHaveApplied` for mutations.

The actual OrganizationManager, BulkOrganizeModal, CollectionManager,
SavedViewsBar, LiveOperatorNavigator, LiveBuildingPlan and cameraFavorites hook
now call this adapter. Their query caches and success callbacks receive only
validated data. The existing background image URL consumes validated plan UUIDs;
its binary content is checked independently in protocol tests.

## RED → GREEN and independent evidence

The first discovery test was run against the baseline native binary and failed:
none of the 31 new operations appeared in administrator discovery. After canonical
schemas, generated native bytes and projection changes, that same test passes.
The independent permission oracle is authored in `fleetWrites.api.spec.ts`, not
computed from the schema metadata.

Five owned native-server API cases cover:

1. Exact 31-operation discovery inventory.
2. Location/tag CRUD, assignment and merge; denied/invalid replacement no-effects;
   collection membership, scoped reads, static/smart transition and deletion.
3. Cross-owner shared view/layout denial, optimistic updates/deletion, and favorite
   isolation between users and scoped tokens.
4. Floor-plan create/read/update/delete and scoped placement filtering; stale and
   denied writes; exact PNG upload/download bytes; invalid upload preserves old
   bytes; JPEG replacement exact bytes; old-format file removal and deletion cleanup.
   The test inspects only its owned `lightnvr-contract-*` directory.
5. Independently enumerated Basic/session/token/empty-policy discovery expectations,
   session-owned view lifecycle, private cross-owner 404, and a globally configured
   scoped token still unable to delete a floor plan or discover that operation.

All reviewed JSON operations are exercised through the acceptance helper, which
validates both the canonical schema and the actual principal-projected response
schema; additional setup and no-effects inspection reads use the native wire API. The binary tests use
actual response bytes, content types and owned filesystem state, not only a JSON
acknowledgement. Existing F2 tests continue checking selectors, hidden ancestors,
scoped counts, collection privacy and saved-selector visibility.

A real Chromium session exercises SavedViewsBar: a malformed 201 response cannot
close/reset the form or display success, is never automatically retried, and a
subsequent valid native response refreshes the saved-view catalog. Unit adapter
cases additionally cover invalid outbound data before transport, malformed envelopes,
secret-bearing extra fields, sanitized failures, cancellation and bounded uploads.

## Validation receipt

Commands run from this worktree:

- `npm run contracts:generate`, `npm run contracts:check`: pass, deterministic artifacts.
- `npm run contracts:lint`: pass; four pre-existing recordings route ambiguity warnings.
- `npm run contracts:test-tools`: 10 tests pass.
- `npm run contracts:typecheck`, `npm run test:typecheck`: pass.
- `npm --prefix web run typecheck`, `npm --prefix web run build`: pass.
- `npm --prefix web test -- --runInBand`: 72 suites / 864 tests pass.
- `LIGHTNVR_CONTRACT_BIN=build/fleet-writes/bin/lightnvr npx playwright test -c tests/contracts/playwright.config.ts fleetWrites.api.spec.ts fleetWrites.ui.spec.ts fleet.api.spec.ts --project=api --project=ui`:
  10 tests pass; the final expanded PNG/JPEG/cleanup API suite additionally reran
  independently, 5/5 pass.
- Focused optimized CTest regex `(locations|camera_tags|fleet|live_saved_layouts|live_layouts|camera_favorites|operator_floor_plans|floor_plan_background|camera_collections|openapi_projection)`:
  15 domain binaries pass; the projection binary initially failed only its stale
  fixed path-count assertion. After changing that immutability assertion to compare
  the canonical input's path count, all 18 projection cases pass.
- Debug ASan/UBSan `test_openapi_projection`: 18 cases pass with leak detection and
  halt-on-error enabled. Existing domain handlers are unchanged by this slice.
- `git diff --check`: pass.

The native builds disable managed go2rtc, MQTT, SOD, LiteRT and Sentry. Fixtures
use isolated loopback servers/databases/storage, synthetic cameras and synthetic
image bytes. They do not qualify hardware/media/vendor behavior. No operator
configuration, production database or maintained-go2rtc fork was changed.

## Shared integration files

Canonical source, coverage and generated artifacts must be reconciled and regenerated
with the other slices. `generate_native.py`, `openapi_projection.c` and
`permissionCases.ts` reuse the identity-admin agent's shared predicate/discovery patch.
`contractErrors.ts` adds the fleet operation union and mutation uncertainty flags;
merge additively with identity changes. `test_openapi_projection.c` removes only a
stale fixed count and preserves the canonical-admin immutability assertion. Root
should retain the identity agent's new combined-predicate native tests on integration.
