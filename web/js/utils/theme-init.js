/**
 * Theme initialization script for LightNVR
 * This script runs immediately to prevent FOUC (Flash of Unstyled Content)
 * Based on the accounting app's theme system
 */

// Color theme definitions
export const COLOR_THEMES = {
  default: {
    name: 'Default',
    light: { hue: 240, saturation: 5.9 },
    dark: { hue: 240, saturation: 3.7 },
    icon: '🎨'
  },
  blue: {
    name: 'Ocean Blue',
    light: { hue: 217, saturation: 32 },
    dark: { hue: 217, saturation: 25 },
    icon: '🌊'
  },
  emerald: {
    name: 'Forest Green',
    light: { hue: 160, saturation: 30 },
    dark: { hue: 160, saturation: 25 },
    icon: '🌲'
  },
  purple: {
    name: 'Royal Purple',
    light: { hue: 265, saturation: 28 },
    dark: { hue: 265, saturation: 20 },
    icon: '👑'
  },
  rose: {
    name: 'Sunset Rose',
    light: { hue: 350, saturation: 25 },
    dark: { hue: 350, saturation: 18 },
    icon: '🌹'
  },
  amber: {
    name: 'Golden Amber',
    light: { hue: 45, saturation: 28 },
    dark: { hue: 45, saturation: 20 },
    icon: '⚡'
  },
  slate: {
    name: 'Cool Slate',
    light: { hue: 215, saturation: 8 },
    dark: { hue: 215, saturation: 6 },
    icon: '🗿'
  }
};

/**
 * Apply theme colors to CSS variables
 * @param {boolean} isDark - Whether dark mode is active
 * @param {string} colorTheme - The color theme ID
 * @param {number} colorIntensity - The color intensity (0-100)
 */
export function applyThemeColors(isDark, colorTheme, colorIntensity) {
  const root = document.documentElement;
  const selectedTheme = COLOR_THEMES[colorTheme] || COLOR_THEMES.default;
  const themeColors = isDark ? selectedTheme.dark : selectedTheme.light;
  const intensity = colorIntensity / 100;

  if (isDark) {
    // Dark mode: higher intensity = brighter dark theme
    const bgLightness = Math.round(8 + (intensity * 12)); // 8% to 20%
    const cardLightness = Math.round(10 + (intensity * 15)); // 10% to 25%
    const accentLightness = Math.round(22 + (intensity * 18)); // 22% to 40%
    const borderLightness = Math.round(20 + (intensity * 20)); // 20% to 40%
    const primaryLightness = Math.round(45 + (intensity * 15)); // 45% to 60%

    root.style.setProperty('--background', `${themeColors.hue} 10% ${bgLightness}%`);
    root.style.setProperty('--foreground', `${themeColors.hue} 5% 90%`);
    root.style.setProperty('--card', `${themeColors.hue} 10% ${cardLightness}%`);
    root.style.setProperty('--card-foreground', `${themeColors.hue} 5% 90%`);
    root.style.setProperty('--popover', `${themeColors.hue} 10% ${cardLightness}%`);
    root.style.setProperty('--popover-foreground', `${themeColors.hue} 5% 90%`);
    root.style.setProperty('--primary', `${themeColors.hue} ${themeColors.saturation}% ${primaryLightness}%`);
    root.style.setProperty('--primary-foreground', `0 0% 98%`);
    root.style.setProperty('--secondary', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(18 + (intensity * 15))}%`);
    root.style.setProperty('--secondary-foreground', `0 0% 98%`);
    root.style.setProperty('--muted', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(16 + (intensity * 12))}%`);
    root.style.setProperty('--muted-foreground', `${themeColors.hue} 5% 65%`);
    root.style.setProperty('--accent', `${themeColors.hue} ${themeColors.saturation}% ${accentLightness}%`);
    root.style.setProperty('--accent-foreground', `${themeColors.hue} 5% 90%`);
    root.style.setProperty('--border', `${themeColors.hue} ${themeColors.saturation}% ${borderLightness}%`);
    root.style.setProperty('--input', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(18 + (intensity * 12))}%`);

    // Semantic colors - dark mode
    root.style.setProperty('--success', `142 76% 45%`);
    root.style.setProperty('--success-foreground', `0 0% 98%`);
    root.style.setProperty('--success-muted', `142 76% 15%`);
    root.style.setProperty('--success-muted-foreground', `142 76% 70%`);
    root.style.setProperty('--warning', `38 92% 55%`);
    root.style.setProperty('--warning-foreground', `0 0% 10%`);
    root.style.setProperty('--warning-muted', `38 92% 15%`);
    root.style.setProperty('--warning-muted-foreground', `38 92% 75%`);
    root.style.setProperty('--danger', `0 84% 60%`);
    root.style.setProperty('--danger-foreground', `0 0% 98%`);
    root.style.setProperty('--danger-muted', `0 84% 15%`);
    root.style.setProperty('--danger-muted-foreground', `0 84% 75%`);
    root.style.setProperty('--info', `217 91% 60%`);
    root.style.setProperty('--info-foreground', `0 0% 98%`);
    root.style.setProperty('--info-muted', `217 91% 15%`);
    root.style.setProperty('--info-muted-foreground', `217 91% 75%`);
  } else {
    // Light mode: higher intensity = more contrast
    const bgLightness = Math.round(98 - (intensity * 5)); // 98% to 93%
    const cardLightness = Math.round(100 - (intensity * 2)); // 100% to 98%
    const foregroundLightness = Math.round(15 + (intensity * 10)); // 15% to 25%
    const accentLightness = Math.round(88 + (intensity * 8)); // 88% to 96%
    const borderLightness = Math.round(85 + (intensity * 10)); // 85% to 95%
    const primaryLightness = Math.round(50 - (intensity * 15)); // 50% to 35%

    root.style.setProperty('--background', `${themeColors.hue} 10% ${bgLightness}%`);
    root.style.setProperty('--foreground', `${themeColors.hue} 10% ${foregroundLightness}%`);
    root.style.setProperty('--card', `${themeColors.hue} 10% ${cardLightness}%`);
    root.style.setProperty('--card-foreground', `${themeColors.hue} 10% ${foregroundLightness}%`);
    root.style.setProperty('--popover', `${themeColors.hue} 10% ${cardLightness}%`);
    root.style.setProperty('--popover-foreground', `${themeColors.hue} 10% ${foregroundLightness}%`);
    root.style.setProperty('--primary', `${themeColors.hue} ${themeColors.saturation}% ${primaryLightness}%`);
    root.style.setProperty('--primary-foreground', `0 0% 98%`);
    root.style.setProperty('--secondary', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(92 + (intensity * 6))}%`);
    root.style.setProperty('--secondary-foreground', `0 0% 15%`);
    root.style.setProperty('--muted', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(92 + (intensity * 6))}%`);
    root.style.setProperty('--muted-foreground', `${themeColors.hue} 10% 45%`);
    root.style.setProperty('--accent', `${themeColors.hue} ${themeColors.saturation}% ${accentLightness}%`);
    root.style.setProperty('--accent-foreground', `${themeColors.hue} 10% ${foregroundLightness}%`);
    root.style.setProperty('--border', `${themeColors.hue} ${themeColors.saturation}% ${borderLightness}%`);
    root.style.setProperty('--input', `${themeColors.hue} ${themeColors.saturation}% ${Math.round(92 + (intensity * 6))}%`);

    // Semantic colors - light mode
    root.style.setProperty('--success', `142 76% 36%`);
    root.style.setProperty('--success-foreground', `0 0% 98%`);
    root.style.setProperty('--success-muted', `142 76% 95%`);
    root.style.setProperty('--success-muted-foreground', `142 76% 25%`);
    root.style.setProperty('--warning', `38 92% 50%`);
    root.style.setProperty('--warning-foreground', `0 0% 98%`);
    root.style.setProperty('--warning-muted', `38 92% 95%`);
    root.style.setProperty('--warning-muted-foreground', `38 92% 30%`);
    root.style.setProperty('--danger', `0 84% 60%`);
    root.style.setProperty('--danger-foreground', `0 0% 98%`);
    root.style.setProperty('--danger-muted', `0 84% 95%`);
    root.style.setProperty('--danger-muted-foreground', `0 84% 40%`);
    root.style.setProperty('--info', `217 91% 60%`);
    root.style.setProperty('--info-foreground', `0 0% 98%`);
    root.style.setProperty('--info-muted', `217 91% 95%`);
    root.style.setProperty('--info-muted-foreground', `217 91% 35%`);
  }
}

/**
 * Initialize theme on page load
 * This should be called as early as possible to prevent FOUC
 */
export function initTheme() {
  try {
    // Get user preferences from localStorage
    const savedTheme = localStorage.getItem('lightnvr-theme');
    const savedColorIntensity = localStorage.getItem('lightnvr-color-intensity');
    const savedColorTheme = localStorage.getItem('lightnvr-color-theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

    // Determine final theme
    let finalTheme = 'light'; // default
    if (savedTheme) {
      finalTheme = savedTheme;
    } else if (systemPrefersDark) {
      finalTheme = 'dark';
    }

    // Apply theme class immediately
    if (finalTheme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }

    // Apply custom color theme and intensity if saved
    if (savedColorIntensity && savedColorTheme) {
      const colorTheme = savedColorTheme in COLOR_THEMES ? savedColorTheme : 'default';
      const colorIntensity = parseInt(savedColorIntensity) || 50;
      const isDark = finalTheme === 'dark';

      applyThemeColors(isDark, colorTheme, colorIntensity);
    }

    // Set a flag that preferences have been applied
    window.__LIGHTNVR_THEME_APPLIED__ = true;

  } catch (e) {
    // Fallback to system preference if anything fails
    console.warn('Theme initialization failed:', e);
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark');
    }
  }
}

/**
 * Get the inline script for theme initialization
 * This can be embedded in HTML to run before page render
 */
export function getThemeInitScript() {
  return `
(function() {
  try {
    const COLOR_THEMES = ${JSON.stringify(COLOR_THEMES)};
    
    const savedTheme = localStorage.getItem('lightnvr-theme');
    const savedColorIntensity = localStorage.getItem('lightnvr-color-intensity');
    const savedColorTheme = localStorage.getItem('lightnvr-color-theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    let finalTheme = 'light';
    if (savedTheme) {
      finalTheme = savedTheme;
    } else if (systemPrefersDark) {
      finalTheme = 'dark';
    }
    
    if (finalTheme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    
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
      }
    }
    
    window.__LIGHTNVR_THEME_APPLIED__ = true;
  } catch (e) {
    console.warn('Theme script failed:', e);
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark');
    }
  }
})();
`;
}

