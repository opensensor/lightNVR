# Event routing contract evidence (F8)

This slice accepts 13 registrations: event catalog; destination list/create/detail/
update/delete; route list/create/detail/update/delete/preview; external motion
trigger. Administration requires effective global `events.configure`. Motion
requires `camera.configure` on its named camera, not event administration.

Canonical schemas distinguish redacted destination responses from credential
writes, creation from revisioned updates, selectors, event predicates, schedules,
suppression, preview and motion effects. Deletes require a positive safe revision.
Native checks remain authoritative for registered event/health names, referenced
MQTT destinations, timezone availability, TLS combinations and template syntax.
Event scopes support `all` or `selector`; collection references are not supported.
Tests explicitly reject actual private and shared collection references rather
than claiming collection-based event authorization exists.

The closed `eventJSON` boundary validates real EventRoutingView reads and writes,
checks status/content type, preserves aborts and headers, sanitizes failures and
never retries a mutation. Existing-route previews now strip the mutation revision:
the native preview handler uses creation semantics and rejects a revision.
Malformed acknowledgments keep editors open without reporting success.

## Independent evidence

RED: the baseline real-server discovery test lacked all 13 expected operations.
GREEN: four owned API cases cover the independently authored credential/action
matrix; effective policy revocation; CRUD and exact status/shape; credential
preservation/clearing/redaction; revision/in-use/missing conflicts; side-effect-free
preview; malformed and denied mutation no-effects; actual private/shared collection
reference rejection; and per-camera motion authorization with annotation/recording
results reported separately.

A fifth real-server case uses the MQTT-enabled build and an owned loopback TCP
sink implementing the small MQTT 3.1.1 exchange needed by the fixture. It inspects
actual PUBLISH topic/envelope bytes, withholds PUBACK, disconnects the connection,
then observes retry of the same envelope ID and durable delivered state with at
least two attempts. A second eligible camera event during cooldown is suppressed.
Unauthorized motion, an excluded camera and a disabled route produce no delivery.
This is a real native MQTT client/worker/queue test, not a mocked publisher.
The contract CI job now installs libmosquitto, enables MQTT and requires this case;
explicit MQTT-disabled local builds retain a visibly skipped delivery case.

Two Chromium tests exercise the actual workspace: malformed destination creation
cannot close the editor or claim success, no duplicate write occurs, and an
explicit retry succeeds; editing an existing route previews successfully without
changing its revision. Eight adapter tests cover validation, credentials, revisions,
unsupported URLs, no retries, content types/status and cancellation.

Validation passed:

- Five API cases including real MQTT retry/cooldown; two Chromium cases.
- Eight frontend adapter tests; frontend/root test typing, production build, ESLint.
- Canonical generation/drift and lint (four existing recording ambiguity warnings).
- Ten tooling tests and 13,858 validator equivalence cases.
- Twelve optimized native event/router/outbox/destination/motion suites.
- Three Debug ASan/UBSan event-handler/motion suites with leak detection.

Raw development logs: ignored `build/hardening-four/events-*`. Final combined
suite results are recorded in the execution plan. Fixtures use synthetic owned
camera metadata, loopback sinks and temporary databases. No physical camera,
remote broker, certificate deployment or user configuration was exercised.

Transport review follow-up: event requests force `redirect: error` and
`credentials: same-origin` after caller options. Real 307/308 responses from an
owned loopback application endpoint previously forwarded the synthetic broker
password to a second owned origin before response validation. Both regressions
now verify zero forwarding and exactly one application request, even when callers
request `redirect: follow`, `credentials: include` and seven retries. Errors stay
sanitized and ambiguous writes remain marked `mayHaveApplied`. The shared
redirect harness and system/event suite pass all 22 tests; TypeScript and adapter
ESLint pass. Evidence: `build/evidence-contracts/transport-{red,green}.log`.
