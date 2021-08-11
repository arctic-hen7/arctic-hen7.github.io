const plugins = [
    require("postcss-import"),
    require("tailwindcss/nesting"),
    require("tailwindcss"),
];
if (process.env.NODE_ENV === "production") {
    plugins.push(
        require("autoprefixer"),
        require("cssnano")({
            preset: "default",
        }),
        require("@fullhuman/postcss-purgecss")({
            content: [
                "src/**/*.njk",
                "src/**/*.md",
                "src/**/*.html",
                "src/**/*.css",
            ],
            defaultExtractor: (content) =>
                content.match(/[\w-/:]+(?<!:)/g) || [],
        })
    );
}

module.exports = {
    plugins,
};
