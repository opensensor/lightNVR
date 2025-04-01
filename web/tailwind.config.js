/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.snowpack.html',
    './index.html',
    './js/**/*.js',
    './js/**/*.jsx',
    './js/**/*.ts',
    './js/**/*.tsx',
    './css/**/*.css',
    './*.html',
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