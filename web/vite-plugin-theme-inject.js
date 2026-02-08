/**
 * Vite Plugin: Theme Injection
 * 
 * This plugin reads COLOR_THEMES from theme-init.js and injects the theme
 * initialization script into all HTML files, replacing the <!-- THEME_INIT_SCRIPT --> placeholder.
 * 
 * This ensures themes are defined in a single place (theme-init.js) and automatically
 * synced across all HTML files during build.
 */

import { readFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Extract COLOR_THEMES object from theme-init.js
 */
function extractColorThemes() {
  const themeInitPath = resolve(__dirname, 'js/utils/theme-init.js');
  const content = readFileSync(themeInitPath, 'utf-8');
  
  // Extract the COLOR_THEMES object using regex
  const match = content.match(/export const COLOR_THEMES = (\{[\s\S]*?\n\});/);
  if (!match) {
    throw new Error('Could not find COLOR_THEMES in theme-init.js');
  }
  
  // Evaluate the object (safely, since it's our own code)
  // We need to handle the object literal format
  const objectStr = match[1];
  
  // Use Function constructor to safely evaluate the object literal
  const colorThemes = new Function(`return ${objectStr}`)();
  return colorThemes;
}

/**
 * Generate the inline theme initialization script
 */
function generateThemeScript(colorThemes) {
  const themesJson = JSON.stringify(colorThemes);
  
  return `<script>
    (function() {
      try {
        const COLOR_THEMES = ${themesJson};
        const savedTheme = localStorage.getItem('lightnvr-theme');
        const savedColorIntensity = localStorage.getItem('lightnvr-color-intensity');
        const savedColorTheme = localStorage.getItem('lightnvr-color-theme');
        const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        let finalTheme = 'light';
        if (savedTheme) { finalTheme = savedTheme; } else if (systemPrefersDark) { finalTheme = 'dark'; }
        if (finalTheme === 'dark') { document.documentElement.classList.add('dark'); } else { document.documentElement.classList.remove('dark'); }
        if (savedColorIntensity && savedColorTheme) {
          const colorTheme = savedColorTheme in COLOR_THEMES ? savedColorTheme : 'default';
          const colorIntensity = parseInt(savedColorIntensity) || 50;
          const isDark = finalTheme === 'dark';
          const selectedTheme = COLOR_THEMES[colorTheme];
          const themeColors = isDark ? selectedTheme.dark : selectedTheme.light;
          const intensity = colorIntensity / 100;
          const root = document.documentElement;
          if (isDark) {
            const bgLightness = Math.round(8 + (intensity * 12));
            const cardLightness = Math.round(10 + (intensity * 15));
            const accentLightness = Math.round(22 + (intensity * 18));
            const borderLightness = Math.round(20 + (intensity * 20));
            const primaryLightness = Math.round(45 + (intensity * 15));
            root.style.setProperty('--background', themeColors.hue + ' 10% ' + bgLightness + '%');
            root.style.setProperty('--foreground', themeColors.hue + ' 5% 90%');
            root.style.setProperty('--card', themeColors.hue + ' 10% ' + cardLightness + '%');
            root.style.setProperty('--card-foreground', themeColors.hue + ' 5% 90%');
            root.style.setProperty('--popover', themeColors.hue + ' 10% ' + cardLightness + '%');
            root.style.setProperty('--popover-foreground', themeColors.hue + ' 5% 90%');
            root.style.setProperty('--primary', themeColors.hue + ' ' + themeColors.saturation + '% ' + primaryLightness + '%');
            root.style.setProperty('--primary-foreground', '0 0% 98%');
            root.style.setProperty('--secondary', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(18 + (intensity * 15)) + '%');
            root.style.setProperty('--secondary-foreground', '0 0% 98%');
            root.style.setProperty('--muted', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(16 + (intensity * 12)) + '%');
            root.style.setProperty('--muted-foreground', themeColors.hue + ' 5% 65%');
            root.style.setProperty('--accent', themeColors.hue + ' ' + themeColors.saturation + '% ' + accentLightness + '%');
            root.style.setProperty('--accent-foreground', themeColors.hue + ' 5% 90%');
            root.style.setProperty('--border', themeColors.hue + ' ' + themeColors.saturation + '% ' + borderLightness + '%');
            root.style.setProperty('--input', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(18 + (intensity * 12)) + '%');
            root.style.setProperty('--success', '142 76% 45%');
            root.style.setProperty('--success-foreground', '0 0% 98%');
            root.style.setProperty('--success-muted', '142 76% 15%');
            root.style.setProperty('--success-muted-foreground', '142 76% 70%');
            root.style.setProperty('--warning', '38 92% 55%');
            root.style.setProperty('--warning-foreground', '0 0% 10%');
            root.style.setProperty('--warning-muted', '38 92% 15%');
            root.style.setProperty('--warning-muted-foreground', '38 92% 75%');
            root.style.setProperty('--danger', '0 84% 60%');
            root.style.setProperty('--danger-foreground', '0 0% 98%');
            root.style.setProperty('--danger-muted', '0 84% 15%');
            root.style.setProperty('--danger-muted-foreground', '0 84% 75%');
            root.style.setProperty('--info', '217 91% 60%');
            root.style.setProperty('--info-foreground', '0 0% 98%');
            root.style.setProperty('--info-muted', '217 91% 15%');
            root.style.setProperty('--info-muted-foreground', '217 91% 75%');
          } else {
            const bgLightness = Math.round(98 - (intensity * 5));
            const cardLightness = Math.round(100 - (intensity * 2));
            const foregroundLightness = Math.round(15 + (intensity * 10));
            const accentLightness = Math.round(88 + (intensity * 8));
            const borderLightness = Math.round(85 + (intensity * 10));
            const primaryLightness = Math.round(50 - (intensity * 15));
            root.style.setProperty('--background', themeColors.hue + ' 10% ' + bgLightness + '%');
            root.style.setProperty('--foreground', themeColors.hue + ' 10% ' + foregroundLightness + '%');
            root.style.setProperty('--card', themeColors.hue + ' 10% ' + cardLightness + '%');
            root.style.setProperty('--card-foreground', themeColors.hue + ' 10% ' + foregroundLightness + '%');
            root.style.setProperty('--popover', themeColors.hue + ' 10% ' + cardLightness + '%');
            root.style.setProperty('--popover-foreground', themeColors.hue + ' 10% ' + foregroundLightness + '%');
            root.style.setProperty('--primary', themeColors.hue + ' ' + themeColors.saturation + '% ' + primaryLightness + '%');
            root.style.setProperty('--primary-foreground', '0 0% 98%');
            root.style.setProperty('--secondary', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(92 + (intensity * 6)) + '%');
            root.style.setProperty('--secondary-foreground', '0 0% 15%');
            root.style.setProperty('--muted', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(92 + (intensity * 6)) + '%');
            root.style.setProperty('--muted-foreground', themeColors.hue + ' 10% 45%');
            root.style.setProperty('--accent', themeColors.hue + ' ' + themeColors.saturation + '% ' + accentLightness + '%');
            root.style.setProperty('--accent-foreground', themeColors.hue + ' 10% ' + foregroundLightness + '%');
            root.style.setProperty('--border', themeColors.hue + ' ' + themeColors.saturation + '% ' + borderLightness + '%');
            root.style.setProperty('--input', themeColors.hue + ' ' + themeColors.saturation + '% ' + Math.round(92 + (intensity * 6)) + '%');
            root.style.setProperty('--success', '142 76% 36%');
            root.style.setProperty('--success-foreground', '0 0% 98%');
            root.style.setProperty('--success-muted', '142 76% 95%');
            root.style.setProperty('--success-muted-foreground', '142 76% 25%');
            root.style.setProperty('--warning', '38 92% 50%');
            root.style.setProperty('--warning-foreground', '0 0% 98%');
            root.style.setProperty('--warning-muted', '38 92% 95%');
            root.style.setProperty('--warning-muted-foreground', '38 92% 30%');
            root.style.setProperty('--danger', '0 84% 60%');
            root.style.setProperty('--danger-foreground', '0 0% 98%');
            root.style.setProperty('--danger-muted', '0 84% 95%');
            root.style.setProperty('--danger-muted-foreground', '0 84% 40%');
            root.style.setProperty('--info', '217 91% 60%');
            root.style.setProperty('--info-foreground', '0 0% 98%');
            root.style.setProperty('--info-muted', '217 91% 95%');
            root.style.setProperty('--info-muted-foreground', '217 91% 35%');
          }
        }
        window.__LIGHTNVR_THEME_APPLIED__ = true;
      } catch (e) {
        console.warn('Theme script failed:', e);
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) { document.documentElement.classList.add('dark'); }
      }
    })();
    </script>`;
}

/**
 * Vite plugin that injects theme initialization script
 */
export default function themeInjectPlugin() {
  let colorThemes;
  let themeScript;
  
  return {
    name: 'theme-inject',
    
    // Build start - extract themes once
    buildStart() {
      colorThemes = extractColorThemes();
      themeScript = generateThemeScript(colorThemes);
      console.log(`[theme-inject] Loaded ${Object.keys(colorThemes).length} color themes`);
    },
    
    // Transform HTML files
    transformIndexHtml(html) {
      // Replace the placeholder with the generated script
      if (html.includes('<!-- THEME_INIT_SCRIPT -->')) {
        return html.replace('<!-- THEME_INIT_SCRIPT -->', themeScript);
      }
      return html;
    }
  };
}

