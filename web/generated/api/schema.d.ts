export interface paths {
    "/api/auth/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Start an authenticated session
         * @description Public credential submission. Successful JSON login sets the HttpOnly session cookie; MFA challenge returns totp_required/totp_token without a cookie. Optional inline TOTP is described; separate TOTP completion/enrollment remains unspecified. Form submissions redirect for success, credential rejection and rate limit. Form parsing uses username/password only; JSON optional fields are not form inputs.
         */
        post: operations["login"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/login/config": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Get public login configuration
         * @description Public booleans and timeout values needed before sign-in. No personalized discovery is exposed.
         */
        get: operations["getLoginConfig"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/logout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * End the current browser session
         * @description Public and idempotent. Deletes the session from the supplied cookie and clears session/trusted-device cookies. Does not revoke Basic credentials, legacy API keys or scoped API tokens. JSON200 requires Accept containing application/json or X-Requested-With; otherwise302 redirects to login.
         */
        post: operations["logout"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/sessions": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List own sessions
         * @description Session, Basic or legacy key personal self-service only. Scoped tokens, including users.manage tokens, cannot resolve a principal here; a valid session cookie retains existing precedence. Ownership is always the authenticated account, including administrators. Synthetic auth-disabled principals are rejected. Responses are private, no-store; credential material is never returned. Returns at most 32 unexpired entries ordered by most recent activity, then descending ID. There is no pagination in this bounded contract. Pending MFA challenges are excluded from sessions.
         */
        get: operations["listAuthSessions"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/sessions/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /**
         * Revoke own session
         * @description Session, Basic or legacy key personal self-service only. Scoped tokens, including users.manage tokens, cannot resolve a principal here; a valid session cookie retains existing precedence. Ownership is always the authenticated account, including administrators. Synthetic auth-disabled principals are rejected. Responses are private, no-store; credential material is never returned. The ID must be a canonical positive decimal safe integer without signs, leading zeros, escapes or trailing path segments. Revocation is independent of the 32-entry list limit, can remove expired records, and excludes pending MFA sessions. Current-cookie clearing uses the exact removed credential. Missing or another owner’s ID returns the same 404; database failure is 500. A successful revoke invalidates future use, but does not cancel already-authorized in-flight requests.
         */
        delete: operations["revokeAuthSession"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/trusted-devices": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List own trusted devices
         * @description Session, Basic or legacy key personal self-service only. Scoped tokens, including users.manage tokens, cannot resolve a principal here; a valid session cookie retains existing precedence. Ownership is always the authenticated account, including administrators. Synthetic auth-disabled principals are rejected. Responses are private, no-store; credential material is never returned. Returns at most 32 unexpired entries ordered by most recent activity, then descending ID. There is no pagination in this bounded contract. Pending MFA challenges are excluded from sessions.
         */
        get: operations["listTrustedDevices"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/trusted-devices/{id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /**
         * Revoke own trusted device
         * @description Session, Basic or legacy key personal self-service only. Scoped tokens, including users.manage tokens, cannot resolve a principal here; a valid session cookie retains existing precedence. Ownership is always the authenticated account, including administrators. Synthetic auth-disabled principals are rejected. Responses are private, no-store; credential material is never returned. The ID must be a canonical positive decimal safe integer without signs, leading zeros, escapes or trailing path segments. Revocation is independent of the 32-entry list limit, can remove expired records, and excludes pending MFA sessions. Current-cookie clearing uses the exact removed credential. Missing or another owner’s ID returns the same 404; database failure is 500. A successful revoke invalidates future use, but does not cancel already-authorized in-flight requests.
         */
        delete: operations["revokeTrustedDevice"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/verify": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Verify current account identity
         * @description Session, Basic or legacy key only; scoped tokens do not resolve here. Required-password-change accounts may use verify. Auth-disabled mode returns synthetic admin and anonymous demo returns synthetic viewer; neither permits personalized discovery. Missing authentication clears stale session cookie.
         */
        get: operations["verifyIdentity"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/authorization/users/{user_id}/tokens": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive user ID. Non-scoped caller may manage their own tokens; other owners require users.manage. Visibility never grants authority over arbitrary IDs. */
                user_id: number;
            };
            cookie?: never;
        };
        /**
         * List token metadata for an allowed owner
         * @description Requires a non-scoped principal: self is allowed; another owner requires users.manage. Scoped credentials are always403 even with users.manage. Invalid path may produce400 before authentication. Current target and policy checks still run for each request.
         */
        get: operations["listUserTokens"];
        put?: never;
        /**
         * Create a scoped token
         * @description Requires a non-scoped principal: self is allowed; another owner requires users.manage. Scoped credentials are always403 even with users.manage. Invalid path may produce400 before authentication. Current target and policy checks still run for each request. Requested actions and scope must be contained by both owner and requester. Only201 returns the one-time secret. Denied/invalid issuance leaves tokens and policy unchanged; audit entries may be written.
         */
        post: operations["createUserToken"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/authorization/users/{user_id}/tokens/{token_uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Token UUID belonging to this owner. */
                token_uuid: string;
                /** @description Positive user ID. Non-scoped caller may manage their own tokens; other owners require users.manage. Visibility never grants authority over arbitrary IDs. */
                user_id: number;
            };
            cookie?: never;
        };
        get?: never;
        put?: never;
        post?: never;
        /**
         * Revoke an allowed owner token
         * @description Requires a non-scoped principal: self is allowed; another owner requires users.manage. Scoped credentials are always403 even with users.manage. Invalid path may produce400 before authentication. Current target and policy checks still run for each request. Revocation invalidates subsequent credential requests; it does not rewrite owner policy.
         */
        delete: operations["revokeUserToken"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/client-config": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Get safe browser playback configuration
         * @description Any usable principal, including scoped tokens, can read this intentionally limited shape. Existing auth-disabled/demo behavior remains; this is not public in normal authenticated mode.
         */
        get: operations["getClientConfig"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/openapi.json": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Discover the current principal’s API contract
         * @description Uses actual session, Basic, legacy API-key or scoped-token identity with existing credential precedence and restrictions. Synthetic auth_disabled/anonymous demo principals are rejected with403. Real authenticated demo users retain their actual action permissions. No requested-user/role override. Every response is private and non-cacheable; downloaded documentation never grants authority.
         */
        get: operations["getOpenApi"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List authorized recordings
         * @description Replay-authorized rows and counts; empty scope without an explicit stream returns an empty page. Every explicitly named stream must be authorized or the request returns generic403. Completed rows and sufficiently recent in-progress rows are included; stale incomplete rows age out. Filters and collection membership intersect authorization. Current handlers preserve auth-disabled/demo deployment behavior.
         */
        get: operations["listRecordings"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/{recording_id}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        /**
         * Read recording detail
         * @description Authorizes recordings.replay using current raw facts for stored capture identity before emitting metadata. This response does not include tags; use the separate tags endpoint.
         */
        get: operations["getRecording"];
        put?: never;
        post?: never;
        /**
         * Delete a recording
         * @description Authorizes the stored capture camera UUID using current raw facts. A deleted capture camera returns404; legacy null/empty UUID rows fall back to the current stream name. Protected/in-progress/active-transfer media returns409. A202 response means deletion is queued, not that bytes have disappeared.
         */
        delete: operations["deleteRecording"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/{recording_id}/protect": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        get?: never;
        /**
         * Set recording protection
         * @description Authorizes the stored capture camera UUID using current raw facts. A deleted capture camera returns404; legacy null/empty UUID rows fall back to the current stream name. Protected media cannot be deleted by ordinary recording deletion.
         */
        put: operations["setRecordingProtection"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/{recording_id}/retention": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        get?: never;
        /**
         * Set or clear a recording retention override
         * @description Authorizes the stored capture camera UUID using current raw facts. A deleted capture camera returns404; legacy null/empty UUID rows fall back to the current stream name. Negative input clears the override, zero means indefinite. The returned integer reflects the converted input; clearing does not normalize it to -1.
         */
        put: operations["setRecordingRetention"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/{recording_id}/tags": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        /**
         * Read an authorized recording’s tags
         * @description Uses stored capture identity and recordings.replay before returning tags.
         */
        get: operations["getRecordingTags"];
        /**
         * Replace recording tags
         * @description Authorizes the stored capture camera UUID using current raw facts. A deleted capture camera returns404; legacy null/empty UUID rows fall back to the current stream name. Authorization precedes tag mutation.
         */
        put: operations["setRecordingTags"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-delete": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Create a recording deletion job
         * @description Fixed IDs preflight every existing member before dispatch; forbidden members reject the whole batch. Filter jobs capture authorized UUID scope before dispatch. Operational missing/protected failures are counted separately and can coexist with successes. Poll the owned job; cancellation is unsupported.
         */
        post: operations["deleteRecordings"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-delete/progress/{job_id}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read owned recording deletion progress
         * @description Requires the exact creator user ID and exact scoped-token UUID, when created by a scoped token. An administrator, sibling token or owner using another credential cannot borrow a scoped-token-owned job. Unknown or differently owned jobs return404. Discovery visibility is authenticated; camera grants are not rechecked for job lookup. Terminal complete includes partial operational failures; inspect all counters. There is no cancel operation.
         */
        get: operations["getRecordingDeletionProgress"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-download": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Create an authorized ZIP export job
         * @description Preflights every requested recording before creating the job. Missing404/in-progress409 may abort in input order before a later forbidden member; these early errors create no job or archive. ZIP32 uses original media, including recovery-download files with saved playback issues. Detected unavailable-source/member-count and ZIP32-limit failures reject the result. The current writer does not consistently detect filesystem read/write/close failures; complete alone does not prove archive integrity. Cancellation is unsupported.
         */
        post: operations["createRecordingExport"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-download/result/{export_token}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Download an owned completed ZIP export
         * @description Requires the exact creator user ID and exact scoped-token UUID, when created by a scoped token. An administrator, sibling token or owner using another credential cannot borrow a scoped-token-owned job. Unknown or differently owned jobs return404. Discovery visibility is authenticated; camera grants are not rechecked for job lookup. Export ownership also matches username. Pending/running jobs return409 with error Job is not_ready; failed jobs return409 with error Job is error. Only complete archives are downloadable. Results can be read repeatedly while retained; result access refreshes job retention. This is not a one-time token. No cancellation endpoint exists.
         */
        get: operations["downloadRecordingExport"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-download/status/{export_token}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read owned recording export progress
         * @description Requires the exact creator user ID and exact scoped-token UUID, when created by a scoped token. An administrator, sibling token or owner using another credential cannot borrow a scoped-token-owned job. Unknown or differently owned jobs return404. Discovery visibility is authenticated; camera grants are not rechecked for job lookup. Export ownership also matches the stored username. Jobs can be reclaimed after their retention interval; polling does not make a job permanent.
         */
        get: operations["getRecordingExportStatus"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-protect": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Set protection on recording IDs
         * @description An otherwise-valid forbidden existing member yields403 before any protection writes. Invalid/missing IDs and subsequent database write failures are counted, so operational failures may produce partial success.
         */
        post: operations["protectRecordings"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/batch-tags": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Add or remove tags on recording IDs
         * @description Preflights all IDs and rejects missing/forbidden members before writes. Invalid IDs return400. Operational database failures do not promise an all-or-nothing transaction.
         */
        post: operations["updateRecordingTagsBatch"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/detection-labels": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List authorized detection label facets
         * @description Unique detection labels within replay-authorized camera scope. An empty scope returns labels:[].
         */
        get: operations["listRecordingDetectionLabels"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/download/{recording_id}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        /**
         * Download or prepare original recording media
         * @description Authorizes the stored capture camera UUID using current raw facts. A deleted capture camera returns404; legacy null/empty UUID rows fall back to the current stream name. Requires recordings.export before source probes. In-progress media returns409. Saved playback failures do not block original-byte recovery download. prepare=1 returns ready/preparing JSON; ordinary success is an attachment. No compatibility transcoding is requested by this route. Byte ranges use the same permissive local parser as playback: prefix/suffix/open-ended accepted and clamped, malformed missing-dash/multiple ranges may parse as a single range. Local416 omits Content-Range.
         */
        get: operations["downloadRecording"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/play/{recording_id}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        /**
         * Prepare or play an authorized recording
         * @description Requires recordings.replay for capture identity before inspecting source availability. In-progress409; saved invalid_media/no_video/decode_error422 with recovery explanation. prepare=1 returns JSON ready/preparing; ordinary success serves media bytes. Archive/HEVC preparation can return202 (prepare) or503 (ordinary) with Retry-After2. Authorization uses the logical recording, never a caller-supplied filesystem path. Local byte ranges accept prefix, suffix and open-ended forms; excessive end/suffix clamp to EOF. Invalid unit, start≥size, reversed bounds and zero suffix yield416 JSON. Local416 currently omits Content-Range; archive416 supplies bytes *\/size. Existing parser is permissive: bytes=0 (missing dash) becomes full206 and multi-ranges are treated as their first numeric range, not multipart. Do not infer strict RFC rejection of malformed ranges.
         */
        get: operations["playRecording"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/tags": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List authorized recording tag facets
         * @description Unique recording tags within replay-authorized camera scope. An empty scope returns tags:[]; no hidden-camera facet values are published.
         */
        get: operations["listRecordingTags"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/recordings/thumbnail/{recording_id}/{index}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Thumbnail sample index. */
                index: number;
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        /**
         * Read an authorized recording thumbnail
         * @description JPEG cache/generation requires capture-camera recordings.replay. Global disabled thumbnails403 and disabled hover frames403 are checked before auth. Otherwise authorization precedes cache/file access. Index0,1,2 selects start/middle/end samples; index>0 may be disabled by thumbnails_per_recording=1. Cache uses private max-age86400; source preparation/deletion can return503/409. Thumbnails do not assert original media is playable.
         */
        get: operations["getRecordingThumbnail"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/streams": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List authorized streams
         * @description Authenticated list discovery remains available even with no matching camera grants. Actual rows and totals are filtered per camera. Permission checks use runtime plus bounded go2rtc health observations; this can differ from full-read raw-health authorization. This pilot describes the existing auth-disabled/demo behavior only as a deployment caveat; personalized discovery itself requires a real identity.
         */
        get: operations["listStreams"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/streams/{name}": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Current stream name, percent-encoded as one URL segment. The runtime stores up to 255 UTF-8 bytes. Embedded slash is unsupported by the full-read suffix parser; normal creation restricts names to letters, digits, dot, hyphen and underscore. Legacy configured names may require encoding. UUID is not accepted in place of the name. */
                name: string;
            };
            cookie?: never;
        };
        get?: never;
        /**
         * Update stream configuration or privacy
         * @description Requires camera.configure on the named camera using raw DB facts; no read permission is implied. Boolean set_privacy_mode selects a privacy-only request and rejects accompanying configuration fields. Ordinary updates are partial; enable_disabled=true with enabled=false is rejected. Unknown ordinary fields are ignored, and 202 is not a durable-success guarantee. A 502 privacy response can follow a saved flag when go2rtc removal fails; retry is required and the stream may remain viewable in the meantime.
         */
        put: operations["updateStream"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/streams/{name}/full": {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Current stream name, percent-encoded as one URL segment. The runtime stores up to 255 UTF-8 bytes. Embedded slash is unsupported by the full-read suffix parser; normal creation restricts names to letters, digits, dot, hyphen and underscore. Legacy configured names may require encoding. UUID is not accepted in place of the name. */
                name: string;
            };
            cookie?: never;
        };
        /**
         * Read full stream configuration
         * @description Requires live.view on this camera using raw DB facts. camera.configure on the same camera using runtime facts determines sensitive fields. A configure-only grant does not imply read access.
         */
        get: operations["getStreamFull"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/timeline/segments": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read an authorized camera timeline
         * @description Requires stream or camera_uuid. Explicit camera_uuid is authoritative even with a stale stream name. Raw capture-camera replay authorization precedes timeline lookup; deleted cameras return404. Timeline presence does not imply playable media.
         */
        get: operations["getTimelineSegments"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/timeline/segments-by-ids": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read authorized timeline segments by stable IDs
         * @description Filters each recording through stored capture identity and raw replay permission. Hidden/missing/orphaned entries do not contribute counts or overall time bounds. Does not reveal which requested IDs were denied.
         */
        get: operations["getTimelineSegmentsByIds"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
}
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        /** @description Most errors use a string error. YAML validation may use valid=false and error.message. Extra diagnostic fields may be added; clients must not log privileged request or response data. */
        ApiError: {
            error: string | ({
                message: string;
            } & {
                [key: string]: unknown;
            });
            valid?: boolean;
        } & {
            [key: string]: unknown;
        };
        /** @description Only successful token creation returns a secret. No secret value is included in examples. */
        ApiTokenCreated: {
            /** @description One-time bearer secret; store securely after creation. Ordinary token-list/identity responses never include it. */
            readonly secret: string;
            token: components["schemas"]["ApiTokenMetadata"];
        } & {
            [key: string]: unknown;
        };
        /** @description Requested action and camera scope must be contained in BOTH token owner and requester authority. At most256 active tokens per owner. Additional request keys do not rewrite user policy. */
        ApiTokenCreateRequest: {
            /** @description Duplicates collapse into an action mask. */
            actions: components["schemas"]["AuthorizationAction"][];
            /** @description Trimmed by server; must contain non-whitespace text, no control characters, and fewer than128 UTF-8 bytes. */
            description: string;
            /** @description Unix seconds strictly after server time and at most366 days ahead. */
            expires_at: number;
            scope: components["schemas"]["ApiTokenScope"];
        } & {
            [key: string]: unknown;
        };
        /** @description Lists token metadata, including revoked/expired records. Secret is never returned. */
        ApiTokenList: {
            api_key?: never;
            count: number;
            password?: never;
            password_hash?: never;
            secret?: never;
            token_hash?: never;
            tokens: components["schemas"]["ApiTokenMetadata"][];
            totp_secret?: never;
            totp_token?: never;
            user_id: number;
        } & {
            [key: string]: unknown;
        };
        /** @description Non-secret token record; prefix is an identifier, never a reusable credential. Secret/hash fields are explicitly forbidden. */
        ApiTokenMetadata: {
            actions: components["schemas"]["AuthorizationAction"][];
            api_key?: never;
            created_at: number;
            description: string;
            expires_at: number;
            last_used_at: number | null;
            password?: never;
            password_hash?: never;
            prefix: string;
            revoked_at: number | null;
            scope: components["schemas"]["ApiTokenStoredScope"];
            secret?: never;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            uuid: string;
        } & {
            [key: string]: unknown;
        };
        /** @description All, selector or collection camera scope. Inapplicable selector/collection fields may be absent or null. A narrow owner cannot delegate all scope simply because current matches happen to be identical. */
        ApiTokenScope: ({
            collection_uuid?: null;
            selector?: null;
            /** @constant */
            type: "all";
        } & {
            [key: string]: unknown;
        }) | ({
            collection_uuid?: null;
            selector: components["schemas"]["TokenSelector"];
            /** @constant */
            type: "selector";
        } & {
            [key: string]: unknown;
        }) | ({
            collection_uuid: string;
            selector?: null;
            /** @constant */
            type: "collection";
        } & {
            [key: string]: unknown;
        });
        ApiTokenStoredScope: ({
            collection_uuid: null;
            selector: null;
            /** @constant */
            type: "all";
        } & {
            [key: string]: unknown;
        }) | ({
            collection_uuid: null;
            selector: components["schemas"]["TokenSelector"];
            /** @constant */
            type: "selector";
        } & {
            [key: string]: unknown;
        }) | ({
            collection_uuid: string;
            selector: null;
            /** @constant */
            type: "collection";
        } & {
            [key: string]: unknown;
        });
        AuthDemoIdentity: {
            api_key?: never;
            /** @constant */
            authenticated: false;
            /** @constant */
            demo_mode: true;
            /** @constant */
            must_change_password: false;
            password?: never;
            password_hash?: never;
            /** @constant */
            role: "viewer";
            secret?: never;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            /** @constant */
            username: "demo";
        } & {
            [key: string]: unknown;
        };
        AuthDisabledIdentity: {
            api_key?: never;
            auth_absolute_timeout_hours: number;
            /** @constant */
            auth_enabled: false;
            auth_timeout_hours: number;
            /** @constant */
            authenticated: true;
            /** @constant */
            must_change_password: false;
            password?: never;
            password_hash?: never;
            /** @constant */
            role: "admin";
            secret?: never;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            trusted_device_days: number;
            /** @constant */
            username: "admin";
        } & {
            [key: string]: unknown;
        };
        /** @enum {string} */
        AuthorizationAction: "live.view" | "audio.listen" | "audio.talk" | "recordings.replay" | "recordings.export" | "snapshot.create" | "ptz.control" | "evidence.protect" | "recording.delete" | "camera.configure" | "fleet.execute_job" | "storage.configure" | "events.configure" | "users.manage" | "system.admin" | "lpr.read" | "lpr.search" | "lpr.export" | "lpr.delete";
        AuthSession: {
            created_at: number;
            current: boolean;
            expires_at: number;
            id: number;
            idle_expires_at: number;
            ip_address: string;
            last_activity_at: number;
            user_agent: string;
        };
        AuthSessionsResponse: {
            sessions: components["schemas"]["AuthSession"][];
        };
        AuthVerifiedIdentity: {
            api_key?: never;
            auth_absolute_timeout_hours: number;
            /** @constant */
            auth_enabled: true;
            auth_timeout_hours: number;
            /** @constant */
            authenticated: true;
            email: string;
            id: number;
            is_active: boolean;
            must_change_password: boolean;
            password?: never;
            password_change_locked: boolean;
            password_hash?: never;
            role: string;
            role_id: number;
            secret?: never;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            trusted_device_days: number;
            username: string;
        } & {
            [key: string]: unknown;
        };
        /** @description Real credential identity, auth-disabled synthetic admin, or unauthenticated demo viewer. Synthetic modes do not authorize personalized discovery. */
        AuthVerifyResponse: components["schemas"]["AuthVerifiedIdentity"] | components["schemas"]["AuthDisabledIdentity"] | components["schemas"]["AuthDemoIdentity"];
        /** @description Safe browser playback configuration only. Deliberately closed shape: no storage paths, integrations, private settings or credentials. go2rtc_available may stay true while its lifecycle guard is busy. */
        ClientConfig: {
            api_key?: never;
            audio_disabled: boolean;
            auto_disabled: boolean;
            /** @enum {string} */
            default_playback_transport: "auto" | "webrtc_only" | "mse_only" | "hls_only" | "webrtc_then_mse" | "mse_then_hls";
            demo_mode: boolean;
            generate_thumbnails: boolean;
            go2rtc_api_port: number;
            go2rtc_available: boolean;
            go2rtc_base_path: string;
            go2rtc_enabled: boolean;
            go2rtc_force_native_hls: boolean;
            hls_disabled: boolean;
            mse_disabled: boolean;
            password?: never;
            password_hash?: never;
            secret?: never;
            thumbnails_per_recording: number;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            web_auth_enabled: boolean;
            webrtc_connection_timeout_ms: number;
            webrtc_disabled: boolean;
            webrtc_ice_recovery_timeout_ms: number;
        };
        CredentialRevoked: {
            /** @constant */
            success: true;
        };
        /** @description summary=true, surface=live. Uses the same filtered pagination envelope, with playback-specific row fields. */
        LiveStreamSummaryPage: {
            page: number;
            page_size: number;
            streams: (components["schemas"]["LiveStreamSummaryRedacted"] | components["schemas"]["LiveStreamSummaryPrivileged"])[];
            total: number;
            total_pages: number;
        } & {
            [key: string]: unknown;
        };
        /** @description surface=live includes playback and capability hints; admin_url is never emitted by this surface. */
        LiveStreamSummaryPrivileged: components["schemas"]["StreamSummaryBase"] & ({
            backchannel_enabled: boolean;
            /** @constant */
            can_configure: true;
            /** @constant */
            can_control_privacy: true;
            detection_model: string;
            eptz_config: string;
            go2rtc_hls_available: boolean;
            has_sub_stream: boolean;
            playback_transport: components["schemas"]["PlaybackTransport"];
            privacy_mode: boolean;
            ptz_enabled: boolean;
            streaming_enabled: boolean;
        } & {
            [key: string]: unknown;
        });
        /** @description surface=live includes playback and capability hints; admin_url is never emitted by this surface. */
        LiveStreamSummaryRedacted: components["schemas"]["StreamSummaryBase"] & ({
            backchannel_enabled: boolean;
            /** @constant */
            can_configure: false;
            /** @constant */
            can_control_privacy: false;
            detection_model: string;
            eptz_config: string;
            go2rtc_hls_available: boolean;
            has_sub_stream: boolean;
            playback_transport: components["schemas"]["PlaybackTransport"];
            privacy_mode: boolean;
            ptz_enabled: boolean;
            streaming_enabled: boolean;
        } & {
            [key: string]: unknown;
        });
        LoginConfig: {
            api_key?: never;
            auth_absolute_timeout_hours: number;
            auth_timeout_hours: number;
            force_mfa_on_login: boolean;
            password?: never;
            password_hash?: never;
            remember_device_enabled: boolean;
            secret?: never;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
            trusted_device_days: number;
        };
        LoginMfaChallenge: {
            api_key?: never;
            password?: never;
            password_hash?: never;
            secret?: never;
            token_hash?: never;
            /** @constant */
            totp_required: true;
            totp_secret?: never;
            /** @description Ephemeral MFA challenge credential. Returned only by login; never an ordinary identity/token-list field or documentation example. */
            readonly totp_token: string;
        } & {
            [key: string]: unknown;
        };
        /** @description JSON credentials. The C handler copies username/password into 64-byte buffers (63-byte payload); it does not reject oversized strings before truncating. Invalid credentials return401. Optional fields with wrong types are ignored by current C; this schema describes typed client input. */
        LoginRequest: {
            password: string;
            remember_device?: boolean;
            /** @description Optional inline MFA code; enrollment and challenge completion endpoints remain outside this slice. */
            totp_code?: string;
            username: string;
        } & {
            [key: string]: unknown;
        };
        LoginResponse: components["schemas"]["LoginSuccess"] | components["schemas"]["LoginMfaChallenge"];
        LoginSuccess: {
            api_key?: never;
            must_change_password: boolean;
            password?: never;
            password_hash?: never;
            /** @constant */
            redirect: "/index.html";
            secret?: never;
            /** @constant */
            success: true;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
        } & {
            [key: string]: unknown;
        };
        LogoutResponse: {
            api_key?: never;
            password?: never;
            password_hash?: never;
            /** @constant */
            redirect: "/login.html?logout=true";
            secret?: never;
            /** @constant */
            success: true;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
        } & {
            [key: string]: unknown;
        };
        /** @description surface=lpr exposes only camera identity and requires both lpr.search and lpr.read for each returned camera. It does not imply live.view or configuration access. */
        LprStreamSummary: {
            camera_uuid: string;
            name: string;
        } & {
            [key: string]: unknown;
        };
        /** @description summary=true, surface=lpr. Empty results remain a valid response even with no LPR grants. */
        LprStreamSummaryPage: {
            page: number;
            page_size: number;
            streams: components["schemas"]["LprStreamSummary"][];
            total: number;
            total_pages: number;
        } & {
            [key: string]: unknown;
        };
        /** @description Describes the canonical pilot coverage, not user or camera identities. Omission of an unspecified API is not a denial of access. */
        OpenApiCoverage: {
            canonical_revision: string;
            covered_operations: string[];
            description?: string;
            /** @constant */
            partial: true;
        } & {
            [key: string]: unknown;
        };
        /** @description A valid OpenAPI 3.1.1 document projected for the actual authenticated principal. This response schema checks its envelope; independent OpenAPI lint checks document validity. It can contain no camera-specific eligible operations while retaining authenticated discovery/list operations. */
        OpenApiDocument: {
            components?: {
                [key: string]: unknown;
            };
            info: {
                title: string;
                version: string;
            } & {
                [key: string]: unknown;
            };
            /** @constant */
            openapi: "3.1.1";
            paths: {
                [key: string]: unknown;
            };
            "x-lightnvr-coverage": components["schemas"]["OpenApiCoverage"];
        } & {
            [key: string]: unknown;
        };
        /** @enum {string} */
        PlaybackTransport: "auto" | "webrtc_only" | "mse_only" | "hls_only" | "webrtc_then_mse" | "mse_then_hls";
        /** @description Empty arrays are accepted. Invalid or missing IDs count as failures; an existing forbidden member rejects the entire request before writes. */
        RecordingBatchProtectionRequest: {
            ids: unknown[];
            protected: boolean;
        } & {
            [key: string]: unknown;
        };
        RecordingBatchProtectionResult: {
            fail_count: number;
            message: string;
            protected: boolean;
            success_count: number;
        } & {
            [key: string]: unknown;
        };
        /** @description Add/remove are optional; wrong-typed arrays and non-string or empty elements are ignored by the handler. All IDs are authorized before writes. Database failures may follow earlier successful tag writes. */
        RecordingBatchTagsRequest: {
            add?: unknown[];
            ids: number[];
            remove?: unknown[];
        } & {
            [key: string]: unknown;
        };
        RecordingBatchTagsResult: {
            /** @description Number of authorized input IDs processed, including duplicate IDs. */
            recordings_count: number;
            /** @description Successful per-recording add statement executions, including INSERT OR IGNORE no-ops; not changed rows. */
            tags_added: number;
            /** @description Successful per-recording remove statement executions, including absent-tag no-ops; not changed rows. */
            tags_removed: number;
        } & {
            [key: string]: unknown;
        };
        /** @description stream_name (or stream fallback) selects comma-separated current camera names. Without a nonempty stream filter, all-fleet recording.delete authority is required. Authorized camera UUIDs and collection membership are captured before dispatch; caller-supplied camera UUID fields do not grant scope. start/end are UTC timestamp strings. detection/protected numeric values use C integer conversion; protected is clamped to -1..1. In-progress rows are excluded. */
        RecordingDeletionFilter: {
            capture_method?: string;
            collection_uuid?: string;
            detection?: number;
            detection_label?: string;
            end?: string;
            protected?: number;
            start?: string;
            stream?: string;
            stream_name?: string;
            tag?: string;
        } & {
            [key: string]: unknown;
        };
        RecordingDeletionJobAccepted: {
            job_id: string;
            /** @constant */
            status: "started";
        } & {
            [key: string]: unknown;
        };
        /** @description complete is true for both complete and error terminal states. A complete job can have failed members or queued pending deletions; it does not imply every file is already removed. SQL failure is error; an empty successful filter completes with zero counts. No cancellation endpoint/state exists. */
        RecordingDeletionProgress: {
            complete: boolean;
            current: number;
            error_message?: string;
            failed: number;
            job_id: string;
            pending_deletions: number;
            /** @enum {string} */
            status: "pending" | "running" | "complete" | "error" | "unknown";
            status_message: string;
            succeeded: number;
            total: number;
        } & {
            [key: string]: unknown;
        };
        RecordingDeletionRequest: ({
            ids: unknown[];
        } & {
            [key: string]: unknown;
        }) | ({
            filter: components["schemas"]["RecordingDeletionFilter"];
            /** @description Array-valued ids take precedence over filter and must be nonempty. Other ids values fall through to the filter handler. */
            ids?: unknown[] | (null | boolean | Record<string, never> | number | string);
        } & {
            [key: string]: unknown;
        });
        /** @description Immediate200 has success/message. Queued202 also carries status deletion_pending; neither outcome promises deletion of unrelated recordings. */
        RecordingDeletionResult: {
            message: string;
            /** @constant */
            status?: "deletion_pending";
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        /** @description Authorized recording metadata. id is durable database identity, not list position. Times are UTC display strings with separate Unix seconds. In-progress rows have null end_time/end_time_unix and changing effective duration. playback_issue is empty for no saved issue; invalid_media/no_video/decode_error cause playback422. Other diagnostic values do not conclusively prohibit playback. Optional storage fields are omitted if their lookup fails. file_path is existing authorized metadata, not a client-selected playback path. A finalized row or timeline presence does not guarantee playable media. */
        RecordingDetail: {
            camera_uuid: string | null;
            capture_method: string;
            deletion_pending?: boolean;
            detection_labels?: ({
                count: number;
                label: string;
            } & {
                [key: string]: unknown;
            })[];
            durable_copy_count?: number;
            duration: number;
            end_time: string | null;
            end_time_unix: number | null;
            external_source?: boolean;
            file_path: string;
            has_detection: boolean;
            id: number;
            in_progress: boolean;
            playback_issue: string;
            protected: boolean;
            /** @enum {unknown} */
            retention_mode?: "inherit" | "indefinite" | "finite";
            schedule_restricted: boolean | null;
            size: string;
            start_time: string;
            start_time_unix: number;
            storage_policy_uuid?: string;
            storage_policy_version?: number;
            /** @enum {unknown} */
            storage_state?: "deletion_pending" | "preparing" | "unavailable" | "archiving" | "archived" | "hot_and_archive" | "hot";
            storage_target_uuid?: string;
            stream: string;
        } & {
            [key: string]: unknown;
        };
        RecordingDetectionLabels: {
            labels: string[];
        } & {
            [key: string]: unknown;
        };
        RecordingExportAccepted: {
            token: string;
            total: number;
        } & {
            [key: string]: unknown;
        };
        /** @description Positive numeric IDs are converted to uint64. Filename is sanitized for attachment delivery. Every member is authorized before a job is created; existing operational missing/incomplete errors can abort earlier in input order. */
        RecordingExportRequest: {
            filename?: string;
            ids: number[];
        } & {
            [key: string]: unknown;
        };
        /** @description error is always present, usually empty until failure. Detected unavailable sources or ZIP32 member/size-limit failures put the job in error and remove its temporary result. The current writer does not consistently check filesystem read/write/close errors: complete is not a guarantee against undetected I/O failure or a valid intact archive. No cancellation endpoint/state exists. */
        RecordingExportStatus: {
            current: number;
            error: string;
            /** @enum {string} */
            status: "pending" | "running" | "complete" | "error";
            total: number;
        } & {
            [key: string]: unknown;
        };
        RecordingListItem: {
            camera_uuid: string | null;
            capture_method: string;
            deletion_pending?: boolean;
            detection_labels?: ({
                count: number;
                label: string;
            } & {
                [key: string]: unknown;
            })[];
            durable_copy_count?: number;
            duration: number;
            end_time: string | null;
            end_time_unix: number | null;
            external_source?: boolean;
            file_path: string;
            has_detection: boolean;
            id: number;
            in_progress: boolean;
            playback_issue: string;
            protected: boolean;
            /** @enum {unknown} */
            retention_mode?: "inherit" | "indefinite" | "finite";
            schedule_restricted: boolean | null;
            size: string;
            start_time: string;
            start_time_unix: number;
            storage_policy_uuid?: string;
            storage_policy_version?: number;
            /** @enum {unknown} */
            storage_state?: "deletion_pending" | "preparing" | "unavailable" | "archiving" | "archived" | "hot_and_archive" | "hot";
            storage_target_uuid?: string;
            stream: string;
            tags: string[];
        } & {
            [key: string]: unknown;
        };
        /** @description Counts use the same authorized predicate as rows. Ordinary totals may use a ten-second cache invalidated by supported database mutations; limit=all recounts and resets page1. No top-level total is emitted. When replay scope is empty, the early empty envelope retains the normalized requested page and ordinary limit (all is treated as20). */
        RecordingPagination: {
            limit: number;
            page: number;
            pages: number;
            total: number;
        } & {
            [key: string]: unknown;
        };
        RecordingPlaybackPreparing: {
            error?: string;
            /** @constant */
            source?: "archive";
            /** @constant */
            status: "preparing";
        } & {
            [key: string]: unknown;
        };
        RecordingPlaybackReady: {
            /** @constant */
            source?: "archive";
            /** @constant */
            status: "ready";
        } & {
            [key: string]: unknown;
        };
        RecordingProtectionRequest: {
            protected: boolean;
        } & {
            [key: string]: unknown;
        };
        RecordingProtectionResult: {
            id: number;
            message: string;
            protected: boolean;
        } & {
            [key: string]: unknown;
        };
        /** @description Numeric input is truncated toward zero through the C JSON integer accessor. Any negative value clears the override; zero means indefinite; positive values are days. */
        RecordingRetentionRequest: {
            retention_days: number;
        } & {
            [key: string]: unknown;
        };
        RecordingRetentionResult: {
            id: number;
            message: string;
            retention_days: number;
        } & {
            [key: string]: unknown;
        };
        RecordingsPage: {
            pagination: components["schemas"]["RecordingPagination"];
            recordings: components["schemas"]["RecordingListItem"][];
        } & {
            [key: string]: unknown;
        };
        RecordingTags: {
            tags: string[];
        } & {
            [key: string]: unknown;
        };
        RecordingTagsById: {
            id: number;
            tags: string[];
        } & {
            [key: string]: unknown;
        };
        /** @description Only nonempty string elements are used; other elements are ignored. The database trims tag text, truncates to its fixed UTF-8 byte buffer, and deduplicates. This is replacement, including clearing with an empty array. */
        RecordingTagsRequest: {
            tags: unknown[];
        } & {
            [key: string]: unknown;
        };
        /** @description One authorized camera and time window, capped at8640 segments. Optional exact external-motion intervals may be absent on allocation failure. */
        RecordingTimeline: {
            detection_interval_count?: number;
            detection_intervals?: ({
                end_timestamp: number;
                start_timestamp: number;
            } & {
                [key: string]: unknown;
            })[];
            end_time: string;
            segment_count: number;
            segments: components["schemas"]["RecordingTimelineSegment"][];
            start_time: string;
            stream: string;
        } & {
            [key: string]: unknown;
        };
        /** @description Requested IDs are processed in request order. Missing, deleted-camera and denied recordings are omitted. Counts and overall range reflect retained segments only; an empty result has empty start_time/end_time. Duplicate input IDs can produce duplicate segments. */
        RecordingTimelineByIds: {
            end_time: string;
            /** @constant */
            multi_stream: true;
            segment_count: number;
            segments: components["schemas"]["RecordingTimelineSegment"][];
            start_time: string;
            /** @constant */
            stream: "(selected)";
        } & {
            [key: string]: unknown;
        };
        /** @description Timeline timestamps are Unix seconds; display strings use server local YYYY-MM-DD HH:MM:SS. local_* fields currently equal the Unix timestamp fields. In-progress segments use an effective end for display. Timeline segments omit playback_issue and protected/storage metadata: use detail/playback before treating a finalized segment as playable. */
        RecordingTimelineSegment: {
            camera_uuid: string | null;
            capture_method: string;
            duration: number;
            end_time: string;
            end_timestamp: number;
            has_detection: boolean;
            id: number;
            in_progress: boolean;
            local_end_timestamp: number;
            local_start_timestamp: number;
            schedule_restricted: boolean | null;
            size: string;
            start_time: string;
            start_timestamp: number;
            stream: string;
        } & {
            [key: string]: unknown;
        };
        /** @description Default GET /api/streams response. An empty array is valid. Each row is filtered and redacted using runtime plus go2rtc health observations. */
        StreamCompatibilityList: (components["schemas"]["StreamRedacted"] | components["schemas"]["StreamPrivileged"])[];
        /** @description Partial ordinary update. All fields are optional and omission preserves existing state. Unknown fields (including detected width/height/fps) are currently ignored. This typed contract describes intended values; existing handlers can ignore wrong types instead of returning 400. It does not promise every malformed value is rejected by C. */
        StreamConfigurationUpdate: {
            /** @description Null or empty string clears this override; omission preserves the current value. */
            admin_url?: string | null;
            /** @description Omission or null preserves the current value. */
            audio_voice_enhancement?: boolean | null;
            /** @description Omission or null preserves the current value. */
            backchannel_enabled?: boolean | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. */
            codec?: string | null;
            /** @description Omission or null preserves the current value. */
            detection_based_recording?: boolean | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            detection_interval?: number | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. */
            detection_model?: string | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. */
            detection_object_filter?: string | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. */
            detection_object_filter_list?: string | null;
            /** @description Omission or null preserves the current value. */
            detection_record_on_schedule?: boolean | null;
            /** @description Omission/null preserves; use exactly 168 booleans. The current handler ignores wrong-length arrays and treats each non-true element as false, rather than rejecting all malformed schedule input. */
            detection_recording_schedule?: components["schemas"]["WeeklySchedule"] | null;
            /** @description Numeric input is truncated toward zero and must yield -1..365: -1 inherits global, 0 unlimited, positive days. Null/omission preserves. For predictable clients, send an integer. */
            detection_retention_days?: number | null;
            /** @description Percentage confidence, including zero; divided by 100 for storage. Omission/null preserves the current value. The handler does not clamp this numeric field. */
            detection_threshold?: number | null;
            /** @description Null or empty string clears this override; omission preserves the current value. Nonempty values must start with http://, https://, rtsp:// or rtsps:// (case-insensitive), at most 511 UTF-8 bytes. */
            detection_url?: string | null;
            /** @description Omission or null preserves the current value. */
            enable_disabled?: boolean | null;
            /** @description Omission or null preserves the current value. */
            enabled?: boolean | null;
            /** @description Empty string disables; otherwise at most 1023 UTF-8 bytes containing version=1, projection=equidistant, mount=ceiling and bounded centerX/centerY [0,1], radius [.05,1], fov [160,360], rotation/defaultYaw [-360,360], defaultTilt [-90,30], defaultViewFov [20,120]. Null is rejected. Unknown document fields are tolerated. */
            eptz_config?: string;
            /** @description Null or empty string clears this override; omission preserves the current value. Whitespace-only also clears. Maximum 8191 UTF-8 bytes. Nonblank multiline values are YAML-validated when YAML support is enabled; malformed YAML can return a structured error. Single-line values are not YAML-checked. */
            go2rtc_source_override?: string | null;
            /** @description Omission or null preserves the current value. */
            isOnvif?: boolean | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            max_storage_mb?: number | null;
            /** @description Null or empty string clears this override; omission preserves the current value. */
            motion_trigger_source?: string | null;
            /** @description Optional identifying metadata; ignored. This operation does not rename the stream. */
            name?: string;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. Processed only when the resulting isOnvif value is true; empty string clears this field. ONVIF probing may precede the 202 response. */
            onvif_password?: string | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            onvif_port?: number | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. Processed only when the resulting isOnvif value is true; empty string clears this field. ONVIF probing may precede the 202 response. */
            onvif_profile?: string | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. Processed only when the resulting isOnvif value is true; empty string clears this field. ONVIF probing may precede the 202 response. */
            onvif_username?: string | null;
            playback_transport?: components["schemas"]["PlaybackTransport"];
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            post_detection_buffer?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            pre_detection_buffer?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            priority?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            protocol?: number | null;
            /** @description Omission or null preserves the current value. */
            ptz_enabled?: boolean | null;
            /** @description Omission or null preserves the current value. */
            ptz_has_home?: boolean | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            ptz_max_x?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            ptz_max_y?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            ptz_max_z?: number | null;
            /** @description Null or empty string clears this override; omission preserves the current value. Nonempty values are a supported RTMP/RTMPS destination list, at most 511 UTF-8 bytes; each destination must pass the publish URL validator. */
            publish_url?: string | null;
            /** @description Omission or null preserves the current value. */
            record?: boolean | null;
            /** @description Omission or null preserves the current value. */
            record_audio?: boolean | null;
            /** @description Omission or null preserves the current value. */
            record_on_schedule?: boolean | null;
            /** @description Omission/null preserves; use exactly 168 booleans. The current handler ignores wrong-length arrays and treats each non-true element as false, rather than rejecting all malformed schedule input. */
            recording_schedule?: components["schemas"]["WeeklySchedule"] | null;
            /** @description Numeric input is truncated toward zero and must yield -1..365: -1 inherits global, 0 unlimited, positive days. Null/omission preserves. For predictable clients, send an integer. */
            retention_days?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            segment_duration?: number | null;
            /** @description A boolean selects the separate privacy-only form; null is ignored in the ordinary-update form. */
            set_privacy_mode?: null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            storage_priority?: number | null;
            /** @description Omission or null preserves the current value. */
            streaming_enabled?: boolean | null;
            /** @description Null or empty string clears this override; omission preserves the current value. */
            sub_stream_url?: string | null;
            /** @description Null or empty string clears this override; omission preserves the current value. */
            tags?: string | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            tier_critical_multiplier?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            tier_ephemeral_multiplier?: number | null;
            /** @description Omission or null preserves the current value. Integer-backed fields truncate numeric input toward zero. */
            tier_important_multiplier?: number | null;
            /** @description Omission preserves the current value. Null is ignored by this typed field handler. */
            url?: string | null;
        } & {
            [key: string]: unknown;
        };
        /** @description Fields shared by compatibility-list rows and full detail. Strings use empty values rather than null; additional fields are preserved for additive compatibility. */
        StreamDetailsBase: {
            audio_voice_enhancement: boolean;
            backchannel_enabled: boolean;
            /** @description Stable camera UUID. Older migrated rows may have an empty string; no instance identifiers are embedded in this document. */
            camera_uuid: string;
            codec: string;
            detection_based_recording: boolean;
            detection_interval: number;
            detection_model: string;
            detection_object_filter: string;
            detection_object_filter_list: string;
            detection_record_on_schedule: boolean;
            detection_recording_schedule: components["schemas"]["WeeklySchedule"];
            /** @description -1 inherits the global policy; 0 means unlimited; positive values are days. PUT validates the truncated integer to -1 through 365. */
            detection_retention_days: number;
            /** @description Integer percentage emitted from stored float confidence, including zero. The existing update handler does not clamp this to 0–100. */
            detection_threshold: number;
            enabled: boolean;
            /** @description Empty string disables browser dewarping; otherwise a serialized version-1 JSON document. It is a string on the wire, not an object. */
            eptz_config: string;
            /** @description Present only when an Error state has a recorded reason. */
            error_message?: string;
            /** @description Detected metadata; zero is valid when unavailable. */
            fps: number;
            go2rtc_hls_available: boolean;
            has_sub_stream: boolean;
            /** @description Detected metadata; zero is valid when unavailable. */
            height: number;
            isOnvif: boolean;
            /** @description Primary location UUID, or an empty string when unset. */
            location_uuid: string;
            /** @description Storage quota in MB; zero means unlimited. */
            max_storage_mb: number;
            motion_trigger_source: string;
            /** @description Current stream name; lookup is by name, not camera_uuid. */
            name: string;
            /** @description Zero requests automatic port detection. */
            onvif_port: number;
            onvif_profile: string;
            playback_transport: components["schemas"]["PlaybackTransport"];
            post_detection_buffer: number;
            pre_detection_buffer: number;
            priority: number;
            privacy_mode: boolean;
            /** @description Stored transport code: 0 TCP, 1 UDP. Existing configuration paths may retain other numeric values. */
            protocol: number;
            ptz_enabled: boolean;
            ptz_has_home: boolean;
            ptz_max_x: number;
            ptz_max_y: number;
            ptz_max_z: number;
            record: boolean;
            record_audio: boolean;
            record_on_schedule: boolean;
            recording_schedule: components["schemas"]["WeeklySchedule"];
            /** @description -1 inherits the global policy; 0 means unlimited; positive values are days. PUT validates the truncated integer to -1 through 365. */
            retention_days: number;
            segment_duration: number;
            status: components["schemas"]["StreamStatus"];
            storage_priority: number;
            streaming_enabled: boolean;
            tags: string;
            /** @description Stored tier multiplier. Zero is preserved; no client default is inserted. */
            tier_critical_multiplier: number;
            /** @description Stored tier multiplier. Zero is preserved; no client default is inserted. */
            tier_ephemeral_multiplier: number;
            /** @description Stored tier multiplier. Zero is preserved; no client default is inserted. */
            tier_important_multiplier: number;
            /** @description Configured input URL. For readers without camera.configure, URL userinfo is stripped; unsupported URL formats may become an empty string. For configurators the legacy/custom form may remain, while ONVIF/separate-credential configurations still strip URL userinfo. */
            url: string;
            /** @description Detected metadata; zero is valid when unavailable. */
            width: number;
        } & {
            [key: string]: unknown;
        };
        StreamFullPrivilegedResponse: {
            stream: components["schemas"]["StreamPrivileged"];
        } & {
            [key: string]: unknown;
        };
        StreamFullRedactedResponse: {
            stream: components["schemas"]["StreamRedacted"];
        } & {
            [key: string]: unknown;
        };
        /** @description Full details are wrapped in stream. Read authorization uses raw camera facts; privileged fields additionally use runtime-enriched facts on the same camera. */
        StreamFullResponse: components["schemas"]["StreamFullRedactedResponse"] | components["schemas"]["StreamFullPrivilegedResponse"];
        /** @description Select the response with the request query. Consumers validating an admin or live summary must use its specific page schema: the identity-only LPR variant is deliberately minimal. */
        StreamListResponse: components["schemas"]["StreamCompatibilityList"] | components["schemas"]["StreamSummaryPage"] | components["schemas"]["LiveStreamSummaryPage"] | components["schemas"]["LprStreamSummaryPage"];
        /** @description Synchronous lightweight privacy toggle. No other update fields may accompany it; unknown fields are rejected in this form. */
        StreamPrivacyUpdate: {
            /** @description Optional ignored identifying metadata. */
            name?: string;
            set_privacy_mode: boolean;
        };
        /** @description HTTP 200 privacy result. A disabled stream can succeed with stream_registered=false; this is not evidence of a live stream. */
        StreamPrivacyUpdated: {
            privacy_mode: boolean;
            stream_registered: boolean;
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        /** @description This camera is both readable and configurable for the current principal. Privileged values may still be empty; authority over another camera is insufficient. */
        StreamPrivileged: components["schemas"]["StreamDetailsBase"] & ({
            admin_url: string;
            /** @constant */
            can_control_privacy: true;
            detection_url: string;
            /** @description Single source string or multiline YAML source list; empty means generated configuration. */
            go2rtc_source_override: string;
            /** @description Configuration credential, conditionally visible for this resource. Empty is valid when unset. */
            onvif_password: string;
            onvif_username: string;
            publish_url: string;
            sub_stream_url: string;
        } & {
            [key: string]: unknown;
        });
        /** @description This camera is readable but not configurable. Sensitive fields remain present as empty strings; has_sub_stream can still be true. */
        StreamRedacted: components["schemas"]["StreamDetailsBase"] & ({
            /** @constant */
            admin_url: "";
            /** @constant */
            can_control_privacy: false;
            /** @constant */
            detection_url: "";
            /** @constant */
            go2rtc_source_override: "";
            /** @constant */
            onvif_password: "";
            /** @constant */
            onvif_username: "";
            /** @constant */
            publish_url: "";
            /** @constant */
            sub_stream_url: "";
        } & {
            [key: string]: unknown;
        });
        /** @enum {string} */
        StreamStatus: "Stopped" | "Starting" | "Running" | "Stopping" | "Error" | "Reconnecting" | "Degraded" | "Unknown";
        /** @description Administrative and live summary fields. Unlike detail, summary rows contain no input URL, credentials or error_message. */
        StreamSummaryBase: {
            /** @enum {string} */
            availability: "live" | "offline" | "never_connected" | "disabled";
            camera_uuid: string;
            codec: string;
            current_fps: number;
            detection_based_recording: boolean;
            enabled: boolean;
            /** @description Unix seconds; zero means not observed. */
            first_video_at: number;
            fps: number;
            height: number;
            isOnvif: boolean;
            /** @description Unix seconds; zero means not observed. */
            last_frame_ts: number;
            /** @description Unix seconds; zero means not observed. */
            last_recording_at: number;
            location_name: string;
            location_path: string;
            location_uuid: string;
            name: string;
            protocol: number;
            record: boolean;
            record_on_schedule: boolean;
            recording_active: boolean;
            status: components["schemas"]["StreamStatus"];
            tags: string;
            width: number;
        } & {
            [key: string]: unknown;
        };
        /** @description summary=true, surface=admin (default). Totals count only authorized, searched and availability-filtered rows before pagination. A page beyond the end has an empty streams array; total_pages is zero when total is zero. */
        StreamSummaryPage: {
            page: number;
            page_size: number;
            streams: (components["schemas"]["StreamSummaryRedacted"] | components["schemas"]["StreamSummaryPrivileged"])[];
            total: number;
            total_pages: number;
        } & {
            [key: string]: unknown;
        };
        StreamSummaryPrivileged: components["schemas"]["StreamSummaryBase"] & ({
            /** @description Only emitted for surface=admin with include_admin_url=true or 1. Empty when not configurable. */
            admin_url?: string;
            /** @constant */
            can_configure: true;
        } & {
            [key: string]: unknown;
        });
        StreamSummaryRedacted: components["schemas"]["StreamSummaryBase"] & ({
            /**
             * @description Only emitted for surface=admin with include_admin_url=true or 1. Empty when not configurable.
             * @constant
             */
            admin_url?: "";
            /** @constant */
            can_configure: false;
        } & {
            [key: string]: unknown;
        });
        /** @description HTTP 202 acknowledges a queued update, not successful durable application. ONVIF results are optional. Re-fetch detail to observe eventual state; a request requiring ONVIF work may perform that probe before acceptance. */
        StreamUpdateAccepted: {
            message: string;
            onvif_message?: string;
            /** @enum {string} */
            onvif_status?: "success" | "error";
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        StreamUpdateRequest: components["schemas"]["StreamConfigurationUpdate"] | components["schemas"]["StreamPrivacyUpdate"];
        SuccessResponse: {
            api_key?: never;
            password?: never;
            password_hash?: never;
            secret?: never;
            /** @constant */
            success: true;
            token_hash?: never;
            totp_secret?: never;
            totp_token?: never;
        } & {
            [key: string]: unknown;
        };
        /** @description Existing fleet selector grammar. Server additionally enforces depth≤8, total nodes≤64, UTF-8 byte limits and serialized selector<8192 bytes. Evaluation and delegation containment remain server decisions. */
        TokenSelector: {
            expression: components["schemas"]["TokenSelectorExpression"];
            /** @constant */
            version: 1;
        } & {
            [key: string]: unknown;
        };
        TokenSelectorExpression: ({
            /** @constant */
            op: "all";
        } & {
            [key: string]: unknown;
        }) | ({
            children: components["schemas"]["TokenSelectorExpression"][];
            /** @enum {unknown} */
            op: "and" | "or";
        } & {
            [key: string]: unknown;
        }) | ({
            child: components["schemas"]["TokenSelectorExpression"];
            /** @constant */
            op: "not";
        } & {
            [key: string]: unknown;
        }) | ({
            /** @constant */
            op: "location_subtree";
            uuid: string;
        } & {
            [key: string]: unknown;
        }) | ({
            /** @constant */
            op: "enabled";
            value: boolean;
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "camera_uuid";
            values: string[];
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "tag_any" | "tag_all" | "tag_none";
            uuids: string[];
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "recording_mode";
            values: ("off" | "continuous" | "detection")[];
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "vendor" | "model";
            values: string[];
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "capability_any" | "capability_all";
            values: ("onvif" | "ptz" | "backchannel")[];
        } & {
            [key: string]: unknown;
        }) | ({
            /** @enum {unknown} */
            op: "health";
            values: ("unknown" | "up" | "degraded" | "down" | "disabled")[];
        } & {
            [key: string]: unknown;
        });
        TrustedDevice: {
            created_at: number;
            current: boolean;
            expires_at: number;
            id: number;
            ip_address: string;
            last_used_at: number;
            user_agent: string;
        };
        TrustedDevicesResponse: {
            trusted_devices: components["schemas"]["TrustedDevice"][];
        };
        /** @description Seven days × 24 hours, indexed day*24+hour. true permits recording in that slot; interpreted using the configured recorder schedule timezone. */
        WeeklySchedule: boolean[];
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
    login: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["LoginRequest"];
                "application/x-www-form-urlencoded": {
                    password: string;
                    username: string;
                } & {
                    [key: string]: unknown;
                };
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    /** @description Session cookie on full login success; absent for MFA challenge. Additional trusted-device cookie may be set. */
                    "Set-Cookie"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["LoginResponse"];
                };
            };
            /** @description Browser/form redirect; Location identifies the next page and no JSON body is required. */
            302: {
                headers: {
                    /** @description Relative application page. */
                    Location?: string;
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Invalid request/path. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Login rate limited. */
            429: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal or policy evaluation failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getLoginConfig: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["LoginConfig"];
                };
            };
        };
    };
    logout: {
        parameters: {
            query?: never;
            header?: {
                /** @description Use application/json for the JSON success envelope. */
                Accept?: string;
                /** @description Presence also selects the JSON success envelope. */
                "X-Requested-With"?: string;
            };
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["LogoutResponse"];
                };
            };
            /** @description Browser/form redirect; Location identifies the next page and no JSON body is required. */
            302: {
                headers: {
                    /** @description Relative application page. */
                    Location?: string;
                    [name: string]: unknown;
                };
                content?: never;
            };
        };
    };
    listAuthSessions: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["AuthSessionsResponse"];
                };
            };
            /** @description No usable personal credential, including scoped-token-only, revoked/inactive/CIDR-restricted credentials or a pending password change for session/Basic authentication. Legacy keys retain their existing independent policy. */
            401: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Synthetic auth-disabled principal has no personal credential inventory. */
            403: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database or internal failure. */
            500: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    revokeAuthSession: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Canonical positive decimal safe integer; no leading zeros or encoded aliases. */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    /** @description Clears session or trusted_device only when the exact current credential was removed. */
                    "Set-Cookie"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CredentialRevoked"];
                };
            };
            /** @description Invalid credential ID. */
            400: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable personal credential, including scoped-token-only, revoked/inactive/CIDR-restricted credentials or a pending password change for session/Basic authentication. Legacy keys retain their existing independent policy. */
            401: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Synthetic auth-disabled principal has no personal credential inventory. */
            403: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No matching owned credential. */
            404: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database or internal failure. */
            500: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    listTrustedDevices: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TrustedDevicesResponse"];
                };
            };
            /** @description No usable personal credential, including scoped-token-only, revoked/inactive/CIDR-restricted credentials or a pending password change for session/Basic authentication. Legacy keys retain their existing independent policy. */
            401: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Synthetic auth-disabled principal has no personal credential inventory. */
            403: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database or internal failure. */
            500: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    revokeTrustedDevice: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Canonical positive decimal safe integer; no leading zeros or encoded aliases. */
                id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    /** @description Clears session or trusted_device only when the exact current credential was removed. */
                    "Set-Cookie"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CredentialRevoked"];
                };
            };
            /** @description Invalid credential ID. */
            400: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable personal credential, including scoped-token-only, revoked/inactive/CIDR-restricted credentials or a pending password change for session/Basic authentication. Legacy keys retain their existing independent policy. */
            401: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Synthetic auth-disabled principal has no personal credential inventory. */
            403: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No matching owned credential. */
            404: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database or internal failure. */
            500: {
                headers: {
                    /** @description Includes private and no-store. The common JSON response helper also appends no-cache, no-store, must-revalidate. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    verifyIdentity: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["AuthVerifyResponse"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    listUserTokens: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive user ID. Non-scoped caller may manage their own tokens; other owners require users.manage. Visibility never grants authority over arbitrary IDs. */
                user_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiTokenList"];
                };
            };
            /** @description Invalid request/path. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Operation or requested scope forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description User or token not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal or policy evaluation failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    createUserToken: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive user ID. Non-scoped caller may manage their own tokens; other owners require users.manage. Visibility never grants authority over arbitrary IDs. */
                user_id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["ApiTokenCreateRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiTokenCreated"];
                };
            };
            /** @description Invalid request/path. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Operation or requested scope forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description User or token not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Active token limit reached. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal or policy evaluation failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    revokeUserToken: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Token UUID belonging to this owner. */
                token_uuid: string;
                /** @description Positive user ID. Non-scoped caller may manage their own tokens; other owners require users.manage. Visibility never grants authority over arbitrary IDs. */
                user_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SuccessResponse"];
                };
            };
            /** @description Invalid request/path. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Operation or requested scope forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description User or token not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal or policy evaluation failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getClientConfig: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ClientConfig"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal or policy evaluation failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getOpenApi: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Permission-aware partial document. Never a fallback to the canonical unfiltered source. */
            200: {
                headers: {
                    /** @description Applied to success and error responses. No shared cache, ETag or persistent personalized-spec cache. */
                    "Cache-Control"?: "private, no-store";
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["OpenApiDocument"];
                };
            };
            /** @description No usable authenticated credential, including expired/revoked tokens, inactive users, CIDR or password-change restrictions. */
            401: {
                headers: {
                    /** @description Applied to success and error responses. No shared cache, ETag or persistent personalized-spec cache. */
                    "Cache-Control"?: "private, no-store";
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Synthetic auth_disabled or anonymous demo identity is excluded from personalized discovery. */
            403: {
                headers: {
                    /** @description Applied to success and error responses. No shared cache, ETag or persistent personalized-spec cache. */
                    "Cache-Control"?: "private, no-store";
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal, database, policy evaluation, serialization or update scheduling failure. */
            500: {
                headers: {
                    /** @description Applied to success and error responses. No shared cache, ETag or persistent personalized-spec cache. */
                    "Cache-Control"?: "private, no-store";
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    listRecordings: {
        parameters: {
            query?: {
                /** @description Comma-separated capture methods, including continuous,scheduled,detection,motion,manual. */
                capture_method?: string;
                /** @description Optional collection UUID, resolved on the server and intersected with replay scope. */
                collection_uuid?: string;
                /** @description Comma-separated labels; implies detection-only filter. */
                detection_label?: string;
                /** @description UTC ISO8601 end bound; invalid strings currently behave as no bound. */
                end?: string;
                /** @description 0 all,1 with detection,-1 without; numeric values clamp to this interval. */
                has_detection?: number;
                /** @description Default20, numeric maximum1000; nonpositive/invalid values normalize to20. Case-insensitive all returns all matching authorized rows, sets page1 and recounts. */
                limit?: number | "all";
                /** @description asc or desc; unsupported values fall back to desc. */
                order?: "asc" | "desc";
                /** @description Default1; nonpositive/invalid numeric values normalize to1. */
                page?: number;
                /** @description Leading0 means unprotected, leading1 protected; otherwise all. */
                protected?: string;
                /** @description Unsupported fields fall back to start_time. */
                sort?: "id" | "stream_name" | "start_time" | "end_time" | "size_bytes";
                /** @description UTC ISO8601 start bound; invalid strings currently behave as no bound. */
                start?: string;
                /** @description Comma-separated current stream names; every requested name must be replay authorized. */
                stream?: string;
                /** @description Comma-separated recording tags. */
                tag?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingsPage"];
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Shutdown, unavailable source or busy media preparation. */
            503: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getRecording: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDetail"];
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Shutdown, unavailable source or busy media preparation. */
            503: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    deleteRecording: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDeletionResult"];
                };
            };
            /** @description Deletion accepted and pending; status is deletion_pending. */
            202: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDeletionResult"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Protected/in-progress media, unavailable source, or job not ready. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    setRecordingProtection: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingProtectionRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingProtectionResult"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    setRecordingRetention: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingRetentionRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingRetentionResult"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getRecordingTags: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingTagsById"];
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    setRecordingTags: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingTagsRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingTagsById"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    deleteRecordings: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingDeletionRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            202: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDeletionJobAccepted"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getRecordingDeletionProgress: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Opaque deletion job identifier; possession alone grants no access. */
                job_id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDeletionProgress"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    createRecordingExport: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingExportRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            202: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingExportAccepted"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Protected/in-progress media, unavailable source, or job not ready. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Busy job slots, unavailable source, or shutdown. */
            503: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    downloadRecordingExport: {
        parameters: {
            query?: never;
            header?: {
                /** @description Single byte range using the common file-serving parser. */
                Range?: string;
            };
            path: {
                /** @description Opaque export job identifier, not a bearer credential. */
                export_token: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Complete ZIP archive. */
            200: {
                headers: {
                    "Accept-Ranges"?: "bytes";
                    "Content-Disposition"?: string;
                    "Content-Length"?: number;
                    [name: string]: unknown;
                };
                content: {
                    "application/zip": string;
                };
            };
            /** @description Single ZIP byte range. */
            206: {
                headers: {
                    "Accept-Ranges"?: "bytes";
                    "Content-Disposition"?: string;
                    "Content-Length"?: number;
                    "Content-Range"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/zip": string;
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Protected/in-progress media, unavailable source, or job not ready. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested byte range is not satisfiable. */
            416: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getRecordingExportStatus: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Opaque export job identifier, not a bearer credential. */
                export_token: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingExportStatus"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    protectRecordings: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingBatchProtectionRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingBatchProtectionResult"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    updateRecordingTagsBatch: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["RecordingBatchTagsRequest"];
            };
        };
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingBatchTagsResult"];
                };
            };
            /** @description Malformed path or request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Capture identity or batch member forbidden; authorization denial occurs before mutation or job creation. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection, or owned job not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, allocation, worker creation or file failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    listRecordingDetectionLabels: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingDetectionLabels"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    downloadRecording: {
        parameters: {
            query?: {
                /** @description UI hint for externally stored recordings. Actual source selection follows stored metadata; does not grant authority. */
                archive?: string;
                /** @description Exactly1 selects JSON readiness without media bytes. */
                prepare?: "0" | "1";
            };
            header?: {
                /** @description Single bytes=start-end, bytes=start-, or bytes=-suffix range. Actual permissive parser limitations are documented above. */
                Range?: string;
            };
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description prepare=1 JSON ready, or complete media body. */
            200: {
                headers: {
                    "Accept-Ranges"?: "bytes";
                    /** @description Attachment with sanitized recording filename; absent on JSON readiness. */
                    "Content-Disposition"?: string;
                    "Content-Length"?: number;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingPlaybackReady"];
                    "video/mp4": string;
                    "video/webm": string;
                    "video/x-matroska": string;
                    "video/x-msvideo": string;
                };
            };
            /** @description Requested media is preparing; retry after header interval. */
            202: {
                headers: {
                    "Retry-After"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingPlaybackPreparing"];
                };
            };
            /** @description A single satisfiable media byte range. Validate bytes/protocol separately from JSON. */
            206: {
                headers: {
                    "Accept-Ranges"?: "bytes";
                    /** @description Attachment with sanitized recording filename; absent on JSON readiness. */
                    "Content-Disposition"?: string;
                    "Content-Length"?: number;
                    /** @description For206, bytes start-end/total. Local416 currently omits it. */
                    "Content-Range"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "video/mp4": string;
                    "video/webm": string;
                    "video/x-matroska": string;
                    "video/x-msvideo": string;
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording still in progress, deletion pending or source conflict. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested byte range is not satisfiable. */
            416: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Shutdown, unavailable source or busy media preparation. */
            503: {
                headers: {
                    /** @description Present when preparation is pending, usually2seconds. */
                    "Retry-After"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"] | components["schemas"]["RecordingPlaybackPreparing"];
                };
            };
        };
    };
    playRecording: {
        parameters: {
            query?: {
                /** @description UI hint for externally stored recordings. Actual source selection follows stored metadata; does not grant authority. */
                archive?: string;
                /** @description Exactly1 selects JSON readiness without media bytes. */
                prepare?: "0" | "1";
                /** @description Exactly1 requests compatibility cache;0 requests original. Absent defaults to prepare=1 compatibility behavior. */
                transcode?: "0" | "1";
            };
            header?: {
                /** @description Single bytes=start-end, bytes=start-, or bytes=-suffix range. Actual permissive parser limitations are documented above. */
                Range?: string;
            };
            path: {
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description prepare=1 JSON ready, or complete media body. */
            200: {
                headers: {
                    /** @description The local playback handler and file server both emit this header; clients can observe combined bytes, bytes. Archive responses emit one bytes value. */
                    "Accept-Ranges"?: "bytes" | "bytes, bytes";
                    "Content-Length"?: number;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingPlaybackReady"];
                    "video/mp4": string;
                    "video/webm": string;
                    "video/x-matroska": string;
                    "video/x-msvideo": string;
                };
            };
            /** @description Requested media is preparing; retry after header interval. */
            202: {
                headers: {
                    "Retry-After"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingPlaybackPreparing"];
                };
            };
            /** @description A single satisfiable media byte range. Validate bytes/protocol separately from JSON. */
            206: {
                headers: {
                    /** @description The local playback handler and file server both emit this header; clients can observe combined bytes, bytes. Archive responses emit one bytes value. */
                    "Accept-Ranges"?: "bytes" | "bytes, bytes";
                    "Content-Length"?: number;
                    /** @description For206, bytes start-end/total. Local416 currently omits it. */
                    "Content-Range"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "video/mp4": string;
                    "video/webm": string;
                    "video/x-matroska": string;
                    "video/x-msvideo": string;
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording still in progress, deletion pending or source conflict. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested byte range is not satisfiable. */
            416: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Conclusive saved interrupted-media failure; playback is unavailable. Download recovery is a separately authorized export operation. */
            422: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Shutdown, unavailable source or busy media preparation. */
            503: {
                headers: {
                    /** @description Present when preparation is pending, usually2seconds. */
                    "Retry-After"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"] | components["schemas"]["RecordingPlaybackPreparing"];
                };
            };
        };
    };
    listRecordingTags: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingTags"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getRecordingThumbnail: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Thumbnail sample index. */
                index: number;
                /** @description Stable recording database ID. Most existing handlers use permissive strtoull parsing; zero/invalid IDs fail400, unknown IDs fail404 before authorization. Stored camera UUID is authoritative; a deleted capture camera yields404, without rebinding to a reused display name. Legacy rows with no UUID use the current stream name. */
                recording_id: number;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description JPEG thumbnail bytes. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "image/jpeg": string;
                };
            };
            /** @description Single cached/generated JPEG byte range. */
            206: {
                headers: {
                    /** @description The local playback handler and file server both emit this header; clients can observe combined bytes, bytes. Archive responses emit one bytes value. */
                    "Accept-Ranges"?: "bytes" | "bytes, bytes";
                    "Content-Length"?: number;
                    /** @description For206, bytes start-end/total. Local416 currently omits it. */
                    "Content-Range"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "image/jpeg": string;
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording still in progress, deletion pending or source conflict. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested byte range is not satisfiable. */
            416: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Shutdown, unavailable source or busy media preparation. */
            503: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"] | components["schemas"]["RecordingPlaybackPreparing"];
                };
            };
        };
    };
    listStreams: {
        parameters: {
            query?: {
                /** @description Summary availability filter, applied before total. */
                availability?: "all" | "live" | "offline" | "never_connected" | "disabled";
                /** @description Exactly true or 1 requests admin_url for the admin summary surface. Other values omit it; live/LPR surfaces never include it. Values are redacted unless that same camera is configurable. */
                include_admin_url?: string;
                /** @description Summary page number. */
                page?: number;
                /** @description Summary page size. */
                page_size?: number;
                /** @description Case-insensitive substring over authorized name, UUID, tags, location path and safe address. Handler buffer retains up to 255 bytes. */
                search?: string;
                /** @description Summary sort key; equal values break ties by name. */
                sort_by?: "name" | "status" | "resolution" | "fps" | "recording";
                /** @description Summary sort direction. */
                sort_order?: "asc" | "desc";
                /** @description Exactly true or 1 selects a paginated summary envelope. All other values, including absence, select the compatibility array. Other list query options only apply to summary mode. */
                summary?: string;
                /** @description Summary row shape: admin/default or live filters live.view; lpr filters the conjunction lpr.search and lpr.read and emits only identity. */
                surface?: "admin" | "live" | "lpr";
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Compatibility array or requested summary page; empty authorized results are valid. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["StreamListResponse"];
                };
            };
            /** @description Invalid path, query or typed update values. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated credential, including expired/revoked tokens, inactive users, CIDR or password-change restrictions. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal, database, policy evaluation, serialization or update scheduling failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    updateStream: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Current stream name, percent-encoded as one URL segment. The runtime stores up to 255 UTF-8 bytes. Embedded slash is unsupported by the full-read suffix parser; normal creation restricts names to letters, digits, dot, hyphen and underscore. Legacy configured names may require encoding. UUID is not accepted in place of the name. */
                name: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["StreamUpdateRequest"];
            };
        };
        responses: {
            /** @description Privacy-only update completed. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["StreamPrivacyUpdated"];
                };
            };
            /** @description Ordinary update accepted for asynchronous application. */
            202: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["StreamUpdateAccepted"];
                };
            };
            /** @description Invalid path, query or typed update values. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated credential, including expired/revoked tokens, inactive users, CIDR or password-change restrictions. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description The authenticated principal cannot perform the action on this camera. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No runtime stream with this name exists. Existing stream handlers may resolve this before checking credentials. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description An explicitly bounded override exceeds its supported byte size. */
            413: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal, database, policy evaluation, serialization or update scheduling failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Privacy flag was saved, but removing the stream from go2rtc failed. The camera may still be viewable; retry the privacy-only request. This error is not a rollback of the saved privacy flag. */
            502: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getStreamFull: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Current stream name, percent-encoded as one URL segment. The runtime stores up to 255 UTF-8 bytes. Embedded slash is unsupported by the full-read suffix parser; normal creation restricts names to letters, digits, dot, hyphen and underscore. Legacy configured names may require encoding. UUID is not accepted in place of the name. */
                name: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Wrapped details with per-camera conditional redaction. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["StreamFullResponse"];
                };
            };
            /** @description Invalid path, query or typed update values. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated credential, including expired/revoked tokens, inactive users, CIDR or password-change restrictions. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description The authenticated principal cannot perform the action on this camera. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No runtime stream with this name exists. Existing stream handlers may resolve this before checking credentials. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal, database, policy evaluation, serialization or update scheduling failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getTimelineSegments: {
        parameters: {
            query?: {
                /** @description Capture camera UUID; takes precedence over stream, invalid UUID400. */
                camera_uuid?: string;
                /** @description ISO8601/date bound; defaultsnow. Date-only end expands to end of day. */
                end?: string;
                /** @description ISO8601/date bound; defaults24hours ago. Invalid nonempty time parses as epoch rather than400. */
                start?: string;
                /** @description Current stream name, required unless camera_uuid is present. */
                stream?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingTimeline"];
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Requested camera or media option forbidden. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Recording, current capture camera, collection or source file not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
    getTimelineSegmentsByIds: {
        parameters: {
            query: {
                /** @description Required comma-separated positive recording IDs; parses up to8640 IDs from an8192-byte buffer. Invalid/zero tokens are ignored; no valid IDs gives400. Numeric-prefix parsing remains permissive. */
                ids: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["RecordingTimelineByIds"];
                };
            };
            /** @description Invalid path or query. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable credential. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, policy, media preparation or serialization failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
        };
    };
}
