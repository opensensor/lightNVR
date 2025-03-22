/**
 * LightNVR Web Interface Authentication Module
 * Handles authentication for all pages
 */

// Check if authentication is required and redirect if needed
function checkAuthentication() {
    // Skip authentication check for login page
    if (window.location.pathname.endsWith('login.html')) {
        return;
    }
    
    // Get stored credentials
    const auth = localStorage.getItem('auth');
    
    // Also check for auth cookie
    const hasCookie = document.cookie.split(';').some(item => item.trim().startsWith('auth='));
    
    // If we have neither auth in localStorage nor a cookie, redirect to login
    if (!auth && !hasCookie) {
        window.location.href = 'login.html';
        return;
    }
    
    // Make a test request to check if authentication is valid
    fetch('/api/settings', {
        headers: auth ? {
            'Authorization': 'Basic ' + auth
        } : {},
        credentials: 'include' // Include cookies in the request
    })
    .then(response => {
        if (response.status === 401) {
            // Authentication required but invalid credentials
            localStorage.removeItem('auth'); // Clear invalid auth
            window.location.href = 'login.html';
        }
    })
    .catch(error => {
        console.error('Authentication check failed:', error);
        // Continue anyway in case of network error
    });
}

// Add authentication header to all fetch requests
function setupAuthInterceptor() {
    const originalFetch = window.fetch;
    
    window.fetch = function(url, options = {}) {
        const auth = localStorage.getItem('auth');
        
        // Set credentials to include cookies
        options.credentials = options.credentials || 'include';
        
        if (auth) {
            options.headers = options.headers || {};
            options.headers['Authorization'] = 'Basic ' + auth;
        }
        
        // Check if this is an HLS request and we just redirected from login
        const lastRedirectTime = localStorage.getItem('lastRedirectTime');
        const currentTime = new Date().getTime();
        
        // If this is an HLS request within 5 seconds of a redirect, ensure auth headers are set
        if (lastRedirectTime && (currentTime - parseInt(lastRedirectTime) < 5000) && 
            (url.includes('/hls/') || url.includes('/api/streaming/'))) {
            console.log('Recent redirect detected, ensuring auth headers for HLS request');
            options.headers = options.headers || {};
            if (auth) {
                options.headers['Authorization'] = 'Basic ' + auth;
            }
            options.credentials = 'include';
        }
        
        return originalFetch(url, options)
            .then(response => {
                // If we get a 401, redirect to login page
                if (response.status === 401 && !window.location.pathname.endsWith('login.html')) {
                    window.location.href = 'login.html?t=' + new Date().getTime();
                }
                return response;
            });
    };
    
    // Also intercept link clicks to add authentication
    document.addEventListener('click', function(e) {
        // Check if the clicked element is a link
        let target = e.target;
        while (target && target.tagName !== 'A') {
            target = target.parentElement;
        }
        
        // If it's a link and not an external link or anchor
        if (target && target.tagName === 'A' && 
            !target.getAttribute('href').startsWith('http') && 
            !target.getAttribute('href').startsWith('#') &&
            !target.getAttribute('href').startsWith('javascript:')) {
            
            // Get the current href
            let href = target.getAttribute('href');
            
            // If it doesn't already have a timestamp parameter
            if (!href.includes('t=')) {
                // Add a timestamp parameter to force a fresh request
                const separator = href.includes('?') ? '&' : '?';
                href = href + separator + 't=' + new Date().getTime();
                target.setAttribute('href', href);
            }
        }
    });
}

// Handle logout
function logout() {
    // Clear localStorage
    localStorage.removeItem('auth');
    localStorage.removeItem('lastRedirectTime');
    
    // Call the logout endpoint to clear browser's basic auth cache
    fetch('/api/auth/logout', {
        method: 'POST',
        credentials: 'include'
    }).then(() => {
        // Clear any cookies by setting them to expire in the past
        document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
        
        // Use a form submission to navigate to login page without carrying over auth headers
        const form = document.createElement('form');
        form.method = 'GET';
        form.action = 'login.html?logout=true&t=' + new Date().getTime();
        document.body.appendChild(form);
        form.submit();
    }).catch(() => {
        // Redirect even if the request fails
        document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
        
        const form = document.createElement('form');
        form.method = 'GET';
        form.action = 'login.html?logout=true&t=' + new Date().getTime();
        document.body.appendChild(form);
        form.submit();
    });
}

// Initialize authentication
document.addEventListener('DOMContentLoaded', function() {
    checkAuthentication();
    setupAuthInterceptor();
    
    // Add event listener to logout button if it exists
    const logoutButton = document.getElementById('logout');
    if (logoutButton) {
        logoutButton.addEventListener('click', function(e) {
            e.preventDefault();
            logout();
        });
    }
});
