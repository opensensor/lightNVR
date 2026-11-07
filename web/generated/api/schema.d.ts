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
         * @description Public credential submission. Successful JSON login sets the HttpOnly session cookie; MFA challenge returns totp_required/totp_token without a cookie. Optional inline TOTP is described; MFA completion and enrollment have separate identity contracts. Form submissions redirect for success, credential rejection and rate limit. Form parsing uses username/password only; JSON optional fields are not form inputs.
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
    "/api/auth/login/totp": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Complete an MFA login challenge
         * @description Consumes an unexpired password-verified challenge bound to the issuing IP/user agent. Six-digit codes are single-use per account time step; account throttling applies. Success sets an HttpOnly session cookie, and optionally a trusted-device cookie. Never automatically retry a completion after an ambiguous response.
         */
        post: operations["completeTotpLogin"];
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
    "/api/auth/users/{id}/password": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        /**
         * Change or reset a password
         * @description A session, Basic credential or legacy key may manage its own account. Other accounts, and all scoped-token requests, require effective users.manage (owner permission intersected with token action/scope). Discovery describes the operation; it does not grant access to an arbitrary target ID. Requires old_password for self-service without users.manage and for a forced own password change. Success atomically changes the password and revokes sessions, pending challenges and trusted devices. Changing your own password clears browser cookies. Basic/key compatibility remains supported.
         */
        put: operations["changePassword"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/users/{id}/totp/disable": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Disable MFA
         * @description A session, Basic credential or legacy key may manage its own account. Other accounts, and all scoped-token requests, require effective users.manage (owner permission intersected with token action/scope). Discovery describes the operation; it does not grant access to an arbitrary target ID. Disables MFA, erases the secret and revokes other sessions/challenges/trusted devices. The initiating authenticated session is retained.
         */
        post: operations["disableTotp"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/users/{id}/totp/setup": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Start MFA enrollment
         * @description A session, Basic credential or legacy key may manage its own account. Other accounts, and all scoped-token requests, require effective users.manage (owner permission intersected with token action/scope). Discovery describes the operation; it does not grant access to an arbitrary target ID. Returns enrollment secret only here; setup expires after ten minutes. Replacing pending enrollment invalidates the previous secret. An enabled account returns 409.
         */
        post: operations["setupTotp"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/users/{id}/totp/status": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read MFA enrollment status
         * @description A session, Basic credential or legacy key may manage its own account. Other accounts, and all scoped-token requests, require effective users.manage (owner permission intersected with token action/scope). Discovery describes the operation; it does not grant access to an arbitrary target ID. Returns enabled/configured booleans without secrets. Unavailability is not evidence that MFA is disabled.
         */
        get: operations["getTotpStatus"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/auth/users/{id}/totp/verify": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Verify and enable MFA
         * @description A session, Basic credential or legacy key may manage its own account. Other accounts, and all scoped-token requests, require effective users.manage (owner permission intersected with token action/scope). Discovery describes the operation; it does not grant access to an arbitrary target ID. Consumes a fresh enrollment code, enables MFA and revokes other sessions/challenges/trusted devices. The initiating authenticated session is retained.
         */
        post: operations["verifyTotp"];
        delete?: never;
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
    "/api/camera-collections": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List visible camera collections
         * @description Collection visibility requires global camera.configure, ownership or sharing; a shared collection with zero visible live.view matches is hidden from a non-owner/non-manager. Member/effective counts and samples are scoped to authorized cameras. Smart selectors are disclosed only to the owner or manager; other readers receive null plus selector_redacted. Read-like preview performs no mutation.
         */
        get: operations["listCameraCollections"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/camera-collections/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read a visible camera collection
         * @description Collection visibility requires global camera.configure, ownership or sharing; a shared collection with zero visible live.view matches is hidden from a non-owner/non-manager. Member/effective counts and samples are scoped to authorized cameras. Smart selectors are disclosed only to the owner or manager; other readers receive null plus selector_redacted. Read-like preview performs no mutation.
         */
        get: operations["getCameraCollection"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/camera-collections/{uuid}/members": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read authorized static collection members
         * @description Collection visibility requires global camera.configure, ownership or sharing; a shared collection with zero visible live.view matches is hidden from a non-owner/non-manager. Member/effective counts and samples are scoped to authorized cameras. Smart selectors are disclosed only to the owner or manager; other readers receive null plus selector_redacted. Read-like preview performs no mutation.
         */
        get: operations["getCameraCollectionMembers"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/camera-collections/{uuid}/preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Preview authorized collection matches
         * @description Collection visibility requires global camera.configure, ownership or sharing; a shared collection with zero visible live.view matches is hidden from a non-owner/non-manager. Member/effective counts and samples are scoped to authorized cameras. Smart selectors are disclosed only to the owner or manager; other readers receive null plus selector_redacted. Read-like preview performs no mutation.
         */
        post: operations["previewCameraCollection"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/camera-tags": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List visible camera tags
         * @description Visibility and counts follow existing live.view scope; location ancestors may be included. Global live.view or camera.configure can expose the full organization catalog. Camera-specific tag access requires live.view on that raw camera identity; missing objects may return 404 before authentication.
         */
        get: operations["listFleetTags"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/camera-tags/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read a visible camera tag
         * @description Visibility and counts follow existing live.view scope; location ancestors may be included. Global live.view or camera.configure can expose the full organization catalog. Camera-specific tag access requires live.view on that raw camera identity; missing objects may return 404 before authentication.
         */
        get: operations["getFleetTag"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/cameras/{uuid}/tags": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read tags for an authorized camera
         * @description Visibility and counts follow existing live.view scope; location ancestors may be included. Global live.view or camera.configure can expose the full organization catalog. Camera-specific tag access requires live.view on that raw camera identity; missing objects may return 404 before authentication.
         */
        get: operations["getCameraTagAssignments"];
        put?: never;
        post?: never;
        delete?: never;
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
    "/api/event-destinations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List event destinations
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        get: operations["listEventDestinations"];
        put?: never;
        /**
         * Create event destination
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        post: operations["createEventDestination"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/event-destinations/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Get event destination
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        get: operations["getEventDestination"];
        /**
         * Update event destination
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        put: operations["updateEventDestination"];
        post?: never;
        /**
         * Delete event destination
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        delete: operations["deleteEventDestination"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/event-routes": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List event routes
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        get: operations["listEventRoutes"];
        put?: never;
        /**
         * Create event route
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        post: operations["createEventRoute"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/event-routes/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Get event route
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        get: operations["getEventRoute"];
        /**
         * Update event route
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        put: operations["updateEventRoute"];
        post?: never;
        /**
         * Delete event route
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        delete: operations["deleteEventRoute"];
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/event-routes/preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Preview event route
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        post: operations["previewEventRoute"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/events/catalog": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Get event catalog
         * @description Global event administration. Native validation additionally enforces referenced destinations, registered event/health types, compatible predicates, available timezones and topic/TLS constraints. Preview never publishes; mutation acknowledgments do not imply delivery.
         */
        get: operations["getEventCatalog"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/fleet/cameras/query": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Query authorized fleet cameras
         * @description Read-only query over live.view-filtered, runtime-enriched inventory. Rows, totals and facets use matching authorized cameras; operational queues use all authorized cameras before query filters. can_configure is evaluated separately for each returned camera. Unknown collection is 404; out-of-range pages may be empty with nonzero totals. No stream credentials are returned.
         */
        post: operations["queryFleetCameras"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/fleet/selectors/preview": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Preview a selector over authorized fleet cameras
         * @description Read-only query over live.view-filtered, runtime-enriched inventory. Rows, totals and facets use matching authorized cameras; operational queues use all authorized cameras before query filters. can_configure is evaluated separately for each returned camera. Unknown collection is 404; out-of-range pages may be empty with nonzero totals. No stream credentials are returned.
         */
        post: operations["previewFleetSelector"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/fleet/views": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List owned and shared fleet views
         * @description Owned and shared saved filter configurations; up to 256 list entries. Stored selector references do not grant camera access and must be reauthorized on query. Existing can_share reflects the legacy admin role, not arbitrary action grants. Mutation contracts remain outside this read slice.
         */
        get: operations["listFleetViews"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/fleet/views/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read an owned or shared fleet view
         * @description Owned and shared saved filter configurations; up to 256 list entries. Stored selector references do not grant camera access and must be reauthorized on query. Existing can_share reflects the legacy admin role, not arbitrary action grants. Mutation contracts remain outside this read slice.
         */
        get: operations["getFleetView"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/health": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get stream health */
        get: operations["getStreamHealth"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/health/hls": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get hls health */
        get: operations["getHlsHealth"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/locations": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * List visible camera locations
         * @description Visibility and counts follow existing live.view scope; location ancestors may be included. Global live.view or camera.configure can expose the full organization catalog. Camera-specific tag access requires live.view on that raw camera identity; missing objects may return 404 before authentication.
         */
        get: operations["listFleetLocations"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/locations/{uuid}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * Read a visible camera location
         * @description Visibility and counts follow existing live.view scope; location ancestors may be included. Global live.view or camera.configure can expose the full organization catalog. Camera-specific tag access requires live.view on that raw camera identity; missing objects may return 404 before authentication.
         */
        get: operations["getFleetLocation"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/metrics": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get metrics */
        get: operations["getMetrics"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/motion/trigger": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Trigger motion
         * @description Requires camera.configure on the named stream. Start/stop/pulse report annotation, tag and recording effects independently; tags require an active recording and detection-recording schedules may suppress recording. Pulse is bounded to 600000 ms. Invalid requests may return 400 before principal evaluation.
         */
        post: operations["triggerMotion"];
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
    "/api/settings": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get system settings */
        get: operations["getSystemSettings"];
        put?: never;
        /** Update system settings */
        post: operations["updateSystemSettings"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/settings/go2rtc/validate": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Validate go2rtc configuration */
        post: operations["validateGo2rtcConfiguration"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/setup/status": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get setup status */
        get: operations["getSetupStatus"];
        put?: never;
        /**
         * Update setup status
         * @description Before setup completion this bootstrap operation is public. Once complete it requires system.admin; resetting requires that authority. Malformed/absent optional fields retain the native default of completing setup.
         */
        post: operations["updateSetupStatus"];
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
    "/api/system": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get system information legacy */
        get: operations["getSystemInformationLegacy"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/backup": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Create system backup */
        post: operations["createSystemBackup"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/go2rtc/effective-config": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get go2rtc effective configuration */
        get: operations["getGo2rtcEffectiveConfiguration"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/go2rtc/override-status": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get go2rtc override diagnostics */
        get: operations["getGo2rtcOverrideDiagnostics"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/health": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get host health */
        get: operations["getHostHealth"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/health/incidents": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get host health incidents */
        get: operations["getHostHealthIncidents"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/info": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get system information */
        get: operations["getSystemInformation"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/logs": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get system logs */
        get: operations["getSystemLogs"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/logs/clear": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Clear system logs */
        post: operations["clearSystemLogs"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/restart": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Restart system */
        post: operations["restartSystem"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/shutdown": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /** Shutdown system */
        post: operations["shutdownSystem"];
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/system/status": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get system status */
        get: operations["getSystemStatus"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/api/telemetry/player": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get?: never;
        put?: never;
        /**
         * Record player telemetry
         * @description A named stream requires live.view on that stream. Empty/malformed unnamed telemetry is ignored with 204; an acknowledgment does not imply storage of an event.
         */
        post: operations["recordPlayerTelemetry"];
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
    "/api/ui/workspaces": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /** Get workspaces */
        get: operations["getWorkspaces"];
        /** Update workspaces */
        put: operations["updateWorkspaces"];
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/logout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        /**
         * End a browser session using the legacy route
         * @description Public and idempotent. Deletes the session from the supplied cookie and clears session/trusted-device cookies. Does not revoke Basic credentials, legacy API keys or scoped API tokens. JSON200 requires Accept containing application/json or X-Requested-With; otherwise302 redirects to login.
         */
        get: operations["legacyLogout"];
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
        CameraCollection: {
            created_at: number;
            description: string;
            effective_count: number;
            member_count: number;
            name: string;
            owner_user_id: number | null;
            selector: components["schemas"]["FleetSelector"] | null;
            selector_redacted: boolean;
            shared: boolean;
            /** @enum {unknown} */
            type: "static" | "smart";
            updated_at: number;
            uuid: string;
        };
        CameraCollectionMembersResponse: {
            camera_uuids: string[];
            collection_uuid: string;
            count: number;
        };
        CameraCollectionPreview: {
            collection_uuid: string;
            matched_count: number;
            sample: {
                camera_uuid: string;
                location_path: string;
                name: string;
            }[];
        };
        CameraCollectionsResponse: {
            collections: components["schemas"]["CameraCollection"][];
            count: number;
        };
        CameraTagAssignmentsResponse: {
            camera_uuid: string;
            count: number;
            tags: components["schemas"]["FleetTag"][];
        };
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
        EventCameraScope: {
            /** @constant */
            type: "all";
        } | {
            selector: components["schemas"]["FleetSelector"];
            /** @constant */
            type: "selector";
        };
        EventCatalog: {
            count: number;
            event_types: {
                default_expiry_seconds: number;
                description: string;
                dynamic_severity: boolean;
                /** @enum {string} */
                expected_rate: "low" | "medium" | "high";
                family: string;
                /** @enum {string} */
                maximum_severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                media_policy: string;
                /** @enum {string} */
                minimum_severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                sensitivity: string;
                /** @enum {string} */
                severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                /** @enum {string} */
                subject_kind: "camera" | "storage" | "system";
                type: string;
            }[];
            /** @constant */
            schema_version: 1;
        };
        EventDestination: {
            authentication: {
                password_configured: boolean;
                username: string;
            };
            broker: {
                client_id: string;
                host: string;
                keepalive_seconds: number;
                port: number;
                qos: number;
                status_topic_template: string;
                topic_template: string;
            };
            created_at: number;
            description: string;
            enabled: boolean;
            key: string;
            /** @constant */
            managed: true;
            name: string;
            revision: number;
            tls: {
                ca_file: string;
                cert_file: string;
                key_file: string;
                /** @enum {string} */
                mode: "disabled" | "system" | "custom_ca" | "mutual";
            };
            /** @constant */
            type: "mqtt";
            updated_at: number;
            /** Format: uuid */
            uuid: string;
        };
        EventDestinationCreate: {
            authentication?: {
                password?: string | null;
                username?: string;
            };
            broker: {
                client_id?: string;
                host: string;
                keepalive_seconds?: number;
                port?: number;
                qos?: number;
                status_topic_template?: string;
                topic_template?: string;
            };
            description?: string;
            enabled?: boolean;
            name: string;
            tls?: {
                ca_file?: string;
                cert_file?: string;
                key_file?: string;
                /** @enum {string} */
                mode?: "disabled" | "system" | "custom_ca" | "mutual";
            };
            /** @constant */
            type?: "mqtt";
        };
        EventDestinations: {
            count: number;
            default_destination: {
                /** @constant */
                configuration_source: "settings";
                /** @constant */
                key: "mqtt:default";
                /** @constant */
                managed: false;
                name: string;
            };
            destinations: components["schemas"]["EventDestination"][];
        };
        EventDestinationUpdate: {
            authentication?: {
                password?: string | null;
                username?: string;
            };
            broker?: {
                client_id?: string;
                host?: string;
                keepalive_seconds?: number;
                port?: number;
                qos?: number;
                status_topic_template?: string;
                topic_template?: string;
            };
            description?: string;
            enabled?: boolean;
            name?: string;
            revision: number;
            tls?: {
                ca_file?: string;
                cert_file?: string;
                key_file?: string;
                /** @enum {string} */
                mode?: "disabled" | "system" | "custom_ca" | "mutual";
            };
            /** @constant */
            type?: "mqtt";
        };
        EventMutationAck: {
            /** @constant */
            success: true;
        };
        EventPredicate: {
            detection?: {
                labels_any?: string[];
                min_confidence?: number;
                zone_ids_any?: string[];
            };
            health?: {
                condition_codes_any?: string[];
                severities_any?: ("warning" | "error" | "critical")[];
            };
            /** @constant */
            version: 1;
        };
        EventRoute: {
            camera_scope: components["schemas"]["EventCameraScope"];
            created_at: number;
            description: string;
            destination: string;
            enabled: boolean;
            event_types: string[];
            name: string;
            predicate: components["schemas"]["EventPredicate"];
            revision: number;
            schedule: components["schemas"]["EventSchedule"];
            suppression: {
                cooldown_seconds: number;
                debounce_seconds: number;
                grouping_window_seconds: number;
                max_events_per_minute: number;
            };
            updated_at: number;
            /** Format: uuid */
            uuid: string;
        };
        EventRouteCreate: {
            camera_scope?: components["schemas"]["EventCameraScope"];
            description?: string;
            destination?: string;
            enabled?: boolean;
            event_types: string[];
            name: string;
            predicate?: components["schemas"]["EventPredicate"];
            schedule?: components["schemas"]["EventSchedule"];
            suppression?: {
                cooldown_seconds?: number;
                debounce_seconds?: number;
                grouping_window_seconds?: number;
                max_events_per_minute?: number;
            };
        };
        EventRoutePreview: {
            camera_sample: {
                /** Format: uuid */
                camera_uuid: string;
                location_path: string;
                name: string;
            }[];
            event_types: {
                dynamic_severity: boolean;
                /** @enum {string} */
                expected_rate: "low" | "medium" | "high";
                /** @enum {string} */
                maximum_severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                /** @enum {string} */
                minimum_severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                /** @enum {string} */
                severity: "debug" | "info" | "notice" | "warning" | "error" | "critical";
                /** @enum {string} */
                subject_kind: "camera" | "storage" | "system";
                type: string;
            }[];
            matched_camera_count: number;
            /** @constant */
            sample_limit: 20;
            /** @constant */
            would_publish: false;
        };
        EventRoutePreviewRequest: {
            camera_scope?: components["schemas"]["EventCameraScope"];
            description?: string;
            destination?: string;
            enabled?: boolean;
            event_types: string[];
            name: string;
            predicate?: components["schemas"]["EventPredicate"];
            schedule?: components["schemas"]["EventSchedule"];
            suppression?: {
                cooldown_seconds?: number;
                debounce_seconds?: number;
                grouping_window_seconds?: number;
                max_events_per_minute?: number;
            };
        };
        EventRoutes: {
            count: number;
            routes: components["schemas"]["EventRoute"][];
        };
        EventRouteUpdate: {
            camera_scope?: components["schemas"]["EventCameraScope"];
            description?: string;
            destination?: string;
            enabled?: boolean;
            event_types?: string[];
            name?: string;
            predicate?: components["schemas"]["EventPredicate"];
            revision: number;
            schedule?: components["schemas"]["EventSchedule"];
            suppression?: {
                cooldown_seconds?: number;
                debounce_seconds?: number;
                grouping_window_seconds?: number;
                max_events_per_minute?: number;
            };
        };
        EventSchedule: {
            timezone: string;
            /** @constant */
            version: 1;
            windows: {
                days: number[];
                end: string;
                start: string;
            }[];
        };
        FleetCamera: {
            address: string;
            /** @enum {unknown} */
            availability: "live" | "offline" | "never_connected" | "disabled";
            camera_uuid: string;
            can_configure: boolean;
            capabilities: ("onvif" | "ptz" | "backchannel")[];
            current_fps: number;
            enabled: boolean;
            first_video_at: number;
            /** @enum {unknown} */
            health: "unknown" | "up" | "degraded" | "down" | "disabled";
            health_changed_at: number;
            last_frame_ts: number;
            last_recording_at: number;
            location: {
                name: string;
                path: string;
                uuid: string;
            };
            manufacturer: string;
            matched_clauses?: string[];
            model: string;
            name: string;
            recording_active: boolean;
            /** @enum {unknown} */
            recording_mode: "off" | "continuous" | "detection";
            tags: {
                label: string;
                uuid: string;
            }[];
        };
        FleetFacetEntity: {
            count: number;
            label?: string;
            uuid: string;
        };
        FleetFacets: {
            availability: {
                count: number;
                /** @enum {unknown} */
                value: "live" | "offline" | "never_connected" | "disabled";
            }[];
            enabled: {
                count: number;
                value: boolean;
            }[];
            health: {
                count: number;
                /** @enum {unknown} */
                value: "unknown" | "up" | "degraded" | "down" | "disabled";
            }[];
            locations: components["schemas"]["FleetFacetEntity"][];
            recording_mode: {
                count: number;
                /** @enum {unknown} */
                value: "off" | "continuous" | "detection";
            }[];
            tags: components["schemas"]["FleetFacetEntity"][];
        };
        FleetLocation: {
            camera_count: number;
            child_count: number;
            created_at: number;
            is_system: boolean;
            metadata: {
                [key: string]: unknown;
            };
            name: string;
            parent_uuid: string | null;
            sort_order: number;
            type: string;
            updated_at: number;
            uuid: string;
        };
        FleetLocationsResponse: {
            count: number;
            locations: components["schemas"]["FleetLocation"][];
            unassigned_uuid?: string;
        };
        FleetOperationalQueue: {
            count: number;
            key: string;
            oldest_unresolved_at: number | null;
            remediation: string;
            selector: components["schemas"]["FleetSelector"];
            severity: string;
        };
        /** @description Read-only POST. Defaults: all selector, page 1, page_size 50, name/asc, facets true, explain false, availability all. Server enforces UTF-8 byte length for search and bounded selector parsing; an invalid or stale selector never grants visibility. */
        FleetPreviewRequest: {
            /** @enum {unknown} */
            availability?: "all" | "live" | "offline" | "never_connected" | "disabled";
            camera_uuid?: string;
            collection_uuid?: string;
            explain?: boolean;
            facets?: boolean;
            page?: number;
            page_size?: number;
            search?: string;
            selector?: components["schemas"]["FleetSelector"];
            /** @enum {unknown} */
            sort_by?: "name" | "camera_uuid" | "location" | "health" | "enabled" | "recording_mode" | "address";
            /** @enum {unknown} */
            sort_order?: "asc" | "desc";
        };
        FleetPreviewResponse: {
            cameras: components["schemas"]["FleetCamera"][];
            collection_uuid?: string;
            facets?: components["schemas"]["FleetFacets"];
            page: number;
            page_size: number;
            /** @constant */
            preview: true;
            /** @constant */
            queue_version: 1;
            queues: components["schemas"]["FleetOperationalQueue"][];
            /** @constant */
            selector_version: 1;
            /** @enum {unknown} */
            sort_by: "name" | "camera_uuid" | "location" | "health" | "enabled" | "recording_mode" | "address";
            /** @enum {unknown} */
            sort_order: "asc" | "desc";
            total: number;
            total_pages: number;
        };
        /** @description Read-only POST. Defaults: all selector, page 1, page_size 50, name/asc, facets true, explain false, availability all. Server enforces UTF-8 byte length for search and bounded selector parsing; an invalid or stale selector never grants visibility. */
        FleetQueryRequest: {
            /** @enum {unknown} */
            availability?: "all" | "live" | "offline" | "never_connected" | "disabled";
            camera_uuid?: string;
            collection_uuid?: string;
            explain?: boolean;
            facets?: boolean;
            page?: number;
            page_size?: number;
            search?: string;
            selector?: components["schemas"]["FleetSelector"];
            /** @enum {unknown} */
            sort_by?: "name" | "camera_uuid" | "location" | "health" | "enabled" | "recording_mode" | "address";
            /** @enum {unknown} */
            sort_order?: "asc" | "desc";
        };
        FleetQueryResponse: {
            cameras: components["schemas"]["FleetCamera"][];
            collection_uuid?: string;
            facets?: components["schemas"]["FleetFacets"];
            page: number;
            page_size: number;
            preview: boolean;
            /** @constant */
            queue_version: 1;
            queues: components["schemas"]["FleetOperationalQueue"][];
            /** @constant */
            selector_version: 1;
            /** @enum {unknown} */
            sort_by: "name" | "camera_uuid" | "location" | "health" | "enabled" | "recording_mode" | "address";
            /** @enum {unknown} */
            sort_order: "asc" | "desc";
            total: number;
            total_pages: number;
        };
        FleetSavedView: {
            collection_uuid: string | null;
            columns: string[];
            created_at: number;
            is_shared: boolean;
            name: string;
            owned: boolean;
            revision: number;
            search: string;
            selector: components["schemas"]["FleetSelector"];
            /** @enum {unknown} */
            sort_by: "name" | "camera_uuid" | "location" | "health" | "enabled" | "recording_mode" | "address";
            /** @enum {unknown} */
            sort_order: "asc" | "desc";
            updated_at: number;
            uuid: string;
        };
        FleetSavedViewsResponse: {
            can_share: boolean;
            count: number;
            views: components["schemas"]["FleetSavedView"][];
        };
        FleetSelector: components["schemas"]["TokenSelector"];
        FleetTag: {
            camera_count: number;
            color: string;
            created_at: number;
            description: string;
            label: string;
            updated_at: number;
            uuid: string;
        };
        FleetTagsResponse: {
            count: number;
            tags: components["schemas"]["FleetTag"][];
        };
        Go2rtcConfigurationInput: {
            override?: string;
        } & {
            [key: string]: unknown;
        };
        Go2rtcConfigurationValidation: {
            error?: null | ({
                column?: number;
                line?: number;
                message?: string;
            } & {
                [key: string]: unknown;
            });
            libyaml_available: boolean;
            skip_reason?: {
                message: string;
            } & {
                [key: string]: unknown;
            };
            skipped: boolean;
            valid: boolean;
            warnings: string[];
        } & {
            [key: string]: unknown;
        };
        Go2rtcEffectiveConfiguration: {
            base: string;
            go2rtc_initialized: boolean;
            managed_listener_policy: string;
            merged_source_order: string[];
            override: string;
            override_in_use: boolean;
            redaction_available: boolean;
            warnings: string[];
        } & {
            [key: string]: unknown;
        };
        Go2rtcOverrideDiagnostics: {
            content_matches: boolean;
            db_bytes: number;
            expected_file_path: string;
            file_bytes: number;
            file_present: boolean;
            file_read_error?: string;
            process_pid: number;
            process_running: boolean;
            process_state_known: boolean;
            quarantine_active: boolean;
            quarantine_reason?: string;
            quarantined_copy_bytes: number;
            validated_version: string;
        } & {
            [key: string]: unknown;
        };
        HealthResponse: components["schemas"]["StreamHealth"] | components["schemas"]["ProcessLiveness"] | components["schemas"]["StreamHealthHistory"];
        HlsHealth: {
            healthy: boolean;
            status: string;
            timestamp: string;
            watchdogRestartCount: number;
        } & {
            [key: string]: unknown;
        };
        HostHealth: {
            active_incidents: ({
                condition: string;
                first_observed_at_ms: number;
                incident_id: string;
                last_observed_at_ms: number;
                observation: {
                    capability: string;
                    freshness: string;
                    metric: string;
                    observed_at_ms?: number;
                    resource: string;
                    sampled_monotonic_ms?: number;
                    scope: string;
                    unit: string;
                    value: null | number;
                } & {
                    [key: string]: unknown;
                };
                persistence_pending: boolean;
                scope: string;
                severity: string;
                state: string;
                subject: string;
                thresholds?: {
                    condition: string;
                    critical_for_seconds: number;
                    critical_threshold: number;
                    direction: string;
                    enabled: boolean;
                    overridden: boolean;
                    recovery_for_seconds: number;
                    recovery_threshold: number;
                    unit: string;
                    warning_for_seconds: number;
                    warning_threshold: number;
                } & {
                    [key: string]: unknown;
                };
            } & {
                [key: string]: unknown;
            })[];
            coverage: {
                abandoned_helpers: number;
                collection_errors: number;
                collection_timeouts: number;
                complete: boolean;
                coverage_overflows: number;
                incomplete_count: number;
                observations_dropped: number;
                unavailable_observations: number;
            } & {
                [key: string]: unknown;
            };
            evaluator?: {
                active_incidents: number;
                pending_persistence: number;
                tracked_conditions: number;
            } & {
                [key: string]: unknown;
            };
            monitoring: {
                configured_enabled: boolean;
                enabled: boolean;
                initialized: boolean;
                restart_required: boolean;
                worker_threads: number;
            } & {
                [key: string]: unknown;
            };
            observations: ({
                capability: string;
                freshness: string;
                metric: string;
                observed_at_ms?: number;
                resource: string;
                sampled_monotonic_ms?: number;
                scope: string;
                unit: string;
                value: null | number;
            } & {
                [key: string]: unknown;
            })[];
            overall_state: string;
            recent_samples: ({
                completed_at_ms: number;
                observation_count: number;
                observations_dropped: number;
                sequence: number;
            } & {
                [key: string]: unknown;
            })[];
            schema_version: number;
            self_observability: {
                collectors: ({
                    attempts: number;
                    busy: boolean;
                    collector: string;
                    completions: number;
                    failures: number;
                    last_duration_ms: number;
                    maximum_duration_ms: number;
                    overlap_skips: number;
                    scope: string;
                    stale: boolean;
                    tier: number;
                    timeouts: number;
                } & {
                    [key: string]: unknown;
                })[];
                coverage_overflows: number;
                event_delivery: {
                    all_destinations_unavailable?: boolean;
                    circular_report_path?: boolean;
                    degraded?: boolean;
                    outbox?: {
                        available?: boolean;
                    } & {
                        [key: string]: unknown;
                    };
                    outbox_full?: boolean;
                } & {
                    [key: string]: unknown;
                };
                incident_persistence: {
                    failures?: number;
                    pending?: number;
                    retries?: number;
                } & {
                    [key: string]: unknown;
                };
            } & {
                [key: string]: unknown;
            };
            snapshot: {
                age_ms: null | number;
                available: boolean;
                completed_at_ms: null | number;
                freshness: string;
                sequence: null | number;
            } & {
                [key: string]: unknown;
            };
            thresholds: ({
                condition: string;
                critical_for_seconds: number;
                critical_threshold: number;
                direction: string;
                enabled: boolean;
                overridden: boolean;
                recovery_for_seconds: number;
                recovery_threshold: number;
                unit: string;
                warning_for_seconds: number;
                warning_threshold: number;
            } & {
                [key: string]: unknown;
            })[];
            visibility: {
                coverage_boundary: string;
                effective_scope: string;
                host_hardware_visible: boolean;
            } & {
                [key: string]: unknown;
            };
        } & {
            [key: string]: unknown;
        };
        HostHealthIncidents: {
            count: number;
            incidents: ({
                closed_at_ms: null | number;
                condition: string;
                first_observed_at_ms: number;
                incident_id: string;
                last_observed_at_ms: number;
                observation: {
                    capability: string;
                    freshness: string;
                    metric: string;
                    observed_at_ms?: number;
                    resource: string;
                    sampled_monotonic_ms?: number;
                    scope: string;
                    unit: string;
                    value: null | number;
                } & {
                    [key: string]: unknown;
                };
                reconciliation: string;
                revision: number;
                scope: string;
                severity: string;
                state: string;
                subject: string;
            } & {
                [key: string]: unknown;
            })[];
            next_cursor: null | string;
            schema_version: number;
        } & {
            [key: string]: unknown;
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
            must_change_password?: never;
            password?: never;
            password_hash?: never;
            redirect?: never;
            secret?: never;
            success?: never;
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
            totp_required?: never;
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
        MotionTrigger: {
            /** @enum {string} */
            action: "start" | "stop" | "pulse";
            confidence?: number;
            duration_ms?: number;
            label?: string;
            objects?: (string | {
                confidence?: number;
                label: string;
            })[];
            stream: string;
            tags?: string[];
        };
        MotionTriggered: {
            /** @enum {string} */
            action: "start" | "stop" | "pulse";
            detections_stored: number;
            duration_ms?: number;
            recording_triggered: boolean;
            stream: string;
            /** @constant */
            success: true;
            tags_applied: number;
            warning?: string;
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
        PasswordAdminReset: {
            new_password: string;
            old_password?: string;
        };
        PasswordChangedResponse: {
            /** @constant */
            must_change_password: false;
            /** @constant */
            success: true;
        };
        /** @description New password must contain at least eight UTF-8 bytes. Self-service requires the current password; effective users.manage can reset passwords except its own forced first-login change. Locked/default-password rules are enforced by the server. */
        PasswordChangeRequest: components["schemas"]["PasswordSelfChange"] | components["schemas"]["PasswordAdminReset"];
        PasswordSelfChange: {
            new_password: string;
            old_password: string;
        };
        /** @enum {string} */
        PlaybackTransport: "auto" | "webrtc_only" | "mse_only" | "hls_only" | "webrtc_then_mse" | "mse_then_hls";
        PlayerTelemetry: {
            rebuffer_count?: number;
            rebuffer_duration_ms?: number;
            resolution_switches?: number;
            session_id?: string;
            stream_name: string;
            transport?: string;
            ttff_ms?: number;
            webrtc_rtt_ms?: number;
        } & {
            [key: string]: unknown;
        };
        ProcessLiveness: {
            failedRequests: number;
            healthy: boolean;
            /** @constant */
            liveness: true;
            timestamp: string;
            totalRequests: number;
            uptime: number;
        } & {
            [key: string]: unknown;
        };
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
        SetupStatus: {
            complete: boolean;
            setup_completed_at: number;
        } & {
            [key: string]: unknown;
        };
        SetupUpdate: {
            complete?: boolean;
        } & {
            [key: string]: unknown;
        };
        SetupUpdated: {
            /** @constant */
            success: true;
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
        StreamHealth: {
            failedRequests: number;
            healthy: boolean;
            status: string;
            storage: {
                available_bytes: number;
                used_bytes: number;
            } & {
                [key: string]: unknown;
            };
            streams: {
                degraded: number;
                down: number;
                total: number;
                up: number;
            } & {
                [key: string]: unknown;
            };
            streams_detail: ({
                bitrate_bps: number;
                configured_fps: number;
                connection_latency_ms: number;
                current_fps: number;
                detector_operating: boolean;
                expected_fps: number;
                fps: number;
                frames_dropped: number;
                frames_total: number;
                last_completed_segment_ts: number;
                last_frame_ts: number;
                last_packet_ts: number;
                name: string;
                observed_fps: number;
                reconnects: number;
                recorder_session_active: boolean;
                recorder_writing: boolean;
                recording_active: boolean;
                recording_gaps: number;
                source_connected: boolean;
                sparkline_bitrate?: number[];
                sparkline_fps?: number[];
                status: string;
                uptime_seconds: number;
                video_advancing: boolean;
            } & {
                [key: string]: unknown;
            })[];
            timestamp: string;
            totalRequests: number;
            uptime: number;
        } & {
            [key: string]: unknown;
        };
        StreamHealthHistory: {
            failedRequests: number;
            healthy: boolean;
            streams_detail: ({
                name: string;
                sparkline_bitrate?: number[];
                sparkline_fps?: number[];
            } & {
                [key: string]: unknown;
            })[];
            timestamp: string;
            totalRequests: number;
            uptime: number;
        } & {
            [key: string]: unknown;
        };
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
        SystemActionAccepted: {
            message: string;
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        SystemBackupCreated: {
            backupUrl: string;
            filename: string;
            message: string;
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        SystemInformation: {
            cpu: {
                cores?: number;
                model?: string;
                usage?: null | number;
                usageCapability?: string;
            } & {
                [key: string]: unknown;
            };
            detectorMemory?: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            disk: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            git_commit: string;
            global_retention_days: number;
            go2rtcMemory?: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            memory: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            network: {
                interfaces?: ({
                    address?: null | string;
                    capability?: string;
                    ipv6?: string;
                    mac?: string;
                    name: string;
                    up?: boolean;
                } & {
                    [key: string]: unknown;
                })[];
            } & {
                [key: string]: unknown;
            };
            recordings: {
                count: number;
                size: number;
            } & {
                [key: string]: unknown;
            };
            streams: {
                active: number;
                total: number;
            } & {
                [key: string]: unknown;
            };
            streamStorage: ({
                count: number;
                detection_retention_days: number;
                max_storage_mb: number;
                name: string;
                retention_days: number;
                size: number;
            } & {
                [key: string]: unknown;
            })[];
            systemDisk: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            systemMemory?: {
                capability?: string;
                free?: null | number;
                total?: null | number;
                used?: null | number;
            } & {
                [key: string]: unknown;
            };
            threads: null | number;
            uptime: number;
            uptimeCapability?: string;
            version: string;
            versions: {
                items: ({
                    category: string;
                    details?: string;
                    name: string;
                    version: string;
                } & {
                    [key: string]: unknown;
                })[];
            } & {
                [key: string]: unknown;
            };
            webThreadPoolSize: number;
        } & {
            [key: string]: unknown;
        };
        SystemLogs: {
            file: string;
            level: string;
            logs: ({
                level?: string;
                message?: string;
                timestamp?: string;
            } & {
                [key: string]: unknown;
            })[];
        } & {
            [key: string]: unknown;
        };
        SystemSettings: {
            api_detection_backend: string;
            api_detection_detector_name: string;
            api_detection_format: string;
            api_detection_url: string;
            audio_disabled: boolean;
            auth_absolute_timeout_hours: number;
            auth_timeout_hours: number;
            auto_delete_oldest: boolean;
            auto_disabled: boolean;
            buffer_size: number;
            db_backup_interval_minutes: number;
            db_backup_retention_count: number;
            db_backup_verify: string;
            db_path: string;
            db_post_backup_script: string;
            default_detection_threshold: number;
            default_playback_transport: string;
            demo_mode: boolean;
            detection_engine_delegate: string;
            detection_engine_enabled: boolean;
            detection_engine_threads: number;
            detection_grace_period: number;
            force_mfa_on_login: boolean;
            generate_thumbnails: boolean;
            go2rtc_api_port: number;
            go2rtc_base_path: string;
            go2rtc_binary_path: string;
            go2rtc_config_dir: string;
            go2rtc_config_override: string;
            go2rtc_config_override_disabled_reason: string;
            go2rtc_config_override_quarantined: string;
            go2rtc_enabled: boolean;
            go2rtc_external_ip: string;
            go2rtc_force_native_hls: boolean;
            go2rtc_ice_servers: string;
            go2rtc_rtsp_port: number;
            go2rtc_stun_enabled: boolean;
            go2rtc_stun_server: string;
            go2rtc_webrtc_enabled: boolean;
            go2rtc_webrtc_listen_port: number;
            health_condition_overrides: null | ({
                conditions?: ({
                    code: string;
                    critical?: number;
                    critical_for_seconds?: number;
                    profile?: string;
                    recovery?: number;
                    recovery_for_seconds?: number;
                    unit?: string;
                    warning?: number;
                    warning_for_seconds?: number;
                } & {
                    [key: string]: unknown;
                })[];
                version?: number;
            } & {
                [key: string]: unknown;
            });
            health_device_interval_seconds: number;
            health_effective_policy?: {
                conditions?: ({
                    code: string;
                    critical?: number;
                    critical_for_seconds?: number;
                    recovery?: number;
                    recovery_for_seconds?: number;
                    unit?: string;
                    warning?: number;
                    warning_for_seconds?: number;
                } & {
                    [key: string]: unknown;
                })[];
            } & {
                [key: string]: unknown;
            };
            health_enabled: boolean;
            health_fast_interval_seconds: number;
            health_hardware_provider: string;
            health_incident_retention_days: number;
            health_normal_interval_seconds: number;
            health_presence_interval_seconds: number;
            health_profile: string;
            health_slow_interval_seconds: number;
            health_write_probe_enabled: boolean;
            hls_disabled: boolean;
            log_file: string;
            log_level: number;
            login_rate_limit_enabled: boolean;
            login_rate_limit_max_attempts: number;
            login_rate_limit_window_seconds: number;
            lpr_retention_days: number;
            max_storage_size: number;
            max_streams: number;
            max_streams_ceiling: number;
            models_path: string;
            mp4_directory_format: string;
            mp4_faststart: boolean;
            mqtt_broker_host: string;
            mqtt_broker_port: number;
            mqtt_client_id: string;
            mqtt_enabled: boolean;
            mqtt_ha_discovery: boolean;
            mqtt_ha_discovery_prefix: string;
            mqtt_ha_snapshot_interval: number;
            mqtt_keepalive: number;
            /** @enum {string} */
            mqtt_password: "" | "********";
            mqtt_qos: number;
            mqtt_retain: boolean;
            mqtt_tls_enabled: boolean;
            mqtt_topic_prefix: string;
            mqtt_username: string;
            mse_disabled: boolean;
            onvif_discovery_enabled: boolean;
            onvif_discovery_interval: number;
            onvif_discovery_network: string;
            pid_file: string;
            post_detection_buffer: number;
            pre_detection_buffer: number;
            retention_days: number;
            storage_min_free_pct: number;
            storage_path: string;
            storage_path_hls: string;
            storage_pressure_critical_pct: number;
            storage_pressure_emergency_pct: number;
            storage_pressure_warning_pct: number;
            swap_size: number;
            syslog_enabled: boolean;
            syslog_facility: string;
            syslog_ident: string;
            thumbnails_per_recording: number;
            trusted_device_days: number;
            trusted_proxy_cidrs: string;
            turn_enabled: boolean;
            /** @enum {string} */
            turn_password: "" | "********";
            turn_server_url: string;
            turn_username: string;
            use_swap: boolean;
            web_auth_enabled: boolean;
            web_bind_ip: string;
            web_port: number;
            web_root: string;
            web_thread_pool_size: number;
            webrtc_connection_timeout_ms: number;
            webrtc_disabled: boolean;
            webrtc_ice_recovery_timeout_ms: number;
        } & {
            [key: string]: unknown;
        };
        SystemSettingsSaved: {
            restart_required: boolean;
            restart_required_message?: string;
            /** @constant */
            success: true;
        } & {
            [key: string]: unknown;
        };
        /** @description Partial settings update; absent fields are retained. Secret mask values preserve the stored password. */
        SystemSettingsUpdate: {
            api_detection_backend?: string;
            api_detection_detector_name?: string;
            /** @enum {string} */
            api_detection_format?: "light-object-detect" | "doods2";
            api_detection_url?: string;
            audio_disabled?: boolean;
            auth_absolute_timeout_hours?: number;
            auth_timeout_hours?: number;
            auto_delete_oldest?: boolean;
            auto_disabled?: boolean;
            buffer_size?: number;
            db_backup_interval_minutes?: number;
            db_backup_retention_count?: number;
            db_backup_verify?: string;
            db_path?: string;
            db_post_backup_script?: string;
            default_detection_threshold?: number;
            default_playback_transport?: string;
            demo_mode?: boolean;
            detection_engine_delegate?: string;
            detection_engine_enabled?: boolean;
            detection_engine_threads?: number;
            detection_grace_period?: number;
            force_mfa_on_login?: boolean;
            generate_thumbnails?: boolean;
            go2rtc_api_port?: number;
            go2rtc_binary_path?: string;
            go2rtc_config_dir?: string;
            go2rtc_config_override?: string;
            go2rtc_enabled?: boolean;
            go2rtc_external_ip?: string;
            go2rtc_force_native_hls?: boolean;
            go2rtc_ice_servers?: string;
            go2rtc_rtsp_port?: number;
            go2rtc_stun_enabled?: boolean;
            go2rtc_stun_server?: string;
            go2rtc_webrtc_enabled?: boolean;
            go2rtc_webrtc_listen_port?: number;
            health_condition_overrides?: null | ({
                conditions?: ({
                    code: string;
                    critical?: number;
                    critical_for_seconds?: number;
                    profile?: string;
                    recovery?: number;
                    recovery_for_seconds?: number;
                    unit?: string;
                    warning?: number;
                    warning_for_seconds?: number;
                } & {
                    [key: string]: unknown;
                })[];
                version?: number;
            } & {
                [key: string]: unknown;
            });
            health_device_interval_seconds?: number;
            health_enabled?: boolean;
            health_fast_interval_seconds?: number;
            health_hardware_provider?: string;
            health_incident_retention_days?: number;
            health_normal_interval_seconds?: number;
            health_presence_interval_seconds?: number;
            health_profile?: string;
            health_slow_interval_seconds?: number;
            health_write_probe_enabled?: boolean;
            hls_disabled?: boolean;
            log_file?: string;
            log_level?: number;
            login_rate_limit_enabled?: boolean;
            login_rate_limit_max_attempts?: number;
            login_rate_limit_window_seconds?: number;
            lpr_retention_days?: number;
            max_storage_size?: number;
            max_streams?: number;
            models_path?: string;
            mp4_directory_format?: string;
            mp4_faststart?: boolean;
            mqtt_broker_host?: string;
            mqtt_broker_port?: number;
            mqtt_client_id?: string;
            mqtt_enabled?: boolean;
            mqtt_ha_discovery?: boolean;
            mqtt_ha_discovery_prefix?: string;
            mqtt_ha_snapshot_interval?: number;
            mqtt_keepalive?: number;
            mqtt_password?: string;
            mqtt_qos?: number;
            mqtt_retain?: boolean;
            mqtt_tls_enabled?: boolean;
            mqtt_topic_prefix?: string;
            mqtt_username?: string;
            mse_disabled?: boolean;
            onvif_discovery_enabled?: boolean;
            onvif_discovery_interval?: number;
            onvif_discovery_network?: string;
            pid_file?: string;
            post_detection_buffer?: number;
            pre_detection_buffer?: number;
            retention_days?: number;
            storage_min_free_pct?: number;
            storage_path?: string;
            storage_path_hls?: string;
            storage_pressure_critical_pct?: number;
            storage_pressure_emergency_pct?: number;
            storage_pressure_warning_pct?: number;
            swap_size?: number;
            syslog_enabled?: boolean;
            syslog_facility?: string;
            syslog_ident?: string;
            thumbnails_per_recording?: number;
            trusted_device_days?: number;
            trusted_proxy_cidrs?: string;
            turn_enabled?: boolean;
            turn_password?: string;
            turn_server_url?: string;
            turn_username?: string;
            use_swap?: boolean;
            web_auth_enabled?: boolean;
            web_bind_ip?: string;
            web_port?: number;
            web_root?: string;
            web_thread_pool_size?: number;
            webrtc_connection_timeout_ms?: number;
            webrtc_disabled?: boolean;
            webrtc_ice_recovery_timeout_ms?: number;
        } & {
            [key: string]: unknown;
        };
        SystemStatus: {
            message: string;
            /** @constant */
            status: "ok";
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
        TotpChangedResponse: {
            message: string;
            /** @constant */
            success: true;
        };
        TotpCodeRequest: {
            code: string;
        };
        TotpLoginRequest: {
            code: string;
            remember_device?: boolean;
            totp_token: string;
        };
        TotpLoginResponse: {
            /** @constant */
            redirect: "/index.html";
            /** @constant */
            success: true;
        };
        TotpSetupResponse: {
            message: string;
            /** Format: uri */
            readonly otpauth_uri: string;
            readonly secret: string;
            /** @constant */
            success: true;
        };
        TotpStatusResponse: {
            totp_configured: boolean;
            totp_enabled: boolean;
        };
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
        WorkspacesResponse: {
            configurable: boolean;
            /** @enum {string} */
            scope: "user" | "installation";
            workspaces: ({
                /** @constant */
                authorization_unchanged: true;
                configurable: boolean;
                description: string;
                /** @enum {string} */
                key: "live.navigator" | "investigation";
                label: string;
                visible: boolean;
            } & {
                [key: string]: unknown;
            })[];
        } & {
            [key: string]: unknown;
        };
        WorkspacesUpdate: {
            workspaces: {
                investigation?: boolean;
                "live.navigator"?: boolean;
            };
        } & {
            [key: string]: unknown;
        };
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
    completeTotpLogin: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TotpLoginRequest"];
            };
        };
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    /** @description Authenticated session; optional trusted device if enabled and requested. */
                    "Set-Cookie"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TotpLoginResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Enrollment expired or state changed; refresh status. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description TOTP attempts are account-throttled. Do not automatically retry. */
            429: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    changePassword: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive decimal signed 64-bit user ID. Leading zeroes are accepted by the handler; browser adapters require safe positive integer IDs. Concrete self-service targets only the current account. */
                id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["PasswordChangeRequest"];
            };
        };
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["PasswordChangedResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Principal lacks self-service or effective users.manage; password may be locked. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Target user not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    disableTotp: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive decimal signed 64-bit user ID. Leading zeroes are accepted by the handler; browser adapters require safe positive integer IDs. Concrete self-service targets only the current account. */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TotpChangedResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Principal lacks self-service or effective users.manage; password may be locked. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Target user not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Enrollment expired or state changed; refresh status. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description TOTP attempts are account-throttled. Do not automatically retry. */
            429: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    setupTotp: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive decimal signed 64-bit user ID. Leading zeroes are accepted by the handler; browser adapters require safe positive integer IDs. Concrete self-service targets only the current account. */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TotpSetupResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Principal lacks self-service or effective users.manage; password may be locked. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Target user not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Enrollment expired or state changed; refresh status. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description TOTP attempts are account-throttled. Do not automatically retry. */
            429: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    getTotpStatus: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive decimal signed 64-bit user ID. Leading zeroes are accepted by the handler; browser adapters require safe positive integer IDs. Concrete self-service targets only the current account. */
                id: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TotpStatusResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Principal lacks self-service or effective users.manage; password may be locked. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Target user not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    verifyTotp: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                /** @description Positive decimal signed 64-bit user ID. Leading zeroes are accepted by the handler; browser adapters require safe positive integer IDs. Concrete self-service targets only the current account. */
                id: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["TotpCodeRequest"];
            };
        };
        responses: {
            /** @description Validated success. */
            200: {
                headers: {
                    /** @description Includes no-store; authentication data must not be cached. */
                    "Cache-Control"?: string;
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["TotpChangedResponse"];
                };
            };
            /** @description Invalid path or body. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Invalid credential, current password, challenge or code (including replay). */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Principal lacks self-service or effective users.manage; password may be locked. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Target user not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Enrollment expired or state changed; refresh status. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description TOTP attempts are account-throttled. Do not automatically retry. */
            429: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Internal/database failure. */
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
    listCameraCollections: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CameraCollectionsResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getCameraCollection: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CameraCollection"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getCameraCollectionMembers: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CameraCollectionMembersResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Only static collections have explicit members. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    previewCameraCollection: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CameraCollectionPreview"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    listFleetTags: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetTagsResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getFleetTag: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetTag"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getCameraTagAssignments: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["CameraTagAssignmentsResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    listEventDestinations: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventDestinations"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    createEventDestination: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EventDestinationCreate"];
            };
        };
        responses: {
            /** @description Successful response. */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventDestination"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    getEventDestination: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventDestination"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    updateEventDestination: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EventDestinationUpdate"];
            };
        };
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventDestination"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    deleteEventDestination: {
        parameters: {
            query: {
                revision: number;
            };
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventMutationAck"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    listEventRoutes: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventRoutes"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    createEventRoute: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EventRouteCreate"];
            };
        };
        responses: {
            /** @description Successful response. */
            201: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventRoute"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    getEventRoute: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventRoute"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    updateEventRoute: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EventRouteUpdate"];
            };
        };
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventRoute"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    deleteEventRoute: {
        parameters: {
            query: {
                revision: number;
            };
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventMutationAck"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    previewEventRoute: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["EventRoutePreviewRequest"];
            };
        };
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventRoutePreview"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    getEventCatalog: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["EventCatalog"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    queryFleetCameras: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["FleetQueryRequest"];
            };
        };
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetQueryResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    previewFleetSelector: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["FleetPreviewRequest"];
            };
        };
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetPreviewResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    listFleetViews: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetSavedViewsResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getFleetView: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetSavedView"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getStreamHealth: {
        parameters: {
            query?: {
                /** @description Return authorized stream histories. */
                history_only?: boolean;
                /** @description Return only process liveness. */
                liveness?: boolean;
                /** @description Include stream history arrays. */
                sparklines?: boolean;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HealthResponse"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getHlsHealth: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HlsHealth"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    listFleetLocations: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetLocationsResponse"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getFleetLocation: {
        parameters: {
            query?: never;
            header?: never;
            path: {
                uuid: string;
            };
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful authorized read. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["FleetLocation"];
                };
            };
            /** @description Invalid request/path or selector. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description No usable authenticated principal. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Camera or action access denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Missing or invisible object. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Database, authorization or serialization failure. */
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
    getMetrics: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "text/plain": string;
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    triggerMotion: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["MotionTrigger"];
            };
        };
        responses: {
            /** @description Accepted motion effects are enumerated separately. */
            202: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["MotionTriggered"];
                };
            };
            /** @description Invalid request or reference. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required effective authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Object not found. */
            404: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Revision conflict, name conflict, limit or referenced object. */
            409: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Persistence or internal failure. */
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
    getSystemSettings: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemSettings"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    updateSystemSettings: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["SystemSettingsUpdate"];
            };
        };
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemSettingsSaved"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    validateGo2rtcConfiguration: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: {
            content: {
                "application/json": components["schemas"]["Go2rtcConfigurationInput"];
            };
        };
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Go2rtcConfigurationValidation"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getSetupStatus: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SetupStatus"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    updateSetupStatus: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: {
            content: {
                "application/json": components["schemas"]["SetupUpdate"];
            };
        };
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SetupUpdated"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getSystemInformationLegacy: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemInformation"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    createSystemBackup: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemBackupCreated"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getGo2rtcEffectiveConfiguration: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Go2rtcEffectiveConfiguration"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getGo2rtcOverrideDiagnostics: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["Go2rtcOverrideDiagnostics"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getHostHealth: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HostHealth"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getHostHealthIncidents: {
        parameters: {
            query?: {
                /** @description Opaque versioned cursor from the preceding page; malformed cursors return 400. */
                cursor?: string;
                include_closed?: boolean;
                limit?: number;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["HostHealthIncidents"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getSystemInformation: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemInformation"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getSystemLogs: {
        parameters: {
            query?: {
                count?: number;
                last_ts?: string;
                level?: string;
            };
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemLogs"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    clearSystemLogs: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemActionAccepted"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    restartSystem: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemActionAccepted"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    shutdownSystem: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemActionAccepted"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getSystemStatus: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["SystemStatus"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    recordPlayerTelemetry: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: {
            content: {
                "application/json": components["schemas"]["PlayerTelemetry"];
            };
        };
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            204: {
                headers: {
                    [name: string]: unknown;
                };
                content?: never;
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    getWorkspaces: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody?: never;
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["WorkspacesResponse"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    updateWorkspaces: {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        requestBody: {
            content: {
                "application/json": components["schemas"]["WorkspacesUpdate"];
            };
        };
        responses: {
            /** @description Successful response. Process-control responses acknowledge a request; verify eventual completion independently. */
            200: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["WorkspacesResponse"];
                };
            };
            /** @description Invalid request. */
            400: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Authentication required. */
            401: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Required authority denied. */
            403: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Server or persistence failure. */
            500: {
                headers: {
                    [name: string]: unknown;
                };
                content: {
                    "application/json": components["schemas"]["ApiError"];
                };
            };
            /** @description Service temporarily unavailable. */
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
    legacyLogout: {
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
}
