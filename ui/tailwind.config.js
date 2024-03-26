/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.tsx",
    "./index.html",
  ],
  theme: {
    spacing: {
      '0': '0',
      '1': '1px',
      '2': '2px',
      '3': '3px',
      '4': '4px',
      '5': '5px',
      '8': '8px',
      '10': '10px',
      '20': '20px',
      '76': '76px',
    },
    extend: {
      spacing: {
        '76': '76px',
      }
    },
  },
  plugins: [],
}

