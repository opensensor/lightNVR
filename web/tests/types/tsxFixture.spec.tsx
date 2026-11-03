/**
 * Proves the TS toolchain end to end: this TSX goes through Babel and runs on the
 * real Preact renderer in Jest, imports a real legacy module, and is checked by
 * `npm run typecheck`. Do not remove the @ts-expect-error below: it is the
 * permanent negative control showing that strict prop checking is live.
 *
 * The project has no DOM renderer (see jest.config.cjs), so a minimal DOM stands
 * in for the few node operations Preact performs here.
 */
import { render } from 'preact';
import type { ComponentChildren } from 'preact';
import { useState } from 'preact/hooks';
import { REDUCE_MOTION_STORAGE_KEY, REDUCE_MOTION_VALUES, getReduceMotionPref } from '../../js/utils/reduceMotion.js';

class FakeNode {
  parentNode: FakeNode | null = null;
  childNodes: FakeNode[] = [];
  attributes: Record<string, string> = {};
  style: Record<string, string> = {};
  data = '';

  constructor(readonly localName: string) {}

  get firstChild(): FakeNode | null {
    return this.childNodes[0] ?? null;
  }

  get nextSibling(): FakeNode | null {
    const siblings = this.parentNode?.childNodes ?? [];
    return siblings[siblings.indexOf(this) + 1] ?? null;
  }

  get textContent(): string {
    return this.data + this.childNodes.map((child) => child.textContent).join('');
  }

  insertBefore(child: FakeNode, before: FakeNode | null): FakeNode {
    child.parentNode?.removeChild(child);
    const index = before ? this.childNodes.indexOf(before) : this.childNodes.length;
    this.childNodes.splice(index, 0, child);
    child.parentNode = this;
    return child;
  }

  appendChild(child: FakeNode): FakeNode {
    return this.insertBefore(child, null);
  }

  removeChild(child: FakeNode): FakeNode {
    this.childNodes.splice(this.childNodes.indexOf(child), 1);
    child.parentNode = null;
    return child;
  }

  remove(): void {
    this.parentNode?.removeChild(this);
  }

  setAttribute(name: string, value: unknown): void {
    this.attributes[name] = String(value);
  }

  removeAttribute(name: string): void {
    delete this.attributes[name];
  }
}

function createFakeDocument() {
  return {
    createElement: (name: string) => new FakeNode(name),
    createElementNS: (_namespace: string, name: string) => new FakeNode(name),
    createTextNode: (data: string) => Object.assign(new FakeNode('#text'), { data }),
  };
}

interface PrefSummaryProps {
  storageKey: string;
  count: number;
  children?: ComponentChildren;
}

function PrefSummary({ storageKey, count, children }: PrefSummaryProps) {
  const [label] = useState(() => `${storageKey}:${count}`);
  return <p data-count={count}>{label}{children}</p>;
}

// @ts-expect-error -- negative control: `count` must be a number, so tsc rejects this string prop.
const wrongProp = <PrefSummary storageKey={REDUCE_MOTION_STORAGE_KEY} count="three" />;

describe('TS toolchain fixture', () => {
  const globals = globalThis as { document?: unknown };

  beforeEach(() => {
    globals.document = createFakeDocument();
  });

  afterEach(() => {
    delete globals.document;
  });

  test('TSX renders through Preact using values from a legacy module', () => {
    const root = new FakeNode('main');

    // The fake covers only what Preact touches here, so it cannot satisfy the
    // full DOM `ContainerNode` type that render() declares.
    render(
      <PrefSummary storageKey={REDUCE_MOTION_STORAGE_KEY} count={REDUCE_MOTION_VALUES.length}>
        <b>{String(getReduceMotionPref())}</b>
      </PrefSummary>,
      root as unknown as Element,
    );

    const paragraph = root.childNodes[0];
    expect(paragraph?.localName).toBe('p');
    expect(paragraph?.attributes['data-count']).toBe('3');
    expect(root.textContent).toBe('lightnvr.reduceMotion:3auto');
  });

  test('the negative control is an ordinary vnode at runtime', () => {
    expect(wrongProp.type).toBe(PrefSummary);
  });
});
