// The number of posts to display as most recent
const NUM_LATEST_POSTS = 3;

module.exports = (cfg) => {
    // This removes all metadata tags
    const filterTagsList = (tags) =>
        (tags || []).filter((tag) => !tag.startsWith("_") && tag !== "all");
    cfg.addFilter("filterTagsList", filterTagsList);

    // We build CSS externally, so Eleventy just needs to look on
    cfg.addWatchTarget("./_tmp/");
    cfg.addPassthroughCopy({ "./_tmp/style.css": "./style.css" });

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
    cfg.addCollection("_latest", (coll) => {
        let arr = coll.getFilteredByTag("_blog");
        return arr.slice(arr.length - NUM_LATEST_POSTS, arr.length);
    });
    cfg.addCollection("_latest_categoryDev", (coll) => {
        let arr = coll.getFilteredByTag("_categoryDev");
        return arr.slice(arr.length - NUM_LATEST_POSTS, arr.length);
    });
    cfg.addCollection("_latest_categoryProd", (coll) => {
        let arr = coll.getFilteredByTag("_categoryProd");
        return arr.slice(arr.length - NUM_LATEST_POSTS, arr.length);
    });
    cfg.addCollection("_latest_categorySophos", (coll) => {
        let arr = coll.getFilteredByTag("_categorySophos");
        return arr.slice(arr.length - NUM_LATEST_POSTS, arr.length);
    });

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

    // Add plugins
    cfg.addPlugin(require("@11ty/eleventy-plugin-rss"));
    cfg.addPlugin(require("@11ty/eleventy-plugin-syntaxhighlight"));

    return {
        passthroughFileCopy: true,
        markdownTemplateEngine: "njk",
        dir: {
            input: "src",
            output: "dist",
        },
    };
};
