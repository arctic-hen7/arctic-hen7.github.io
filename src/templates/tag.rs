use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use crate::post::*;
use crate::container::{Container, CurrentRoute};
use crate::BLOG_DIR;

#[perseus::template_rx]
pub fn tag_page<'rx, G: Html>(cx: Scope<'rx>, tag: TagRx<'rx>) -> View<G> {
    let posts = tag.posts_with_tag.get();
    let posts_view = View::new_fragment(posts.iter().cloned().map(|post| {
        let post_link = format!("post/{}", post.id);
        view! { cx,
            li { a(href = post_link) {
                span { (post.title) }
            } }
        }
    }).collect::<Vec<_>>());

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::Tag) {
            p { (tag.name.get()) }
            ul {
                (posts_view)
            }
        }
    }
}

#[perseus::make_rx(TagRx)]
struct Tag {
    name: String,
    posts_with_tag: Vec<Post>,
}

#[perseus::head]
pub fn head(cx: Scope, tag: Tag) -> View<SsrNode> {
    view! { cx,
        title { (format!("Posts tagged '{}' | The Arctic Circle", tag.name)) }
    }
}

#[perseus::build_state]
fn get_build_state(path: String, _: String) -> RenderFnResultWithCause<Tag> {
    use std::fs;

    let tag = path.strip_prefix("tag/").unwrap();

    // Get all the blog posts, and figure out which ones have this tag
    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut posts_with_tag = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path())?;
        let mut post: FullPost = serde_json::from_str(&contents)?;
        // Truncate the contents to a description
        post.post.contents.truncate(50);

        if post.post.tags.iter().any(|t| t == tag) {
            posts_with_tag.push(post.post);
        }
    };

    Ok(Tag { posts_with_tag, name: tag.to_string() })
}

#[perseus::build_paths]
fn get_build_paths() -> RenderFnResult<Vec<String>> {
    use std::fs;
    use anyhow::Context;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut tags = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path()).context("Failed to read file in blog index")?;
        let post: FullPost = serde_json::from_str(&contents).context("Failed to deserialize file in blog index")?;

        // Add all the tags of this post to the list of paths, making sure we don't have any duplications
        for tag in post.post.tags.into_iter() {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
    };

    Ok(tags)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("tag")
        .template(tag_page)
        .head(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
}
