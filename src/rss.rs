#[cfg(engine)]
use crate::{post::FullPost, BLOG_DIR};
#[cfg(engine)]
use perseus::plugins::PluginAction;
use perseus::plugins::{empty_control_actions_registrar, Plugin, PluginEnv};

/// Gets an instance of the RSS plugin, which takes no data.
///
/// Note: you should add `.feed.*.rss` to your `.gitignore`, as it will be auto-generated
/// on every build from the render context.
pub fn get_rss_plugin() -> Plugin<()> {
    Plugin::new(
        "rss-plugin",
        |mut actions| {
            #[cfg(engine)]
            actions
                .settings_actions
                .add_static_aliases
                .register_plugin("rss-plugin", |_, _| {
                    use std::fs;

                    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
                    // None of these errors should materialise, since we've actually done all this in creating the post pages
                    let mut rss_items = Vec::new();
                    for entry in fs::read_dir(BLOG_DIR).expect("Couldn't read from blog directory") {
                        let entry = entry.unwrap();

                        let contents = fs::read_to_string(entry.path()).expect("Failed to read file in blog index");
                        let post: FullPost = serde_json::from_str(&contents).expect("Failed to deserialize file in blog index");

                        let rss_item = format!(
                            r#"
<item>
    <title>{title}</title>
    <link>{root_path}/post/{post_id}</link>
    <description>{description}</description>
    <language>en-us</language>
</item>
"#,
                            title = post.post.title,
                            root_path = perseus::utils::get_path_prefix_server(),
                            post_id = post.post.id,
                            description = post.post.description,
                        );
                        let rss_item = rss_item.trim();

                        rss_items.push(rss_item.to_string());
                    };
                    let rss_feed = format!(
                        r#"
<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">

<channel>
    <title>The Arctic Circle</title>
    <link>{blog_link}</link>
    <description>Blog posts and papers about programming, economics, philosophy, and just about everything else.</description>
    {blog_items}
</channel>
<channel>
    <title>The Ice Floes</title>
    <link>{shortform_link}</link>
    <description>Short posts covering all topics, otherwise known as the social media hermit's Twitter.</description>
    {shortform_items}
</channel>

</rss>
"#,
                        // This will get the root of the site, no matter where we put it!
                        blog_link = perseus::utils::get_path_prefix_server() + "/posts",
                        shortform_link = perseus::utils::get_path_prefix_server() + "/shortform",
                        blog_items = rss_items.join("\n"),
                        shortform_items = "", // TODO Get shortforms from GH
                    );

                    fs::write("dist/.feed.xml", rss_feed.trim()).expect("Couldn't write RSS feed");

                    let mut map = std::collections::HashMap::new();
                    map.insert("/feed.xml".to_string(), "dist/.feed.xml".to_string());
                    Ok(map)
                });
            actions
        },
        empty_control_actions_registrar,
        // We're just generating a nice static alias
        PluginEnv::Server,
    )
}
