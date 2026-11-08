/** Isolated current-source server. All identities and credentials are test-only. */
import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { mkdtemp, mkdir, writeFile, readFile, rm, open } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

// Invoke from repository root, or pass LIGHTNVR_CONTRACT_ROOT explicitly.
const root = path.resolve(process.env.LIGHTNVR_CONTRACT_ROOT || process.cwd());
export const credentials = Object.freeze({admin: {username: 'admin', password: 'ContractAdmin123!'}, viewer: {username: 'contract_viewer', password: 'ContractViewer123!'}});
export const basic = ({username, password}) => `Basic ${Buffer.from(`${username}:${password}`).toString('base64')}`;
const pause = ms => new Promise(resolve => setTimeout(resolve, ms));

/** Reserve a loopback port. Passing a port fails on collision without touching its owner. */
export async function reservePort(port = 0) {
  const server = createServer();
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(port, '127.0.0.1', resolve); });
  return {port: server.address().port, release: () => new Promise((resolve, reject) => server.close(e => e ? reject(e) : resolve()))};
}

export async function startFixture(options = {}) {
  const maxStreams = options.maxStreams ?? 8;
  if (!Number.isInteger(maxStreams) || maxStreams < 1 || maxStreams > 1024) {
    throw new Error('Fixture maxStreams must be an integer from 1 to 1024');
  }
  const directory = await mkdtemp(path.join(tmpdir(), 'lightnvr-contract-'));
  let child, output, reservation, exit, stopped = false;
  const fixture = {
    directory, logPath: path.join(directory, 'process.log'), credentials, users: {}, cameras: {}, tokens: {},
    async request(route, {principal = 'admin', method = 'GET', body, headers = {}, ...rest} = {}) {
      const identity = credentials[principal];
      const authorization = identity ? basic(identity) : fixture.tokens[principal] ? `Bearer ${fixture.tokens[principal]}` : undefined;
      return fetch(`${fixture.baseURL}${route}`, {method, signal: AbortSignal.timeout(10000), ...rest,
        headers: {...(authorization ? {Authorization: authorization} : {}), ...(body !== undefined ? {'Content-Type': 'application/json'} : {}), ...headers},
        ...(body !== undefined ? {body: JSON.stringify(body)} : {})});
    },
    async json(route, options) {
      const response = await fixture.request(route, options);
      const text = await response.text();
      if (!response.ok) throw new Error(`${options?.method || 'GET'} ${route}: ${response.status} ${text}`);
      return JSON.parse(text);
    },
    async apiContext(request, principal = 'admin') {
      const authorization = credentials[principal] ? basic(credentials[principal]) : fixture.tokens[principal] ? `Bearer ${fixture.tokens[principal]}` : undefined;
      return request.newContext({baseURL: fixture.baseURL, extraHTTPHeaders: authorization ? {Authorization: authorization} : {}, timeout: 10000});
    },
    async browserContext(browser, principal = 'admin') {
      const context = await browser.newContext({baseURL: fixture.baseURL});
      if (credentials[principal]) {
        const response = await context.request.post('/api/auth/login', {data: credentials[principal], timeout: 10000});
        if (!response.ok()) { await context.close(); throw new Error(`UI session login failed: ${response.status()}`); }
      } else if (fixture.tokens[principal]) await context.setExtraHTTPHeaders({Authorization: `Bearer ${fixture.tokens[principal]}`});
      return context;
    },
    async stop({failed = false} = {}) {
      if (stopped) return;
      stopped = true;
      if (reservation) await reservation.release();
      let shutdownRequested = false, shutdownTimedOut = false;
      if (child && child.exitCode === null && child.signalCode === null) {
        shutdownRequested = true;
        child.kill('SIGTERM');
        let timer;
        const done = await Promise.race([exit.then(() => true), new Promise(resolve => { timer = setTimeout(() => resolve(false), 15000); })]);
        clearTimeout(timer);
        if (!done) { shutdownTimedOut = true; child.kill('SIGKILL'); await exit; failed = true; }
      }
      if (output) await output.close();
      const logs = await readFile(fixture.logPath, 'utf8').catch(() => '');
      const nativeFailure = /ERROR: (AddressSanitizer|LeakSanitizer)|runtime error:/.test(logs) || (child && child.exitCode !== 0 && child.exitCode !== null) || shutdownTimedOut ||
        (child?.signalCode && !(shutdownRequested && child.signalCode === 'SIGTERM'));
      if (failed || nativeFailure || options.retain) console.error(`Contract fixture retained: ${directory}`);
      else await rm(directory, {recursive: true, force: true});
      if (nativeFailure) throw new Error(`Native fixture failed; inspect ${fixture.logPath}`);
    },
  };
  try {
    reservation = await reservePort(options.port);
    fixture.port = reservation.port;
    fixture.baseURL = `http://127.0.0.1:${fixture.port}`;
    for (const dir of ['recordings', 'models']) await mkdir(path.join(directory, dir));
    const config = `[general]\npid_file = ${directory}/lightnvr.pid\nlog_file = ${directory}/server.log\nlog_level = 2\nsyslog_enabled = false\n[storage]\npath = ${directory}/recordings\nmax_size = 0\nrecord_mp4_directly = false\n[database]\npath = ${directory}/lightnvr.db\n[web]\nport = ${fixture.port}\nbind_ip = 127.0.0.1\nroot = ${options.webRoot || `${root}/web/dist`}\nauth_enabled = true\nusername = admin\npassword = admin\nweb_thread_pool_size = 4\n[streams]\nmax_streams = ${maxStreams}\n[models]\npath = ${directory}/models\n[go2rtc]\nenabled = false\nauto_start = false\n` + (options.extraConfig ? `${options.extraConfig}\n` : '');
    const configPath = path.join(directory, 'lightnvr.ini');
    await writeFile(configPath, config);
    output = await open(fixture.logPath, 'w', 0o600);
    await reservation.release(); reservation = null;
    const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('LIGHTNVR_')));
    if(options.lprMasterKeyHex) env.LIGHTNVR_LPR_MASTER_KEY_HEX=options.lprMasterKeyHex;
    child = spawn(path.resolve(root, options.binary || process.env.LIGHTNVR_CONTRACT_BIN || 'build/eng-api-T2/bin/lightnvr'), ['-c', configPath], {cwd: root, env, stdio: ['ignore', output.fd, output.fd]});
    fixture.pid = child.pid;
    let spawnError;
    child.on('error', error => { spawnError = error; });
    exit = new Promise(resolve => child.once('close', resolve));
    const deadline = Date.now() + (options.startupTimeoutMs || 30000);
    let ready = false;
    while (Date.now() < deadline) {
      if (spawnError) throw spawnError;
      if (child.exitCode !== null || child.signalCode !== null) throw new Error(`Server exited during startup (${child.exitCode}/${child.signalCode})`);
      // Require our PID and successful bind in our own log before connecting.
      // A PID file alone does not prove ownership of a port lost after reservation.
      const pid = await readFile(path.join(directory, 'lightnvr.pid'), 'utf8').catch(() => '');
      const serverLog = await readFile(path.join(directory, 'server.log'), 'utf8').catch(() => '');
      if (Number(pid.trim()) === child.pid && serverLog.includes(`libuv_server_start: Listening on 127.0.0.1:${fixture.port}`)) {
        try { const r = await fetch(`${fixture.baseURL}/api/streams`, {signal: AbortSignal.timeout(500)}); ready = r.status === 401; } catch {}
      }
      if (ready) break;
      await pause(100);
    }
    if (!ready) throw new Error('Server startup timed out (or port was taken after reservation)');
    await fixture.json('/api/auth/users/1/password', {method: 'PUT', principal: 'anonymous', headers: {Authorization: basic({username: 'admin', password: 'admin'})}, body: {old_password: 'admin', new_password: credentials.admin.password}});
    if (options.seed !== false) await seedFixture(fixture);
    return fixture;
  } catch (error) {
    await fixture.stop({failed: true}).catch(stopError => { error.message += `; ${stopError.message}`; });
    error.message += `; fixture: ${directory}`;
    throw error;
  }
}

export function cameraScope(uuid) {
  return {type: 'selector', selector: {version: 1, expression: {op: 'camera_uuid', values: [uuid]}}};
}
export async function seedFixture(fixture) {
  await fixture.json('/api/setup/status', {method: 'POST', body: {complete: true}});
  const cameras = JSON.parse(await readFile(path.join(root, 'tests/contracts/fixtures/cameras.json'), 'utf8'));
  for (const camera of cameras) {
    await fixture.json('/api/streams', {method: 'POST', body: camera});
    fixture.cameras[camera.name] = (await fixture.json(`/api/streams/${camera.name}/full`)).stream;
  }
  const user = await fixture.json('/api/auth/users', {method: 'POST', body: {...credentials.viewer, role: 2, is_active: true}});
  fixture.users.viewer = user;
  const policy = await fixture.json(`/api/authorization/users/${user.id}`);
  await fixture.json(`/api/authorization/users/${user.id}`, {method: 'PUT', body: {mode: 'policy', expected_policy_version: policy.policy_version, grants: [{role_uuid: '00000000-0000-4000-8000-000000000003', scope: cameraScope(fixture.cameras.A.camera_uuid)}]}});
  const token = await fixture.json('/api/authorization/users/1/tokens', {method: 'POST', body: {description: 'contract-test-read-A', expires_at: Math.floor(Date.now()/1000)+3600, actions: ['live.view'], scope: cameraScope(fixture.cameras.A.camera_uuid)}});
  fixture.tokens.readA = token.secret;
}
