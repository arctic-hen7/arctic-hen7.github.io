module.exports = (cfg) => {
    // We build Tailwind externally, so Eleventy just needs to look on
    cfg.addWatchTarget("./_tmp/style.css");
    cfg.addPassthroughCopy({ "./_tmp/tailwind.css": "./tailwind.css" });

    return {
        passthroughFileCopy: true,
        dir: {
            input: "src",
            output: "dist",
        },
    };
};
