const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
    purge: {
        mode: "all",
        // This runs before we've built any content, so we have to depend on the raw stuff
        content: [
            "src/**/*.html",
            "src/**/*.css",
            "src/**/*.njk",
            "src/**/*.md",
        ],
    },
    darkMode: process.env.NODE_ENV == "production" ? "media" : "class",
    theme: {
        fontFamily: {
            sans: ["Comfortaa", "sans-serif"],
        },
        screens: {
            xs: "475px",
            ...defaultTheme.screens,
        },
    },
    variants: {},
    plugins: [],
};
