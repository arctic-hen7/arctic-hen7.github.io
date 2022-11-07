use std::env;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use super::{Shortform, list_shortform::get_shortforms_raw};

/// Creates a new shortform post and publishes it to GitHub, from which it can be imported
/// to my personal site automatically.
///
/// This takes the HTML content of the post.
pub async fn create_shortform(html_content: &str) -> Result<()> {
    let shortform = Shortform {
        id: Uuid::new_v4(),
        content: html_content.to_string(),
        time: Utc::now()
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
