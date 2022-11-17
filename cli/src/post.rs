// WARNING: This must be kept in sync with the site version!

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};

/// A post on the site.
#[derive(Serialize, Deserialize)]
pub struct Post {
    /// The title.
    pub title: String,
    /// The author.
    pub author: PostAuthor,
    /// A short description of the post.
    pub description: String,
    /// The ID, which is already URL encoded (composed of the title and the Org Roam ID)
    pub id: String,
    /// The HTML contents, with MathJax etc all handled. These should be inserted into a `<div>`.
    pub contents: String,
    /// The tags this post has.
    pub tags: Vec<String>,
    /// The series that this post is part of, if it's part of any, and its index therein. (A post can only be a part of one series.)
    pub series: Option<(String, usize)>,
    /// The HTML table of contents, to be interpolated as a sticky sidebar.
    pub toc: String,
}

/// A post's full representation on-disk (including information that won't be sent to the browser).
#[derive(Serialize, Deserialize)]
pub struct FullPost {
    /// Everything else.
    pub post: Post,
    /// The path to the source of the file.
    pub source: PathBuf,
    /// The last modification time of the file, so that we can check if it has been modified since.
    /// This is hardly foolproof, but it's pretty good.
    pub mtime: SystemTime,
}

/// The author of a post.
#[derive(Serialize, Deserialize)]
pub enum PostAuthor {
    /// Me (the default).
    Me,
    /// Another guest.
    Guest {
        /// The guest's display name.
        name: String,
        /// Their profile picture URL, if they have one.
        profile_pic_url: Option<String>,
        /// Their home page URL (this could be a Twitter account, GitHub, a personal site, anything).
        home_url: String,
    },
}
impl PostAuthor {
    /// Parses an author as specified in a file.
    ///
    /// The format for this is `<name> (<gh-username>|<home-url>\<profile-pic-url>)`.
    pub fn from_metadata(prop: &str) -> Result<Self> {
        let mut parts = prop.split('(').collect::<Vec<_>>();
        if parts.len() == 1 {
            // There is just an author's name
            if parts[0] == "arctic-hen7" {
                Ok(PostAuthor::Me)
            } else {
                bail!("Guest authors must have their personal details specified in brackets: `<gh-username>|<home-url>\\<profile-pic-url>`")
            }
        } else {
            let bracketed = parts.remove(parts.len() - 1).strip_suffix(")").unwrap();
            let name = parts.join("(");
            let name = name.trim();
            if bracketed.starts_with("http") {
                // The bracketed part is a home URL and profile picture
                let parts = bracketed.split('\\').collect::<Vec<_>>();
                let home_url = parts[0].to_string();
                let profile_pic_url = parts.get(1).map(|s| s.to_string());
                Ok(PostAuthor::Guest {
                    name: name.to_string(),
                    home_url,
                    profile_pic_url,
                })
            } else {
                // The bracketed part is a GH username (from which we can derive a profile picture)
                // I love GH for just supporting this!
                let profile_pic_url = format!("https://github.com/{}.png", bracketed);
                Ok(PostAuthor::Guest {
                    name: name.to_string(),
                    home_url: format!("https://github.com/{}", bracketed),
                    profile_pic_url: Some(profile_pic_url),
                })
            }
        }
    }
}
