/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{svelte,js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        pico: {
          dark: '#141416',
          panel: '#1e1e24',
          border: '#2e2e38',
          accent: '#3b82f6',
          hover: '#2a2b36',
        },
      },
    },
  },
  plugins: [],
}
