module.exports = (cfg) => {
    // We build Tailwind externally, so Eleventy just needs to look on
    cfg.addWatchTarget("./_tmp/style.css");
    cfg.addPassthroughCopy({ "./src/styles/style.css": "./style.css" });
    cfg.addPassthroughCopy({ "./_tmp/tailwind.css": "./tailwind.css" });

    // Add custom collections for filtering
    cfg.addCollection("_pinned_categoryDev", (coll) =>
        coll.getFilteredByTags("_pinned", "_categoryDev")
    );
    cfg.addCollection("_pinned_categoryProd", (coll) =>
        coll.getFilteredByTags("_pinned", "_categoryProd")
    );
    cfg.addCollection("_pinned_categorySophos", (coll) =>
        coll.getFilteredByTags("_pinned", "_categorySophos")
    );

    return {
        passthroughFileCopy: true,
        dir: {
            input: "src",
            output: "dist",
        },
    };
};
