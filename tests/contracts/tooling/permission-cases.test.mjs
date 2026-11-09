import test from 'node:test';
import assert from 'node:assert/strict';
import {createUser} from '../permissionCases.ts';
import {basic} from '../fixture-server.mjs';

for (const extra of [{}, {role:0,is_active:false,allowed_login_cidrs:'192.0.2.0/24',username:'other',password:'other'}]) {
  test(`createUser keeps credentials consistent and preserves ${Object.keys(extra).length ? 'explicit policy overrides' : 'defaults'}`, async () => {
    let sent;
    const server = {async request(route, options) {
      assert.equal(route, '/api/auth/users');
      assert.equal(options.method, 'POST');
      sent = options.body;
      return Response.json({id:123});
    }};
    const identity = await createUser(server, 'owned-user', extra);
    assert.equal(sent.username, 'owned-user');
    assert.equal(sent.password, 'ContractMatrix123!');
    assert.equal(sent.role, extra.role ?? 2);
    assert.equal(sent.is_active, extra.is_active ?? true);
    assert.equal(sent.allowed_login_cidrs, extra.allowed_login_cidrs);
    assert.equal(identity.id, 123);
    assert.equal(identity.username, sent.username);
    assert.equal(identity.password, sent.password);
    assert.equal(identity.headers.Authorization, basic(sent));
  });
}
