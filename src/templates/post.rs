use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use crate::post::*;
use crate::container::Container;
use crate::BLOG_DIR;

#[perseus::template_rx]
pub fn post_page<'rx, G: Html>(cx: Scope<'rx>, post: PostRx<'rx>) -> View<G> {
    let tags = post.tags.get();
    let tags_view = View::new_fragment(tags.iter().cloned().map(|tag| {
        let tag_link = format!("tag/{}", &tag);
        view! { cx,
            li { a(href = tag_link) { (tag) } }
        }
    }).collect::<Vec<_>>());

    view! { cx,
        Container(offset_top = true) {
            // This will include the title!
            div(class = "styled-prose", dangerously_set_inner_html = &post.contents.get())
            ul {
                (tags_view)
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope, post: Post) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | The Arctic Circle", post.title)) }
        script(type="text/x-mathjax-config") {
            r#"MathJax.Hub.Config({
                displayAlign: "center",
                displayIndent: "0em",
                "HTML-CSS": { scale: 100,
                              linebreaks: { automatic: "false" },
                              webFont: "TeX"
                },
                SVG: {scale: 100,
                      linebreaks: { automatic: "false" },
                      font: "TeX"},
                NativeMML: {scale: 100},
                TeX: { equationNumbers: {autoNumber: "AMS"},
                       MultLineWidth: "85%",
                       TagSide: "right",
                       TagIndent: ".8em"
                }
            });"#
        }
        script(src = "https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.0/MathJax.js?config=TeX-AMS_HTML")
    }
}

#[perseus::build_state]
fn get_build_state(path: String, _: String) -> RenderFnResultWithCause<Post> {
    use std::path::Path;
    use std::fs;

    let id = path.strip_prefix("post/").unwrap();
    let filename = format!("{id}.json");
    let file_path = Path::new(BLOG_DIR).join(&filename);

    let contents = fs::read_to_string(file_path)?;
    let post: FullPost = serde_json::from_str(&contents)?;

    Ok(post.post)
}

#[perseus::build_paths]
fn get_build_paths() -> RenderFnResult<Vec<String>> {
    use std::fs;
    use anyhow::Context;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut paths = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path()).context("Failed to read file in blog index")?;
        let post: FullPost = serde_json::from_str(&contents).context("Failed to deserialize file in blog index")?;
        // TODO Modify the post contents so we can display loaders instead of equations, until MathJax is ready

        paths.push(post.post.id)
    };

    Ok(paths)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("post")
        .template(post_page)
        .head(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
}
