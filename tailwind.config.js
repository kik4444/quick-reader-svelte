const withMT = require("@material-tailwind/html/utils/withMT");

/** @type {import('tailwindcss').Config} */
module.exports = withMT({
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
    transitionDuration: {
      DEFAULT: "250ms"
    }
  },
  plugins: [],
  darkMode: "class"
});
