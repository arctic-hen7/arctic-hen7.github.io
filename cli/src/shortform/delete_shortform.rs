use anyhow::Result;
use std::env;
use uuid::Uuid;
use super::{list_shortform::list_shortforms};

/// Deletes the given post from the public repository. Note that this
/// deletion is hardly infallible, since whatever the original post
/// was will be in the commit history permanently, and, since the repo
/// is public, it will likely also be on archive.org etc.
///
/// In short, don't make posts you'll regret! (Which really should be
/// rule #1 of the internet.)
///
/// This takes the UUID of the post.
pub async fn delete_shortform(id: &str) -> Result<()> {
    let id = Uuid::parse_str(id).unwrap();

    let (posts, sha) = list_shortforms().await?;
    // Find the post we want
    let posts = posts
        .into_iter()
        .filter(|post| post.id != id)
        .map(|post| serde_json::to_string(&post).unwrap())
        .collect::<Vec<String>>();
    // This will preserve order
    let posts = posts.join("\n");
    // Update the `posts` file at the root
    octocrab::instance()
        .repos(env::var("GITHUB_USERNAME")?, env::var("GITHUB_REPO_NAME")?)
        .update_file("posts", "Removed post from CLI", &posts, sha)
        // Committer/author will be set to the authenticated user by default
        .send()
        .await?;
    Ok(())
}
