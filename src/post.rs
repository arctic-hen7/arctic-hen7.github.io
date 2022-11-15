// WARNING: This must be kept in sync with the CLI version!

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};

/// A post on the site.
#[perseus::make_rx(PostRx)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    }
}
