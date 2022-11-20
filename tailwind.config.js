const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

module.exports = {
    content: [
        "./src/**/*.{rs,html,css}",
        "./index.html",
        "./static/styles/**/*.css",
    ],
    theme: {
        fontFamily: {
            heading: ["Comfortaa", "sans"],
            ...defaultTheme.fontFamily,
        },
        screens: {
            "2xs": "370px",
            xs: "475px",
            ...defaultTheme.screens,
            "3xl": "1792px",
        },
        extend: {
            colors: {},
        },
    },
    variants: {},
    plugins: [require("tailwind-hamburgers")],
};
