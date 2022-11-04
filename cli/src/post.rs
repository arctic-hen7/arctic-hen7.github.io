// WARNING: This must be kept in sync with the site version!

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};

/// A post on the site.
#[derive(Serialize, Deserialize)]
pub struct Post {
    /// The title.
    pub title: String,
    /// The author.
    pub author: PostAuthor,
    /// The ID, which is already URL encoded (composed of the title and the Org Roam ID)
    pub id: String,
    /// The HTML contents, with MathJax etc all handled. These should be inserted into a `<div>`.
    pub contents: String,
    /// The tags this post has.
    pub tags: Vec<String>,
    /// The series that this post is part of, if it's part of any, and its index therein. (A post can only be a part of one series.)
    pub series: Option<(String, usize)>,
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
    /// Another guest, whose name will be shown,
    Guest(String)
}
