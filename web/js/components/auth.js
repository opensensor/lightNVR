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
            window.location.href = 'login.html?auth_required=1';
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
        
        // Always ensure auth headers are set for HLS requests
        if (url.includes('/hls/') || url.includes('/api/streaming/')) {
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
                    window.location.href = 'login.html?auth_required=1&t=' + new Date().getTime();
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
    
    // Clear any cookies
    document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Strict";
    
    // Create a timeout to handle potential stalls
    const timeoutId = setTimeout(() => {
        console.log('Logout request timed out, proceeding anyway');
        // Even if the request stalls, redirect to login page
        window.location.href = '/login.html?t=' + new Date().getTime();
    }, 2000); // 2 second timeout
    
    // Use fetch to call the logout API
    fetch('/api/auth/logout', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'X-Requested-With': 'XMLHttpRequest'
        },
        credentials: 'include'
    })
    .then(response => {
        clearTimeout(timeoutId); // Clear the timeout
        // Redirect to login page regardless of response
        window.location.href = '/login.html?t=' + new Date().getTime();
    })
    .catch(error => {
        clearTimeout(timeoutId); // Clear the timeout
        console.error('Logout error:', error);
        // Still redirect to login page even if there's an error
        window.location.href = '/login.html?t=' + new Date().getTime();
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
