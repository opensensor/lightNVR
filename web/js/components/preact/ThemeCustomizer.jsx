/**
 * LightNVR Theme Customizer Component
 * Allows users to customize color themes and intensity
 * Based on the accounting app's theme customization system
 */

import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';
import { COLOR_THEMES, applyThemeColors } from '../../utils/theme-init.js';

/**
 * ThemeCustomizer component
 * @returns {JSX.Element} ThemeCustomizer component
 */
export function ThemeCustomizer() {
  const [mounted, setMounted] = useState(false);
  const [isDark, setIsDark] = useState(false);
  const [colorIntensity, setColorIntensity] = useState(50);
  const [colorTheme, setColorTheme] = useState('default');

  // Load saved preferences from localStorage
  useEffect(() => {
    setMounted(true);
    
    const savedTheme = localStorage.getItem('lightnvr-theme');
    const savedIntensity = localStorage.getItem('lightnvr-color-intensity');
    const savedColorTheme = localStorage.getItem('lightnvr-color-theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

    // Determine theme
    let finalTheme = 'light';
    if (savedTheme) {
      finalTheme = savedTheme;
    } else if (systemPrefersDark) {
      finalTheme = 'dark';
    }
    setIsDark(finalTheme === 'dark');

    // Load intensity
    if (savedIntensity) {
      setColorIntensity(parseInt(savedIntensity));
    }

    // Load color theme
    if (savedColorTheme && COLOR_THEMES[savedColorTheme]) {
      setColorTheme(savedColorTheme);
    }
  }, []);

  // Apply theme changes
  useEffect(() => {
    if (!mounted) return;

    // Apply dark/light mode class
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }

    // Apply color theme and intensity
    applyThemeColors(isDark, colorTheme, colorIntensity);

    // Save to localStorage
    localStorage.setItem('lightnvr-theme', isDark ? 'dark' : 'light');
    localStorage.setItem('lightnvr-color-intensity', colorIntensity.toString());
    localStorage.setItem('lightnvr-color-theme', colorTheme);
  }, [mounted, isDark, colorIntensity, colorTheme]);

  const toggleDarkMode = () => {
    setIsDark(!isDark);
  };

  const handleIntensityChange = (e) => {
    setColorIntensity(parseInt(e.target.value));
  };

  const handleThemeChange = (themeId) => {
    setColorTheme(themeId);
  };

  const handlePresetIntensity = (value) => {
    setColorIntensity(value);
  };

  if (!mounted) {
    return (
      <div class="animate-pulse">
        <div class="h-8 bg-gray-200 dark:bg-gray-700 rounded mb-4"></div>
        <div class="h-32 bg-gray-200 dark:bg-gray-700 rounded"></div>
      </div>
    );
  }

  return (
    <div class="space-y-6">
      {/* Dark Mode Toggle */}
      <div class="flex items-center justify-between p-4 bg-card rounded-lg border border-border">
        <div class="flex items-center gap-3">
          <div class="text-2xl">
            {isDark ? '🌙' : '☀️'}
          </div>
          <div>
            <h3 class="font-semibold text-card-foreground">Dark Mode</h3>
            <p class="text-sm text-muted-foreground">
              {isDark ? 'Switch to light mode' : 'Switch to dark mode'}
            </p>
          </div>
        </div>
        <button
          onClick={toggleDarkMode}
          class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
          style={{
            backgroundColor: isDark ? 'hsl(var(--primary))' : 'hsl(var(--muted))'
          }}
        >
          <span
            class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
            style={{
              transform: isDark ? 'translateX(1.5rem)' : 'translateX(0.25rem)'
            }}
          />
        </button>
      </div>

      {/* Color Theme Selection */}
      <div class="p-4 bg-card rounded-lg border border-border">
        <div class="flex items-center gap-2 mb-4">
          <span class="text-xl">🎨</span>
          <h3 class="font-semibold text-card-foreground">Color Theme</h3>
        </div>
        
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
          {Object.entries(COLOR_THEMES).map(([themeId, themeConfig]) => (
            <button
              key={themeId}
              onClick={() => handleThemeChange(themeId)}
              class={`flex flex-col items-center gap-2 p-3 rounded-lg border-2 transition-all ${
                colorTheme === themeId
                  ? 'border-primary bg-primary/10 shadow-md'
                  : 'border-border bg-card hover:border-primary/50 hover:bg-accent'
              }`}
            >
              <span class="text-2xl">{themeConfig.icon}</span>
              <span class="text-xs font-medium text-center text-card-foreground">
                {themeConfig.name}
              </span>
            </button>
          ))}
        </div>
      </div>

      {/* Intensity Control */}
      <div class="p-4 bg-card rounded-lg border border-border">
        <div class="flex items-center gap-2 mb-4">
          <span class="text-xl">🎚️</span>
          <h3 class="font-semibold text-card-foreground">Color Intensity</h3>
        </div>

        <div class="space-y-4">
          <div class="flex items-center justify-between text-sm text-muted-foreground">
            <span>{isDark ? 'Darker' : 'Lighter'}</span>
            <span class="font-semibold text-card-foreground">{colorIntensity}%</span>
            <span>{isDark ? 'Brighter' : 'Higher Contrast'}</span>
          </div>

          <input
            type="range"
            min="0"
            max="100"
            step="5"
            value={colorIntensity}
            onChange={handleIntensityChange}
            class="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer accent-primary"
          />

          <div class="flex gap-2">
            <button
              onClick={() => handlePresetIntensity(25)}
              class={`flex-1 px-3 py-2 text-sm rounded-md transition-colors ${
                colorIntensity === 25
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
              }`}
            >
              Subtle
            </button>
            <button
              onClick={() => handlePresetIntensity(50)}
              class={`flex-1 px-3 py-2 text-sm rounded-md transition-colors ${
                colorIntensity === 50
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
              }`}
            >
              Balanced
            </button>
            <button
              onClick={() => handlePresetIntensity(75)}
              class={`flex-1 px-3 py-2 text-sm rounded-md transition-colors ${
                colorIntensity === 75
                  ? 'bg-primary text-primary-foreground'
                  : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
              }`}
            >
              Bold
            </button>
          </div>
        </div>
      </div>

      {/* Current Theme Info */}
      <div class="p-4 bg-accent/50 rounded-lg border border-border">
        <div class="flex items-center justify-between mb-2">
          <span class="text-sm font-medium text-accent-foreground">
            {COLOR_THEMES[colorTheme].icon} {COLOR_THEMES[colorTheme].name}
          </span>
          <span class="text-sm text-muted-foreground">
            {isDark ? 'Dark' : 'Light'} Mode
          </span>
        </div>
        <p class="text-xs text-muted-foreground">
          💡 Tip: Themes work in both light and dark modes with automatic color adaptation
        </p>
      </div>

      {/* Preview Section */}
      <div class="p-4 bg-card rounded-lg border border-border">
        <h3 class="font-semibold text-card-foreground mb-3">Preview</h3>
        <div class="space-y-2">
          <div class="flex gap-2">
            <div class="flex-1 h-12 rounded bg-primary flex items-center justify-center text-primary-foreground text-xs font-medium">
              Primary
            </div>
            <div class="flex-1 h-12 rounded bg-secondary flex items-center justify-center text-secondary-foreground text-xs font-medium">
              Secondary
            </div>
          </div>
          <div class="flex gap-2">
            <div class="flex-1 h-12 rounded bg-accent flex items-center justify-center text-accent-foreground text-xs font-medium">
              Accent
            </div>
            <div class="flex-1 h-12 rounded bg-muted flex items-center justify-center text-muted-foreground text-xs font-medium">
              Muted
            </div>
          </div>
          <div class="p-3 rounded border border-border bg-background">
            <p class="text-sm text-foreground">
              This is how text looks on the background color
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

