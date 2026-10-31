# T7 remediation expansion — coordinator insertion source

Append the full task records below to engineering-foundation-plan.md, apply gate dependency additions from quality/remediation-map.json, and run the T7 graph validator before marking T7 complete. These records do not approve exceptions or claim implemented fixes.

## Dependency graph and wave additions

T7 → T7.1 approval enforcement → T14; T8 → dependency remediations → T15; T10 → web remediation → overlapping pilot migrations and T25; T13 → native/pilot remediations; T12 + pilot remediations → contract reconciliation → T18/T19; every T7.* → T33. Within these frontiers, depends_on includes each previous writer of the same path.

| Frontier | Tasks | Resource constraints |
| --- | --- | --- |
| Policy/reviews | Tasks whose only producer is T6/T7/T11 | No production workload; approvals remain pending |
| Dependency | kind=dependency after T8 | One package/install/build writer, then T15 |
| Web | kind=web-fix after T10 | Max three workers; unique focused Jest files, exclusive full lint/build |
| Native/pilot | kind=native-fix/pilot-fix after T13 | Shared server/native operations serialized by runner lease; exact CMake fragments |
| Reconciliation | kind=integration after T12 and all pilot fixes | Exclusive generation, followed by T18/T19 |
| Closure | all T7.* then T33 | Fresh complete scanners; unresolved reviews block |

### T7.1: Reject unapproved findings and coverage exceptions
- **depends_on**: [T7, T6]
- **agent_type**: local
- **ownership**: `scripts/quality/findings.py`, `tests/quality/test_findings.py`, `quality/findings-policy.json`, `quality/evidence/T7.1.md`.
- **priority**: high
- **description**: Require explicit reviewed approval evidence before an exception can affect --check or --release. The existing proposed-by-T1/T6 dates and maintainers/<area> owners are proposals, not approvals. Preserve diagnostic reporting and the read-only normal check. Do not silently bless old records during schema migration.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - Tests first prove proposed, missing, expired and malformed approvals fail; an explicitly approved current exception succeeds only in its permitted mode.
  - An approval must name an accountable reviewer/owner, date and reason; placeholders do not qualify. --release still rejects coverage gaps.
  - Run against actual T1 reports: unapproved proposals must cause policy failure. Hash the inventory before/after --check.
- **validation**:
  - `python3 -m unittest discover -s tests/quality -p test_findings.py`
  - `python3 scripts/quality/findings.py --check --reports build/eng-artifacts/T1 --inventory quality/findings.json`
- **status**: Completed (commit `c285a0c2`)
- **log**: Coordinator implemented RED -> GREEN approval enforcement: 65 tests pass; real T1 fails 575 inventory and 4 gap approval proposals; no accepted gaps. Metadata validation does not authenticate a human approval; do not fabricate production approval records.
- **files edited/created**:

### T7.2: Verify disposition and regression evidence for ADVISORY-GHSA-cv53-mq7p-6jp2
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-cv53-mq7p-6jp2.json`, `quality/evidence/T7.2.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-cv53-mq7p-6jp2
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-cv53-mq7p-6jp2.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.3: Verify disposition and regression evidence for ADVISORY-repo-advisory-draft-1
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-draft-1.json`, `quality/evidence/T7.3.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-draft-1
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-draft-1.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.4: Verify disposition and regression evidence for ADVISORY-repo-advisory-closed-1
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-closed-1.json`, `quality/evidence/T7.4.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-closed-1
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-closed-1.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.5: Verify disposition and regression evidence for ADVISORY-repo-advisory-draft-2
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-repo-advisory-draft-2.json`, `quality/evidence/T7.5.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-repo-advisory-draft-2
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-repo-advisory-draft-2.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.6: Verify disposition and regression evidence for ADVISORY-GHSA-rqj3-pch5-rq4f
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-rqj3-pch5-rq4f.json`, `quality/evidence/T7.6.md`.
- **priority**: critical
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-rqj3-pch5-rq4f
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-rqj3-pch5-rq4f.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.7: Verify disposition and regression evidence for ADVISORY-GHSA-pvw7-7j8c-qx96
- **depends_on**: [T7]
- **agent_type**: local
- **ownership**: `quality/reviews/advisory-ghsa-pvw7-7j8c-qx96.json`, `quality/evidence/T7.7.md`.
- **priority**: high
- **description**: Read the source advisory through authorized access; record sanitized affected paths, fixed revision and exact existing regression command without publishing private identifiers or text. Published-version exclusion alone is not regression proof. If a regression or fix is absent, append a bounded child task and add it to T33 before this review can close; inaccessible private evidence leaves this task blocked.
- **findings**: ADVISORY-GHSA-pvw7-7j8c-qx96
- **acceptance_criteria**:
  - Confirm state and affected/current versions at the actual execution date.
  - Record reviewer, evidence reference and passing regression, or explicit unresolved blocker; a draft or closed label alone is not a verified fix.
  - No private advisory text, IDs or credentials in committed evidence.
- **validation**:
  - `python3 -m json.tool quality/reviews/advisory-ghsa-pvw7-7j8c-qx96.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.8: Remediate glob dependency advisory GHSA-3g3m-2fjr-6pw4
- **depends_on**: [T7, T8]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `quality/evidence/T7.8.md`.
- **priority**: medium
- **description**: Resolve GHSA-3g3m-2fjr-6pw4 for glob using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: NPM-WEB-GHSA-3g3m-2fjr-6pw4-glob
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain glob`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.9: Remediate minimatch dependency advisory GHSA-g3hv-75wg-m249
- **depends_on**: [T7, T8, T7.8]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `package.json`, `package-lock.json`, `quality/evidence/T7.9.md`.
- **priority**: medium
- **description**: Resolve GHSA-g3hv-75wg-m249 for minimatch using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: NPM-WEB-GHSA-g3hv-75wg-m249-minimatch
 T8 adds the root openapi-typescript/redocly dev path for the same minimatch advisory; root manifests are included and T33 must reconcile the new occurrence without automatic baseline changes.
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain minimatch`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
  - `npm explain minimatch`
  - `npm audit --json`
  - `npm audit --omit=dev --json`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.10: Remediate deepmerge dependency advisory GHSA-gcqx-w48m-w2h4
- **depends_on**: [T7, T8, T7.9]
- **agent_type**: local
- **ownership**: `web/package.json`, `web/package-lock.json`, `quality/evidence/T7.10.md`.
- **priority**: medium
- **description**: Resolve GHSA-gcqx-w48m-w2h4 for deepmerge using an actually available compatible patched transitive version or remove the vulnerable dependency path. Inspect npm explain and production/dev reachability. Preserve Node 24 and installed direct major versions. If no compatible fix exists, submit a specific owner/expiry/rationale proposal and keep this task open until explicitly reviewed; do not force an unrelated major upgrade.
- **findings**: DEPENDABOT-101, NPM-WEB-GHSA-gcqx-w48m-w2h4-deepmerge
- **acceptance_criteria**:
  - npm audit no longer reports the advisory on each original dependency path, or an explicitly reviewed current exception documents the residual exposure.
  - Dependency graph, unit tests and both web builds remain compatible; no broad overrides that violate parent ranges.
- **validation**:
  - `npm --prefix web explain deepmerge`
  - `npm --prefix web audit --json`
  - `npm --prefix web audit --omit=dev --json`
  - `npm --prefix web test -- --runInBand`
  - `npm --prefix web run build`
  - `LIGHTNVR_WEB_LEGACY=false npm --prefix web run build`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.11: cpp/cleartext-storage-database in db_event_destinations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_event_destinations.c`, `tests/unit/test_t7_11.c`, `tests/unit/cmake/eng/t7_11.cmake`, `quality/evidence/T7.11.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-316, CODEQL-307
- **acceptance_criteria**:
  - Trace credential persistence and read access; document existing at-rest security and migration constraints. Test redaction and restricted access. If encryption changes the storage contract, materialize a separate bounded migration task instead of inventing a key-management scheme.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.11 --regex '^test_t7_11$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.11`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.12: cpp/cleartext-storage-database in db_streams.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_streams.c`, `tests/unit/test_t7_12.c`, `tests/unit/cmake/eng/t7_12.cmake`, `quality/evidence/T7.12.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-185, CODEQL-183, CODEQL-181
- **acceptance_criteria**:
  - Trace credential persistence and read access; document existing at-rest security and migration constraints. Test redaction and restricted access. If encryption changes the storage contract, materialize a separate bounded migration task instead of inventing a key-management scheme.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.12 --regex '^test_t7_12$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.12`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.13: cpp/cleartext-storage-file in config.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/config.c`, `tests/unit/test_t7_13.c`, `tests/unit/cmake/eng/t7_13.cmake`, `quality/evidence/T7.13.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-267, CODEQL-64, CODEQL-63
- **acceptance_criteria**:
  - Verify file permissions, creation umask, redacted logs and credential read access; preserve compatible config loading. Residual necessary plaintext needs explicit reviewed exposure acceptance.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.13 --regex '^test_t7_13$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.13`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.14: cpp/cleartext-transmission in hls_unified_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_14.c`, `tests/unit/cmake/eng/t7_14.cmake`, `quality/evidence/T7.14.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-264
- **acceptance_criteria**:
  - Assert credential-bearing payloads are not disclosed to an untrusted/insecure destination; verify supported camera transport behavior. Any required legacy cleartext transport needs an explicit reviewed exposure exception.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.14 --regex '^test_t7_14$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.14`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.15: cpp/cleartext-transmission in stream_protocol.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_15.c`, `tests/unit/cmake/eng/t7_15.cmake`, `quality/evidence/T7.15.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-14
- **acceptance_criteria**:
  - Assert credential-bearing payloads are not disclosed to an untrusted/insecure destination; verify supported camera transport behavior. Any required legacy cleartext transport needs an explicit reviewed exposure exception.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.15 --regex '^test_t7_15$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.15`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.16: cpp/integer-multiplication-cast-to-long in mp4_writer_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_16.c`, `tests/unit/cmake/eng/t7_16.cmake`, `quality/evidence/T7.16.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-329
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.16 --regex '^test_t7_16$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.16`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.17: cpp/non-https-url in go2rtc_api.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `tests/unit/test_t7_17.c`, `tests/unit/cmake/eng/t7_17.cmake`, `quality/evidence/T7.17.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-78, CODEQL-77, CODEQL-76, CODEQL-75, CODEQL-71, CODEQL-70, CODEQL-69, CODEQL-20, CODEQL-19
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.17 --regex '^test_t7_17$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.17`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.18: cpp/non-https-url in onvif_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_18.c`, `tests/unit/cmake/eng/t7_18.cmake`, `quality/evidence/T7.18.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-337, CODEQL-336, CODEQL-335, CODEQL-334, CODEQL-272, CODEQL-271, CODEQL-270
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.18 --regex '^test_t7_18$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.18`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.19: cpp/non-https-url in onvif_discovery.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_discovery.c`, `tests/unit/test_t7_19.c`, `tests/unit/cmake/eng/t7_19.cmake`, `quality/evidence/T7.19.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-59
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.19 --regex '^test_t7_19$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.19`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.20: cpp/non-https-url in api_handlers_ptz.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_ptz.c`, `tests/unit/test_t7_20.c`, `tests/unit/cmake/eng/t7_20.cmake`, `quality/evidence/T7.20.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-73
- **acceptance_criteria**:
  - Prove endpoint trust and transport policy for each reported URL; retain HTTP-only camera compatibility only with an explicitly reviewed exposure exception. Do not rewrite device endpoints to HTTPS without negotiation/support.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.20 --regex '^test_t7_20$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.20`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.21: cpp/path-injection in storage_s3.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_21.c`, `tests/unit/cmake/eng/t7_21.cmake`, `quality/evidence/T7.21.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-325
- **acceptance_criteria**:
  - Reject dot segments, separators and encoded traversal from untrusted names before filesystem access; assert normal names still select the intended file and no outside-tree path is opened.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.21 --regex '^test_t7_21$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.21`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.22: cpp/path-injection in linux_cgroup.c, linux_network.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_22.c`, `tests/unit/cmake/eng/t7_22.cmake`, `quality/evidence/T7.22.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-320, CODEQL-319, CODEQL-318, CODEQL-317
- **acceptance_criteria**:
  - Reject dot segments, separators and encoded traversal from untrusted names before filesystem access; assert normal names still select the intended file and no outside-tree path is opened.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.22 --regex '^test_t7_22$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.22`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.23: cpp/sql-injection in sqlite_migrate.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/sqlite_migrate.c`, `tests/unit/test_t7_23.c`, `tests/unit/cmake/eng/t7_23.cmake`, `quality/evidence/T7.23.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-67, CODEQL-66
- **acceptance_criteria**:
  - Use quote, semicolon and comment-bearing identifiers in a disposable database; ensure they cannot execute a second statement or alter unrelated schema, while valid migration names still work.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.23 --regex '^test_t7_23$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.23`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.24: cpp/suspicious-pointer-scaling in frame.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_24-upstream.json`, `quality/evidence/T7.24.md`.
- **priority**: high
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CODEQL-330
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_24-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.24`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.25: cpp/toctou-race-condition in db_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_core.c`, `tests/unit/test_t7_25.c`, `tests/unit/cmake/eng/t7_25.cmake`, `quality/evidence/T7.25.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-321
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.25 --regex '^test_t7_25$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.25`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.26: cpp/toctou-race-condition in storage_migration.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_26.c`, `tests/unit/cmake/eng/t7_26.cmake`, `quality/evidence/T7.26.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-310
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.26 --regex '^test_t7_26$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.26`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.27: cpp/toctou-race-condition in api_handlers_operator_floor_plans.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_operator_floor_plans.c`, `tests/unit/test_t7_27.c`, `tests/unit/cmake/eng/t7_27.cmake`, `quality/evidence/T7.27.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-315, CODEQL-314, CODEQL-313
- **acceptance_criteria**:
  - Exercise replacement/symlink races between validation and use; operations must hold and use the validated descriptor or reject changed identity, preserving normal success and cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.27 --regex '^test_t7_27$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.27`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.28: cpp/uncontrolled-allocation-size in event_router.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_28.c`, `tests/unit/cmake/eng/t7_28.cmake`, `quality/evidence/T7.28.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-306, CODEQL-305
- **acceptance_criteria**:
  - Exercise zero, maximum accepted, one-past maximum and overflow-sized user inputs; reject before allocation/multiplication, leave state unchanged, and preserve ordinary delivery.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.28 --regex '^test_t7_28$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.28`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.29: cpp/world-writable-file-creation in mqtt_client.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_client.c`, `tests/unit/test_t7_29.c`, `tests/unit/cmake/eng/t7_29.cmake`, `quality/evidence/T7.29.md`.
- **priority**: high
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CODEQL-293
- **acceptance_criteria**:
  - Create the credential-bearing file under a permissive umask and assert effective permissions exclude group/other access; retain existing owner and normal reload semantics.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.29 --regex '^test_t7_29$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.29`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.30: arrayIndexOutOfBounds in stream_protocol.c
- **depends_on**: [T7, T13, T7.15]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_30.c`, `tests/unit/cmake/eng/t7_30.cmake`, `quality/evidence/T7.30.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-0dfdce44e3f2c522
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.30 --regex '^test_t7_30$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.30`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.31: clang-analyzer-core.CallAndMessage in system_health_policy.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_policy.c`, `tests/unit/test_t7_31.c`, `tests/unit/cmake/eng/t7_31.cmake`, `quality/evidence/T7.31.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9b21a14b97b3a15e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.31 --regex '^test_t7_31$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.31`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.32: clang-analyzer-core.DivideZero in db_storage_pools.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_pools.c`, `tests/unit/test_t7_32.c`, `tests/unit/cmake/eng/t7_32.cmake`, `quality/evidence/T7.32.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b7cb62f9287bca53
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.32 --regex '^test_t7_32$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.32`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.33: clang-analyzer-core.NonNullParamChecker in camera_collection_filter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/camera_collection_filter.c`, `tests/unit/test_t7_33.c`, `tests/unit/cmake/eng/t7_33.cmake`, `quality/evidence/T7.33.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a918c6008bfe9234
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.33 --regex '^test_t7_33$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.33`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.34: clang-analyzer-core.NonNullParamChecker in db_storage_policies.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_policies.c`, `tests/unit/test_t7_34.c`, `tests/unit/cmake/eng/t7_34.cmake`, `quality/evidence/T7.34.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a5ab5fe6add78d8e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.34 --regex '^test_t7_34$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.34`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.35: clang-analyzer-core.NonNullParamChecker in storage_s3.c
- **depends_on**: [T7, T13, T7.21]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_35.c`, `tests/unit/cmake/eng/t7_35.cmake`, `quality/evidence/T7.35.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cb260984bd5750f8
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.35 --regex '^test_t7_35$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.35`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.36: clang-analyzer-core.NullDereference in event_envelope.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_envelope.c`, `tests/unit/test_t7_36.c`, `tests/unit/cmake/eng/t7_36.cmake`, `quality/evidence/T7.36.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f240093f03b567fb
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.36 --regex '^test_t7_36$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.36`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.37: clang-analyzer-core.NullDereference in db_live_saved_layouts.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_live_saved_layouts.c`, `tests/unit/test_t7_37.c`, `tests/unit/cmake/eng/t7_37.cmake`, `quality/evidence/T7.37.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f8203edcc8863cd8
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.37 --regex '^test_t7_37$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.37`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.38: clang-analyzer-core.NullDereference in db_storage_policies.c
- **depends_on**: [T7, T13, T7.34]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_policies.c`, `tests/unit/test_t7_38.c`, `tests/unit/cmake/eng/t7_38.cmake`, `quality/evidence/T7.38.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-179513c629f1ecac
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.38 --regex '^test_t7_38$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.38`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.39: clang-analyzer-core.NullDereference in smartctl_provider.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/smartctl_provider.c`, `tests/unit/test_t7_39.c`, `tests/unit/cmake/eng/t7_39.cmake`, `quality/evidence/T7.39.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-33d1c32daba1778e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.39 --regex '^test_t7_39$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.39`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.40: clang-analyzer-core.NullDereference in go2rtc_api.c
- **depends_on**: [T7, T13, T7.17]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `tests/unit/test_t7_40.c`, `tests/unit/cmake/eng/t7_40.cmake`, `quality/evidence/T7.40.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-14fc6cb721e4ff46
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.40 --regex '^test_t7_40$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.40`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.41: clang-analyzer-core.NullDereference in hls_writer.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls_writer.c`, `tests/unit/test_t7_41.c`, `tests/unit/cmake/eng/t7_41.cmake`, `quality/evidence/T7.41.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-848024a1059f9105, CLANGTIDY-c18dbfa67bfd519e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.41 --regex '^test_t7_41$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.41`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.42: clang-analyzer-core.NullDereference in api_handlers_authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_authorization.c`, `tests/unit/test_t7_42.c`, `tests/unit/cmake/eng/t7_42.cmake`, `quality/evidence/T7.42.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3853c804fd823d76, CLANGTIDY-e903e0b58c1ab7fb, CLANGTIDY-7037bcf50d8dd6e1, CLANGTIDY-468c829b1cc867f5, CLANGTIDY-5546a3c4710ffc07, CLANGTIDY-9c42b9e1d1f45117, CLANGTIDY-b45db718e3e94b0d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.42 --regex '^test_t7_42$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.42`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.43: clang-analyzer-core.NullDereference in api_handlers_fleet.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_43.c`, `tests/unit/cmake/eng/t7_43.cmake`, `quality/evidence/T7.43.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-dc1d8989b6dc0f0a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.43 --regex '^test_t7_43$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.43`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.44: clang-analyzer-core.NullDereference in api_handlers_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_44.c`, `tests/unit/cmake/eng/t7_44.cmake`, `quality/evidence/T7.44.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18224b07c8f96008
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.44 --regex '^test_t7_44$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.44`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.45: clang-analyzer-core.NullDereference in api_handlers_lpr.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_lpr.c`, `tests/unit/test_t7_45.c`, `tests/unit/cmake/eng/t7_45.cmake`, `quality/evidence/T7.45.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-36ab8111fbec813b, CLANGTIDY-21f4fa94b0f2d17d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.45 --regex '^test_t7_45$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.45`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.46: clang-analyzer-core.NullDereference in api_handlers_recording_control.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recording_control.c`, `tests/unit/test_t7_46.c`, `tests/unit/cmake/eng/t7_46.cmake`, `quality/evidence/T7.46.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e722e2369c13444f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.46 --regex '^test_t7_46$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.46`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.47: clang-analyzer-core.NullDereference in api_handlers_retention.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_retention.c`, `tests/unit/test_t7_47.c`, `tests/unit/cmake/eng/t7_47.cmake`, `quality/evidence/T7.47.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-67d853afc9b7eefe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.47 --regex '^test_t7_47$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.47`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.48: clang-analyzer-core.NullDereference in api_handlers_storage_migrations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_migrations.c`, `tests/unit/test_t7_48.c`, `tests/unit/cmake/eng/t7_48.cmake`, `quality/evidence/T7.48.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-44c26e7cc4f14d25, CLANGTIDY-7f68b57abf95cdae
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.48 --regex '^test_t7_48$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.48`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.49: clang-analyzer-core.NullDereference in api_handlers_streams_get.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/unit/test_t7_49.c`, `tests/unit/cmake/eng/t7_49.cmake`, `quality/evidence/T7.49.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2b9b559954a51bc7
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.49 --regex '^test_t7_49$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.49`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.50: clang-analyzer-core.NullDereference in api_handlers_streams_modify.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/unit/test_t7_50.c`, `tests/unit/cmake/eng/t7_50.cmake`, `quality/evidence/T7.50.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3ac69a72123df761
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.50 --regex '^test_t7_50$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.50`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.51: clang-analyzer-core.NullDereference in api_handlers_system_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_health.c`, `tests/unit/test_t7_51.c`, `tests/unit/cmake/eng/t7_51.cmake`, `quality/evidence/T7.51.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2eb8f8220e64672b, CLANGTIDY-8935718b76906f1e, CLANGTIDY-db6d4840afaa43ce, CLANGTIDY-6253a319b7ed3b31
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.51 --regex '^test_t7_51$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.51`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.52: clang-analyzer-core.UndefinedBinaryOperatorResult in path_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_52.c`, `tests/unit/cmake/eng/t7_52.cmake`, `quality/evidence/T7.52.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d86514e7015dbd70
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.52 --regex '^test_t7_52$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.52`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.53: clang-analyzer-security.ArrayBound in authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_53.c`, `tests/unit/cmake/eng/t7_53.cmake`, `quality/evidence/T7.53.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c4536c7eb269c97d, CLANGTIDY-dc57edf3e0b50ae4
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.53 --regex '^test_t7_53$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.53`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.54: clang-analyzer-security.ArrayBound in logger_json.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/logger_json.c`, `tests/unit/test_t7_54.c`, `tests/unit/cmake/eng/t7_54.cmake`, `quality/evidence/T7.54.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ff30996a0be1b3ef
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.54 --regex '^test_t7_54$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.54`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.55: clang-analyzer-security.ArrayBound in main.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/main.c`, `tests/unit/test_t7_55.c`, `tests/unit/cmake/eng/t7_55.cmake`, `quality/evidence/T7.55.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-470e897ffa76864e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.55 --regex '^test_t7_55$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.55`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.56: clang-analyzer-security.ArrayBound in sqlite_migrate.c
- **depends_on**: [T7, T13, T7.23]
- **agent_type**: worker
- **ownership**: `src/database/sqlite_migrate.c`, `tests/unit/test_t7_56.c`, `tests/unit/cmake/eng/t7_56.cmake`, `quality/evidence/T7.56.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2772ec37ee463512
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.56 --regex '^test_t7_56$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.56`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.57: clang-analyzer-security.ArrayBound in linux_network.c, linux_process.c, linux_restart.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.22]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_restart.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_57.c`, `tests/unit/cmake/eng/t7_57.cmake`, `quality/evidence/T7.57.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9259d0adb44e3ab3, CLANGTIDY-55f597a830e65b4d, CLANGTIDY-eaf20c1f0de62870, CLANGTIDY-c05a923060c50809, CLANGTIDY-30cd36651cb697e6
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.57 --regex '^test_t7_57$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.57`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.58: clang-analyzer-security.ArrayBound in linux_hardware.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_58.c`, `tests/unit/cmake/eng/t7_58.cmake`, `quality/evidence/T7.58.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-29fe00c504db3f7e, CLANGTIDY-93f7df7451e39223
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.58 --regex '^test_t7_58$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.58`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.59: clang-analyzer-security.ArrayBound in system_health.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health.c`, `tests/unit/test_t7_59.c`, `tests/unit/cmake/eng/t7_59.cmake`, `quality/evidence/T7.59.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-192595ad5f7437f2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.59 --regex '^test_t7_59$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.59`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.60: clang-analyzer-security.ArrayBound in go2rtc_process.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_60.c`, `tests/unit/cmake/eng/t7_60.cmake`, `quality/evidence/T7.60.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6eb7be7d5d4be88a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.60 --regex '^test_t7_60$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.60`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.61: clang-analyzer-security.ArrayBound in hls_cache_cleanup.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_61.c`, `tests/unit/cmake/eng/t7_61.cmake`, `quality/evidence/T7.61.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e88c078f85b44a5e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.61 --regex '^test_t7_61$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.61`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.62: clang-analyzer-security.ArrayBound in stream_manager.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_manager.c`, `tests/unit/test_t7_62.c`, `tests/unit/cmake/eng/t7_62.cmake`, `quality/evidence/T7.62.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-87fe5c4f9b2c4103
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.62 --regex '^test_t7_62$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.62`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.63: clang-analyzer-security.ArrayBound in api_handlers_fleet.c
- **depends_on**: [T7, T13, T7.43]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_63.c`, `tests/unit/cmake/eng/t7_63.cmake`, `quality/evidence/T7.63.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b6500f12e4ee30b7, CLANGTIDY-a01b119b5b6a4463
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.63 --regex '^test_t7_63$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.63`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.64: clang-analyzer-security.ArrayBound in api_handlers_streams_get.c
- **depends_on**: [T7, T13, T7.49]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/unit/test_t7_64.c`, `tests/unit/cmake/eng/t7_64.cmake`, `quality/evidence/T7.64.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d562d8dbfbb1dcc3, CLANGTIDY-24b2873ab4fb113e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.64 --regex '^test_t7_64$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.64`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.65: clang-analyzer-security.ArrayBound in api_handlers_system.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system.c`, `tests/unit/test_t7_65.c`, `tests/unit/cmake/eng/t7_65.cmake`, `quality/evidence/T7.65.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4f6abbd42f9a1855, CLANGTIDY-c7df8f3cce68c5a4
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.65 --regex '^test_t7_65$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.65`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.66: clang-analyzer-security.insecureAPI.strcpy in recording_transcode.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/recording_transcode.c`, `tests/unit/test_t7_66.c`, `tests/unit/cmake/eng/t7_66.cmake`, `quality/evidence/T7.66.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d6eef8565b934dd8, CLANGTIDY-4976fee144691228
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.66 --regex '^test_t7_66$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.66`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.67: clang-analyzer-security.insecureAPI.strcpy in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_67.c`, `tests/unit/cmake/eng/t7_67.cmake`, `quality/evidence/T7.67.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4c710ebaa0f3ef18, CLANGTIDY-e7f2b500346b1db4, CLANGTIDY-59d7e244d1294daa, CLANGTIDY-de5c9750f9e973f2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.67 --regex '^test_t7_67$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.67`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.68: clang-analyzer-unix.BlockInCriticalSection in health_helper_runner.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/health_helper_runner.c`, `tests/unit/test_t7_68.c`, `tests/unit/cmake/eng/t7_68.cmake`, `quality/evidence/T7.68.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8a0289975bb00ddb
- **acceptance_criteria**:
  - Exercise the blocking operation and concurrent shutdown/contending thread with bounded deadlines; assert no lock is held across unbounded I/O and lock-protected invariants remain intact.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.68 --regex '^test_t7_68$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.68`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.69: clang-analyzer-unix.BlockInCriticalSection in hls_unified_thread.c
- **depends_on**: [T7, T13, T7.14]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_69.c`, `tests/unit/cmake/eng/t7_69.cmake`, `quality/evidence/T7.69.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1728287dd1cfa5a4
- **acceptance_criteria**:
  - Exercise the blocking operation and concurrent shutdown/contending thread with bounded deadlines; assert no lock is held across unbounded I/O and lock-protected invariants remain intact.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.69 --regex '^test_t7_69$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.69`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.70: clang-analyzer-unix.Errno in storage_migration.c
- **depends_on**: [T7, T13, T7.26]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_70.c`, `tests/unit/cmake/eng/t7_70.cmake`, `quality/evidence/T7.70.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f15ef4cfb6675401, CLANGTIDY-d8903761e409b32d
- **acceptance_criteria**:
  - Inject failing and successful calls with stale errno; branch only on actual call failure and preserve the original error before cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.70 --regex '^test_t7_70$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.70`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.71: clang-analyzer-unix.StdCLibraryFunctions in hls_cache_cleanup.c
- **depends_on**: [T7, T13, T7.61]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_71.c`, `tests/unit/cmake/eng/t7_71.cmake`, `quality/evidence/T7.71.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f70e12d3b12447f9, CLANGTIDY-81dd4bf2eb1bd4a8
- **acceptance_criteria**:
  - Exercise every reported format/library call with boundary values; argument types/counts, buffer extents and returned errors must match the library contract with no UB.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.71 --regex '^test_t7_71$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.71`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.72: clang-analyzer-unix.Stream in linux_network.c
- **depends_on**: [T7, T13, T7.57]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_72.c`, `tests/unit/cmake/eng/t7_72.cmake`, `quality/evidence/T7.72.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-b38fd5e59ea07d0e, CLANGTIDY-ebeb3bd843bc6421
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.72 --regex '^test_t7_72$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.72`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.73: integerOverflow in config.c
- **depends_on**: [T7, T13, T7.13]
- **agent_type**: worker
- **ownership**: `src/core/config.c`, `tests/unit/test_t7_73.c`, `tests/unit/cmake/eng/t7_73.cmake`, `quality/evidence/T7.73.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-a8a2fee09e04d7a2
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.73 --regex '^test_t7_73$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.73`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.74: memleak in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_74-upstream.json`, `quality/evidence/T7.74.md`.
- **priority**: medium
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-582ed0e8d5a10fc1
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_74-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.74`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.75: memleak in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_75-upstream.json`, `quality/evidence/T7.75.md`.
- **priority**: medium
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-d14ab8cc9d321477, CPPCHECK-7bc58272fe78e01c, CPPCHECK-d1406d76ca075628, CPPCHECK-187d49bac01afe0b, CPPCHECK-01d35c4f8b07a420, CPPCHECK-7ed0f3ac85abfc0f, CPPCHECK-afd0bb6171ab9102, CPPCHECK-6c275f8832cf9762, CPPCHECK-340c5f9dec875049, CPPCHECK-94d7d3c523b01523, CPPCHECK-65f97e38c1e1c5df, CPPCHECK-377feb103d4c3b66
- **acceptance_criteria**:
  - Inject each allocation/open and downstream failure at the reported site; assert allocations, FILE handles and descriptors return to baseline on every exit and successful ownership transfers remain valid.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_75-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.75`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.76: preprocessorErrorDirective in nvme_provider.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_76.c`, `tests/unit/cmake/eng/t7_76.cmake`, `quality/evidence/T7.76.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1c7b293c20ba8b3d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.76 --regex '^test_t7_76$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.76`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.77: preprocessorErrorDirective in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.16]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_77.c`, `tests/unit/cmake/eng/t7_77.cmake`, `quality/evidence/T7.77.md`.
- **priority**: medium
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1afeb1315d85ff61
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.77 --regex '^test_t7_77$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.77`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.78: arrayIndexOutOfBoundsCond in stream_protocol.c
- **depends_on**: [T7, T13, T7.30]
- **agent_type**: worker
- **ownership**: `src/video/stream_protocol.c`, `tests/unit/test_t7_78.c`, `tests/unit/cmake/eng/t7_78.cmake`, `quality/evidence/T7.78.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-1d3bfb4e5fc184fe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.78 --regex '^test_t7_78$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.78`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.79: bugprone-assignment-in-if-condition in db_system_health_incidents.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_system_health_incidents.c`, `tests/unit/test_t7_79.c`, `tests/unit/cmake/eng/t7_79.cmake`, `quality/evidence/T7.79.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-808f26556ac98110, CLANGTIDY-7e1ce15fe947a21d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.79 --regex '^test_t7_79$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.79`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.80: bugprone-assignment-in-if-condition in api_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_80.c`, `tests/unit/cmake/eng/t7_80.cmake`, `quality/evidence/T7.80.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c34509f97f5eb3bf, CLANGTIDY-6ac00b1420332d40, CLANGTIDY-1848d1ca57216f44, CLANGTIDY-be52eb80a1a9b4e2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.80 --regex '^test_t7_80$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.80`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.81: bugprone-assignment-in-if-condition in api_handlers_metrics.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_metrics.c`, `tests/unit/test_t7_81.c`, `tests/unit/cmake/eng/t7_81.cmake`, `quality/evidence/T7.81.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-02365646d53a2c50, CLANGTIDY-a219d74f760de10e, CLANGTIDY-96f8eef93ec11127, CLANGTIDY-221d97bd8b8ca157, CLANGTIDY-b6b7c0de32a0c6b7, CLANGTIDY-4edbb01e2c9075ed, CLANGTIDY-ec9cd5d6ab7a3d6e, CLANGTIDY-2f30db61a3888f56
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.81 --regex '^test_t7_81$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.81`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.82: bugprone-assignment-in-if-condition in go2rtc_websocket.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_82.c`, `tests/unit/cmake/eng/t7_82.cmake`, `quality/evidence/T7.82.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-82315a3f137f4aff
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.82 --regex '^test_t7_82$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.82`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.83: bugprone-branch-clone in linux_cgroup.c
- **depends_on**: [T7, T13, T7.22]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `tests/unit/test_t7_83.c`, `tests/unit/cmake/eng/t7_83.cmake`, `quality/evidence/T7.83.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eac1097bd3780c16
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.83 --regex '^test_t7_83$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.83`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.84: bugprone-branch-clone in api_handlers_authorization.c
- **depends_on**: [T7, T13, T7.42]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_authorization.c`, `tests/unit/test_t7_84.c`, `tests/unit/cmake/eng/t7_84.cmake`, `quality/evidence/T7.84.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a03bd79d6040e625
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.84 --regex '^test_t7_84$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.84`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.85: bugprone-branch-clone in api_handlers_timeline.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_85.c`, `tests/unit/cmake/eng/t7_85.cmake`, `quality/evidence/T7.85.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-f1f70f55c95dfbaf
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.85 --regex '^test_t7_85$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.85`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.86: bugprone-branch-clone in audit_log.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/audit_log.c`, `tests/unit/test_t7_86.c`, `tests/unit/cmake/eng/t7_86.cmake`, `quality/evidence/T7.86.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5bdf00371aedee4a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.86 --regex '^test_t7_86$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.86`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.87: bugprone-branch-clone in thumbnail_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/thumbnail_thread.c`, `tests/unit/test_t7_87.c`, `tests/unit/cmake/eng/t7_87.cmake`, `quality/evidence/T7.87.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ccff9a5b097b862e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.87 --regex '^test_t7_87$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.87`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.88: bugprone-implicit-widening-of-multiplication-result in event_bus.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/event_bus.c`, `tests/unit/test_t7_88.c`, `tests/unit/cmake/eng/t7_88.cmake`, `quality/evidence/T7.88.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-73322317a31f8bcc, CLANGTIDY-dbe8bf00e4b146ee
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.88 --regex '^test_t7_88$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.88`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.89: bugprone-implicit-widening-of-multiplication-result in event_envelope.c
- **depends_on**: [T7, T13, T7.36]
- **agent_type**: worker
- **ownership**: `src/core/event_envelope.c`, `tests/unit/test_t7_89.c`, `tests/unit/cmake/eng/t7_89.cmake`, `quality/evidence/T7.89.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-2fcad01867c4b146, CLANGTIDY-51ab98673b990b2c, CLANGTIDY-a48d8d980b8dbfcc
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.89 --regex '^test_t7_89$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.89`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.90: bugprone-implicit-widening-of-multiplication-result in event_router.c
- **depends_on**: [T7, T13, T7.28]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_90.c`, `tests/unit/cmake/eng/t7_90.cmake`, `quality/evidence/T7.90.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-767515747827ef14
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.90 --regex '^test_t7_90$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.90`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.91: bugprone-implicit-widening-of-multiplication-result in mqtt_delivery_worker.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_delivery_worker.c`, `tests/unit/test_t7_91.c`, `tests/unit/cmake/eng/t7_91.cmake`, `quality/evidence/T7.91.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ced78137a6871fda
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.91 --regex '^test_t7_91$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.91`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.92: bugprone-implicit-widening-of-multiplication-result in db_operator_floor_plans.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_operator_floor_plans.c`, `tests/unit/test_t7_92.c`, `tests/unit/cmake/eng/t7_92.cmake`, `quality/evidence/T7.92.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4ce72e7a43d0bcce
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.92 --regex '^test_t7_92$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.92`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.93: bugprone-implicit-widening-of-multiplication-result in storage_migration.c
- **depends_on**: [T7, T13, T7.70]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_93.c`, `tests/unit/cmake/eng/t7_93.cmake`, `quality/evidence/T7.93.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-50703d3018f9bed5, CLANGTIDY-15fa53ad8b574808, CLANGTIDY-e5960a2d15356c27, CLANGTIDY-1fac122c6010eb36
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.93 --regex '^test_t7_93$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.93`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.94: bugprone-implicit-widening-of-multiplication-result in storage_s3.c
- **depends_on**: [T7, T13, T7.35]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_94.c`, `tests/unit/cmake/eng/t7_94.cmake`, `quality/evidence/T7.94.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-612c846365824225, CLANGTIDY-1e0ea76108754cd5, CLANGTIDY-ab4dc4b3768799ca, CLANGTIDY-14f55668eaa243b0, CLANGTIDY-fbf040917d8b6cf7, CLANGTIDY-52dada64b4a6e625, CLANGTIDY-d1d1cf8dcb7070f8, CLANGTIDY-bfb7d2825452ce0c, CLANGTIDY-842c6bb171a64e75, CLANGTIDY-4a2a60d787933d7d, CLANGTIDY-a145bd05b0189193, CLANGTIDY-f72760acc6c09e16, CLANGTIDY-0866b8e57f533141, CLANGTIDY-d08422316656b39c, CLANGTIDY-d7c0ab1bf29a42a3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.94 --regex '^test_t7_94$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.94`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.95: bugprone-implicit-widening-of-multiplication-result in linux_network.c, linux_process.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.72, T7.57]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_95.c`, `tests/unit/cmake/eng/t7_95.cmake`, `quality/evidence/T7.95.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8d33cee7806f5d43, CLANGTIDY-02b18b5e0f4e3b93, CLANGTIDY-b84c577b6e369ac3, CLANGTIDY-8b66085f0250d8e1, CLANGTIDY-15838d8765a7623e, CLANGTIDY-df8a271dac7dc00c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.95 --regex '^test_t7_95$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.95`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.96: bugprone-implicit-widening-of-multiplication-result in ffmpeg_rss_guard.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/tools/ffmpeg_rss_guard.c`, `tests/unit/test_t7_96.c`, `tests/unit/cmake/eng/t7_96.cmake`, `quality/evidence/T7.96.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-802a0570e39a96a6, CLANGTIDY-a072eb98578bd9cf
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.96 --regex '^test_t7_96$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.96`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.97: bugprone-implicit-widening-of-multiplication-result in lpr_crypto.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/lpr_crypto.c`, `tests/unit/test_t7_97.c`, `tests/unit/cmake/eng/t7_97.cmake`, `quality/evidence/T7.97.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-576d7acbaee5cef6
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.97 --regex '^test_t7_97$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.97`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.98: bugprone-implicit-widening-of-multiplication-result in go2rtc_api.c, go2rtc_consumer.c, go2rtc_integration.c, go2rtc_snapshot.c, go2rtc_stream.c
- **depends_on**: [T7, T13, T7.40]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `src/video/go2rtc/go2rtc_consumer.c`, `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_snapshot.c`, `src/video/go2rtc/go2rtc_stream.c`, `tests/unit/test_t7_98.c`, `tests/unit/cmake/eng/t7_98.cmake`, `quality/evidence/T7.98.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eadc49a35e808a77, CLANGTIDY-bdcbae59a275ce50, CLANGTIDY-5582e7df3dd25730, CLANGTIDY-23826be3117b0a5e, CLANGTIDY-cac166160ef5ab94, CLANGTIDY-6eced00c619f459a, CLANGTIDY-a35257b195053d18, CLANGTIDY-9fbcc527095ebbbb, CLANGTIDY-c263543788ddaa36, CLANGTIDY-d06458b2bdfca56d, CLANGTIDY-1592d34d4e3dc54c, CLANGTIDY-957e3e8e5e143726, CLANGTIDY-76ebb6a603073b54, CLANGTIDY-2c1116134a2ee5a3, CLANGTIDY-0ce2e7833fc885fd, CLANGTIDY-4e94aa9ff957dec6, CLANGTIDY-f6ddc3b14e23e020
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.98 --regex '^test_t7_98$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.98`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.99: bugprone-implicit-widening-of-multiplication-result in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.77]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_99.c`, `tests/unit/cmake/eng/t7_99.cmake`, `quality/evidence/T7.99.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a19387fbb98806ae
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.99 --regex '^test_t7_99$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.99`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.100: bugprone-implicit-widening-of-multiplication-result in onvif_detection.c
- **depends_on**: [T7, T13, T7.18]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_100.c`, `tests/unit/cmake/eng/t7_100.cmake`, `quality/evidence/T7.100.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18362aef7fade326, CLANGTIDY-eade1f2d38582a47, CLANGTIDY-8b074e54e83681ff
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.100 --regex '^test_t7_100$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.100`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.101: bugprone-implicit-widening-of-multiplication-result in onvif_event.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_event.c`, `tests/unit/test_t7_101.c`, `tests/unit/cmake/eng/t7_101.cmake`, `quality/evidence/T7.101.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-eb4e649dccdb530f
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.101 --regex '^test_t7_101$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.101`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.102: bugprone-implicit-widening-of-multiplication-result in api_handlers_investigations.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_102.c`, `tests/unit/cmake/eng/t7_102.cmake`, `quality/evidence/T7.102.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6829e02764d9d278, CLANGTIDY-113735d7300e4f01, CLANGTIDY-25b00beae5a3f4a8, CLANGTIDY-2387b22f15e219d3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.102 --regex '^test_t7_102$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.102`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.103: bugprone-implicit-widening-of-multiplication-result in api_handlers_lpr.c
- **depends_on**: [T7, T13, T7.45]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_lpr.c`, `tests/unit/test_t7_103.c`, `tests/unit/cmake/eng/t7_103.cmake`, `quality/evidence/T7.103.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0818b8d62b6628c5
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.103 --regex '^test_t7_103$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.103`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.104: bugprone-implicit-widening-of-multiplication-result in api_handlers_operator_floor_plans.c
- **depends_on**: [T7, T13, T7.27]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_operator_floor_plans.c`, `tests/unit/test_t7_104.c`, `tests/unit/cmake/eng/t7_104.cmake`, `quality/evidence/T7.104.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-53e88d8983baf290, CLANGTIDY-dbd6a5de3a7e191c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.104 --regex '^test_t7_104$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.104`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.105: bugprone-implicit-widening-of-multiplication-result in api_handlers_storage.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage.c`, `tests/unit/test_t7_105.c`, `tests/unit/cmake/eng/t7_105.cmake`, `quality/evidence/T7.105.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-88f84938e1b8feb0
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.105 --regex '^test_t7_105$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.105`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.106: bugprone-implicit-widening-of-multiplication-result in api_handlers_system_go2rtc.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_go2rtc.c`, `tests/unit/test_t7_106.c`, `tests/unit/cmake/eng/t7_106.cmake`, `quality/evidence/T7.106.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-101a7d6e0cfda715, CLANGTIDY-c586f58fa85c527e
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.106 --regex '^test_t7_106$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.106`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.107: bugprone-implicit-widening-of-multiplication-result in api_handlers_timeline.c
- **depends_on**: [T7, T13, T7.85]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_107.c`, `tests/unit/cmake/eng/t7_107.cmake`, `quality/evidence/T7.107.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-18895086ef6f0ed9
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.107 --regex '^test_t7_107$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.107`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.108: bugprone-implicit-widening-of-multiplication-result in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.82]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_108.c`, `tests/unit/cmake/eng/t7_108.cmake`, `quality/evidence/T7.108.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6e45cad03ed5a9f3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.108 --regex '^test_t7_108$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.108`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.109: bugprone-implicit-widening-of-multiplication-result in recording_archive.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_109.c`, `tests/unit/cmake/eng/t7_109.cmake`, `quality/evidence/T7.109.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5bcac8af8164c1b0, CLANGTIDY-e2b358165654b79e
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.109 --regex '^test_t7_109$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.109`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.110: bugprone-inc-dec-in-conditions in storage_source.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_110.c`, `tests/unit/cmake/eng/t7_110.cmake`, `quality/evidence/T7.110.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-9613b39c2e0a1e4c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.110 --regex '^test_t7_110$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.110`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.111: bugprone-macro-parentheses in cJSON.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_111-upstream.json`, `quality/evidence/T7.111.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CLANGTIDY-5f6ae30cde3aaf3d, CLANGTIDY-bc48f96894493869, CLANGTIDY-01e4c541adfa23e3, CLANGTIDY-3abdad2ed8a71fe5, CLANGTIDY-09829500c354c2b5, CLANGTIDY-661c1d9d010f4873, CLANGTIDY-f69ef126d8e55b2b, CLANGTIDY-6d1385324d08479d
- **acceptance_criteria**:
  - Compile/evaluate precedence-sensitive macro arguments and side effects; results must match the intended fully parenthesized expression without double evaluation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_111-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.111`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.112: bugprone-macro-parentheses in api_detection.c
- **depends_on**: [T7, T13, T7.80]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_112.c`, `tests/unit/cmake/eng/t7_112.cmake`, `quality/evidence/T7.112.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ae25a6959341dd10
- **acceptance_criteria**:
  - Compile/evaluate precedence-sensitive macro arguments and side effects; results must match the intended fully parenthesized expression without double evaluation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.112 --regex '^test_t7_112$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.112`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.113: bugprone-misplaced-widening-cast in db_fleet_query.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_fleet_query.c`, `tests/unit/test_t7_113.c`, `tests/unit/cmake/eng/t7_113.cmake`, `quality/evidence/T7.113.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-7bff304a77769e6c
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.113 --regex '^test_t7_113$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.113`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.114: bugprone-multi-level-implicit-pointer-conversion in authorization.c
- **depends_on**: [T7, T13, T7.53]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_114.c`, `tests/unit/cmake/eng/t7_114.cmake`, `quality/evidence/T7.114.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-4505cfde8bb2e42a, CLANGTIDY-ff0e443bb0b29339
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.114 --regex '^test_t7_114$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.114`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.115: bugprone-multi-level-implicit-pointer-conversion in camera_collection_filter.c
- **depends_on**: [T7, T13, T7.33]
- **agent_type**: worker
- **ownership**: `src/core/camera_collection_filter.c`, `tests/unit/test_t7_115.c`, `tests/unit/cmake/eng/t7_115.cmake`, `quality/evidence/T7.115.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3a7569f24a5603a4, CLANGTIDY-54a9eba3b05f0872, CLANGTIDY-a221992baafec96f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.115 --regex '^test_t7_115$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.115`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.116: bugprone-multi-level-implicit-pointer-conversion in camera_selector.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/camera_selector.c`, `tests/unit/test_t7_116.c`, `tests/unit/cmake/eng/t7_116.cmake`, `quality/evidence/T7.116.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-8e1debc0e42f328a, CLANGTIDY-1fb6fbd990b73e55, CLANGTIDY-52e7ee5b7b20725e, CLANGTIDY-a816d3388754e9a3, CLANGTIDY-bdf2049affa5a3fd, CLANGTIDY-946781b07a8a19f6
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.116 --regex '^test_t7_116$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.116`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.117: bugprone-multi-level-implicit-pointer-conversion in db_storage_lifecycle.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_117.c`, `tests/unit/cmake/eng/t7_117.cmake`, `quality/evidence/T7.117.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-39f0ad6017069f02, CLANGTIDY-c666354bdbc1df4b, CLANGTIDY-c8b906651c5b2c7f, CLANGTIDY-8a2d068dc3e8b0c0, CLANGTIDY-ad7afdbd70ca0a46
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.117 --regex '^test_t7_117$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.117`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.118: bugprone-multi-level-implicit-pointer-conversion in hls_cache_cleanup.c
- **depends_on**: [T7, T13, T7.71]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_cache_cleanup.c`, `tests/unit/test_t7_118.c`, `tests/unit/cmake/eng/t7_118.cmake`, `quality/evidence/T7.118.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-48d30c8fb35cc9cd, CLANGTIDY-1634947d8289644f, CLANGTIDY-cb9a5afe872113f7, CLANGTIDY-098d117140abdf3a, CLANGTIDY-a89af13dd80ae187
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.118 --regex '^test_t7_118$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.118`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.119: bugprone-multi-level-implicit-pointer-conversion in mp4_segment_recorder.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_segment_recorder.c`, `tests/unit/test_t7_119.c`, `tests/unit/cmake/eng/t7_119.cmake`, `quality/evidence/T7.119.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-30c3f7edea8fbd8d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.119 --regex '^test_t7_119$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.119`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.120: bugprone-multi-level-implicit-pointer-conversion in mp4_writer_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_core.c`, `tests/unit/test_t7_120.c`, `tests/unit/cmake/eng/t7_120.cmake`, `quality/evidence/T7.120.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d421e791a6b63cc9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.120 --regex '^test_t7_120$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.120`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.121: bugprone-multi-level-implicit-pointer-conversion in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.99]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_121.c`, `tests/unit/cmake/eng/t7_121.cmake`, `quality/evidence/T7.121.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c536433a0089e4b9, CLANGTIDY-4e34d6b524364f03
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.121 --regex '^test_t7_121$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.121`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.122: bugprone-multi-level-implicit-pointer-conversion in unified_detection_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_122.c`, `tests/unit/cmake/eng/t7_122.cmake`, `quality/evidence/T7.122.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5dc4623d12ccf5b2
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.122 --regex '^test_t7_122$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.122`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.123: bugprone-multi-level-implicit-pointer-conversion in api_handlers_camera_collections.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_camera_collections.c`, `tests/unit/test_t7_123.c`, `tests/unit/cmake/eng/t7_123.cmake`, `quality/evidence/T7.123.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1f9c86e17ca383f9, CLANGTIDY-316e32d23b15b780, CLANGTIDY-7464eb7864628fd1, CLANGTIDY-5e2b3b364d163173, CLANGTIDY-81fbfb962103b645, CLANGTIDY-c41e983019497e88, CLANGTIDY-308b0349ef001b56, CLANGTIDY-7a3f68ca1e19c9db, CLANGTIDY-475d0f82a4e9b764, CLANGTIDY-2276a7bff678cf41, CLANGTIDY-63c736995704d2d6, CLANGTIDY-f5ac788b77f1417a, CLANGTIDY-ae6a96ea4d41087d, CLANGTIDY-f617ccfb4cfc3cc2, CLANGTIDY-169001b0559c7bcb, CLANGTIDY-6bce10f282924906
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.123 --regex '^test_t7_123$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.123`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.124: bugprone-multi-level-implicit-pointer-conversion in api_handlers_fleet.c
- **depends_on**: [T7, T13, T7.63]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_fleet.c`, `tests/unit/test_t7_124.c`, `tests/unit/cmake/eng/t7_124.cmake`, `quality/evidence/T7.124.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0b0bcefaf7a0b24f, CLANGTIDY-d8c6998c9398adf1, CLANGTIDY-79ebe40a23b7323a, CLANGTIDY-6da59a8629d16699, CLANGTIDY-773e04e041de5410, CLANGTIDY-f12d6e4a615966e9, CLANGTIDY-ede4da279d79ec69, CLANGTIDY-36c8164a04c2420c, CLANGTIDY-4053884d5188ab2f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.124 --regex '^test_t7_124$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.124`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.125: bugprone-multi-level-implicit-pointer-conversion in api_handlers_retention.c
- **depends_on**: [T7, T13, T7.47]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_retention.c`, `tests/unit/test_t7_125.c`, `tests/unit/cmake/eng/t7_125.cmake`, `quality/evidence/T7.125.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0cc9a9ea64babfdc, CLANGTIDY-234cb88ea5651731
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.125 --regex '^test_t7_125$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.125`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.126: bugprone-multi-level-implicit-pointer-conversion in api_handlers_storage_policies.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_policies.c`, `tests/unit/test_t7_126.c`, `tests/unit/cmake/eng/t7_126.cmake`, `quality/evidence/T7.126.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-312e6eafc5ebdca6, CLANGTIDY-1985156ff8e5c87d, CLANGTIDY-ca7e963b9f3e3214, CLANGTIDY-aede9d1024b66426, CLANGTIDY-07c63776b48ad48f, CLANGTIDY-ea16598ca26b9108
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.126 --regex '^test_t7_126$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.126`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.127: bugprone-narrowing-conversions in api_timing.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/web/api_timing.h`, `tests/unit/test_t7_127.c`, `tests/unit/cmake/eng/t7_127.cmake`, `quality/evidence/T7.127.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ca2f4806dea5f177, CLANGTIDY-054f4f2a2583f814
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.127 --regex '^test_t7_127$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.127`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.128: bugprone-narrowing-conversions in path_utils.c
- **depends_on**: [T7, T13, T7.52]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_128.c`, `tests/unit/cmake/eng/t7_128.cmake`, `quality/evidence/T7.128.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-dda225859c807d72
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.128 --regex '^test_t7_128$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.128`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.129: bugprone-narrowing-conversions in db_storage_lifecycle.c
- **depends_on**: [T7, T13, T7.117]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_129.c`, `tests/unit/cmake/eng/t7_129.cmake`, `quality/evidence/T7.129.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-76e54a4e3ac5efb3, CLANGTIDY-84fbbd387f95947a
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.129 --regex '^test_t7_129$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.129`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.130: bugprone-narrowing-conversions in mp4_writer_utils.c
- **depends_on**: [T7, T13, T7.121]
- **agent_type**: worker
- **ownership**: `src/video/mp4_writer_utils.c`, `tests/unit/test_t7_130.c`, `tests/unit/cmake/eng/t7_130.cmake`, `quality/evidence/T7.130.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c74d7d04edeb4ee2, CLANGTIDY-cb47dbd2bd0937a1, CLANGTIDY-6f7bee0a60d5152f
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.130 --regex '^test_t7_130$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.130`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.131: bugprone-narrowing-conversions in stream_frame_rate.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_frame_rate.c`, `tests/unit/test_t7_131.c`, `tests/unit/cmake/eng/t7_131.cmake`, `quality/evidence/T7.131.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e1b115ce18cb0926, CLANGTIDY-9bb912a1e3d2bf88
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.131 --regex '^test_t7_131$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.131`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.132: bugprone-narrowing-conversions in api_handlers_storage_compliance.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_storage_compliance.c`, `tests/unit/test_t7_132.c`, `tests/unit/cmake/eng/t7_132.cmake`, `quality/evidence/T7.132.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6a756d858b2a4590, CLANGTIDY-0f6724ca27b42ed9, CLANGTIDY-ec3e72154d735330, CLANGTIDY-e85c88c08460d2ef, CLANGTIDY-9c90ce2f531301e0, CLANGTIDY-3e6ff569aac7699e, CLANGTIDY-ca4b8a2f2a2f60bf, CLANGTIDY-c990523bb862f2b6, CLANGTIDY-7a5a584e25342a04, CLANGTIDY-8d9dd908cb4d8452
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.132 --regex '^test_t7_132$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.132`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.133: bugprone-narrowing-conversions in httpd_utils.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/httpd_utils.c`, `tests/unit/test_t7_133.c`, `tests/unit/cmake/eng/t7_133.cmake`, `quality/evidence/T7.133.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cb7511d8b0a3b651
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.133 --regex '^test_t7_133$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.133`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.134: bugprone-not-null-terminated-result in mqtt_destination_client.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_134.c`, `tests/unit/cmake/eng/t7_134.cmake`, `quality/evidence/T7.134.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d7edea27d80824ed
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.134 --regex '^test_t7_134$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.134`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.135: bugprone-not-null-terminated-result in linux_hardware.c
- **depends_on**: [T7, T13, T7.58]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_135.c`, `tests/unit/cmake/eng/t7_135.cmake`, `quality/evidence/T7.135.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-65ab86fae5fd2213
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.135 --regex '^test_t7_135$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.135`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.136: bugprone-suspicious-string-compare in db_storage_targets.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_136.c`, `tests/unit/cmake/eng/t7_136.cmake`, `quality/evidence/T7.136.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-709dbb22240f409f, CLANGTIDY-d68060f1555ca324, CLANGTIDY-b922ced0e34aacea, CLANGTIDY-295d32efbfee9145
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.136 --regex '^test_t7_136$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.136`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.137: bugprone-suspicious-string-compare in storage_deletion.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_137.c`, `tests/unit/cmake/eng/t7_137.cmake`, `quality/evidence/T7.137.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-56955e0bf792f087
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.137 --regex '^test_t7_137$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.137`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.138: bugprone-suspicious-string-compare in storage_migration.c
- **depends_on**: [T7, T13, T7.93]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_138.c`, `tests/unit/cmake/eng/t7_138.cmake`, `quality/evidence/T7.138.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-d6379e87cfa699fa
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.138 --regex '^test_t7_138$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.138`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.139: bugprone-suspicious-string-compare in storage_s3.c
- **depends_on**: [T7, T13, T7.94]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_139.c`, `tests/unit/cmake/eng/t7_139.cmake`, `quality/evidence/T7.139.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-789d9f9254afa1b1, CLANGTIDY-2f98dec54b741594, CLANGTIDY-852f97f8aacc5432, CLANGTIDY-ba57287cdb3eb096, CLANGTIDY-bfbbf95e2bedfe38, CLANGTIDY-65cc9415acf1be4d, CLANGTIDY-5561b78de6be772b, CLANGTIDY-6b719694dd2640c7
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.139 --regex '^test_t7_139$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.139`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.140: bugprone-suspicious-string-compare in storage_source.c
- **depends_on**: [T7, T13, T7.110]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_140.c`, `tests/unit/cmake/eng/t7_140.cmake`, `quality/evidence/T7.140.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-6a07b625f331b20e
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.140 --regex '^test_t7_140$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.140`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.141: bugprone-suspicious-string-compare in recording_archive.c
- **depends_on**: [T7, T13, T7.109]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_141.c`, `tests/unit/cmake/eng/t7_141.cmake`, `quality/evidence/T7.141.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-adf6edf7670e9010
- **acceptance_criteria**:
  - Use equal, unequal, prefix-only and empty strings to prove the branch implements the intended comparison rather than pointer/boolean equality.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.141 --regex '^test_t7_141$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.141`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.142: bugprone-unsafe-functions,cert-msc24-c,cert-msc33-c in storage_s3.c
- **depends_on**: [T7, T13, T7.139]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_142.c`, `tests/unit/cmake/eng/t7_142.cmake`, `quality/evidence/T7.142.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-3a081de1dd30ebb2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.142 --regex '^test_t7_142$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.142`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.143: cert-err34-c in linux_device_map.c, linux_hardware.c
- **depends_on**: [T7, T13, T7.135]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_device_map.c`, `src/telemetry/providers/linux_hardware.c`, `tests/unit/test_t7_143.c`, `tests/unit/cmake/eng/t7_143.cmake`, `quality/evidence/T7.143.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-c52ffccc8c0fb590, CLANGTIDY-811ab08155776051
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.143 --regex '^test_t7_143$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.143`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.144: cert-err34-c in ffmpeg_rss_guard.c
- **depends_on**: [T7, T13, T7.96]
- **agent_type**: worker
- **ownership**: `src/tools/ffmpeg_rss_guard.c`, `tests/unit/test_t7_144.c`, `tests/unit/cmake/eng/t7_144.cmake`, `quality/evidence/T7.144.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-11d1a497d98b8490
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.144 --regex '^test_t7_144$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.144`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.145: cert-err34-c in go2rtc_integration.c, go2rtc_process.c
- **depends_on**: [T7, T13, T7.98, T7.60]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_145.c`, `tests/unit/cmake/eng/t7_145.cmake`, `quality/evidence/T7.145.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a7a71d348e2c49f5, CLANGTIDY-41876b44c3442c4b, CLANGTIDY-3a3cc8c9282e0fb5, CLANGTIDY-43d0d5170d6a8434
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.145 --regex '^test_t7_145$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.145`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.146: cert-err34-c in onvif_detection.c
- **depends_on**: [T7, T13, T7.100]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_146.c`, `tests/unit/cmake/eng/t7_146.cmake`, `quality/evidence/T7.146.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-a6e1105b549e0cf0
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.146 --regex '^test_t7_146$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.146`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.147: cert-err34-c in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.67]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_147.c`, `tests/unit/cmake/eng/t7_147.cmake`, `quality/evidence/T7.147.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-ab5e571ba64af645
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.147 --regex '^test_t7_147$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.147`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.148: cert-err34-c in api_handlers_investigations.c
- **depends_on**: [T7, T13, T7.102]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_148.c`, `tests/unit/cmake/eng/t7_148.cmake`, `quality/evidence/T7.148.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-e5ec24184e5b758c
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.148 --regex '^test_t7_148$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.148`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.149: cert-err34-c in api_handlers_system_logs.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_logs.c`, `tests/unit/test_t7_149.c`, `tests/unit/cmake/eng/t7_149.cmake`, `quality/evidence/T7.149.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-0fda904c59353043
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.149 --regex '^test_t7_149$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.149`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.150: cert-err34-c in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.108]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_150.c`, `tests/unit/cmake/eng/t7_150.cmake`, `quality/evidence/T7.150.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-5c42b38e8a9c25ff
- **acceptance_criteria**:
  - Exercise empty, trailing-junk, negative, out-of-range and valid numeric strings; conversion must check end pointer and range before applying configuration.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.150 --regex '^test_t7_150$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.150`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.151: cert-int09-c in authorization.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/core/authorization.h`, `tests/unit/test_t7_151.c`, `tests/unit/cmake/eng/t7_151.cmake`, `quality/evidence/T7.151.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-1539a5c009afd0df
- **acceptance_criteria**:
  - Verify enum constants remain representable and unique on the supported ABI without changing public numeric values.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.151 --regex '^test_t7_151$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.151`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.152: cert-int09-c in faac.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_152-upstream.json`, `quality/evidence/T7.152.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CLANGTIDY-b6947bb24c18c8ac, CLANGTIDY-ddf685d0be472b86, CLANGTIDY-296d5f7499179bc0, CLANGTIDY-4c6957160cb5ae0c
- **acceptance_criteria**:
  - Verify enum constants remain representable and unique on the supported ABI without changing public numeric values.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_152-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.152`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.153: clang-analyzer-deadcode.DeadStores in db_system_health_incidents.c
- **depends_on**: [T7, T13, T7.79]
- **agent_type**: worker
- **ownership**: `src/database/db_system_health_incidents.c`, `tests/unit/test_t7_153.c`, `tests/unit/cmake/eng/t7_153.cmake`, `quality/evidence/T7.153.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-fe8af642c27ce051
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.153 --regex '^test_t7_153$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.153`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.154: clang-analyzer-deadcode.DeadStores in go2rtc_integration.c
- **depends_on**: [T7, T13, T7.145]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `tests/unit/test_t7_154.c`, `tests/unit/cmake/eng/t7_154.cmake`, `quality/evidence/T7.154.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-88fbdf8930f67549
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.154 --regex '^test_t7_154$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.154`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.155: clang-analyzer-deadcode.DeadStores in mp4_segment_recorder.c
- **depends_on**: [T7, T13, T7.119]
- **agent_type**: worker
- **ownership**: `src/video/mp4_segment_recorder.c`, `tests/unit/test_t7_155.c`, `tests/unit/cmake/eng/t7_155.cmake`, `quality/evidence/T7.155.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-27ac318897664d8a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.155 --regex '^test_t7_155$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.155`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.156: clang-analyzer-deadcode.DeadStores in onvif_ptz.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_156.c`, `tests/unit/cmake/eng/t7_156.cmake`, `quality/evidence/T7.156.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-7d7d3a3c4f1824bd
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.156 --regex '^test_t7_156$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.156`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.157: clang-analyzer-deadcode.DeadStores in recording_path.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/recording_path.c`, `tests/unit/test_t7_157.c`, `tests/unit/cmake/eng/t7_157.cmake`, `quality/evidence/T7.157.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-44c350a8b7e2fd4a, CLANGTIDY-a7fa9aad275f9425
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.157 --regex '^test_t7_157$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.157`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.158: clang-analyzer-deadcode.DeadStores in api_handlers_health.c
- **depends_on**: [T7, T13, T7.44]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_158.c`, `tests/unit/cmake/eng/t7_158.cmake`, `quality/evidence/T7.158.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CLANGTIDY-cf5f09b5a02a9bca
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.158 --regex '^test_t7_158$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.158`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.159: invalidPointerCast in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_159-upstream.json`, `quality/evidence/T7.159.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-7975c2d572bc32c0, CPPCHECK-291bebc3b95eca4e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_159-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.159`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.160: jsx-a11y/anchor-is-valid in Header.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/Header.jsx`, `web/tests/remediation/t7_160.spec.jsx`, `quality/evidence/T7.160.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-366138c5d194685c
- **acceptance_criteria**:
  - Use valid navigable hrefs for links and buttons for actions; verify keyboard behavior and no unexpected page navigation.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_160`
  - `mkdir -p build/eng-artifacts/T7.160`
  - `web/node_modules/.bin/eslint web/js/components/preact/Header.jsx --format json --output-file build/eng-artifacts/T7.160/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.160/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/anchor-is-valid'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.160`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.161: jsx-a11y/click-events-have-key-events in GridPicker.jsx, HLSVideoCell.jsx, HealthView.jsx, MSEVideoCell.jsx, StreamCard.jsx, StreamsView.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/GridPicker.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/HealthView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/tests/remediation/t7_161.spec.jsx`, `quality/evidence/T7.161.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ed278d9594c2abdb, ESLINT-BUCKET-67fcff31d1a4cf75, ESLINT-BUCKET-a642f51dfd2a124a, ESLINT-BUCKET-404de092ea97cc43, ESLINT-BUCKET-6243e36ec0fdb5cd, ESLINT-BUCKET-5c63d53fbef3ba6d
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_161`
  - `mkdir -p build/eng-artifacts/T7.161`
  - `web/node_modules/.bin/eslint web/js/components/preact/GridPicker.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/HealthView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamsView.jsx --format json --output-file build/eng-artifacts/T7.161/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.161/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.161`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.162: jsx-a11y/click-events-have-key-events in ToastContainer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx, LanguageSelector.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/ToastContainer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_162.spec.jsx`, `quality/evidence/T7.162.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ab2c863c82952346, ESLINT-BUCKET-f53a71c368c33ab9, ESLINT-BUCKET-45887456841ead8b, ESLINT-BUCKET-cb235968867c79e3, ESLINT-BUCKET-02418efc105b6ab9, ESLINT-BUCKET-4e8530c09d5fd082
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_162`
  - `mkdir -p build/eng-artifacts/T7.162`
  - `web/node_modules/.bin/eslint web/js/components/preact/ToastContainer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/LanguageSelector.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.162/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.162/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.162`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.163: jsx-a11y/click-events-have-key-events in LiveOperatorNavigator.jsx, RecordingsGrid.jsx, Go2rtcTab.jsx, ClearLogsModal.jsx, RestartModal.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/system/ClearLogsModal.jsx`, `web/js/components/preact/system/RestartModal.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_163.spec.jsx`, `quality/evidence/T7.163.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f16c5e93d86bc6f9, ESLINT-BUCKET-41381cd9bd64279e, ESLINT-BUCKET-422485a76dc61a9b, ESLINT-BUCKET-44ea0e63c82a7151, ESLINT-BUCKET-155480cbfd270011, ESLINT-BUCKET-79b6e7adf4959fbc
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_163`
  - `mkdir -p build/eng-artifacts/T7.163`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/system/ClearLogsModal.jsx web/js/components/preact/system/RestartModal.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.163/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.163/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.163`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.164: jsx-a11y/click-events-have-key-events in AccessPolicyModal.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, DeleteUserModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/DeleteUserModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_164.spec.jsx`, `quality/evidence/T7.164.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-0a786d21e1ee7bd2, ESLINT-BUCKET-c2dc58af6543d1f5, ESLINT-BUCKET-3b71a4c47dfd1075, ESLINT-BUCKET-ea11ae604e62541e, ESLINT-BUCKET-8d573ae471211e73, ESLINT-BUCKET-22666b8c08cd887e
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_164`
  - `mkdir -p build/eng-artifacts/T7.164`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/DeleteUserModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.164/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.164/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.164`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.165: jsx-a11y/click-events-have-key-events in RoleManagerModal.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_165.spec.jsx`, `quality/evidence/T7.165.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-9190884804c0c65d, ESLINT-BUCKET-e8ce699e42be07b3
- **acceptance_criteria**:
  - Assert Enter and Space activation where the control semantics require it, focus visibility, no double dispatch, and unchanged pointer interaction.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_165`
  - `mkdir -p build/eng-artifacts/T7.165`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.165/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.165/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/click-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.165`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.166: jsx-a11y/control-has-associated-label in AuthGate.jsx, BatchDownloadModal.jsx, DetectionOverlay.jsx, FleetView.jsx, FullscreenManager.jsx, HLSVideoCell.jsx
- **depends_on**: [T7, T10, T7.161]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/DetectionOverlay.jsx`, `web/js/components/preact/FleetView.jsx`, `web/js/components/preact/FullscreenManager.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/tests/remediation/t7_166.spec.jsx`, `quality/evidence/T7.166.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-0f4bbeb4ab715fee, ESLINT-BUCKET-ef00a494fd69875c, ESLINT-BUCKET-e93ca9cb727a0aee, ESLINT-BUCKET-20e1a03b32e40002, ESLINT-BUCKET-e7e0cc4f8fd4c513, ESLINT-BUCKET-aa839344c6e5b30d
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_166`
  - `mkdir -p build/eng-artifacts/T7.166`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/DetectionOverlay.jsx web/js/components/preact/FleetView.jsx web/js/components/preact/FullscreenManager.jsx web/js/components/preact/HLSVideoCell.jsx --format json --output-file build/eng-artifacts/T7.166/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.166/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.166`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.167: jsx-a11y/control-has-associated-label in LiveView.jsx, LoginView.jsx, MSEVideoCell.jsx, SetupWizard.jsx, SnapshotManager.jsx, StreamConfigModal.jsx
- **depends_on**: [T7, T10, T7.161]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/SetupWizard.jsx`, `web/js/components/preact/SnapshotManager.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/tests/remediation/t7_167.spec.jsx`, `quality/evidence/T7.167.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ae74d74f4f8f96b4, ESLINT-BUCKET-33c31857a85153c5, ESLINT-BUCKET-2b401ef7608cff74, ESLINT-BUCKET-6b623e0db3f455c7, ESLINT-BUCKET-63e9c68b97a2b704, ESLINT-BUCKET-3d882f111ea61422
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_167`
  - `mkdir -p build/eng-artifacts/T7.167`
  - `web/node_modules/.bin/eslint web/js/components/preact/LiveView.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/SetupWizard.jsx web/js/components/preact/SnapshotManager.jsx web/js/components/preact/StreamConfigModal.jsx --format json --output-file build/eng-artifacts/T7.167/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.167/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.167`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.168: jsx-a11y/control-has-associated-label in StreamsView.jsx, ThemeCustomizer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10, T7.161, T7.162]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/ThemeCustomizer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_168.spec.jsx`, `quality/evidence/T7.168.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-ba71da8483425ebf, ESLINT-BUCKET-6ffa955639ab6418, ESLINT-BUCKET-6faae6bddf434aab, ESLINT-BUCKET-9bd9a8994dceced4, ESLINT-BUCKET-e845a2e465702318, ESLINT-BUCKET-1f35f119bab13f92
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_168`
  - `mkdir -p build/eng-artifacts/T7.168`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamsView.jsx web/js/components/preact/ThemeCustomizer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.168/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.168/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.168`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.169: jsx-a11y/control-has-associated-label in EventDestinationEditor.jsx, EventRouteEditor.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, CollectionSelectorBuilder.jsx, FleetFilters.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/CollectionSelectorBuilder.jsx`, `web/js/components/preact/fleet/FleetFilters.jsx`, `web/tests/remediation/t7_169.spec.jsx`, `quality/evidence/T7.169.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-5088d976b993e707, ESLINT-BUCKET-fea403cece7fffa3, ESLINT-BUCKET-20d0442e85244a5d, ESLINT-BUCKET-03b8b95f3bd44235, ESLINT-BUCKET-993ef9a219cc29a2, ESLINT-BUCKET-eaa9041228587f50
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_169`
  - `mkdir -p build/eng-artifacts/T7.169`
  - `web/node_modules/.bin/eslint web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/CollectionSelectorBuilder.jsx web/js/components/preact/fleet/FleetFilters.jsx --format json --output-file build/eng-artifacts/T7.169/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.169/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.169`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.170: jsx-a11y/control-has-associated-label in OrganizationManager.jsx, SavedViewsBar.jsx, StaticCollectionMembers.jsx, InvestigationActions.jsx, InvestigationBookmarks.jsx, InvestigationView.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/fleet/SavedViewsBar.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/js/components/preact/investigation/InvestigationActions.jsx`, `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/tests/remediation/t7_170.spec.jsx`, `quality/evidence/T7.170.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-19d5fc636e3f15e6, ESLINT-BUCKET-2cdd7ef73529551e, ESLINT-BUCKET-5a25b9ace393f230, ESLINT-BUCKET-12c7bb31966e53aa, ESLINT-BUCKET-274240d1fc0300f7, ESLINT-BUCKET-2a20bf23c61a102e
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_170`
  - `mkdir -p build/eng-artifacts/T7.170`
  - `web/node_modules/.bin/eslint web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/fleet/SavedViewsBar.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx web/js/components/preact/investigation/InvestigationActions.jsx web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/investigation/InvestigationView.jsx --format json --output-file build/eng-artifacts/T7.170/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.170/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.170`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.171: jsx-a11y/control-has-associated-label in LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx, PlanSketchControls.jsx, FiltersSidebar.jsx, RecordingsGrid.jsx, RecordingsTable.jsx
- **depends_on**: [T7, T10, T7.163]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/live/PlanSketchControls.jsx`, `web/js/components/preact/recordings/FiltersSidebar.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/tests/remediation/t7_171.spec.jsx`, `quality/evidence/T7.171.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-bb1a968a245405ec, ESLINT-BUCKET-3a6fb0a94ef6ffdf, ESLINT-BUCKET-e0e4b387af698b40, ESLINT-BUCKET-389226ea0147ea5d, ESLINT-BUCKET-aab67d5bbb7b5489, ESLINT-BUCKET-f492731b13ec277c
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_171`
  - `mkdir -p build/eng-artifacts/T7.171`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/live/PlanSketchControls.jsx web/js/components/preact/recordings/FiltersSidebar.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/recordings/RecordingsTable.jsx --format json --output-file build/eng-artifacts/T7.171/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.171/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.171`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.172: jsx-a11y/control-has-associated-label in TagsOverlay.jsx, AdvancedTab.jsx, AuthTab.jsx, DetectionTab.jsx, GeneralTab.jsx, Go2rtcTab.jsx
- **depends_on**: [T7, T10, T7.163]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/recordings/TagsOverlay.jsx`, `web/js/components/preact/settings/AdvancedTab.jsx`, `web/js/components/preact/settings/AuthTab.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/GeneralTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/tests/remediation/t7_172.spec.jsx`, `quality/evidence/T7.172.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d03afa69ead3203e, ESLINT-BUCKET-8ce22b0413a485e1, ESLINT-BUCKET-40ef7b2f328bb9ef, ESLINT-BUCKET-93d2b8db97a42555, ESLINT-BUCKET-ea86635ccd79513e, ESLINT-BUCKET-5ec0649750441f81
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_172`
  - `mkdir -p build/eng-artifacts/T7.172`
  - `web/node_modules/.bin/eslint web/js/components/preact/recordings/TagsOverlay.jsx web/js/components/preact/settings/AdvancedTab.jsx web/js/components/preact/settings/AuthTab.jsx web/js/components/preact/settings/DetectionTab.jsx web/js/components/preact/settings/GeneralTab.jsx web/js/components/preact/settings/Go2rtcTab.jsx --format json --output-file build/eng-artifacts/T7.172/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.172/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.172`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.173: jsx-a11y/control-has-associated-label in HealthTab.jsx, MqttTab.jsx, StorageTab.jsx, StreamsDefaultsTab.jsx, StreamStorage.jsx, CalendarPicker.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/HealthTab.jsx`, `web/js/components/preact/settings/MqttTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/settings/StreamsDefaultsTab.jsx`, `web/js/components/preact/system/StreamStorage.jsx`, `web/js/components/preact/timeline/CalendarPicker.jsx`, `web/tests/remediation/t7_173.spec.jsx`, `quality/evidence/T7.173.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-fe8c1e237393fb23, ESLINT-BUCKET-59e343114dc251b1, ESLINT-BUCKET-5a1cbbfddf8f3fcc, ESLINT-BUCKET-350da16b1c3b2274, ESLINT-BUCKET-44b516dc2eeb97bf, ESLINT-BUCKET-5da42ce096a9fa82
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_173`
  - `mkdir -p build/eng-artifacts/T7.173`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/HealthTab.jsx web/js/components/preact/settings/MqttTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/settings/StreamsDefaultsTab.jsx web/js/components/preact/system/StreamStorage.jsx web/js/components/preact/timeline/CalendarPicker.jsx --format json --output-file build/eng-artifacts/T7.173/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.173/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.173`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.174: jsx-a11y/control-has-associated-label in TimelineControls.jsx, TimelinePlayer.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10, T7.163, T7.164]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/timeline/TimelineControls.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_174.spec.jsx`, `quality/evidence/T7.174.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-7b00d558899dd292, ESLINT-BUCKET-4e38c49052b49016, ESLINT-BUCKET-6af902622576c683, ESLINT-BUCKET-098293b0fb5320bb, ESLINT-BUCKET-f25f197e02c782fc, ESLINT-BUCKET-ccf3e6e7959d2bd7
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_174`
  - `mkdir -p build/eng-artifacts/T7.174`
  - `web/node_modules/.bin/eslint web/js/components/preact/timeline/TimelineControls.jsx web/js/components/preact/timeline/TimelinePlayer.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.174/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.174/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.174`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.175: jsx-a11y/control-has-associated-label in RoleManagerModal.jsx, TotpSetupModal.jsx, UsersTable.jsx
- **depends_on**: [T7, T10, T7.165]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/js/components/preact/users/UsersTable.jsx`, `web/tests/remediation/t7_175.spec.jsx`, `quality/evidence/T7.175.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-400e44e71536be4e, ESLINT-BUCKET-d3aede71a08c3a5f, ESLINT-BUCKET-fcf1de019f6b4533
- **acceptance_criteria**:
  - Render each reported control and assert a stable accessible name, including icon-only actions, without relying on title alone; retain localized labels and original action callbacks.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_175`
  - `mkdir -p build/eng-artifacts/T7.175`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx web/js/components/preact/users/UsersTable.jsx --format json --output-file build/eng-artifacts/T7.175/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.175/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/control-has-associated-label'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.175`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.176: jsx-a11y/interactive-supports-focus in MobileTileGestures.jsx
- **depends_on**: [T7, T10]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/MobileTileGestures.jsx`, `web/tests/remediation/t7_176.spec.jsx`, `quality/evidence/T7.176.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-3469a0a703cdbc12
- **acceptance_criteria**:
  - Assert each interactive control is keyboard focusable and acts once for its expected activation keys.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_176`
  - `mkdir -p build/eng-artifacts/T7.176`
  - `web/node_modules/.bin/eslint web/js/components/preact/MobileTileGestures.jsx --format json --output-file build/eng-artifacts/T7.176/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.176/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/interactive-supports-focus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.176`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.177: jsx-a11y/label-has-associated-control in BatchDownloadModal.jsx, StreamConfigModal.jsx, EventDestinationEditor.jsx, EventRouteEditor.jsx, CollectionManager.jsx, StaticCollectionMembers.jsx
- **depends_on**: [T7, T10, T7.166, T7.167, T7.169, T7.170]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/tests/remediation/t7_177.spec.jsx`, `quality/evidence/T7.177.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-199d3d91208953b4, ESLINT-BUCKET-81a756fe7228ad5b, ESLINT-BUCKET-ddc6803a9b16e391, ESLINT-BUCKET-7cde28afaa9ffb65, ESLINT-BUCKET-7dcecd444f491721, ESLINT-BUCKET-9308fe6583bcdce7
- **acceptance_criteria**:
  - Assert label/input association is programmatic, unique in repeated forms, localized and preserved during conditional rendering.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_177`
  - `mkdir -p build/eng-artifacts/T7.177`
  - `web/node_modules/.bin/eslint web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx --format json --output-file build/eng-artifacts/T7.177/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.177/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-associated-control'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.177`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.178: jsx-a11y/label-has-associated-control in HealthTab.jsx, StorageTab.jsx, ApiKeyModal.jsx
- **depends_on**: [T7, T10, T7.173, T7.174]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/HealthTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/tests/remediation/t7_178.spec.jsx`, `quality/evidence/T7.178.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-5e7cfcb56208eaa9, ESLINT-BUCKET-e84d458d8fcd3eef, ESLINT-BUCKET-cf4b93673820960b
- **acceptance_criteria**:
  - Assert label/input association is programmatic, unique in repeated forms, localized and preserved during conditional rendering.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_178`
  - `mkdir -p build/eng-artifacts/T7.178`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/HealthTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/users/ApiKeyModal.jsx --format json --output-file build/eng-artifacts/T7.178/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.178/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-associated-control'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.178`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.179: jsx-a11y/label-has-for in AuthGate.jsx, BatchDownloadModal.jsx, FleetView.jsx, LiveView.jsx, LoginView.jsx, SetupWizard.jsx
- **depends_on**: [T7, T10, T7.166, T7.177, T7.167]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/BatchDownloadModal.jsx`, `web/js/components/preact/FleetView.jsx`, `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/SetupWizard.jsx`, `web/tests/remediation/t7_179.spec.jsx`, `quality/evidence/T7.179.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-00c9eb79387b90d1, ESLINT-BUCKET-c603f4c09210c0bd, ESLINT-BUCKET-07e48ad6ea9ce45e, ESLINT-BUCKET-dfeaa6c4a9abf5fa, ESLINT-BUCKET-98f2a6813a7ad225, ESLINT-BUCKET-e73c0530b2243573
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_179`
  - `mkdir -p build/eng-artifacts/T7.179`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/BatchDownloadModal.jsx web/js/components/preact/FleetView.jsx web/js/components/preact/LiveView.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/SetupWizard.jsx --format json --output-file build/eng-artifacts/T7.179/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.179/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.179`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.180: jsx-a11y/label-has-for in StreamCard.jsx, StreamConfigModal.jsx, StreamsView.jsx, UI.jsx, ZoneEditor.jsx, ModalDialog.jsx
- **depends_on**: [T7, T10, T7.161, T7.177, T7.168]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/ModalDialog.jsx`, `web/tests/remediation/t7_180.spec.jsx`, `quality/evidence/T7.180.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-c4c40aa44f4e62e4, ESLINT-BUCKET-d900a116e1f06cf4, ESLINT-BUCKET-de63541e57266afb, ESLINT-BUCKET-e810ef87137eb5d8, ESLINT-BUCKET-38ac9714530327f7, ESLINT-BUCKET-b1a74f710eb47a22
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_180`
  - `mkdir -p build/eng-artifacts/T7.180`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/UI.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/ModalDialog.jsx --format json --output-file build/eng-artifacts/T7.180/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.180/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.180`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.181: jsx-a11y/label-has-for in EventDestinationEditor.jsx, EventRouteEditor.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, CollectionSelectorBuilder.jsx, FleetTable.jsx
- **depends_on**: [T7, T10, T7.177, T7.169]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/CollectionSelectorBuilder.jsx`, `web/js/components/preact/fleet/FleetTable.jsx`, `web/tests/remediation/t7_181.spec.jsx`, `quality/evidence/T7.181.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-2d6e882afea8fed1, ESLINT-BUCKET-f3bd5129ab03f352, ESLINT-BUCKET-7ba2e66a4eb3914f, ESLINT-BUCKET-05ad31a80cc2b3a2, ESLINT-BUCKET-30a2e484727d4a5a, ESLINT-BUCKET-b32370126e568d01
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_181`
  - `mkdir -p build/eng-artifacts/T7.181`
  - `web/node_modules/.bin/eslint web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/CollectionSelectorBuilder.jsx web/js/components/preact/fleet/FleetTable.jsx --format json --output-file build/eng-artifacts/T7.181/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.181/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.181`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.182: jsx-a11y/label-has-for in OrganizationManager.jsx, SavedViewsBar.jsx, StaticCollectionMembers.jsx, InvestigationActions.jsx, InvestigationBookmarks.jsx, InvestigationView.jsx
- **depends_on**: [T7, T10, T7.170, T7.177]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/fleet/SavedViewsBar.jsx`, `web/js/components/preact/fleet/StaticCollectionMembers.jsx`, `web/js/components/preact/investigation/InvestigationActions.jsx`, `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/tests/remediation/t7_182.spec.jsx`, `quality/evidence/T7.182.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f77074e66ed65721, ESLINT-BUCKET-e26c8068ece9c0e0, ESLINT-BUCKET-4246d4277a0b8f04, ESLINT-BUCKET-642b5326a1034302, ESLINT-BUCKET-c0802f3ddbb20b2e, ESLINT-BUCKET-137cbeae12c87193
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_182`
  - `mkdir -p build/eng-artifacts/T7.182`
  - `web/node_modules/.bin/eslint web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/fleet/SavedViewsBar.jsx web/js/components/preact/fleet/StaticCollectionMembers.jsx web/js/components/preact/investigation/InvestigationActions.jsx web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/investigation/InvestigationView.jsx --format json --output-file build/eng-artifacts/T7.182/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.182/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.182`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.183: jsx-a11y/label-has-for in LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx, FiltersSidebar.jsx, RecordingsGrid.jsx, RecordingsTable.jsx, TagsOverlay.jsx
- **depends_on**: [T7, T10, T7.171, T7.172]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/FiltersSidebar.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/recordings/TagsOverlay.jsx`, `web/tests/remediation/t7_183.spec.jsx`, `quality/evidence/T7.183.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a167dfb879b1b9cf, ESLINT-BUCKET-b0e1dbc8e8cc536e, ESLINT-BUCKET-50994f387baeaa69, ESLINT-BUCKET-dcde47440ba091e3, ESLINT-BUCKET-114c375fcc3b4802, ESLINT-BUCKET-8b431e9fa35d81ba
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_183`
  - `mkdir -p build/eng-artifacts/T7.183`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/FiltersSidebar.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/recordings/RecordingsTable.jsx web/js/components/preact/recordings/TagsOverlay.jsx --format json --output-file build/eng-artifacts/T7.183/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.183/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.183`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.184: jsx-a11y/label-has-for in AdvancedTab.jsx, AuthTab.jsx, DetectionTab.jsx, GeneralTab.jsx, Go2rtcTab.jsx, HealthTab.jsx
- **depends_on**: [T7, T10, T7.172, T7.178]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/AdvancedTab.jsx`, `web/js/components/preact/settings/AuthTab.jsx`, `web/js/components/preact/settings/DetectionTab.jsx`, `web/js/components/preact/settings/GeneralTab.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/settings/HealthTab.jsx`, `web/tests/remediation/t7_184.spec.jsx`, `quality/evidence/T7.184.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a4e1217d107c230c, ESLINT-BUCKET-4a0d0e1ea3635a24, ESLINT-BUCKET-d6ed69efe339ce37, ESLINT-BUCKET-61d1473d84fbeea6, ESLINT-BUCKET-dd680edebf6809ff, ESLINT-BUCKET-e267f8f47c3806d3
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_184`
  - `mkdir -p build/eng-artifacts/T7.184`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/AdvancedTab.jsx web/js/components/preact/settings/AuthTab.jsx web/js/components/preact/settings/DetectionTab.jsx web/js/components/preact/settings/GeneralTab.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/settings/HealthTab.jsx --format json --output-file build/eng-artifacts/T7.184/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.184/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.184`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.185: jsx-a11y/label-has-for in MqttTab.jsx, StorageTab.jsx, StreamsDefaultsTab.jsx, WorkspacesTab.jsx, TimelinePage.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10, T7.173, T7.178, T7.174]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/settings/MqttTab.jsx`, `web/js/components/preact/settings/StorageTab.jsx`, `web/js/components/preact/settings/StreamsDefaultsTab.jsx`, `web/js/components/preact/settings/WorkspacesTab.jsx`, `web/js/components/preact/timeline/TimelinePage.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_185.spec.jsx`, `quality/evidence/T7.185.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d5ecbc0292c55216, ESLINT-BUCKET-c067bb3093f57d49, ESLINT-BUCKET-4e8b56001c3a0722, ESLINT-BUCKET-223399111dfd4016, ESLINT-BUCKET-7d2f50f4d066f6e4, ESLINT-BUCKET-00d366c5691b4bb5
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_185`
  - `mkdir -p build/eng-artifacts/T7.185`
  - `web/node_modules/.bin/eslint web/js/components/preact/settings/MqttTab.jsx web/js/components/preact/settings/StorageTab.jsx web/js/components/preact/settings/StreamsDefaultsTab.jsx web/js/components/preact/settings/WorkspacesTab.jsx web/js/components/preact/timeline/TimelinePage.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.185/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.185/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.185`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.186: jsx-a11y/label-has-for in AccessPolicyModal.jsx, AddUserModal.jsx, ApiKeyModal.jsx, AuditDecisionModes.jsx, AuditHistoryModal.jsx, EditUserModal.jsx
- **depends_on**: [T7, T10, T7.164, T7.174, T7.178]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditDecisionModes.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/tests/remediation/t7_186.spec.jsx`, `quality/evidence/T7.186.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-7f43090b7b01626a, ESLINT-BUCKET-17ad0e14291e8ec3, ESLINT-BUCKET-18b7f3add93999e4, ESLINT-BUCKET-d3157833e1568912, ESLINT-BUCKET-184f5809a1f3d621, ESLINT-BUCKET-682efc8151a90daf
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_186`
  - `mkdir -p build/eng-artifacts/T7.186`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditDecisionModes.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx --format json --output-file build/eng-artifacts/T7.186/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.186/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.186`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.187: jsx-a11y/label-has-for in RoleManagerModal.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.175]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/RoleManagerModal.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_187.spec.jsx`, `quality/evidence/T7.187.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-2e14d60ae388c180, ESLINT-BUCKET-9a54a74c58d29d4c
- **acceptance_criteria**:
  - Assert each label resolves to its intended input by id/for with unique IDs across repeated components and that clicking it focuses the input.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_187`
  - `mkdir -p build/eng-artifacts/T7.187`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/RoleManagerModal.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.187/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.187/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/label-has-for'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.187`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.188: jsx-a11y/media-has-caption in HLSVideoCell.jsx, MSEVideoCell.jsx, UI.jsx, WebRTCVideoCell.jsx, InvestigationView.jsx, TimelinePlayer.jsx
- **depends_on**: [T7, T10, T7.166, T7.167, T7.180, T7.168, T7.182, T7.185]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/tests/remediation/t7_188.spec.jsx`, `quality/evidence/T7.188.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-165a348ebdcecd71, ESLINT-BUCKET-1760eab07e0cce49, ESLINT-BUCKET-df0c5f209b6ef6ab, ESLINT-BUCKET-c464a6c1e23bf196, ESLINT-BUCKET-de50a4c99ae7d421, ESLINT-BUCKET-7189b497b5a855df
- **acceptance_criteria**:
  - Identify prerecorded speech versus live/silent surveillance streams; provide applicable caption support, or an explicitly reviewed narrow semantic exception with evidence. Never add a fake empty caption track.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_188`
  - `mkdir -p build/eng-artifacts/T7.188`
  - `web/node_modules/.bin/eslint web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/investigation/InvestigationView.jsx web/js/components/preact/timeline/TimelinePlayer.jsx --format json --output-file build/eng-artifacts/T7.188/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.188/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/media-has-caption'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.188`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.189: jsx-a11y/mouse-events-have-key-events in DayNightControl.jsx, HLSVideoCell.jsx, SnapshotManager.jsx, StreamCard.jsx, StreamsView.jsx, Toast.jsx
- **depends_on**: [T7, T10, T7.188, T7.167, T7.180]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/DayNightControl.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/SnapshotManager.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/Toast.jsx`, `web/tests/remediation/t7_189.spec.jsx`, `quality/evidence/T7.189.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-bc4e9a8b10d83163, ESLINT-BUCKET-c31a4d9816cac7ce, ESLINT-BUCKET-e715a00f3dbe2f4f, ESLINT-BUCKET-9a70a1ccf37b9405, ESLINT-BUCKET-e876245abbef0a79, ESLINT-BUCKET-c811624672d6bfff
- **acceptance_criteria**:
  - Assert focus/blur and mouse enter/leave expose and dismiss the same content without trapping focus.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_189`
  - `mkdir -p build/eng-artifacts/T7.189`
  - `web/node_modules/.bin/eslint web/js/components/preact/DayNightControl.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/SnapshotManager.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/Toast.jsx --format json --output-file build/eng-artifacts/T7.189/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.189/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/mouse-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.189`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.190: jsx-a11y/mouse-events-have-key-events in WebRTCVideoCell.jsx, RecordingsTable.jsx, TimelinePage.jsx
- **depends_on**: [T7, T10, T7.188, T7.183, T7.185]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/recordings/RecordingsTable.jsx`, `web/js/components/preact/timeline/TimelinePage.jsx`, `web/tests/remediation/t7_190.spec.jsx`, `quality/evidence/T7.190.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-a4f59c579d19ce1e, ESLINT-BUCKET-7a540f91903a1e57, ESLINT-BUCKET-dcdb657c2038e332
- **acceptance_criteria**:
  - Assert focus/blur and mouse enter/leave expose and dismiss the same content without trapping focus.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_190`
  - `mkdir -p build/eng-artifacts/T7.190`
  - `web/node_modules/.bin/eslint web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/recordings/RecordingsTable.jsx web/js/components/preact/timeline/TimelinePage.jsx --format json --output-file build/eng-artifacts/T7.190/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.190/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/mouse-events-have-key-events'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.190`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.191: jsx-a11y/no-autofocus in AuthGate.jsx, LoginView.jsx, EventDestinationEditor.jsx, EventRouteEditor.jsx, CollectionManager.jsx, OrganizationManager.jsx
- **depends_on**: [T7, T10, T7.179, T7.181, T7.182]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/AuthGate.jsx`, `web/js/components/preact/LoginView.jsx`, `web/js/components/preact/events/EventDestinationEditor.jsx`, `web/js/components/preact/events/EventRouteEditor.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/tests/remediation/t7_191.spec.jsx`, `quality/evidence/T7.191.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-20016996a3c7e096, ESLINT-BUCKET-92718cbd3fc79c29, ESLINT-BUCKET-71520b6429344586, ESLINT-BUCKET-82f90000b805b238, ESLINT-BUCKET-2acf09b0886c70c8, ESLINT-BUCKET-e6884093b3a77737
- **acceptance_criteria**:
  - Assert initial render does not steal focus; explicit dialog-open focus management must return focus to the invoker on close.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_191`
  - `mkdir -p build/eng-artifacts/T7.191`
  - `web/node_modules/.bin/eslint web/js/components/preact/AuthGate.jsx web/js/components/preact/LoginView.jsx web/js/components/preact/events/EventDestinationEditor.jsx web/js/components/preact/events/EventRouteEditor.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/OrganizationManager.jsx --format json --output-file build/eng-artifacts/T7.191/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.191/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-autofocus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.191`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.192: jsx-a11y/no-autofocus in InvestigationBookmarks.jsx, TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.182, T7.187]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/investigation/InvestigationBookmarks.jsx`, `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_192.spec.jsx`, `quality/evidence/T7.192.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-24d767472f9eac0b, ESLINT-BUCKET-6040ad491a730d6d
- **acceptance_criteria**:
  - Assert initial render does not steal focus; explicit dialog-open focus management must return focus to the invoker on close.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_192`
  - `mkdir -p build/eng-artifacts/T7.192`
  - `web/node_modules/.bin/eslint web/js/components/preact/investigation/InvestigationBookmarks.jsx web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.192/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.192/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-autofocus'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.192`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.193: jsx-a11y/no-noninteractive-element-interactions in FisheyeEptzCanvas.jsx, StreamCard.jsx, ZoneEditor.jsx, LanguageSelector.jsx, InvestigationView.jsx, LiveBuildingPlan.jsx
- **depends_on**: [T7, T10, T7.189, T7.180, T7.162, T7.188, T7.183]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/FisheyeEptzCanvas.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/js/components/preact/investigation/InvestigationView.jsx`, `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/tests/remediation/t7_193.spec.jsx`, `quality/evidence/T7.193.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-d8c2127f8046b560, ESLINT-BUCKET-e4342be069acaf24, ESLINT-BUCKET-141e103765b7c505, ESLINT-BUCKET-370c727cb2690828, ESLINT-BUCKET-bf7012d93c48094c, ESLINT-BUCKET-11c6ba5b7bf02e40
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_193`
  - `mkdir -p build/eng-artifacts/T7.193`
  - `web/node_modules/.bin/eslint web/js/components/preact/FisheyeEptzCanvas.jsx web/js/components/preact/StreamCard.jsx web/js/components/preact/ZoneEditor.jsx web/js/components/preact/common/LanguageSelector.jsx web/js/components/preact/investigation/InvestigationView.jsx web/js/components/preact/live/LiveBuildingPlan.jsx --format json --output-file build/eng-artifacts/T7.193/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.193/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.193`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.194: jsx-a11y/no-noninteractive-element-interactions in LiveOperatorNavigator.jsx, RecordingsGrid.jsx, TimelineCursor.jsx, TimelineSegments.jsx, AccessPolicyModal.jsx, AddUserModal.jsx
- **depends_on**: [T7, T10, T7.183, T7.186]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/timeline/TimelineCursor.jsx`, `web/js/components/preact/timeline/TimelineSegments.jsx`, `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/js/components/preact/users/AddUserModal.jsx`, `web/tests/remediation/t7_194.spec.jsx`, `quality/evidence/T7.194.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-87cfe9ce5d499faf, ESLINT-BUCKET-4d679d89b4da6865, ESLINT-BUCKET-d670cad0625d4f2c, ESLINT-BUCKET-03caeab009bdc4fd, ESLINT-BUCKET-939df4abd7adefdd, ESLINT-BUCKET-4cf2eab3dda13b90
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_194`
  - `mkdir -p build/eng-artifacts/T7.194`
  - `web/node_modules/.bin/eslint web/js/components/preact/live/LiveOperatorNavigator.jsx web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/timeline/TimelineCursor.jsx web/js/components/preact/timeline/TimelineSegments.jsx web/js/components/preact/users/AccessPolicyModal.jsx web/js/components/preact/users/AddUserModal.jsx --format json --output-file build/eng-artifacts/T7.194/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.194/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.194`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.195: jsx-a11y/no-noninteractive-element-interactions in ApiKeyModal.jsx, AuditHistoryModal.jsx, EditUserModal.jsx, RoleManagerModal.jsx
- **depends_on**: [T7, T10, T7.186, T7.187]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/js/components/preact/users/RoleManagerModal.jsx`, `web/tests/remediation/t7_195.spec.jsx`, `quality/evidence/T7.195.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-68103860cfe78d89, ESLINT-BUCKET-be996c224380b85e, ESLINT-BUCKET-65f403c439392041, ESLINT-BUCKET-752682010e0c301c
- **acceptance_criteria**:
  - Remove unintended interactive behavior from structural elements or expose correct interactive semantics; preserve nested-control propagation and keyboard behavior.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_195`
  - `mkdir -p build/eng-artifacts/T7.195`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/EditUserModal.jsx web/js/components/preact/users/RoleManagerModal.jsx --format json --output-file build/eng-artifacts/T7.195/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.195/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.195`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.196: jsx-a11y/no-noninteractive-tabindex in FisheyeEptzCanvas.jsx, LanguageSelector.jsx
- **depends_on**: [T7, T10, T7.193]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/FisheyeEptzCanvas.jsx`, `web/js/components/preact/common/LanguageSelector.jsx`, `web/tests/remediation/t7_196.spec.jsx`, `quality/evidence/T7.196.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-f32c4fd5a51337d3, ESLINT-BUCKET-d5958c65428cdc31
- **acceptance_criteria**:
  - Assert the tab order includes only intended interactive controls; keyboard navigation of composite widgets must follow their documented role.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_196`
  - `mkdir -p build/eng-artifacts/T7.196`
  - `web/node_modules/.bin/eslint web/js/components/preact/FisheyeEptzCanvas.jsx web/js/components/preact/common/LanguageSelector.jsx --format json --output-file build/eng-artifacts/T7.196/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.196/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-noninteractive-tabindex'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.196`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.197: jsx-a11y/no-static-element-interactions in GridPicker.jsx, HLSVideoCell.jsx, HealthView.jsx, LiveView.jsx, MSEVideoCell.jsx, StreamCard.jsx
- **depends_on**: [T7, T10, T7.161, T7.189, T7.179, T7.188, T7.193]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/GridPicker.jsx`, `web/js/components/preact/HLSVideoCell.jsx`, `web/js/components/preact/HealthView.jsx`, `web/js/components/preact/LiveView.jsx`, `web/js/components/preact/MSEVideoCell.jsx`, `web/js/components/preact/StreamCard.jsx`, `web/tests/remediation/t7_197.spec.jsx`, `quality/evidence/T7.197.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-45ebdb33421d89d5, ESLINT-BUCKET-d6af678aacfbb026, ESLINT-BUCKET-e48ab90f6d76f69a, ESLINT-BUCKET-6cf65d94c0e817c8, ESLINT-BUCKET-d0493f3174391c77, ESLINT-BUCKET-134e1087e8cda477
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_197`
  - `mkdir -p build/eng-artifacts/T7.197`
  - `web/node_modules/.bin/eslint web/js/components/preact/GridPicker.jsx web/js/components/preact/HLSVideoCell.jsx web/js/components/preact/HealthView.jsx web/js/components/preact/LiveView.jsx web/js/components/preact/MSEVideoCell.jsx web/js/components/preact/StreamCard.jsx --format json --output-file build/eng-artifacts/T7.197/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.197/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.197`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.198: jsx-a11y/no-static-element-interactions in StreamConfigModal.jsx, StreamsView.jsx, ToastContainer.jsx, UI.jsx, WebRTCVideoCell.jsx, ZoneEditor.jsx
- **depends_on**: [T7, T10, T7.180, T7.189, T7.162, T7.188, T7.190, T7.193]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/StreamConfigModal.jsx`, `web/js/components/preact/StreamsView.jsx`, `web/js/components/preact/ToastContainer.jsx`, `web/js/components/preact/UI.jsx`, `web/js/components/preact/WebRTCVideoCell.jsx`, `web/js/components/preact/ZoneEditor.jsx`, `web/tests/remediation/t7_198.spec.jsx`, `quality/evidence/T7.198.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-8a059d804561f236, ESLINT-BUCKET-21e6c2fa9c26c964, ESLINT-BUCKET-b18881a630321191, ESLINT-BUCKET-d9d81163d1e1193d, ESLINT-BUCKET-3322f39793a9258c, ESLINT-BUCKET-6bdf7c909180cb5e
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_198`
  - `mkdir -p build/eng-artifacts/T7.198`
  - `web/node_modules/.bin/eslint web/js/components/preact/StreamConfigModal.jsx web/js/components/preact/StreamsView.jsx web/js/components/preact/ToastContainer.jsx web/js/components/preact/UI.jsx web/js/components/preact/WebRTCVideoCell.jsx web/js/components/preact/ZoneEditor.jsx --format json --output-file build/eng-artifacts/T7.198/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.198/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.198`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.199: jsx-a11y/no-static-element-interactions in ModalDialog.jsx, BulkOrganizeModal.jsx, CollectionManager.jsx, OrganizationManager.jsx, LiveBuildingPlan.jsx, LiveOperatorNavigator.jsx
- **depends_on**: [T7, T10, T7.180, T7.181, T7.191, T7.193, T7.194]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/common/ModalDialog.jsx`, `web/js/components/preact/fleet/BulkOrganizeModal.jsx`, `web/js/components/preact/fleet/CollectionManager.jsx`, `web/js/components/preact/fleet/OrganizationManager.jsx`, `web/js/components/preact/live/LiveBuildingPlan.jsx`, `web/js/components/preact/live/LiveOperatorNavigator.jsx`, `web/tests/remediation/t7_199.spec.jsx`, `quality/evidence/T7.199.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-243fa74ece4acc83, ESLINT-BUCKET-aa87081e964848f0, ESLINT-BUCKET-b9126919c0a748d2, ESLINT-BUCKET-ac70bbab0047fa80, ESLINT-BUCKET-23073b0c71d7d2e3, ESLINT-BUCKET-be705595f12d1850
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_199`
  - `mkdir -p build/eng-artifacts/T7.199`
  - `web/node_modules/.bin/eslint web/js/components/preact/common/ModalDialog.jsx web/js/components/preact/fleet/BulkOrganizeModal.jsx web/js/components/preact/fleet/CollectionManager.jsx web/js/components/preact/fleet/OrganizationManager.jsx web/js/components/preact/live/LiveBuildingPlan.jsx web/js/components/preact/live/LiveOperatorNavigator.jsx --format json --output-file build/eng-artifacts/T7.199/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.199/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.199`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.200: jsx-a11y/no-static-element-interactions in RecordingsGrid.jsx, Go2rtcTab.jsx, ClearLogsModal.jsx, RestartModal.jsx, TimelinePlayer.jsx, AccessPolicyModal.jsx
- **depends_on**: [T7, T10, T7.194, T7.184, T7.163, T7.188]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/recordings/RecordingsGrid.jsx`, `web/js/components/preact/settings/Go2rtcTab.jsx`, `web/js/components/preact/system/ClearLogsModal.jsx`, `web/js/components/preact/system/RestartModal.jsx`, `web/js/components/preact/timeline/TimelinePlayer.jsx`, `web/js/components/preact/users/AccessPolicyModal.jsx`, `web/tests/remediation/t7_200.spec.jsx`, `quality/evidence/T7.200.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-55a74c5d3180cf15, ESLINT-BUCKET-fd743aa60d9abb1f, ESLINT-BUCKET-fbc7ca1e0083c1d1, ESLINT-BUCKET-259ca1824a1d1447, ESLINT-BUCKET-c06de8e6c5473e61, ESLINT-BUCKET-6bab06cb3c8102a9
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_200`
  - `mkdir -p build/eng-artifacts/T7.200`
  - `web/node_modules/.bin/eslint web/js/components/preact/recordings/RecordingsGrid.jsx web/js/components/preact/settings/Go2rtcTab.jsx web/js/components/preact/system/ClearLogsModal.jsx web/js/components/preact/system/RestartModal.jsx web/js/components/preact/timeline/TimelinePlayer.jsx web/js/components/preact/users/AccessPolicyModal.jsx --format json --output-file build/eng-artifacts/T7.200/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.200/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.200`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.201: jsx-a11y/no-static-element-interactions in AddUserModal.jsx, ApiKeyModal.jsx, AuditHistoryModal.jsx, DeleteUserModal.jsx, EditUserModal.jsx, RoleManagerModal.jsx
- **depends_on**: [T7, T10, T7.194, T7.195, T7.164]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/AddUserModal.jsx`, `web/js/components/preact/users/ApiKeyModal.jsx`, `web/js/components/preact/users/AuditHistoryModal.jsx`, `web/js/components/preact/users/DeleteUserModal.jsx`, `web/js/components/preact/users/EditUserModal.jsx`, `web/js/components/preact/users/RoleManagerModal.jsx`, `web/tests/remediation/t7_201.spec.jsx`, `quality/evidence/T7.201.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-3ace786e8c2cd1a8, ESLINT-BUCKET-08d8785eb7476330, ESLINT-BUCKET-51093f104f4e1cbb, ESLINT-BUCKET-7c82f257c4fb1103, ESLINT-BUCKET-c4dd2be3483455e5, ESLINT-BUCKET-5c8c07852ed0b107
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_201`
  - `mkdir -p build/eng-artifacts/T7.201`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/AddUserModal.jsx web/js/components/preact/users/ApiKeyModal.jsx web/js/components/preact/users/AuditHistoryModal.jsx web/js/components/preact/users/DeleteUserModal.jsx web/js/components/preact/users/EditUserModal.jsx web/js/components/preact/users/RoleManagerModal.jsx --format json --output-file build/eng-artifacts/T7.201/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.201/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.201`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.202: jsx-a11y/no-static-element-interactions in TotpSetupModal.jsx
- **depends_on**: [T7, T10, T7.192]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/users/TotpSetupModal.jsx`, `web/tests/remediation/t7_202.spec.jsx`, `quality/evidence/T7.202.md`.
- **priority**: low
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: ESLINT-BUCKET-e046fbe269eed0dc
- **acceptance_criteria**:
  - Make reported actions native interactive controls or provide correct role, focus and keyboard activation; pointer and keyboard must execute the same action once.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_202`
  - `mkdir -p build/eng-artifacts/T7.202`
  - `web/node_modules/.bin/eslint web/js/components/preact/users/TotpSetupModal.jsx --format json --output-file build/eng-artifacts/T7.202/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.202/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'jsx-a11y/no-static-element-interactions'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.202`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.203: nullPointerArithmeticRedundantCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_203-upstream.json`, `quality/evidence/T7.203.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-38d664fd8e77f607, CPPCHECK-3d1693ad77a15fa2, CPPCHECK-3508a7db3ef7fc03, CPPCHECK-ca79e52beb5f22bb, CPPCHECK-5941fdb9ccb989f3
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_203-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.203`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.204: nullPointerRedundantCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_204-upstream.json`, `quality/evidence/T7.204.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-10a0fa2f24fefc90, CPPCHECK-ffdd97ac6ebb6b4b, CPPCHECK-7dc87bf617f52146, CPPCHECK-347fec696ca3fcab, CPPCHECK-1980a8233237856a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_204-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.204`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.205: wrongPrintfScanfArgNum in linux_thermal.c
- **depends_on**: [T7, T13, T7.95]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_205.c`, `tests/unit/cmake/eng/t7_205.cmake`, `quality/evidence/T7.205.md`.
- **priority**: low
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-e9cd8f29e136cbb8
- **acceptance_criteria**:
  - Exercise every reported format/library call with boundary values; argument types/counts, buffer extents and returned errors must match the library contract with no UB.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.205 --regex '^test_t7_205$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.205`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.206: zerodivcond in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_206-upstream.json`, `quality/evidence/T7.206.md`.
- **priority**: low
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-ac6f81ae03a685f3
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_206-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.206`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.207: arrayIndexThenCheck in sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_207-upstream.json`, `quality/evidence/T7.207.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-8bb35cef096bd68d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_207-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.207`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.208: constParameter in linux_network.c
- **depends_on**: [T7, T13, T7.95]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `tests/unit/test_t7_208.c`, `tests/unit/cmake/eng/t7_208.cmake`, `quality/evidence/T7.208.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-7e66ded89b37cb4f
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.208 --regex '^test_t7_208$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.208`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.209: constParameter in api_handlers_investigations.c
- **depends_on**: [T7, T13, T7.148]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_investigations.c`, `tests/unit/test_t7_209.c`, `tests/unit/cmake/eng/t7_209.cmake`, `quality/evidence/T7.209.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c6d1df4e6770f40e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.209 --regex '^test_t7_209$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.209`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.210: constParameterCallback in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_210-upstream.json`, `quality/evidence/T7.210.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-fb37582a72f59582, CPPCHECK-BUCKET-c7b3ef530f996e26
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_210-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.210`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.211: constParameterCallback in storage_migration.c
- **depends_on**: [T7, T13, T7.138]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_211.c`, `tests/unit/cmake/eng/t7_211.cmake`, `quality/evidence/T7.211.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-90ec02278e01bcac
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.211 --regex '^test_t7_211$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.211`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.212: constParameterCallback in yaml_redact.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/yaml_redact.c`, `tests/unit/test_t7_212.c`, `tests/unit/cmake/eng/t7_212.cmake`, `quality/evidence/T7.212.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-6291a48798ed350c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.212 --regex '^test_t7_212$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.212`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.213: constParameterCallback in api_detection.c
- **depends_on**: [T7, T13, T7.112]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_213.c`, `tests/unit/cmake/eng/t7_213.cmake`, `quality/evidence/T7.213.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-681aedac30d87a1e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.213 --regex '^test_t7_213$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.213`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.214: constParameterCallback in go2rtc_api.c, go2rtc_integration.c, go2rtc_snapshot.c
- **depends_on**: [T7, T13, T7.98, T7.154]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_api.c`, `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_snapshot.c`, `tests/unit/test_t7_214.c`, `tests/unit/cmake/eng/t7_214.cmake`, `quality/evidence/T7.214.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-ee781b371da39bf5, CPPCHECK-BUCKET-dc056676c7084f53, CPPCHECK-BUCKET-0130dae8fb623afa
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.214 --regex '^test_t7_214$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.214`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.215: constParameterCallback in onvif_detection.c
- **depends_on**: [T7, T13, T7.146]
- **agent_type**: worker
- **ownership**: `src/video/onvif_detection.c`, `tests/unit/test_t7_215.c`, `tests/unit/cmake/eng/t7_215.cmake`, `quality/evidence/T7.215.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-365ebbda46eacf74
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.215 --regex '^test_t7_215$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.215`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.216: constParameterCallback in onvif_device_management.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_device_management.c`, `tests/unit/test_t7_216.c`, `tests/unit/cmake/eng/t7_216.cmake`, `quality/evidence/T7.216.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8737ababcc06f484
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.216 --regex '^test_t7_216$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.216`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.217: constParameterCallback in onvif_imaging.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/onvif_imaging.c`, `tests/unit/test_t7_217.c`, `tests/unit/cmake/eng/t7_217.cmake`, `quality/evidence/T7.217.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-baf28ba17bc25dd0
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.217 --regex '^test_t7_217$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.217`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.218: constParameterCallback in onvif_ptz.c
- **depends_on**: [T7, T13, T7.156]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_218.c`, `tests/unit/cmake/eng/t7_218.cmake`, `quality/evidence/T7.218.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-462bf13ae3867115
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.218 --regex '^test_t7_218$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.218`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.219: constParameterCallback in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.147]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_219.c`, `tests/unit/cmake/eng/t7_219.cmake`, `quality/evidence/T7.219.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b6f82c4c9b287eba
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.219 --regex '^test_t7_219$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.219`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.220: constParameterCallback in api_handlers_health.c
- **depends_on**: [T7, T13, T7.158]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_health.c`, `tests/unit/test_t7_220.c`, `tests/unit/cmake/eng/t7_220.cmake`, `quality/evidence/T7.220.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c7a8e9a51802c60d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.220 --regex '^test_t7_220$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.220`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.221: constParameterCallback in go2rtc_proxy_thread.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_proxy_thread.c`, `tests/unit/test_t7_221.c`, `tests/unit/cmake/eng/t7_221.cmake`, `quality/evidence/T7.221.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d3985740b2ac6585
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.221 --regex '^test_t7_221$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.221`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.222: constParameterPointer in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_222-upstream.json`, `quality/evidence/T7.222.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-82a085f0cc04468f, CPPCHECK-BUCKET-27ab7ab1e74c2e72
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_222-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.222`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.223: constParameterPointer in authorization.c
- **depends_on**: [T7, T13, T7.114]
- **agent_type**: worker
- **ownership**: `src/core/authorization.c`, `tests/unit/test_t7_223.c`, `tests/unit/cmake/eng/t7_223.cmake`, `quality/evidence/T7.223.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d0de2e4ac47c3191
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.223 --regex '^test_t7_223$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.223`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.224: constParameterPointer in mqtt_destination_client.c
- **depends_on**: [T7, T13, T7.134]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_224.c`, `tests/unit/cmake/eng/t7_224.cmake`, `quality/evidence/T7.224.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-fbb9e9402242eaba
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.224 --regex '^test_t7_224$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.224`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.225: constParameterPointer in db_camera_collections.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_camera_collections.c`, `tests/unit/test_t7_225.c`, `tests/unit/cmake/eng/t7_225.cmake`, `quality/evidence/T7.225.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-fd8d6a4b8cc77400
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.225 --regex '^test_t7_225$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.225`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.226: constParameterPointer in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_226-upstream.json`, `quality/evidence/T7.226.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-43fcd9340a9e63ed
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_226-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.226`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.227: constParameterPointer in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_227-upstream.json`, `quality/evidence/T7.227.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-e7f8f80f0cd95f0d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_227-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.227`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.228: constParameterPointer in linux_cgroup.c
- **depends_on**: [T7, T13, T7.83]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `tests/unit/test_t7_228.c`, `tests/unit/cmake/eng/t7_228.cmake`, `quality/evidence/T7.228.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-70e0496568faa30f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.228 --regex '^test_t7_228$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.228`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.229: constParameterPointer in nvme_provider.c
- **depends_on**: [T7, T13, T7.76]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_229.c`, `tests/unit/cmake/eng/t7_229.cmake`, `quality/evidence/T7.229.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5bdf073f41a6c65d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.229 --regex '^test_t7_229$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.229`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.230: constParameterPointer in system_health_evaluator.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_evaluator.c`, `tests/unit/test_t7_230.c`, `tests/unit/cmake/eng/t7_230.cmake`, `quality/evidence/T7.230.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-527aa4750d3ac84a
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.230 --regex '^test_t7_230$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.230`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.231: constParameterPointer in go2rtc_base_path.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/go2rtc_base_path.c`, `tests/unit/test_t7_231.c`, `tests/unit/cmake/eng/t7_231.cmake`, `quality/evidence/T7.231.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d9684d3aa203568c
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.231 --regex '^test_t7_231$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.231`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.232: constParameterPointer in strings.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/utils/strings.c`, `tests/unit/test_t7_232.c`, `tests/unit/cmake/eng/t7_232.cmake`, `quality/evidence/T7.232.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-e228634622467328
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.232 --regex '^test_t7_232$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.232`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.233: constParameterPointer in faac_encoder.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/faac_encoder.c`, `tests/unit/test_t7_233.c`, `tests/unit/cmake/eng/t7_233.cmake`, `quality/evidence/T7.233.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-bdb708e597b5cade
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.233 --regex '^test_t7_233$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.233`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.234: constParameterPointer in hls_unified_thread.c
- **depends_on**: [T7, T13, T7.69]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_unified_thread.c`, `tests/unit/test_t7_234.c`, `tests/unit/cmake/eng/t7_234.cmake`, `quality/evidence/T7.234.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-cbab871259b9963d
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.234 --regex '^test_t7_234$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.234`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.235: constParameterPointer in unified_detection_thread.c
- **depends_on**: [T7, T13, T7.122]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_235.c`, `tests/unit/cmake/eng/t7_235.cmake`, `quality/evidence/T7.235.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8c644cc847574bfe
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.235 --regex '^test_t7_235$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.235`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.236: constParameterPointer in go2rtc_websocket.c
- **depends_on**: [T7, T13, T7.150]
- **agent_type**: worker
- **ownership**: `src/web/go2rtc_websocket.c`, `tests/unit/test_t7_236.c`, `tests/unit/cmake/eng/t7_236.cmake`, `quality/evidence/T7.236.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4c0052bc9de6e496
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.236 --regex '^test_t7_236$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.236`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.237: constParameterPointer in recording_archive.c
- **depends_on**: [T7, T13, T7.141]
- **agent_type**: worker
- **ownership**: `src/web/recording_archive.c`, `tests/unit/test_t7_237.c`, `tests/unit/cmake/eng/t7_237.cmake`, `quality/evidence/T7.237.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-45cfee19afe8668f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.237 --regex '^test_t7_237$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.237`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.238: constVariable in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_238-upstream.json`, `quality/evidence/T7.238.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-933d7ae37be3c395, CPPCHECK-BUCKET-b1c0c559c6e17a23
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_238-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.238`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.239: constVariable in mqtt_delivery_worker.c
- **depends_on**: [T7, T13, T7.91]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_delivery_worker.c`, `tests/unit/test_t7_239.c`, `tests/unit/cmake/eng/t7_239.cmake`, `quality/evidence/T7.239.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8596888f0dd1cc7e
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.239 --regex '^test_t7_239$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.239`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.240: constVariable in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_240-upstream.json`, `quality/evidence/T7.240.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-298feb4a490d6252
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_240-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.240`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.241: constVariable in sod_ingenic_mxu.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_241-upstream.json`, `quality/evidence/T7.241.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-a8ce2c3168c021de
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_241-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.241`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.242: constVariablePointer in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_242-upstream.json`, `quality/evidence/T7.242.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-9575437f939001d5, CPPCHECK-BUCKET-df230b0a5672f192
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_242-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.242`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.243: constVariablePointer in path_utils.c
- **depends_on**: [T7, T13, T7.128]
- **agent_type**: worker
- **ownership**: `src/core/path_utils.c`, `tests/unit/test_t7_243.c`, `tests/unit/cmake/eng/t7_243.cmake`, `quality/evidence/T7.243.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-8e110c5360beb299
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.243 --regex '^test_t7_243$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.243`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.244: constVariablePointer in db_auth.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_auth.c`, `tests/unit/test_t7_244.c`, `tests/unit/cmake/eng/t7_244.cmake`, `quality/evidence/T7.244.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-6c5cf9bd3f338f84
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.244 --regex '^test_t7_244$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.244`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.245: constVariablePointer in db_authorization.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_authorization.c`, `tests/unit/test_t7_245.c`, `tests/unit/cmake/eng/t7_245.cmake`, `quality/evidence/T7.245.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-08f60b7ec76ce9d3
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.245 --regex '^test_t7_245$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.245`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.246: constVariablePointer in db_camera_tags.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_camera_tags.c`, `tests/unit/test_t7_246.c`, `tests/unit/cmake/eng/t7_246.cmake`, `quality/evidence/T7.246.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-7e7f6cc12d937123
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.246 --regex '^test_t7_246$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.246`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.247: constVariablePointer in db_core.c
- **depends_on**: [T7, T13, T7.25]
- **agent_type**: worker
- **ownership**: `src/database/db_core.c`, `tests/unit/test_t7_247.c`, `tests/unit/cmake/eng/t7_247.cmake`, `quality/evidence/T7.247.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f21c65912e355ace
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.247 --regex '^test_t7_247$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.247`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.248: constVariablePointer in db_fleet_query.c
- **depends_on**: [T7, T13, T7.113]
- **agent_type**: worker
- **ownership**: `src/database/db_fleet_query.c`, `tests/unit/test_t7_248.c`, `tests/unit/cmake/eng/t7_248.cmake`, `quality/evidence/T7.248.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9f651ab0c8ea6422
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.248 --regex '^test_t7_248$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.248`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.249: constVariablePointer in db_storage_lifecycle.c
- **depends_on**: [T7, T13, T7.129]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_lifecycle.c`, `tests/unit/test_t7_249.c`, `tests/unit/cmake/eng/t7_249.cmake`, `quality/evidence/T7.249.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-353deb8bba479029
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.249 --regex '^test_t7_249$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.249`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.250: constVariablePointer in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_250-upstream.json`, `quality/evidence/T7.250.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-c6ee03168003661f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_250-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.250`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.251: constVariablePointer in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_251-upstream.json`, `quality/evidence/T7.251.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-5d8aaf042fefce78
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_251-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.251`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.252: constVariablePointer in storage_deletion.c
- **depends_on**: [T7, T13, T7.137]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_252.c`, `tests/unit/cmake/eng/t7_252.cmake`, `quality/evidence/T7.252.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b4e128a1af325d4f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.252 --regex '^test_t7_252$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.252`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.253: constVariablePointer in storage_source.c
- **depends_on**: [T7, T13, T7.140]
- **agent_type**: worker
- **ownership**: `src/storage/storage_source.c`, `tests/unit/test_t7_253.c`, `tests/unit/cmake/eng/t7_253.cmake`, `quality/evidence/T7.253.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-77479b4ba7ef9849
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.253 --regex '^test_t7_253$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.253`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.254: constVariablePointer in linux_network.c, linux_process.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.208, T7.95, T7.205]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_network.c`, `src/telemetry/collectors/linux_process.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_254.c`, `tests/unit/cmake/eng/t7_254.cmake`, `quality/evidence/T7.254.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-c9404ed2a73b0977, CPPCHECK-BUCKET-e924700b1413accd, CPPCHECK-BUCKET-2ca7413944026780
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.254 --regex '^test_t7_254$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.254`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.255: constVariablePointer in linux_device_map.c, linux_hardware.c, nvme_provider.c
- **depends_on**: [T7, T13, T7.143, T7.229]
- **agent_type**: worker
- **ownership**: `src/telemetry/providers/linux_device_map.c`, `src/telemetry/providers/linux_hardware.c`, `src/telemetry/providers/nvme_provider.c`, `tests/unit/test_t7_255.c`, `tests/unit/cmake/eng/t7_255.cmake`, `quality/evidence/T7.255.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a1edb744b382f8f5, CPPCHECK-BUCKET-4001e3b91223c7ad, CPPCHECK-BUCKET-ae15015525d96c08
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.255 --regex '^test_t7_255$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.255`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.256: constVariablePointer in system_health.c
- **depends_on**: [T7, T13, T7.59]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health.c`, `tests/unit/test_t7_256.c`, `tests/unit/cmake/eng/t7_256.cmake`, `quality/evidence/T7.256.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f3bb2108694b562f
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.256 --regex '^test_t7_256$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.256`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.257: constVariablePointer in system_health_evaluator.c
- **depends_on**: [T7, T13, T7.230]
- **agent_type**: worker
- **ownership**: `src/telemetry/system_health_evaluator.c`, `tests/unit/test_t7_257.c`, `tests/unit/cmake/eng/t7_257.cmake`, `quality/evidence/T7.257.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-57093ce27e4aabc5
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.257 --regex '^test_t7_257$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.257`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.258: constVariablePointer in go2rtc_base_path.c
- **depends_on**: [T7, T13, T7.231]
- **agent_type**: worker
- **ownership**: `src/utils/go2rtc_base_path.c`, `tests/unit/test_t7_258.c`, `tests/unit/cmake/eng/t7_258.cmake`, `quality/evidence/T7.258.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4dc0544d99c6a120
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.258 --regex '^test_t7_258$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.258`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.259: constVariablePointer in detection_model.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/detection_model.c`, `tests/unit/test_t7_259.c`, `tests/unit/cmake/eng/t7_259.cmake`, `quality/evidence/T7.259.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-aa0082260172b3b9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.259 --regex '^test_t7_259$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.259`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.260: constVariablePointer in go2rtc_integration.c, go2rtc_process.c
- **depends_on**: [T7, T13, T7.214, T7.145]
- **agent_type**: worker
- **ownership**: `src/video/go2rtc/go2rtc_integration.c`, `src/video/go2rtc/go2rtc_process.c`, `tests/unit/test_t7_260.c`, `tests/unit/cmake/eng/t7_260.cmake`, `quality/evidence/T7.260.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5f736ad7b9095aea, CPPCHECK-BUCKET-eabea03354c7b4b7
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.260 --regex '^test_t7_260$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.260`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.261: constVariablePointer in mp4_recording_core.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_recording_core.c`, `tests/unit/test_t7_261.c`, `tests/unit/cmake/eng/t7_261.cmake`, `quality/evidence/T7.261.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-e9a38e72fa2e4a7c
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.261 --regex '^test_t7_261$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.261`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.262: constVariablePointer in mp4_recording_writer.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/mp4_recording_writer.c`, `tests/unit/test_t7_262.c`, `tests/unit/cmake/eng/t7_262.cmake`, `quality/evidence/T7.262.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-3ec7089cfaa16e94
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.262 --regex '^test_t7_262$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.262`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.263: constVariablePointer in stream_state_adapter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/stream_state_adapter.c`, `tests/unit/test_t7_263.c`, `tests/unit/cmake/eng/t7_263.cmake`, `quality/evidence/T7.263.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-4ffd5c5d72b2c6a0
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.263 --regex '^test_t7_263$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.263`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.264: constVariablePointer in api_handlers_go2rtc_proxy.c
- **depends_on**: [T7, T13, T7.219]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_go2rtc_proxy.c`, `tests/unit/test_t7_264.c`, `tests/unit/cmake/eng/t7_264.cmake`, `quality/evidence/T7.264.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f4469b09cc19dae9
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.264 --regex '^test_t7_264$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.264`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.265: constVariablePointer in api_handlers_recordings_list.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recordings_list.c`, `tests/unit/test_t7_265.c`, `tests/unit/cmake/eng/t7_265.cmake`, `quality/evidence/T7.265.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-611000c5c122ad1e
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.265 --regex '^test_t7_265$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.265`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.266: constVariablePointer in api_handlers_recordings_thumbnail.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_recordings_thumbnail.c`, `tests/unit/test_t7_266.c`, `tests/unit/cmake/eng/t7_266.cmake`, `quality/evidence/T7.266.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d41611e4fb1c1abd
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.266 --regex '^test_t7_266$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.266`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.267: constVariablePointer in api_handlers_system_logs.c
- **depends_on**: [T7, T13, T7.149]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_system_logs.c`, `tests/unit/test_t7_267.c`, `tests/unit/cmake/eng/t7_267.cmake`, `quality/evidence/T7.267.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b14c25475c91b307
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.267 --regex '^test_t7_267$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.267`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.268: constVariablePointer in api_handlers_timeline.c
- **depends_on**: [T7, T13, T7.107]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_timeline.c`, `tests/unit/test_t7_268.c`, `tests/unit/cmake/eng/t7_268.cmake`, `quality/evidence/T7.268.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-03f92e362a140589
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.268 --regex '^test_t7_268$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.268`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.269: constVariablePointer in api_handlers_users_backend_agnostic.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_users_backend_agnostic.c`, `tests/unit/test_t7_269.c`, `tests/unit/cmake/eng/t7_269.cmake`, `quality/evidence/T7.269.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d45c6fa42b05db96
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.269 --regex '^test_t7_269$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.269`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.270: constVariablePointer in httpd_utils.c
- **depends_on**: [T7, T13, T7.133]
- **agent_type**: worker
- **ownership**: `src/web/httpd_utils.c`, `tests/unit/test_t7_270.c`, `tests/unit/cmake/eng/t7_270.cmake`, `quality/evidence/T7.270.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9551a431943d8273
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.270 --regex '^test_t7_270$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.270`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.271: duplicateAssignExpression in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_271-upstream.json`, `quality/evidence/T7.271.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-86335be98f6f2e90
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_271-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.271`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.272: duplicateCondition in motion_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/motion_detection.c`, `tests/unit/test_t7_272.c`, `tests/unit/cmake/eng/t7_272.cmake`, `quality/evidence/T7.272.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-f814692d9029a8a1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.272 --regex '^test_t7_272$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.272`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.273: js/unused-local-variable in LanguageSelector.jsx
- **depends_on**: [T7, T10, T7.196]
- **agent_type**: worker
- **ownership**: `web/js/components/preact/common/LanguageSelector.jsx`, `web/tests/remediation/t7_273.spec.jsx`, `quality/evidence/T7.273.md`.
- **priority**: info
- **description**: Reproduce the listed diagnostics and fix this accessibility/quality cause only in the assigned component cluster. Preserve localization, event callbacks, DOM contracts and extension-sensitive test behavior. Narrow an untestable lint-only case with documented static evidence; behavioral accessibility changes require interaction tests.
- **findings**: CODEQUALITY-941
- **acceptance_criteria**:
  - Remove only the proven unused binding; assert language selection/persistence and localized rendering still work.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `npm --prefix web test -- --runInBand --testPathPatterns=t7_273`
  - `mkdir -p build/eng-artifacts/T7.273`
  - `web/node_modules/.bin/eslint web/js/components/preact/common/LanguageSelector.jsx --format json --output-file build/eng-artifacts/T7.273/eslint.json`
  - `python3 -c 'import json; data=json.load(open('"'"'build/eng-artifacts/T7.273/eslint.json'"'"')); assert not [m for f in data for m in f["messages"] if m.get("ruleId")=='"'"'js/unused-local-variable'"'"'], "targeted diagnostic remains"'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.273`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.274: knownConditionTrueFalse in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_274-upstream.json`, `quality/evidence/T7.274.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-1f586f8741e770e9
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_274-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.274`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.275: knownConditionTrueFalse in event_router.c
- **depends_on**: [T7, T13, T7.90]
- **agent_type**: worker
- **ownership**: `src/core/event_router.c`, `tests/unit/test_t7_275.c`, `tests/unit/cmake/eng/t7_275.cmake`, `quality/evidence/T7.275.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-95058fe138843b1f
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.275 --regex '^test_t7_275$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.275`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.276: knownConditionTrueFalse in mqtt_event_adapter.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_event_adapter.c`, `tests/unit/test_t7_276.c`, `tests/unit/cmake/eng/t7_276.cmake`, `quality/evidence/T7.276.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9c71189558451cd4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.276 --regex '^test_t7_276$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.276`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.277: knownConditionTrueFalse in db_storage_targets.c
- **depends_on**: [T7, T13, T7.136]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_277.c`, `tests/unit/cmake/eng/t7_277.cmake`, `quality/evidence/T7.277.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-2f8dc61aeb7cdda6
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.277 --regex '^test_t7_277$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.277`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.278: knownConditionTrueFalse in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_278-upstream.json`, `quality/evidence/T7.278.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-c7e9176ca688d280
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_278-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.278`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.279: knownConditionTrueFalse in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_279-upstream.json`, `quality/evidence/T7.279.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-064299ecf7d09b55
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_279-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.279`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.280: knownConditionTrueFalse in linux_process.c
- **depends_on**: [T7, T13, T7.254]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_process.c`, `tests/unit/test_t7_280.c`, `tests/unit/cmake/eng/t7_280.cmake`, `quality/evidence/T7.280.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-28b7ea4b18c686f4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.280 --regex '^test_t7_280$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.280`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.281: knownConditionTrueFalse in health_helper_runner.c
- **depends_on**: [T7, T13, T7.68]
- **agent_type**: worker
- **ownership**: `src/telemetry/health_helper_runner.c`, `tests/unit/test_t7_281.c`, `tests/unit/cmake/eng/t7_281.cmake`, `quality/evidence/T7.281.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-1ac7ae81e97d3df2
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.281 --regex '^test_t7_281$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.281`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.282: knownConditionTrueFalse in api_handlers_imaging.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_imaging.c`, `tests/unit/test_t7_282.c`, `tests/unit/cmake/eng/t7_282.cmake`, `quality/evidence/T7.282.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-0251d17e5b89df2a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.282 --regex '^test_t7_282$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.282`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.283: redundantCondition in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_283-upstream.json`, `quality/evidence/T7.283.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-eee46ed30daef891
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_283-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.283`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.284: redundantInitialization in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_284-upstream.json`, `quality/evidence/T7.284.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-dcdbd8e6357d7e12
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_284-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.284`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.285: redundantInitialization in db_authorization.c
- **depends_on**: [T7, T13, T7.245]
- **agent_type**: worker
- **ownership**: `src/database/db_authorization.c`, `tests/unit/test_t7_285.c`, `tests/unit/cmake/eng/t7_285.cmake`, `quality/evidence/T7.285.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-bbeef7abfa51d073
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.285 --regex '^test_t7_285$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.285`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.286: redundantInitialization in recording_path.c
- **depends_on**: [T7, T13, T7.157]
- **agent_type**: worker
- **ownership**: `src/video/recording_path.c`, `tests/unit/test_t7_286.c`, `tests/unit/cmake/eng/t7_286.cmake`, `quality/evidence/T7.286.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-0be349a4e3fc0518
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.286 --regex '^test_t7_286$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.286`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.287: redundantPointerOp in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_287-upstream.json`, `quality/evidence/T7.287.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-8f929a70e127f155
- **acceptance_criteria**:
  - Reproduce each reported null/empty/one-past-boundary or incompatible-pointer path under ASan/UBSan; assert no invalid dereference/read/write and correct valid-path result, terminator and ownership cleanup.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_287-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.287`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.288: shadowFunction in db_storage_targets.c
- **depends_on**: [T7, T13, T7.277]
- **agent_type**: worker
- **ownership**: `src/database/db_storage_targets.c`, `tests/unit/test_t7_288.c`, `tests/unit/cmake/eng/t7_288.cmake`, `quality/evidence/T7.288.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-147fba26f7ae351b
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.288 --regex '^test_t7_288$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.288`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.289: shadowFunction in stream_manager.c
- **depends_on**: [T7, T13, T7.62]
- **agent_type**: worker
- **ownership**: `src/video/stream_manager.c`, `tests/unit/test_t7_289.c`, `tests/unit/cmake/eng/t7_289.cmake`, `quality/evidence/T7.289.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-05b22ae697eaa4ca
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.289 --regex '^test_t7_289$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.289`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.290: shadowVariable in config.h
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `include/core/config.h`, `tests/unit/test_t7_290.c`, `tests/unit/cmake/eng/t7_290.cmake`, `quality/evidence/T7.290.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9d2c058ee958f85a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.290 --regex '^test_t7_290$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.290`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.291: unpreciseMathCall in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_291-upstream.json`, `quality/evidence/T7.291.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-426773bc9bc5a5d5
- **acceptance_criteria**:
  - Exercise signed/unsigned boundaries, negative/zero values, maximum accepted values and multiplication overflow before conversion; assert valid results are unchanged and invalid values cannot wrap, divide by zero or allocate oversized memory.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_291-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.291`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.292: unreadVariable in sod_img_reader.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_292-upstream.json`, `quality/evidence/T7.292.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-eda9b7001501e847
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_292-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.292`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.293: unreadVariable in mqtt_client.c
- **depends_on**: [T7, T13, T7.29]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_client.c`, `tests/unit/test_t7_293.c`, `tests/unit/cmake/eng/t7_293.cmake`, `quality/evidence/T7.293.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-779ca0994d939da1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.293 --regex '^test_t7_293$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.293`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.294: unreadVariable in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_294-upstream.json`, `quality/evidence/T7.294.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-93112ea9a6a3c10d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_294-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.294`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.295: unreadVariable in onvif_ptz.c
- **depends_on**: [T7, T13, T7.218]
- **agent_type**: worker
- **ownership**: `src/video/onvif_ptz.c`, `tests/unit/test_t7_295.c`, `tests/unit/cmake/eng/t7_295.cmake`, `quality/evidence/T7.295.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b1b17a4b9a2646a0
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.295 --regex '^test_t7_295$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.295`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.296: unsignedLessThanZero in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_296-upstream.json`, `quality/evidence/T7.296.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-a4ae78869ec196b6
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_296-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.296`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.297: unusedStructMember in mqtt_destination_client.c
- **depends_on**: [T7, T13, T7.224]
- **agent_type**: worker
- **ownership**: `src/core/mqtt_destination_client.c`, `tests/unit/test_t7_297.c`, `tests/unit/cmake/eng/t7_297.cmake`, `quality/evidence/T7.297.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-efd7988ca87c95ed
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.297 --regex '^test_t7_297$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.297`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.298: unusedStructMember in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_298-upstream.json`, `quality/evidence/T7.298.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-49117790281d294c
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_298-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.298`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.299: unusedStructMember in sod_detection.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/sod_detection.c`, `tests/unit/test_t7_299.c`, `tests/unit/cmake/eng/t7_299.cmake`, `quality/evidence/T7.299.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a12a87ba590d2536
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.299 --regex '^test_t7_299$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.299`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.300: uselessAssignmentArg in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_300-upstream.json`, `quality/evidence/T7.300.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-56c122b53a2e50cb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_300-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.300`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.301: variableScope in sod_img_reader.h, sod_img_writer.h
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_301-upstream.json`, `quality/evidence/T7.301.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-ae4dbf08b9cfd1fe, CPPCHECK-BUCKET-b9ff6009cfa26f69
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_301-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.301`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.302: variableScope in db_detection_engines.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_detection_engines.c`, `tests/unit/test_t7_302.c`, `tests/unit/cmake/eng/t7_302.cmake`, `quality/evidence/T7.302.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-b614f3b94230fe9a
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.302 --regex '^test_t7_302$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.302`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.303: variableScope in db_lpr_reads.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_lpr_reads.c`, `tests/unit/test_t7_303.c`, `tests/unit/cmake/eng/t7_303.cmake`, `quality/evidence/T7.303.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-664e70c550c3ef36
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.303 --regex '^test_t7_303$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.303`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.304: variableScope in db_recordings.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/database/db_recordings.c`, `tests/unit/test_t7_304.c`, `tests/unit/cmake/eng/t7_304.cmake`, `quality/evidence/T7.304.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-2afdb0b1f1146eeb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.304 --regex '^test_t7_304$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.304`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.305: variableScope in ezxml.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_305-upstream.json`, `quality/evidence/T7.305.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-96d5b13210206feb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_305-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.305`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.306: variableScope in sod.c
- **depends_on**: [T7, T13]
- **agent_type**: local
- **ownership**: `quality/reviews/t7_306-upstream.json`, `quality/evidence/T7.306.md`.
- **priority**: info
- **description**: Review pinned upstream source and reproduce the listed diagnostics in the actual linked feature profile. Record precise upstream fix/version or evidence-backed false-positive analysis. Do not edit system headers or vendored/submodule source as an untracked patch. If an update/local patch is needed, append an exact bounded integration child, dependency and regression command to T33 before closing this task. Missing upstream or feature evidence blocks completion.
- **findings**: CPPCHECK-BUCKET-50bc6e286ac69abd
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 -m json.tool quality/reviews/t7_306-upstream.json`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.306`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.307: variableScope in storage_deletion.c
- **depends_on**: [T7, T13, T7.252]
- **agent_type**: worker
- **ownership**: `src/storage/storage_deletion.c`, `tests/unit/test_t7_307.c`, `tests/unit/cmake/eng/t7_307.cmake`, `quality/evidence/T7.307.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-89a47a59067748c1
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.307 --regex '^test_t7_307$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.307`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.308: variableScope in storage_migration.c
- **depends_on**: [T7, T13, T7.211]
- **agent_type**: worker
- **ownership**: `src/storage/storage_migration.c`, `tests/unit/test_t7_308.c`, `tests/unit/cmake/eng/t7_308.cmake`, `quality/evidence/T7.308.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-da5006735d5c5165
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.308 --regex '^test_t7_308$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.308`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.309: variableScope in storage_placement.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/storage/storage_placement.c`, `tests/unit/test_t7_309.c`, `tests/unit/cmake/eng/t7_309.cmake`, `quality/evidence/T7.309.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-51dd1a644803f48d
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.309 --regex '^test_t7_309$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.309`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.310: variableScope in storage_s3.c
- **depends_on**: [T7, T13, T7.142]
- **agent_type**: worker
- **ownership**: `src/storage/storage_s3.c`, `tests/unit/test_t7_310.c`, `tests/unit/cmake/eng/t7_310.cmake`, `quality/evidence/T7.310.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-a3e8d56a93d61112
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.310 --regex '^test_t7_310$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.310`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.311: variableScope in linux_cgroup.c, linux_thermal.c
- **depends_on**: [T7, T13, T7.228, T7.254]
- **agent_type**: worker
- **ownership**: `src/telemetry/collectors/linux_cgroup.c`, `src/telemetry/collectors/linux_thermal.c`, `tests/unit/test_t7_311.c`, `tests/unit/cmake/eng/t7_311.cmake`, `quality/evidence/T7.311.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-771b3fde1937be8a, CPPCHECK-BUCKET-e4dae70b5d1bd770
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.311 --regex '^test_t7_311$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.311`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.312: variableScope in stream_metrics.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/telemetry/stream_metrics.c`, `tests/unit/test_t7_312.c`, `tests/unit/cmake/eng/t7_312.cmake`, `quality/evidence/T7.312.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-9cc73ce911540ad4
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.312 --regex '^test_t7_312$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.312`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.313: variableScope in api_detection.c
- **depends_on**: [T7, T13, T7.213]
- **agent_type**: worker
- **ownership**: `src/video/api_detection.c`, `tests/unit/test_t7_313.c`, `tests/unit/cmake/eng/t7_313.cmake`, `quality/evidence/T7.313.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-905555e786df0459
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.313 --regex '^test_t7_313$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.313`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.314: variableScope in hls_directory.c
- **depends_on**: [T7, T13]
- **agent_type**: worker
- **ownership**: `src/video/hls/hls_directory.c`, `tests/unit/test_t7_314.c`, `tests/unit/cmake/eng/t7_314.cmake`, `quality/evidence/T7.314.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-5d0aacd6263110c5
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.314 --regex '^test_t7_314$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.314`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.315: variableScope in recording_transcode.c
- **depends_on**: [T7, T13, T7.66]
- **agent_type**: worker
- **ownership**: `src/video/recording_transcode.c`, `tests/unit/test_t7_315.c`, `tests/unit/cmake/eng/t7_315.cmake`, `quality/evidence/T7.315.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-ec17ee32f6d27cdb
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.315 --regex '^test_t7_315$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.315`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.316: variableScope in unified_detection_thread.c
- **depends_on**: [T7, T13, T7.235]
- **agent_type**: worker
- **ownership**: `src/video/unified_detection_thread.c`, `tests/unit/test_t7_316.c`, `tests/unit/cmake/eng/t7_316.cmake`, `quality/evidence/T7.316.md`.
- **priority**: info
- **description**: Reproduce each listed diagnostic from its raw T1 source report and trace its caller/ownership before changing code. Fix the bounded cause, write focused native regressions and register the dedicated test through the T13 eng fragment seam. These diagnostics are hypotheses until reproduced: if sites have distinct root causes, split explicit child tasks with exact ownership and add them to T33; no catch-all rewrite or blanket suppression.
- **findings**: CPPCHECK-BUCKET-d4393cc822a0c2d3
- **acceptance_criteria**:
  - Inspect the exact reported expression and caller invariants. Add a focused before/after behavioral assertion for the affected branch or document why the change is semantic-preserving and verify compiler/scanner diagnostics disappear. Never delete side effects, suppress a rule wholesale or claim a warning count reduction proves correctness.
  - Each mapped finding has individual fixed-with-regression evidence or an explicitly approved, current, narrowly scoped exception; unobserved/excluded diagnostics are not fixed.
  - Preserve unrelated baseline diagnostics. Final full-profile fresh reports and optional feature coverage are reconciled in T33.
- **validation**:
  - `python3 scripts/quality/run-native.py --task T7.316 --regex '^test_t7_316$'`
  - Reproducer locations and scanner fingerprints: `quality/remediation-map.json` task `T7.316`; inspect the raw T1 report and trace the caller.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.317: Authenticate before exposing stream existence
- **depends_on**: [T7, T11, T13, T7.64, T7.50]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_317.api.spec.ts`, `quality/evidence/T7.317.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-01
- **acceptance_criteria**:
  - Unauthenticated known and unknown names both return the authentication failure; authenticated unauthorized users retain intentional policy response without leaking privileged data.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.317 --project api --grep @t7_317`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.318: Correct documented Streams wire examples
- **depends_on**: [T7, T11, T7.319, T7.320, T7.321, T7.322, T7.323, T7.324, T7.325, T7.326, T7.327, T7.328, T7.329, T7.330, T7.331, T7.332, T7.333]
- **agent_type**: worker
- **ownership**: `docs/API.md`, `quality/evidence/T7.318.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Update documentation only after the wire behavior is reconciled.
- **findings**: F-T11-02
- **acceptance_criteria**:
  - Examples use the real array/page envelopes, integer percentage threshold and emitted names/statuses; every example maps to the revised contract.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `python3 scripts/check_docs_links.py`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.319: Remove query credentials from viewer URLs
- **depends_on**: [T7, T11, T13, T7.317]
- **agent_type**: worker
- **ownership**: `src/core/url_utils.c`, `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_319.api.spec.ts`, `quality/evidence/T7.319.md`.
- **priority**: high
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-03
- **acceptance_criteria**:
  - Viewer responses contain neither userinfo nor sensitive query credentials, including encoded/mixed-case keys; configurers retain required edit data and benign query options remain usable.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.319 --project api --grep @t7_319`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.320: Reject non-object stream updates
- **depends_on**: [T7, T11, T13, T7.317]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_320.api.spec.ts`, `quality/evidence/T7.320.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-04
- **acceptance_criteria**:
  - null, arrays and primitives return 400 with no mutation, ONVIF request or reload; an empty object has explicit unchanged semantics.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.320 --project api --grep @t7_320`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.321: Reject wrong-typed stream fields
- **depends_on**: [T7, T11, T13, T7.320]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_321.api.spec.ts`, `quality/evidence/T7.321.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-05
- **acceptance_criteria**:
  - Wrong types return 400 with state unchanged; supported nullable clears retain their documented semantics and validation precedes all side effects.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.321 --project api --grep @t7_321`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.322: Validate bounded stream numeric fields
- **depends_on**: [T7, T11, T13, T7.321]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_322.api.spec.ts`, `quality/evidence/T7.322.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-06
- **acceptance_criteria**:
  - Protocol enum, priorities, interval, buffer, storage, PTZ and port limits reject out-of-range and fractional values before mutation; boundaries agree with storage and runtime callers.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.322 --project api --grep @t7_322`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.323: Validate complete boolean schedules
- **depends_on**: [T7, T11, T13, T7.322]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_323.api.spec.ts`, `quality/evidence/T7.323.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-07
- **acceptance_criteria**:
  - Only exactly 168 booleans are accepted; short/long/non-array or wrong element types fail atomically and valid week schedules round-trip unchanged.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.323 --project api --grep @t7_323`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.324: Address encoded slash names without aliasing
- **depends_on**: [T7, T11, T13, T7.319]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_324.api.spec.ts`, `quality/evidence/T7.324.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-08
- **acceptance_criteria**:
  - An encoded slash may either address the complete supported legacy name or be explicitly rejected; it must never select/authorize a different truncated camera.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.324 --project api --grep @t7_324`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.325: Enforce decoded stream name limits consistently
- **depends_on**: [T7, T11, T13, T7.324]
- **agent_type**: worker
- **ownership**: `src/web/request_response.c`, `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_325.api.spec.ts`, `quality/evidence/T7.325.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-09
- **acceptance_criteria**:
  - UTF-8 names at the stored byte limit are consistently addressable by PUT/full/zones; encoded suffix length does not truncate/alias a name. Malformed escapes/NUL/dot segments remain rejected. Current encoded suffix includes /full or /zones.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.325 --project api --grep @t7_325`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.326: Handle missing fleet inventory rows consistently
- **depends_on**: [T7, T11, T13, T7.325]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `tests/integration/specs/t7_326.api.spec.ts`, `quality/evidence/T7.326.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-10
- **acceptance_criteria**:
  - A missing authorization inventory row cannot expose a camera; document fail-closed behavior and isolate an inconsistent row only if policy permits, while preserving authorized list results and surfacing the inconsistency.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.326 --project api --grep @t7_326`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.327: Preserve omitted ONVIF update state
- **depends_on**: [T7, T11, T13, T7.323]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_327.api.spec.ts`, `quality/evidence/T7.327.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-11
- **acceptance_criteria**:
  - A tags-only or empty update preserves isOnvif and credentials without an ONVIF network call; explicit ONVIF changes retain their intended behavior.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.327 --project api --grep @t7_327`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.328: Make privacy mutations atomic and report failure
- **depends_on**: [T7, T11, T13, T7.327]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_328.api.spec.ts`, `quality/evidence/T7.328.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-12
- **acceptance_criteria**:
  - Privacy-only success follows a successful database write; database failure returns failure. Mixed privacy/edit requests are rejected before enable/reload/network side effects or handled through a separately specified atomic contract.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.328 --project api --grep @t7_328`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.329: Expose demo mode for authenticated users
- **depends_on**: [T7, T11, T13]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_auth_backend_agnostic.c`, `tests/integration/specs/t7_329.api.spec.ts`, `quality/evidence/T7.329.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-13
- **acceptance_criteria**:
  - Authenticated demo responses carry authoritative demo state while roles remain accurate, server-side redaction remains enforced and normal authenticated responses remain compatible.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.329 --project api --grep @t7_329`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.330: Reject overlong stream strings without truncation
- **depends_on**: [T7, T11, T13, T7.328]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_330.api.spec.ts`, `quality/evidence/T7.330.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-14
- **acceptance_criteria**:
  - Each field checks UTF-8 byte length against the destination buffer before mutation; maximum valid values round-trip and one-byte overflow is rejected atomically, including detection_model temporary-buffer limits.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.330 --project api --grep @t7_330`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.331: Bound ONVIF edit validation latency
- **depends_on**: [T7, T11, T13, T7.330]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_331.api.spec.ts`, `quality/evidence/T7.331.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-15
- **acceptance_criteria**:
  - An unresponsive synthetic ONVIF endpoint cannot monopolize a worker indefinitely; preserve accepted/persisted semantics and avoid blind retries of possibly successful mutations. Record the deadline before the test, below the client timeout or explicitly reconcile client behavior.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.331 --project api --grep @t7_331`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.332: Preserve detection threshold round trips
- **depends_on**: [T7, T11, T13, T7.326, T7.331]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_332.api.spec.ts`, `quality/evidence/T7.332.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-16
- **acceptance_criteria**:
  - All integer percentages 0..100 survive PUT→GET round trips; specifically 53 and 59 must not become 52 and 58. Preserve representation for existing stored floats with a documented rounding policy.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.332 --project api --grep @t7_332`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.333: Specify password omission and explicit clear semantics
- **depends_on**: [T7, T11, T13, T7.332]
- **agent_type**: worker
- **ownership**: `src/web/api_handlers_streams_modify.c`, `tests/integration/specs/t7_333.api.spec.ts`, `quality/evidence/T7.333.md`.
- **priority**: medium
- **description**: Reproduce the T11 candidate against disposable synthetic fixtures and real C handlers before implementation. Add an independent response/state/side-effect regression, then implement the narrow correction. Coordinate schema and generated changes through the separate reconciliation task; do not weaken the contract to conceal a defect.
- **findings**: F-T11-17
- **acceptance_criteria**:
  - Omitted onvif_password leaves the secret unchanged; explicit empty clears only as an intentional authorized action. Client form construction never echoes a redacted empty value as a destructive update.
  - Record the before/after behavior and exact failing-to-passing command; use synthetic credentials and disposable storage.
- **validation**:
  - `node scripts/quality/run-integration.mjs --task T7.333 --project api --grep @t7_333`
  - `npm run typecheck:tests`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.334: Reconcile Streams contracts after behavioral fixes
- **depends_on**: [T7, T12, T7.317, T7.318, T7.319, T7.320, T7.321, T7.322, T7.323, T7.324, T7.325, T7.326, T7.327, T7.328, T7.329, T7.330, T7.331, T7.332, T7.333]
- **agent_type**: local
- **ownership**: `docs/api/contracts/streams.yaml`, `docs/api/contracts/common.yaml`, `docs/api/openapi.yaml`, `tests/contracts/fixtures/streams/auth-verify.auth-disabled.valid.json`, `tests/contracts/fixtures/streams/auth-verify.demo-as-admin.invalid.json`, `tests/contracts/fixtures/streams/auth-verify.demo.valid.json`, `tests/contracts/fixtures/streams/auth-verify.user.valid.json`, `tests/contracts/fixtures/streams/client-config.camelcase-only.invalid.json`, `tests/contracts/fixtures/streams/client-config.valid.json`, `tests/contracts/fixtures/streams/detection-models.403-viewer.valid.json`, `tests/contracts/fixtures/streams/detection-models.no-directory.valid.json`, `tests/contracts/fixtures/streams/detection-models.unsupported-without-reason.invalid.json`, `tests/contracts/fixtures/streams/detection-models.valid.json`, `tests/contracts/fixtures/streams/error.400-retention.valid.json`, `tests/contracts/fixtures/streams/error.400-surface.valid.json`, `tests/contracts/fixtures/streams/error.401.valid.json`, `tests/contracts/fixtures/streams/error.403-put-viewer.valid.json`, `tests/contracts/fixtures/streams/error.404-camera.valid.json`, `tests/contracts/fixtures/streams/error.404-stream.valid.json`, `tests/contracts/fixtures/streams/error.413-publish-url.valid.json`, `tests/contracts/fixtures/streams/error.empty-message.invalid.json`, `tests/contracts/fixtures/streams/error.message-key.invalid.json`, `tests/contracts/fixtures/streams/full.any.generic-schema.valid.json`, `tests/contracts/fixtures/streams/full.configurer.legacy-name-plain-credentials.valid.json`, `tests/contracts/fixtures/streams/full.configurer.missing-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/full.configurer.onvif-url-userinfo.invalid.json`, `tests/contracts/fixtures/streams/full.configurer.onvif.valid.json`, `tests/contracts/fixtures/streams/full.error-message-without-error.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.claims-configure.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.error-status.valid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-admin-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-detection-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-go2rtc-override.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-onvif-username.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-publish-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.leaked-sub-stream-url.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.redacted.valid.json`, `tests/contracts/fixtures/streams/full.live-only.sensitive-keys-absent.valid.json`, `tests/contracts/fixtures/streams/full.live-only.url-query-secret.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.url-userinfo.invalid.json`, `tests/contracts/fixtures/streams/full.live-only.url-username-only.invalid.json`, `tests/contracts/fixtures/streams/full.null-tags.invalid.json`, `tests/contracts/fixtures/streams/full.protocol-out-of-enum.invalid.json`, `tests/contracts/fixtures/streams/full.retention-below-inherit.invalid.json`, `tests/contracts/fixtures/streams/full.retention-over-max.invalid.json`, `tests/contracts/fixtures/streams/full.schedule-167-entries.invalid.json`, `tests/contracts/fixtures/streams/full.status-docs-value.invalid.json`, `tests/contracts/fixtures/streams/full.threshold-as-fraction.invalid.json`, `tests/contracts/fixtures/streams/full.unwrapped-object.invalid.json`, `tests/contracts/fixtures/streams/index.json`, `tests/contracts/fixtures/streams/path-encoding.cases.json`, `tests/contracts/fixtures/streams/put-response.200-privacy.valid.json`, `tests/contracts/fixtures/streams/put-response.202-onvif-failed.valid.json`, `tests/contracts/fixtures/streams/put-response.202-onvif-status-without-message.invalid.json`, `tests/contracts/fixtures/streams/put-response.202.valid.json`, `tests/contracts/fixtures/streams/put.clear-with-null.valid.json`, `tests/contracts/fixtures/streams/put.detection-retention-minus-2.invalid.json`, `tests/contracts/fixtures/streams/put.detection-url-file-scheme.invalid.json`, `tests/contracts/fixtures/streams/put.detection-url-too-long.invalid.json`, `tests/contracts/fixtures/streams/put.edit-form.valid.json`, `tests/contracts/fixtures/streams/put.empty-object.valid.json`, `tests/contracts/fixtures/streams/put.eptz-not-string.invalid.json`, `tests/contracts/fixtures/streams/put.eptz-wrong-projection.invalid.json`, `tests/contracts/fixtures/streams/put.is-deleted.invalid.json`, `tests/contracts/fixtures/streams/put.name-in-body.invalid.json`, `tests/contracts/fixtures/streams/put.playback-transport-bogus.invalid.json`, `tests/contracts/fixtures/streams/put.privacy-mixed-with-edit.invalid.json`, `tests/contracts/fixtures/streams/put.protocol-out-of-enum.invalid.json`, `tests/contracts/fixtures/streams/put.publish-and-detection-url.valid.json`, `tests/contracts/fixtures/streams/put.publish-url-http.invalid.json`, `tests/contracts/fixtures/streams/put.publish-url-quote.invalid.json`, `tests/contracts/fixtures/streams/put.record-as-string.invalid.json`, `tests/contracts/fixtures/streams/put.retention-366.invalid.json`, `tests/contracts/fixtures/streams/put.retention-finite-boundary.valid.json`, `tests/contracts/fixtures/streams/put.retention-tristate.valid.json`, `tests/contracts/fixtures/streams/put.schedule-weekday.valid.json`, `tests/contracts/fixtures/streams/put.schedule-wrong-length.invalid.json`, `tests/contracts/fixtures/streams/put.set-privacy-mode.valid.json`, `tests/contracts/fixtures/streams/put.tags-too-long.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.admin-url-without-configure.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.configurer-with-admin-url.valid.json`, `tests/contracts/fixtures/streams/summary.admin.contains-url.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.empty.valid.json`, `tests/contracts/fixtures/streams/summary.admin.missing-can-configure.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.page-beyond-end.valid.json`, `tests/contracts/fixtures/streams/summary.admin.page-size-over-max.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.status-from-detail-enum.invalid.json`, `tests/contracts/fixtures/streams/summary.admin.viewer.valid.json`, `tests/contracts/fixtures/streams/summary.live.contains-admin-url.invalid.json`, `tests/contracts/fixtures/streams/summary.live.contains-onvif-password.invalid.json`, `tests/contracts/fixtures/streams/summary.live.viewer.valid.json`, `tests/contracts/fixtures/streams/summary.lpr.leaks-location.invalid.json`, `tests/contracts/fixtures/streams/summary.lpr.valid.json`, `tests/contracts/fixtures/streams/zones.empty.valid.json`, `tests/contracts/fixtures/streams/zones.valid.json`, `web/js/api/generated/`, `tests/contracts/generated/`, `quality/evidence/T7.334.md`.
- **priority**: high
- **description**: Under the exclusive generation lease, reconcile the exact schemas and affected fixture files after the preceding fixes. The exact fixture list is recorded in ownership; generated directories are exclusively owned outputs of T12. Regenerate only through T12 tooling; no concurrent generation. Preserve separate desired-client and current-wire cases where intentionally different.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - All valid and invalid fixtures retain explicit reasons and agree with the corrected real backend.
  - Generate deterministic artifacts and run real pilot regressions; T18/T19 cannot use stale contracts.
- **validation**:
  - `node scripts/quality/contracts.mjs --generate`
  - `node scripts/quality/contracts.mjs --check`
  - `node --test tests/quality/contracts.test.mjs`
  - `npm --prefix web run typecheck`
- **status**: Not Completed
- **log**:
- **files edited/created**:

### T7.335: Resolve proposed ownership and exception review records
- **depends_on**: [T7, T7.1]
- **agent_type**: local
- **ownership**: `quality/reviews/exception-ownership.json`, `quality/evidence/T7.335.md`.
- **priority**: high
- **description**: Collect accountable owners and explicit approvals/denials for T1/T6 proposals and any new residual exceptions. Existing maintainers/<area> strings and proposed dates are not acceptance. Prepare a concrete review record; leave pending entries unresolved until actual authorized reviewer input arrives.
- **findings**: policy/integration requirement
- **acceptance_criteria**:
  - Every unresolved proposal names its exact finding/gap, requested owner, exposure, rationale, proposed expiry and approval state.
  - No approval, repository protection change, CODEOWNERS assignment or publication is inferred from implementation authorization.
- **validation**:
  - `python3 -m json.tool quality/reviews/exception-ownership.json`
- **status**: Not Completed
- **log**:
- **files edited/created**:
