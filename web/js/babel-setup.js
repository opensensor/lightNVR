/**
 * Babel setup for JSX transpilation in the browser
 * This script configures Babel to transpile JSX to JavaScript
 */

// Configure Babel to use Preact's h() function for JSX
window.Babel = window.Babel || {};
window.Babel.plugins = window.Babel.plugins || [];
window.Babel.presets = window.Babel.presets || [];

// Add the JSX transform plugin with Preact configuration
window.Babel.plugins.push(['transform-react-jsx', { pragma: 'h' }]);

// Add the ES2015 preset for modern JavaScript features
window.Babel.presets.push('es2015');

// Configure script type for JSX
document.addEventListener('DOMContentLoaded', () => {
  // Find all script tags with type="text/babel"
  const scripts = document.querySelectorAll('script[type="text/babel"]');
  
  // Process each script
  scripts.forEach(script => {
    // For external scripts, load them first
    if (script.src) {
      fetch(script.src)
        .then(response => response.text())
        .then(code => {
          // Transpile the code
          const transpiled = Babel.transform(code, {
            presets: window.Babel.presets,
            plugins: window.Babel.plugins
          }).code;
          
          // Create a new script element with the transpiled code
          const newScript = document.createElement('script');
          newScript.textContent = transpiled;
          newScript.type = 'module';
          
          // Replace the original script with the new one
          script.parentNode.replaceChild(newScript, script);
        })
        .catch(error => {
          console.error('Error loading script:', error);
        });
    } else {
      // For inline scripts
      const code = script.textContent;
      
      // Transpile the code
      const transpiled = Babel.transform(code, {
        presets: window.Babel.presets,
        plugins: window.Babel.plugins
      }).code;
      
      // Create a new script element with the transpiled code
      const newScript = document.createElement('script');
      newScript.textContent = transpiled;
      newScript.type = 'module';
      
      // Replace the original script with the new one
      script.parentNode.replaceChild(newScript, script);
    }
  });
});
