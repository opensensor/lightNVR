/**
 * LightNVR Web Interface Layout Components for Alpine.js
 * Contains functionality for header, footer, and navigation
 */

document.addEventListener('alpine:init', () => {
    // Header component
    Alpine.data('header', () => ({
        version: '0.2.0',
        username: 'Admin',
        
        init() {
            // Fetch system version
            this.fetchSystemVersion();
        },
        
        async fetchSystemVersion() {
            try {
                const response = await fetch('/api/system');
                if (!response.ok) {
                    throw new Error('Failed to load system information');
                }
                
                const data = await response.json();
                if (data.version) {
                    this.version = data.version;
                }
            } catch (error) {
                console.error('Error loading system version:', error);
            }
        }
    }));
    
    // Footer component
    Alpine.data('footer', () => ({
        year: new Date().getFullYear(),
        version: '0.2.0',
        
        init() {
            // Fetch system version
            this.fetchSystemVersion();
        },
        
        async fetchSystemVersion() {
            try {
                const response = await fetch('/api/system');
                if (!response.ok) {
                    throw new Error('Failed to load system information');
                }
                
                const data = await response.json();
                if (data.version) {
                    this.version = data.version;
                }
            } catch (error) {
                console.error('Error loading system version:', error);
            }
        }
    }));
});

/**
 * Load header with active navigation item
 * @param {string} activeNav - ID of the active navigation item
 */
function loadHeader(activeNav = '') {
    const headerContainer = document.getElementById('header-container');
    if (!headerContainer) return;
    
    headerContainer.innerHTML = `
        <header x-data="header" class="bg-gray-800 text-white py-2 px-4 flex justify-between items-center shadow-md mb-4">
            <div class="logo">
                <h1 class="text-xl font-bold m-0">LightNVR</h1>
                <span class="version text-blue-200 text-xs ml-2" x-text="'v' + version">v0.2.0</span>
            </div>
            <nav>
                <ul class="flex list-none m-0 p-0">
                <li class="mx-1"><a href="live.html" id="nav-live" class="text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-live' ? 'bg-blue-600' : 'hover:bg-blue-700'}">Live View</a></li>
                <li class="mx-1"><a href="recordings.html" id="nav-recordings" class="text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-recordings' ? 'bg-blue-600' : 'hover:bg-blue-700'}">Recordings</a></li>
                <li class="mx-1"><a href="streams.html" id="nav-streams" class="text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-streams' ? 'bg-blue-600' : 'hover:bg-blue-700'}">Streams</a></li>
                <li class="mx-1"><a href="settings.html" id="nav-settings" class="text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-settings' ? 'bg-blue-600' : 'hover:bg-blue-700'}">Settings</a></li>
                <li class="mx-1"><a href="system.html" id="nav-system" class="text-white no-underline px-3 py-2 rounded transition-colors ${activeNav === 'nav-system' ? 'bg-blue-600' : 'hover:bg-blue-700'}">System</a></li>
                </ul>
            </nav>
            <div class="user-menu">
                <span class="mr-4" x-text="username">Admin</span>
                <a href="login.html" class="text-white no-underline hover:underline">Logout</a>
            </div>
        </header>
    `;
}

/**
 * Load footer
 */
function loadFooter() {
    const footerContainer = document.getElementById('footer-container');
    if (!footerContainer) return;
    
    footerContainer.innerHTML = `
        <footer x-data="footer" class="bg-gray-800 text-white py-3 px-4 flex justify-between items-center text-sm mt-4">
            <div>&copy; <span x-text="year">2023</span> LightNVR</div>
            <div>Version <span x-text="version">0.2.0</span></div>
        </footer>
    `;
}
