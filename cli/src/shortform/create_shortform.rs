use std::env;
use anyhow::Result;
use chrono::Utc;
use regex::Regex;
use uuid::Uuid;
use pulldown_cmark::{Parser, Options, html};
use crate::post::PostAuthor;
use super::{Shortform, list_shortform::get_shortforms_raw};

/// Creates a new shortform post and publishes it to GitHub, from which it can be imported
/// to my personal site automatically. This takes a post author as they would be specified
/// in a blog post's metadata.
///
/// This takes the HTML content of the post.
pub async fn create_shortform(md_content: &str, author: &str) -> Result<()> {
    // Parse GH @ links
    let md_content = Regex::new(r#"\[\[@(.*?)\]\]"#)
        .unwrap()
        .replace(&md_content, "[@$1](https://github.com/$1)");

    let md_opts = Options::all();
    let parser = Parser::new_ext(&md_content, md_opts);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    let shortform = Shortform {
        id: Uuid::new_v4(),
        content: html_content.to_string(),
        time: Utc::now(),
        author: PostAuthor::from_metadata(author)?,
    };

    // Get the current raw posts and the hash of the file they're in (so we can reference it for editing)
    let (mut raw_posts, sha) = get_shortforms_raw().await?;
    // Construct the new content by appending
    let new_post = serde_json::to_string(&shortform).unwrap();
    raw_posts.push(new_post);
    let new_content = raw_posts.join("\n");
    // Update the `posts` file at the root
    octocrab::instance()
        .repos(
            env::var("GITHUB_USERNAME")?,
            env::var("GITHUB_REPO_NAME")?,
        )
        .update_file(
            "posts",
            "Added new post from CLI",
            &new_content,
            sha,
        )
    // Committer/author will be set to the authenticated user by default
        .send()
        .await?;
    Ok(())
}
