// WARNING: This must be kept in sync with the CLI version!

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::SystemTime};

static ANONYMOUS_USER_PIC_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM7.35 18.5C8.66 17.56 10.26 17 12 17s3.34.56 4.65 1.5c-1.31.94-2.91 1.5-4.65 1.5s-3.34-.56-4.65-1.5zm10.79-1.38C16.45 15.8 14.32 15 12 15s-4.45.8-6.14 2.12C4.7 15.73 4 13.95 4 12c0-4.42 3.58-8 8-8s8 3.58 8 8c0 1.95-.7 3.73-1.86 5.12z"/><path d="M12 6c-1.93 0-3.5 1.57-3.5 3.5S10.07 13 12 13s3.5-1.57 3.5-3.5S13.93 6 12 6zm0 5c-.83 0-1.5-.67-1.5-1.5S11.17 8 12 8s1.5.67 1.5 1.5S12.83 11 12 11z"/></g></g></svg>"#;

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
    },
}

/// The information necessary to display a post card.
#[derive(Clone, Serialize, Deserialize)]
pub struct SlimPost {
    pub id: String,
    pub title: String,
    pub author: PostAuthor,
    pub description: String,
    pub series: Option<(String, usize)>,
}

use sycamore::prelude::*;

impl PostAuthor {
    /// Gets the tuple of the author's name, home URL, and profile picture view for display.
    pub fn parse<G: Html>(&self, cx: Scope) -> (String, String, View<G>) {
        let author_pic = match &self {
            PostAuthor::Me => view! { cx,
                img(class = "rounded-full h-8 w-8", src = "https://github.com/arctic-hen7.png", alt = "My profile picture")
            },
            PostAuthor::Guest {
                profile_pic_url, ..
            } => match profile_pic_url {
                Some(url) => {
                    let url = url.to_string();
                    view! { cx,
                        img(class = "rounded-full h-8 w-8", src = url, alt = "Guest profile picture") {}
                    }
                }
                None => view! { cx,
                    span(class = "fill-white", dangerously_set_inner_html = ANONYMOUS_USER_PIC_SVG) {}
                },
            },
        };
        let author_name = match &self {
            PostAuthor::Me => "arctic-hen7",
            PostAuthor::Guest { name, .. } => name,
        }
        .to_string();
        let author_home_url = match &self {
            PostAuthor::Me => "", // The root of this very site
            PostAuthor::Guest { home_url, .. } => home_url,
        }
        .to_string();

        (author_name, author_home_url, author_pic)
    }
}
