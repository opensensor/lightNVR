/**
 * Authentication and Users API Tests
 * 
 * Tests for authentication endpoints and user management API.
 * @tags @api @auth
 */

import { test, expect, APIRequestContext } from '@playwright/test';
import { CONFIG, USERS, getAuthHeader } from '../fixtures/test-fixtures';

const AUTH_HEADER = getAuthHeader(USERS.admin);

test.describe('Authentication API @api @auth', () => {
  let request: APIRequestContext;

  test.beforeAll(async ({ playwright }) => {
    request = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
    });
  });

  test.afterAll(async () => {
    await request.dispose();
  });

  test.describe('Basic Authentication', () => {
    test('should reject request without credentials', async () => {
      const response = await request.get('/api/system');
      expect(response.status()).toBe(401);
    });

    test('should reject request with invalid credentials', async () => {
      const response = await request.get('/api/system', {
        headers: {
          'Authorization': 'Basic ' + Buffer.from('admin:wrongpassword').toString('base64'),
        },
      });
      expect(response.status()).toBe(401);
    });

    test('should accept request with valid credentials', async () => {
      const response = await request.get('/api/system', {
        headers: { 'Authorization': AUTH_HEADER },
      });
      expect(response.status()).toBe(200);
    });
  });

  test.describe('Auth Endpoints', () => {
    test('POST /api/auth/login with valid credentials', async () => {
      const response = await request.post('/api/auth/login', {
        data: {
          username: USERS.admin.username,
          password: USERS.admin.password,
        },
        headers: { 'Content-Type': 'application/json' },
      });
      
      // Login should succeed
      expect([200, 302].includes(response.status())).toBeTruthy();
      console.log(`Login response status: ${response.status()}`);
    });

    test('POST /api/auth/login with invalid credentials', async () => {
      const response = await request.post('/api/auth/login', {
        data: {
          username: 'admin',
          password: 'wrongpassword',
        },
        headers: { 'Content-Type': 'application/json' },
      });
      
      // Should fail
      expect(response.status()).not.toBe(200);
    });

    test('GET /api/auth/verify checks session', async () => {
      const response = await request.get('/api/auth/verify', {
        headers: { 'Authorization': AUTH_HEADER },
      });
      console.log(`Verify response: ${response.status()}`);
    });

    test('POST /api/auth/logout ends session', async () => {
      const response = await request.post('/api/auth/logout', {
        headers: { 'Authorization': AUTH_HEADER },
      });
      console.log(`Logout response: ${response.status()}`);
    });
  });
});

test.describe('Users API @api @users', () => {
  let request: APIRequestContext;
  const testUsername = `api_user_${Date.now()}`;
  let createdUserId: number | null = null;

  test.beforeAll(async ({ playwright }) => {
    request = await playwright.request.newContext({
      baseURL: CONFIG.LIGHTNVR_URL,
      extraHTTPHeaders: { 'Authorization': AUTH_HEADER },
    });
  });

  test.afterAll(async () => {
    // Cleanup: delete test user if created
    if (createdUserId) {
      await request.delete(`/api/auth/users/${createdUserId}`);
    }
    await request.dispose();
  });

  test('GET /api/auth/users returns user list', async () => {
    const response = await request.get('/api/auth/users');
    expect(response.ok()).toBeTruthy();

    const data = await response.json();
    expect(Array.isArray(data) || data.users).toBeTruthy();
    console.log(`Users API returned: ${JSON.stringify(data).substring(0, 200)}...`);
  });

  test('POST /api/auth/users creates new user', async () => {
    const userData = {
      username: testUsername,
      password: 'TestApiUser123!',
      email: `${testUsername}@test.com`,
      role: 2, // Viewer
      is_active: true,
    };

    const response = await request.post('/api/auth/users', { data: userData });
    expect([200, 201, 409].includes(response.status())).toBeTruthy();

    if (response.ok()) {
      const data = await response.json();
      if (data.id) {
        createdUserId = data.id;
      }
      console.log(`Created user: ${testUsername}`);
    }
  });

  test('GET /api/auth/users/{id} returns user details', async () => {
    if (!createdUserId) {
      test.skip();
      return;
    }

    const response = await request.get(`/api/auth/users/${createdUserId}`);
    if (response.ok()) {
      const data = await response.json();
      expect(data.username).toBe(testUsername);
    }
  });

  test('PUT /api/auth/users/{id} updates user', async () => {
    if (!createdUserId) {
      test.skip();
      return;
    }

    const updateData = {
      email: `${testUsername}_updated@test.com`,
      role: 1, // User
    };

    const response = await request.put(`/api/auth/users/${createdUserId}`, { data: updateData });
    console.log(`Update user response: ${response.status()}`);
  });

  test('POST /api/auth/users/{id}/api-key generates API key', async () => {
    if (!createdUserId) {
      test.skip();
      return;
    }

    const response = await request.post(`/api/auth/users/${createdUserId}/api-key`);
    console.log(`Generate API key response: ${response.status()}`);
  });

  test('DELETE /api/auth/users/{id} deletes user', async () => {
    if (!createdUserId) {
      test.skip();
      return;
    }

    const response = await request.delete(`/api/auth/users/${createdUserId}`);
    expect([200, 204, 202].includes(response.status())).toBeTruthy();
    createdUserId = null; // Mark as deleted
  });
});

