/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    //npm install tailwind-scrollbar-hide
    require('tailwind-scrollbar-hide'),
  ],
};
