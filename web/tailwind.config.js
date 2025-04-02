/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './*.html',
    './js/**/*.js',
    './js/**/*.jsx',
    './js/**/*.ts',
    './js/**/*.tsx',
    './css/**/*.css',
  ],
  theme: {
    extend: {
      spacing: {
        '4.5': '1.125rem', // This adds the px-4.5 class
      },
      minWidth: {
        '8': '2rem', // This adds the min-w-8 class
      },
    },
  },
  plugins: [],
}
