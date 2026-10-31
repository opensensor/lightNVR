/** Shared ownership checks for the isolated integration runner, setup and teardown. */
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { spawnSync } from 'node:child_process';

type Env = Record<string, string | undefined>;
export interface RunContext { dir: string; token: string; config: string; binary: string; artifacts: string }
const record = (value: unknown): value is Record<string, unknown> => typeof value === 'object' && value !== null && !Array.isArray(value);
export const fileHash = (file: string): string => crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');

export function readRunContext(env: Env = process.env, verifyConfig = true): RunContext {
  const dir = env.LIGHTNVR_TEST_DIR;
  const token = env.LIGHTNVR_RUN_TOKEN;
  if (!dir || !token || !/^\/tmp\/lightnvr-eng-[A-Za-z0-9_.-]+$/.test(dir) || fs.lstatSync(dir).isSymbolicLink() || fs.realpathSync(dir) !== dir) {
    throw new Error('Use scripts/quality/run-integration.mjs: an owned run directory/token is required');
  }
  const owner: unknown = JSON.parse(fs.readFileSync(path.join(dir, 'owner.json'), 'utf8'));
  if (!record(owner) || owner.token !== token || owner.dir !== dir || owner.config !== path.join(dir, 'lightnvr.ini') || typeof owner.binary !== 'string' || typeof owner.artifacts !== 'string' || (verifyConfig && owner.configHash !== fileHash(owner.config))) {
    throw new Error('Run ownership or generated configuration mismatch; refusing setup/cleanup');
  }
  if (env.LIGHTNVR_TEST_CONFIG !== owner.config || env.LIGHTNVR_TEST_BIN !== owner.binary) throw new Error('Binary/configuration override disagrees with run ownership');
  return { dir, token, config: owner.config, binary: owner.binary, artifacts: owner.artifacts };
}

function identity(pid: number): { start: string; state: string; args: string[]; binary: string } | null {
  if (!Number.isSafeInteger(pid) || pid <= 1) throw new Error('Invalid owned PID');
  try {
    const stat = fs.readFileSync(`/proc/${pid}/stat`, 'utf8').split(') ').slice(1).join(') ').trim().split(/\s+/);
    const start = stat[19]; const state = stat[0];
    if (!start || !state) throw new Error('Malformed process identity');
    if (state === 'Z') return null;
    return { start, state, args: fs.readFileSync(`/proc/${pid}/cmdline`, 'utf8').split('\0'), binary: fs.realpathSync(`/proc/${pid}/exe`) };
  } catch (error) {
    if (record(error) && error.code === 'ENOENT') return null;
    throw error;
  }
}

export function recordOwnedProcess(pid: number, env: Env = process.env): void {
  const ctx = readRunContext(env); const processInfo = identity(pid);
  if (!processInfo || processInfo.binary !== fs.realpathSync(ctx.binary) || !processInfo.args.some((arg, index) => arg === '-c' && processInfo.args[index + 1] === ctx.config)) {
    throw new Error('PID executable/configuration does not belong to this run');
  }
  fs.writeFileSync(path.join(ctx.dir, 'process.json'), JSON.stringify({ pid, start: processInfo.start, token: ctx.token }), { flag: 'wx' });
}

// Linux pidfds close the check-to-signal PID-reuse race. No numeric-PID fallback.
function signalVerifiedProcess(pid: number, start: string, binary: string, args: string[], name: 'SIGTERM' | 'SIGKILL'): void {
  const script = `import os,sys,signal,json
pid,start,binary,expected_args,name=sys.argv[1:]
try:
 fd=os.pidfd_open(int(pid))
except ProcessLookupError:
 sys.exit(0)
try:
 stat=open('/proc/'+pid+'/stat').read().rsplit(') ',1)[1].split()
 args=open('/proc/'+pid+'/cmdline','rb').read().decode().split('\\0')
 valid=stat[19]==start and os.path.realpath('/proc/'+pid+'/exe')==os.path.realpath(binary) and args==json.loads(expected_args)
 if not valid: raise RuntimeError('PID identity changed before pidfd signal')
 signal.pidfd_send_signal(fd,getattr(signal,name))
except (ProcessLookupError,FileNotFoundError):
 pass
finally:
 os.close(fd)
`;
  const result = spawnSync('python3', ['-c', script, String(pid), start, binary, JSON.stringify(args), name], { encoding: 'utf8' });
  if (result.status !== 0) throw new Error(`Owned pidfd signal failed: ${result.stderr || result.error?.message}`);
}

function signalOwnedProcess(pid: number, start: string, ctx: RunContext, name: 'SIGTERM' | 'SIGKILL'): void {
  const current = identity(pid);
  if (current) signalVerifiedProcess(pid, start, ctx.binary, current.args, name);
}

type Descendant = { pid: number; start: string; binary: string; args: string[] };

function descendantSnapshot(parent: number): Descendant[] {
  const processes: Array<{ pid: number; parent: number; start: string }> = [];
  for (const entry of fs.readdirSync('/proc')) {
    if (!/^\d+$/.test(entry)) continue;
    try {
      const stat = fs.readFileSync(`/proc/${entry}/stat`, 'utf8').split(') ').slice(1).join(') ').trim().split(/\s+/);
      const start = stat[19];
      if (start && stat[0] !== 'Z') processes.push({ pid: Number(entry), parent: Number(stat[1]), start });
    } catch { /* Processes can exit while enumerating. */ }
  }
  const parents = new Set([parent]); const descendants: Descendant[] = [];
  for (let changed = true; changed;) {
    changed = false;
    for (const child of processes) if (parents.has(child.parent) && !parents.has(child.pid)) {
      parents.add(child.pid);
      const current = identity(child.pid);
      if (current && current.start === child.start) descendants.push({ pid: child.pid, start: child.start, binary: current.binary, args: current.args });
      changed = true;
    }
  }
  return descendants;
}

function storedDescendants(value: unknown): Descendant[] {
  if (value === undefined) return [];
  if (!Array.isArray(value)) throw new Error('Invalid saved descendant identities');
  return value.map((child: unknown) => {
    if (!record(child) || typeof child.pid !== 'number' || typeof child.start !== 'string' || typeof child.binary !== 'string' || !Array.isArray(child.args) || !child.args.every((arg: unknown) => typeof arg === 'string')) throw new Error('Invalid saved descendant identity');
    return { pid: child.pid, start: child.start, binary: child.binary, args: child.args };
  });
}

export function recordOwnedDescendants(env: Env = process.env): void {
  const ctx = readRunContext(env, false); const filename = path.join(ctx.dir, 'process.json');
  const saved: unknown = JSON.parse(fs.readFileSync(filename, 'utf8'));
  if (!record(saved) || saved.token !== ctx.token || typeof saved.pid !== 'number' || typeof saved.start !== 'string') throw new Error('Invalid server ownership record');
  const current = identity(saved.pid);
  if (!current || current.start !== saved.start || current.binary !== fs.realpathSync(ctx.binary)) throw new Error('Server exited or changed before descendant capture');
  fs.writeFileSync(filename, JSON.stringify({ ...saved, children: descendantSnapshot(saved.pid) }));
}

export async function stopOwnedProcess(env: Env = process.env, graceMs = 60_000): Promise<void> {
  const ctx = readRunContext(env, false); const filename = path.join(ctx.dir, 'process.json');
  if (!fs.existsSync(filename)) return;
  const saved: unknown = JSON.parse(fs.readFileSync(filename, 'utf8'));
  if (!record(saved) || saved.token !== ctx.token || typeof saved.pid !== 'number' || typeof saved.start !== 'string') throw new Error('Invalid process ownership record');
  const pid = saved.pid;
  const owned = (): boolean => {
    const current = identity(pid);
    if (!current) return false;
    if (current.start !== saved.start || current.binary !== fs.realpathSync(ctx.binary) || !current.args.some((arg, index) => arg === '-c' && current.args[index + 1] === ctx.config)) throw new Error('PID identity changed; refusing to signal unrelated process');
    return true;
  };
  const started = Date.now(); let forced = false;
  const children = [...new Map([...storedDescendants(saved.children), ...(owned() ? descendantSnapshot(pid) : [])].map(child => [`${child.pid}:${child.start}`, child])).values()];
  if (owned()) {
    signalOwnedProcess(pid, saved.start, ctx, 'SIGTERM');
    while (owned() && Date.now() - started < graceMs) await new Promise(resolve => setTimeout(resolve, 200));
    if (owned()) { signalOwnedProcess(pid, saved.start, ctx, 'SIGKILL'); forced = true; }
  }
  const lingeringBeforeCleanup = children.filter(child => identity(child.pid)?.start === child.start);
  // Descendants were observed under the verified server before shutdown. Signal
  // only their captured executable/argv/start identities through pidfds.
  for (const child of lingeringBeforeCleanup) signalVerifiedProcess(child.pid, child.start, child.binary, child.args, 'SIGTERM');
  if (lingeringBeforeCleanup.length) await new Promise(resolve => setTimeout(resolve, 1000));
  for (const child of lingeringBeforeCleanup) if (identity(child.pid)?.start === child.start) signalVerifiedProcess(child.pid, child.start, child.binary, child.args, 'SIGKILL');
  if (lingeringBeforeCleanup.length) await new Promise(resolve => setTimeout(resolve, 200));
  const lingeringChildren = children.filter(child => identity(child.pid)?.start === child.start);
  fs.writeFileSync(path.join(ctx.artifacts, 'shutdown.json'), JSON.stringify({ pid, graceful: !forced && !lingeringBeforeCleanup.length, forcedKill: forced, elapsedMs: Date.now() - started, observedChildren: children, forcedChildCleanup: lingeringBeforeCleanup.length > 0, lingeringBeforeCleanup, lingeringChildren }, null, 2));
  fs.renameSync(filename, path.join(ctx.dir, 'process-stopped.json'));
  if (forced) throw new Error('Owned server exceeded graceful shutdown deadline; fallback kill is a failed run');
  if (lingeringBeforeCleanup.length) throw new Error('Owned server left descendants requiring cleanup; retained identities in shutdown.json, refusing to call graceful shutdown successful');
}
