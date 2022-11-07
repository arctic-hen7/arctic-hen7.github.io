use std::env;
use anyhow::{Result, bail};
use super::Shortform;

/// Lists all the current shortform posts.
pub async fn list_shortforms() -> Result<Vec<Shortform>> {
    let (raw, _sha) = get_shortforms_raw().await?;
    let mut parsed = Vec::new();
    for post in raw.into_iter() {
        let post = serde_json::from_str(&post)?;
        parsed.push(post);
    }

    Ok(parsed)
}

/// Gets the raw list of shortforms, without parsing them. This
/// also returns the hash of the file that contains them.
pub async fn get_shortforms_raw() -> Result<(Vec<String>, String)> {
    // First, we need to know the SHA of the file
    let mut repo_matches = octocrab::instance()
        .repos(
            env::var("GITHUB_USERNAME")?,
            env::var("GITHUB_REPO_NAME")?,
        )
        .get_content()
        .path("posts")
        .send()
        .await?;
    let repo_matches = repo_matches.take_items();
    if let Some(file) = repo_matches.get(0) {
        let sha = &file.sha;
        let curr_content = match &file.content {
            Some(content) => content,
            None => ""
        };
        // Now we have to decode that content from base64
        let curr_content = base64::decode_config(curr_content, base64::MIME)?;
        let curr_content = String::from_utf8(curr_content)?;
        let content = curr_content
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok((content, sha.to_string()))
    } else {
        // TODO Automatically create the file?
        bail!("No `posts` file in the root of the GitHub repo (you must manually create this file)")
    }
}
