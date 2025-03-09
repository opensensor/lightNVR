/**
 * LightNVR Web Interface Core JavaScript
 * Contains core functionality
 */

/**
 * Load template into main content area
 * This is kept for backward compatibility but is no longer used in the new architecture
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
