/**
 * Tests for `useAsyncAction` hook behaviour. Verifies the critical
 * double-click guard (PRD UXD_01 §5.1 / #399) by mocking preact/hooks
 * so we exercise the hook body directly without needing a renderer.
 *
 * We don't have jsdom configured in this project (see jest.config.cjs:
 * testEnvironment: 'node'), so component-level tests that mount the
 * JSX `<AsyncButton>` are covered by manual verification / the vite build
 * smoke test — see the task report. This spec exercises the pure-logic
 * half of the primitive, which is where the idempotency guard lives.
 */

const state = {
  values: [],   // useState slot values
  refs: [],     // useRef slot values
  effects: [],  // registered effect teardowns
  cursor: 0,
  rerender: null
};

/**
 * Minimal preact/hooks shim sufficient to exercise useAsyncAction once.
 * Each hook is called in fixed order; we use the slot cursor to persist
 * state between re-invocations (simulated re-renders).
 */
jest.mock('preact/hooks', () => {
  return {
    useState: (initial) => {
      const slot = state.cursor++;
      if (state.values[slot] === undefined) {
        state.values[slot] = { value: initial };
      }
      const set = (next) => {
        const current = state.values[slot].value;
        const resolved = typeof next === 'function' ? next(current) : next;
        if (resolved !== current) {
          state.values[slot].value = resolved;
        }
      };
      return [state.values[slot].value, set];
    },
    useRef: (initial) => {
      const slot = state.cursor++;
      if (state.refs[slot] === undefined) {
        state.refs[slot] = { current: initial };
      }
      return state.refs[slot];
    },
    useEffect: (fn) => {
      const slot = state.cursor++;
      if (!state.effects[slot]) {
        state.effects[slot] = fn() || (() => {});
      }
    },
    useCallback: (fn) => fn,
  };
});

const { useAsyncAction } = require('../js/hooks/useAsyncAction.js');

function resetHookState() {
  state.values = [];
  state.refs = [];
  state.effects = [];
  state.cursor = 0;
}

function invokeHook(fn) {
  state.cursor = 0;
  return useAsyncAction(fn);
}

describe('useAsyncAction', () => {
  beforeEach(resetHookState);

  it('invokes the underlying fn exactly once when called twice rapidly', async () => {
    let resolveHandler;
    const handler = jest.fn(
      () => new Promise((resolve) => { resolveHandler = resolve; })
    );

    const { run } = invokeHook(handler);

    // Fire two rapid-fire calls; the second should be a no-op per the
    // idempotency guard.
    const first = run('arg1');
    const second = run('arg2');

    expect(handler).toHaveBeenCalledTimes(1);
    // The second call should return the same in-flight promise.
    expect(second).toBe(first);

    resolveHandler('ok');
    await first;

    // A third call after settlement is allowed to proceed.
    run('arg3');
    expect(handler).toHaveBeenCalledTimes(2);
    expect(handler).toHaveBeenLastCalledWith('arg3');
    resolveHandler = null;
    // Clean up the dangling promise the mock left.
    // (The third call awaits a new promise whose resolver we captured.)
  });

  it('captures errors on rejection and clears pending', async () => {
    const err = new Error('boom');
    const handler = jest.fn(() => Promise.reject(err));

    const { run } = invokeHook(handler);

    await expect(run()).rejects.toBe(err);

    // Re-invoke the hook to observe the committed state via our shim.
    const { pending, error } = invokeHook(handler);
    expect(pending).toBe(false);
    expect(error).toBe(err);
  });

  it('records lastSuccess timestamp on resolve', async () => {
    const handler = jest.fn(() => Promise.resolve('ok'));
    const before = Date.now();

    const { run } = invokeHook(handler);
    await run();

    const after = Date.now();
    const { lastSuccess, pending, error } = invokeHook(handler);
    expect(pending).toBe(false);
    expect(error).toBeNull();
    expect(lastSuccess).toBeGreaterThanOrEqual(before);
    expect(lastSuccess).toBeLessThanOrEqual(after);
  });

  it('handles synchronous throws from the handler', async () => {
    const err = new Error('sync-boom');
    const handler = () => { throw err; };

    const { run } = invokeHook(handler);

    await expect(run()).rejects.toBe(err);
    const { error } = invokeHook(handler);
    expect(error).toBe(err);
  });
});
