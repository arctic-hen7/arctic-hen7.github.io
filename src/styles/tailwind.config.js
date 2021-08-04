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
    theme: {},
    variants: {},
    plugins: [],
};
