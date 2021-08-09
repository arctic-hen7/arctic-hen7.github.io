const defaultTheme = require("tailwindcss/defaultTheme");
const colors = require("tailwindcss/colors");

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
        colors: {
            transparent: "transparent",
            current: "currentColor",
            black: colors.black,
            white: colors.white,
            gray: colors.coolGray,
            red: colors.red,
            yellow: colors.amber,
            green: colors.emerald,
            blue: colors.blue,
            indigo: colors.indigo,
            purple: colors.violet,
            pink: colors.pink,
            cyan: colors.cyan,
            fuchsia: colors.fuchsia,
            sky: colors.sky,
            teal: colors.teal,
        },
    },
    variants: {},
    plugins: [require("tailwind-hamburgers")],
};
