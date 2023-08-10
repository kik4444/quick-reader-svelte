const withMT = require("@material-tailwind/html/utils/withMT");

/** @type {import('tailwindcss').Config} */
module.exports = withMT({
  content: ["*.html", "./src/**/*.rs", "./leptos-material-tailwind/src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
});
