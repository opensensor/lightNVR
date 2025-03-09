/**
 * LightNVR Web Interface Core JavaScript
 * Contains initialization and router functionality
 */

// Wait for DOM to be fully loaded
document.addEventListener('DOMContentLoaded', function() {
    // Initialize the application
    initApp();
});

/**
 * Initialize the application
 */
function initApp() {
    // Initialize router for URL-based navigation
    initRouter();

    // Set up modals
    setupModals();

    // Set up snapshot modal
    setupSnapshotModal();

    // Add stream styles
    addStreamStyles();

    // Add status message styles
    addStatusMessageStyles();

    console.log('LightNVR Web Interface initialized');
}

/**
 * Initialize the router for URL-based navigation
 */
function initRouter() {
    // Define routes and corresponding handlers
    const router = {
        routes: {
            '/': loadLiveView,
            '/recordings': loadRecordingsView,
            '/streams': loadStreamsView,
            '/settings': loadSettingsView,
            '/system': loadSystemView,
            '/debug': loadDebugView,
            '/logout': handleLogout
        },

        init: function() {
            // Handle initial page load based on URL
            this.navigate(window.location.pathname || '/');

            // Handle navigation clicks
            document.querySelectorAll('nav a').forEach(link => {
                link.addEventListener('click', (e) => {
                    e.preventDefault();
                    const path = link.getAttribute('href');
                    this.navigate(path);
                });
            });

            // Handle browser back/forward buttons
            window.addEventListener('popstate', (e) => {
                this.navigate(window.location.pathname, false);
            });

            // Handle page links outside of navigation
            document.addEventListener('click', e => {
                const link = e.target.closest('a[href^="/"]');
                if (link && !link.hasAttribute('target') && !link.hasAttribute('download')) {
                    e.preventDefault();
                    this.navigate(link.getAttribute('href'));
                }
            });
        },

        navigate: function(path, addToHistory = true) {
            // Default to home if path is not recognized
            if (!this.routes[path]) {
                path = '/';
            }

            // Update URL in the address bar
            if (addToHistory) {
                history.pushState({path: path}, '', path);
            }

            // Update active navigation link
            document.querySelectorAll('nav a').forEach(a => {
                a.classList.remove('active');
            });

            const activeNav = document.querySelector(`nav a[href="${path}"]`);
            if (activeNav) {
                activeNav.classList.add('active');
            }

            // Load the requested page content
            this.routes[path]();
        }
    };

    // Initialize the router
    router.init();

    // Store router in window for global access
    window.appRouter = router;
}

/**
 * Load template into main content area
 */
function loadTemplate(templateId) {
    const mainContent = document.getElementById('main-content');
    const template = document.getElementById(templateId);

    if (template && mainContent) {
        mainContent.innerHTML = '';
        const clone = document.importNode(template.content, true);
        mainContent.appendChild(clone);
        return true;
    }
    return false;
}

/**
 * Handle logout
 */
function handleLogout() {
    if (confirm('Are you sure you want to logout?')) {
        // In a real implementation, this would make an API call to logout
        alert('Logout successful');
        // Redirect to login page (would be implemented in a real app)
        window.appRouter.navigate('/');
    } else {
        // If user cancels logout, go back to previous page
        window.history.back();
    }
}
