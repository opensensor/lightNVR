/**
 * Theme initialization script for LightNVR
 * This script runs immediately to prevent FOUC (Flash of Unstyled Content)
 * Based on the accounting app's theme system
 */

// Color theme definitions
export const COLOR_THEMES = {
  default: {
    name: 'Default',
    nameKey: 'theme.default',
    light: { hue: 240, saturation: 5.9 },
    dark: { hue: 240, saturation: 3.7 },
    icon: '🎨'
  },
  blue: {
    name: 'Ocean Blue',
    nameKey: 'theme.blue',
    light: { hue: 217, saturation: 32 },
    dark: { hue: 217, saturation: 25 },
    icon: '🌊'
  },
  emerald: {
    name: 'Forest Green',
    nameKey: 'theme.emerald',
    light: { hue: 160, saturation: 30 },
    dark: { hue: 160, saturation: 25 },
    icon: '🌲'
  },
  purple: {
    name: 'Royal Purple',
    nameKey: 'theme.purple',
    light: { hue: 265, saturation: 28 },
    dark: { hue: 265, saturation: 20 },
    icon: '👑'
  },
  rose: {
    name: 'Sunset Rose',
    nameKey: 'theme.rose',
    light: { hue: 350, saturation: 25 },
    dark: { hue: 350, saturation: 18 },
    icon: '🌹'
  },
  amber: {
    name: 'Golden Amber',
    nameKey: 'theme.amber',
    light: { hue: 45, saturation: 28 },
    dark: { hue: 45, saturation: 20 },
    icon: '⚡'
  },
  slate: {
    name: 'Cool Slate',
    nameKey: 'theme.slate',
    light: { hue: 215, saturation: 8 },
    dark: { hue: 215, saturation: 6 },
    icon: '🗿'
  },
  retro80s: {
    name: 'Retro 80s',
    nameKey: 'theme.retro80s',
    light: { hue: 18, saturation: 58 },
    dark: { hue: 18, saturation: 48 },
    icon: '📼'
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
