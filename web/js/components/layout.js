/**
 * LightNVR Web Interface Layout Components
 * Contains shared layout elements like header and footer
 */

/**
 * Load the header into the specified element
 */
function loadHeader(activePageId) {
    const headerContainer = document.getElementById('header-container');
    if (!headerContainer) return;

    headerContainer.innerHTML = `
        <div class="logo">
            <h1>LightNVR</h1>
            <span class="version">v0.2.0</span>
        </div>
        <nav>
            <ul>
                <li><a href="live.html" id="nav-live" ${activePageId === 'nav-live' ? 'class="active"' : ''}>Live View</a></li>
                <li><a href="recordings.html" id="nav-recordings" ${activePageId === 'nav-recordings' ? 'class="active"' : ''}>Recordings</a></li>
                <li><a href="streams.html" id="nav-streams" ${activePageId === 'nav-streams' ? 'class="active"' : ''}>Streams</a></li>
                <li><a href="settings.html" id="nav-settings" ${activePageId === 'nav-settings' ? 'class="active"' : ''}>Settings</a></li>
                <li><a href="system.html" id="nav-system" ${activePageId === 'nav-system' ? 'class="active"' : ''}>System</a></li>
            </ul>
        </nav>
        <div class="user-menu">
            <span id="username">Admin</span>
            <a href="index.html?logout=true" id="logout">Logout</a>
        </div>
    `;
}

/**
 * Load the footer into the specified element
 */
function loadFooter() {
    const footerContainer = document.getElementById('footer-container');
    if (!footerContainer) return;

    footerContainer.innerHTML = `
        <div class="status">
            <span id="status-indicator" class="status-ok"></span>
            <span id="status-text">System running normally</span>
        </div>
        <div class="copyright">
            &copy; 2025 LightNVR - <a href="https://github.com/yourusername/lightnvr" target="_blank">GitHub</a>
        </div>
    `;
}
