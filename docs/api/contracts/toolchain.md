# Contract toolchain

T3 installs exact versions without changing any pre-existing locked package
version: Redocly CLI 2.75.0, openapi-typescript 7.13.0, Ajv 8.20.0,
ajv-formats 3.0.1, esbuild 0.28.2, root TypeScript 5.9.3 and Node types
24.19.2. The docs-only web dependency is swagger-ui-dist 5.34.1.
The application retains TypeScript 6.0.3 and its existing dependencies.

Root TypeScript is 5.9.3 because openapi-typescript 7.13.0 declares a `^5.x`
peer dependency. No force/legacy-peer-deps override is used. Generated declarations
are independently compiled with **both** root TS 5.9.3 and application TS 6.0.3,
with strict mode and library checking enabled. The root compiler also checks all
contract TS/config files via `tests/contracts/tsconfig.json`; existing integration
and load test typing remains the separate T18 task.

## Commands and producer interface

Run from the repository root:

```sh
npm ci
npm --prefix web ci
npm run contracts:test-tools
npm run contracts:typecheck
npm run contracts:lint
npm run contracts:generate
npm run contracts:check
npm run contracts:generate -- --target=native
npm run contracts:check -- --target=web
npm run contracts:lint -- --projections /path/to/actual-c-projector-output
```

Canonical commands read `docs/api/openapi.json`. They fail if it is absent;
T3 does not substitute its smoke fixture for the product contract. Generation
and checks accept `--target=native`, `--target=web`, or `--target=all` (default).
`--source FILE --output-root DIR` are available for isolated tooling fixtures.
The projection linter validates every direct `*.json` file in the requested
directory and fails for an empty directory. It is intended for actual C outputs;
the tooling suite's synthetic projections exercise only the command interface.

Native output is `include/web/openapi_document_generated.h` with:

- `lightnvr_openapi_document`: NUL-terminated unsigned byte array.
- `lightnvr_openapi_document_length`: payload bytes excluding the terminator.
- `lightnvr_openapi_document_revision`: SHA-256 of the embedded payload.

The payload uses UTF-8, sorted object keys, compact separators and one final
newline. Python emits this header deterministically using only its standard
library. Release compilation consumes the generated header and needs no Node.
The direct Python producer command is:

```sh
python3 scripts/contracts/generate_native.py --source docs/api/openapi.json --output include/web/openapi_document_generated.h
```

Web output owns the complete `web/generated/api/` directory:

- `schema.d.ts`: `paths`, `operations`, `components` from openapi-typescript.
- `validators.js`: standalone ESM validators with inlined runtime helpers.
- `validators.d.ts`: `validate<SchemaName>(value: unknown): value is components["schemas"]["SchemaName"]` guards.

Choose validators with a canonical root `x-lightnvr-validators` array of schema
names. When omitted, every named component schema is selected. Selected names
must match `[A-Za-z][A-Za-z0-9_]*`. This is **build-only metadata**: T6 must remove
`x-lightnvr-validators` from personalized output together with internal permission
predicates. Other component names may use normal OpenAPI JSON Pointer escaping.
Named guards have `.errors`, either `null` or a list of `{keyword, schemaPath}`.
They do not expose instance paths, input values, arbitrary property names,
request bodies, credentials or Ajv `params`. Diagnostics are replaced on each
call, so consumers should capture them immediately if needed.

Ajv 8.20 uses the reviewed emitters in `scripts/contracts/validator-codegen.mjs`.
Required fields use explicit branches; each property has an independent validity
temporary. Internal errors use arrays throughout, with union rollback truncating
them to the previous length. Public errors still normalize to `null` on success
and expose only `keyword`/`schemaPath`. Unused private message strings are not
emitted. The original MIT attribution is retained alongside the emitters.

Pinned Terser 5.48 performs conservative dead-code and unused-temporary removal,
without unsafe transforms, name/property mangling, or analysis exclusions. Output
remains readable and deterministic. The supported profile is non-mutating and
fail-fast; generation rejects changing it to `$data`, defaults, removal or
all-errors mode. Keep the unmodified-Ajv differential tests and full JavaScript
CodeQL quality scan when upgrading Ajv or changing these emitters. The tests
compare validity, exact sanitized diagnostics and input preservation across all
exported guards; standard generator tests cover strict compilation and drift.

`contracts:check` regenerates into temporary storage, compares expected bytes
and checks for unexpected files in the generated web directory. It never repairs
drift. Generation replaces unexpected files in its exclusively owned generated
directory; do not put handwritten code there. T4 owns native generation, T8 owns
web generation, and later canonical expansion tasks must regenerate both.

## Validation and supported subset

OpenAPI is pinned to **3.1.1**, with JSON Schema **2020-12** validators. Local
JSON Pointer references, including escaped tokens and array indexes, are checked
before generation. Validator schema references target component schemas; the
producer rewrites these into an internal `$defs` document. Static recursive
component references are supported by Ajv. Remote references, dynamic/recursive
reference keywords, anchors and custom `$id` scopes are rejected explicitly.
Discriminator mappings and link `operationRef` values may not introduce external
references. The schema compiler has no asynchronous/network resolver.

The Python preflight validates every canonical operation's authorization
predicate against design version 1 and the actual C action catalog. Missing or
unknown kinds, keys, actions, resources and versions fail. Projection lint allows
permission metadata to have been removed but still validates any metadata that
remains. Redocly independently validates OpenAPI structure. Its stylistic
trailing-slash rule is disabled because the deliberately relative server `../`
resolves the deployment root from `/api/openapi.json`; this does not relax schema
validation. Unused components are allowed in the canonical source while the
projector's own tests are responsible for proving reachability pruning.

Ajv is strict, format-checking and non-mutating: no coercion, default insertion,
or property removal. Schema defaults remain optional in generated types through
`defaultNonNullable: false`, matching runtime behavior. Additional properties
remain available when the schema permits them. OpenAPI annotations and internal
extensions are stripped from the JSON Schema compiler input; unknown JSON Schema
keywords fail compilation rather than becoming unchecked assertions.

The browser gets compiled functions and small runtime helpers, **not Ajv's
schema compiler**. esbuild inlines the standalone code's CommonJS helpers into
ESM. The producer rejects unresolved imports and unexpected runtime modules.
No browser schema compilation or network reference loading is used.
Generated runtime JS is intentionally outside handwritten `web/js`. A declaration
wrapper does not type-check that JS; trust comes from deterministic generation,
drift checks and runtime positive/negative tests. The existing handwritten JS
allowlist and strict application settings are unchanged.

Swagger UI must be imported as its already-bundled standalone distribution only
by the separate docs page. Do not run its source through Preact compatibility
aliases or import it from shared application entries. T15 owns the docs page,
read-only/session configuration and network assertions; installing the package
alone does not implement those controls.

## T3 evidence

- Both `npm ci` commands passed; all pre-existing locked package versions were
  preserved (compared against the pre-install lockfiles).
- `npm run contracts:test-tools`: five test groups passed. They cover 2020-12
  tuples, unions, null/absent/defaulted fields, UUID/date-time formats, local
  references, additive properties, nonmutation and diagnostic sanitization. Literal
  wire properties named `example`, `discriminator`, `xml`, `externalDocs`,
  `x-foo` and `$ref` retain their schemas; default/example data stays literal.
- Two consecutive generations are byte-identical. Python direct generation
  matches the Node-orchestrated native output. Deliberate drift fails `--check`
  and remains unchanged until the fixture explicitly restores it.
- Strict positive and intentionally invalid consumers run against both installed
  TS compilers with `skipLibCheck: false`.
- Canonical/projection lint passes for fixtures, including an empty personalized
  document; empty projection directories fail rather than silently passing.
- `npm run contracts:typecheck` and `npm --prefix web run typecheck` passed.

Local logs are `/tmp/lightnvr-T3-{ci-root,ci-web,tests,typecheck,web-typecheck}.log`.
Canonical/product schema, actual C projection documents and production generated
artifacts remain the responsibility of T4/T6/T8, not claimed by these smoke tests.

Primary tool references: [OpenAPI TypeScript Node API](https://openapi-ts.dev/node),
[Ajv standalone generation](https://ajv.js.org/standalone.html),
[Swagger UI configuration](https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/).


Format-plus-pattern validators can repeat an identical internal error-counter
comparison after optimization. The generator collapses only strict comparisons
of numeric literals and Ajv counter locals, then reruns conservative cleanup.
Property access, calls and coercive comparisons are deliberately untouched.
Differential tests and full quality analysis remain required after schema growth.
