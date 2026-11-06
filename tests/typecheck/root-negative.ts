/** Compile-only fixtures: included by tsconfig.tests.json, outside Playwright discovery. */
import type { LoadTestConfig, Scenario } from '../load/config';
import type { StreamsPage } from '../integration/pages/StreamsPage';
import type { ContractFixture } from '../contracts/fixture-server.mjs';

const validConcurrency: LoadTestConfig['concurrency'] = 8;
const validMethod: Scenario['method'] = 'GET';
// @ts-expect-error Load configuration must keep numeric concurrency, not coerce text.
const invalidConcurrency: LoadTestConfig['concurrency'] = '8';
// @ts-expect-error Strict null checking applies to the actual load configuration URL.
const missingURL: LoadTestConfig['baseURL'] = undefined;
// @ts-expect-error Unannotated callback parameters must remain rejected by noImplicitAny.
const untypedFactory = config => config.concurrency;
// @ts-expect-error Load scenarios allow declared HTTP methods, not arbitrary verbs.
const invalidMethod: Scenario['method'] = 'FETCH';
// @ts-expect-error Page harness methods require a complete stream configuration.
const invalidStream: Parameters<StreamsPage['addStream']>[0] = {name: 'missing-url'};
// @ts-expect-error The real-server helper defaults JSON to unknown, never unchecked data.
const uncheckedWire: {name: string} = {} as Awaited<ReturnType<ContractFixture['json']>>;
void [validConcurrency, validMethod, invalidConcurrency, invalidMethod, invalidStream, uncheckedWire, missingURL, untypedFactory];
