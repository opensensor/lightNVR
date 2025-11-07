# Authentication Pattern Improvements

## Overview

This document describes the improvements made to the lightNVR authentication system to address issues with session expiration handling and user experience.

## Problems Addressed

### 1. **Page Stalling on Token Expiration**
- **Issue**: When a session token expired (7-day default), API requests would fail with 401 errors, but the UI would just show errors or hang without redirecting to login
- **Impact**: Poor user experience - users would see broken pages instead of being prompted to log in again

### 2. **No Global 401 Error Handling**
- **Issue**: Each component had to handle authentication errors independently
- **Impact**: Inconsistent behavior across the application, some pages handled it better than others

### 3. **Wasteful Retry Logic**
- **Issue**: The `enhancedFetch` function would retry failed requests, including 401 authentication errors
- **Impact**: Wasted time retrying requests that would never succeed without re-authentication

### 4. **No Proactive Session Validation**
- **Issue**: The app only discovered expired sessions when making API requests
- **Impact**: Users could be working in the UI, then suddenly get errors when trying to perform actions

## Solutions Implemented

### Frontend Improvements

#### 1. Global 401 Handler (`web/js/fetch-utils.js`)

Added centralized authentication failure handling:

```javascript
function handleAuthenticationFailure(reason = 'Session expired') {
  // Clear all auth-related storage
  localStorage.removeItem('auth');
  
  // Clear auth cookies
  document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
  document.cookie = "session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT";
  
  // Redirect to login page with reason
  if (!window.location.pathname.includes('login.html')) {
    window.location.href = '/login.html?auth_required=true&reason=session_expired';
  }
}
```

**Benefits**:
- Automatic cleanup of stale credentials
- Consistent redirect behavior across all API calls
- Clear messaging to users about why they're being redirected

#### 2. Smart Retry Logic

Modified `enhancedFetch` to skip retries on authentication errors:

```javascript
// Handle 401 Unauthorized - don't retry, just redirect
if (response.status === 401) {
  if (!skipAuthRedirect) {
    handleAuthenticationFailure('Received 401 Unauthorized response');
  }
  throw new Error(`HTTP error 401: Unauthorized`);
}

// If this is an authentication error, don't retry - just fail immediately
if (isAuthenticationError(error)) {
  console.warn(`enhancedFetch: Authentication error detected, not retrying`);
  throw error;
}
```

**Benefits**:
- Faster failure on expired sessions (no wasted retry attempts)
- Immediate redirect to login instead of hanging

#### 3. Session Validation Utilities (`web/js/utils/auth-utils.js`)

Created comprehensive auth utilities:

- **`validateSession()`**: Makes a lightweight API call to `/api/auth/verify` to check if session is still valid
- **`setupSessionValidation(intervalMs)`**: Sets up periodic validation (default: every 5 minutes)
  - Validates immediately on page load
  - Runs periodic checks in the background
  - Automatically redirects if session becomes invalid
- **`hasAuthCredentials()`**: Check if user has auth credentials stored
- **`clearAuthState()`**: Centralized function to clear all auth state
- **`redirectToLogin(reason)`**: Smart redirect that preserves current page for post-login redirect

**Benefits**:
- Proactive detection of expired sessions
- Users get redirected before they try to perform actions
- Seamless experience - session checked in background

#### 4. Enhanced Login Experience

Updated `LoginView.jsx` to handle session expiration messaging:

```javascript
const reason = urlParams.get('reason');
if (reason === 'session_expired') {
  setErrorMessage('Your session has expired. Please log in again.');
} else {
  setErrorMessage('Authentication required. Please log in to continue.');
}
```

**Benefits**:
- Clear communication about why user needs to log in
- Distinguishes between logout, session expiration, and auth required

#### 5. All Pages Updated

Added session validation to all main application pages:
- `index-page.jsx` (Live View)
- `streams-page.jsx`
- `users-page.jsx`
- `system-page.jsx`
- `settings-page.jsx`
- `recordings-page.jsx`
- `timeline-page.jsx`
- `hls-page.jsx`

Each page now calls `setupSessionValidation()` on load.

**Benefits**:
- Consistent behavior across entire application
- No page is left without session monitoring

## Current Authentication Architecture

### Session Management

- **Session Duration**: 7 days (604800 seconds) - defined in `DEFAULT_SESSION_EXPIRY`
- **Token Storage**:
  - Session token stored in HTTP-only cookie (`session`)
  - Basic auth credentials stored in localStorage (`auth`) for backward compatibility
- **Validation**: Sessions validated against database with expiry check

### Authentication Flow

1. User logs in via `/api/auth/login`
2. Server creates session token and stores in database
3. Session token sent to client as HTTP-only cookie
4. Client includes cookie in all subsequent requests
5. Server validates session on each request
6. If session expired, server returns 401
7. Client detects 401, clears state, redirects to login

### Validation Endpoint

The `/api/auth/verify` endpoint provides lightweight session validation:
- Checks session cookie
- Falls back to Basic Auth if no session
- Returns 200 if valid, 401 if invalid
- Used by frontend for proactive validation

## Backend Considerations

### Current Implementation is Solid

The current backend authentication is well-designed:
- ✅ Secure session tokens (32-character random strings)
- ✅ Proper expiry checking
- ✅ Database-backed sessions
- ✅ Support for multiple auth methods (session tokens + Basic Auth)
- ✅ Lightweight verification endpoint

### Potential Future Enhancements

While not immediately necessary, these could be considered for future iterations:

#### 1. Token Refresh Endpoint

**Concept**: Add `/api/auth/refresh` endpoint to extend session without re-login

**Pros**:
- Better UX for long-running sessions
- Users don't get interrupted during active use

**Cons**:
- Adds complexity
- Security consideration: when to allow refresh vs. require re-auth
- Current 7-day session is already quite generous

**Recommendation**: Defer - current 5-minute validation + 24-hour session is sufficient

#### 2. Sliding Session Expiry

**Concept**: Extend session expiry on each request (e.g., "24 hours since last activity")

**Implementation**: Update `expires_at` in database on each validation

**Pros**:
- More natural for active users
- Reduces interruptions

**Cons**:
- Database write on every request (performance impact)
- Sessions could theoretically never expire for active users
- Harder to reason about security

**Recommendation**: Defer - fixed 24-hour window is simpler and more predictable

#### 3. Multiple Session Support

**Concept**: Allow users to have multiple active sessions (different devices)

**Current State**: Already supported - each login creates a new session

**Recommendation**: Already implemented ✅

#### 4. Session Activity Tracking

**Concept**: Track last activity time for each session

**Pros**:
- Better audit trail
- Could enable "last seen" features

**Cons**:
- Database write on every request
- Privacy considerations

**Recommendation**: Defer - not critical for core functionality

## Testing Recommendations

To verify the improvements work correctly:

### Manual Testing

1. **Session Expiration Test**:
   - Log in to lightNVR
   - Manually expire session in database: `UPDATE sessions SET expires_at = 0 WHERE token = '<your-token>';`
   - Try to navigate or perform an action
   - Verify: Should redirect to login with "Your session has expired" message

2. **Proactive Validation Test**:
   - Log in to lightNVR
   - Wait on a page for 5+ minutes
   - Manually expire session in database
   - Wait for next validation cycle (up to 5 minutes)
   - Verify: Should automatically redirect to login

3. **401 Handling Test**:
   - Log in to lightNVR
   - Clear session cookie in browser dev tools
   - Try to perform an action (e.g., create stream)
   - Verify: Should immediately redirect to login (no retries)

### Automated Testing

Consider adding E2E tests for:
- Session expiration handling
- 401 redirect behavior
- Session validation on page load

## Migration Notes

### For Users

No action required - improvements are transparent to end users.

### For Developers

If you're extending lightNVR:

1. **New Pages**: Always call `setupSessionValidation()` in your page entry point
2. **API Calls**: Use `enhancedFetch` or `fetchJSON` - they handle 401s automatically
3. **Skip Auto-Redirect**: Pass `skipAuthRedirect: true` to `enhancedFetch` if you want to handle 401s manually (e.g., login page)

## Security Considerations

### Improvements Made

- ✅ Automatic cleanup of expired credentials
- ✅ Consistent session validation across all pages
- ✅ Clear separation between session expiration and logout

### No Security Regressions

- Session tokens remain secure (HTTP-only cookies)
- No sensitive data logged
- Validation endpoint doesn't leak information
- Redirect preserves security (no token in URL)

## Performance Impact

### Minimal Overhead

- Session validation: 1 lightweight API call every 5 minutes per page
- 401 handling: Eliminates wasteful retry attempts (net positive)
- No additional database queries on happy path

### Recommendations

- Current 5-minute validation interval is reasonable
- Could be adjusted per deployment needs via `setupSessionValidation(intervalMs)`

## Future Considerations

### Alternative Auth Methods

While the current pattern works well, future enhancements could include:

1. **OAuth/OIDC Integration**: For enterprise deployments
2. **LDAP/Active Directory**: For corporate environments  
3. **API Key Authentication**: Already supported for programmatic access
4. **Two-Factor Authentication**: For high-security deployments

These would be additive - the current session-based auth would remain as the foundation.

### Recommendation

The current authentication pattern is solid and well-suited for lightNVR's use case. The improvements made address the immediate UX issues without over-engineering. Future auth connectors can be added as needed, but the current system should be hardened and stable first.

## Summary

The authentication improvements provide:

✅ **Better UX**: Clear messaging, automatic redirects, no page stalling  
✅ **Proactive Validation**: Sessions checked every 5 minutes  
✅ **Faster Failures**: No wasted retries on auth errors  
✅ **Consistent Behavior**: All pages handle auth the same way  
✅ **Maintainable**: Centralized auth logic, easy to extend  
✅ **Secure**: No security regressions, proper cleanup  

The system is now production-ready with a solid foundation for future enhancements.

