module.exports = (cfg) => {
    // This removes all metadata tags
    const filterTagsList = (tags) =>
        (tags || []).filter((tag) => !tag.startsWith("_") && tag !== "all");
    cfg.addFilter("filterTagsList", filterTagsList);

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

    cfg.addCollection("_tags", (coll) => {
        let tags = new Set();
        coll.getAll().map((item) =>
            // Filter out meta tags (they all start with '_')
            (item.data.tags || []).forEach((tag) => {
                tags.add(tag);
            })
        );
        return filterTagsList([...tags]);
    });
    cfg.addCollection("_tags_categoryDev", (coll) => {
        let tags = new Set();
        coll.getFilteredByTag("_categoryDev").map((item) =>
            // Filter out meta tags (they all start with '_')
            (item.data.tags || []).forEach((tag) => {
                tags.add(tag);
            })
        );
        return filterTagsList([...tags]);
    });
    cfg.addCollection("_tags_categoryProd", (coll) => {
        let tags = new Set();
        coll.getFilteredByTag("_categoryProd").map((item) =>
            // Filter out meta tags (they all start with '_')
            (item.data.tags || []).forEach((tag) => {
                tags.add(tag);
            })
        );
        return filterTagsList([...tags]);
    });
    cfg.addCollection("_tags_categorySophos", (coll) => {
        let tags = new Set();
        coll.getFilteredByTag("_categorySophos").map((item) =>
            // Filter out meta tags (they all start with '_')
            (item.data.tags || []).forEach((tag) => {
                tags.add(tag);
            })
        );
        return filterTagsList([...tags]);
    });

    return {
        passthroughFileCopy: true,
        dir: {
            input: "src",
            output: "dist",
        },
    };
};
