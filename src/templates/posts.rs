use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use crate::post::*;
use crate::container::{Container, CurrentRoute};
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
        Container(offset_top = true, route = CurrentRoute::BlogPost) {
            div(class = "flex justify-center") {
                // This will include the title!
                div(class = "styled-prose max-w-5xl", dangerously_set_inner_html = &post.contents.get())
                // The table of contents will be sticky-aligned to the side of the page
                nav(
                    // Yes, this is 63vh high
                    // I use a larger width here than in the child `<div>` to size things nicely while avoiding a horizontal scrollbar
                    // Bottom padding is to offset the upper padding on the heading
                    class = "styled-prose sticky self-start top-14 xs:top-16 sm:top-20 lg:top-24 max-w-md max-h-[63vh] overflow-y-auto border-l border-neutral-600 pl-4 ml-4 py-3"
                ) {
                    div(
                        class = "max-w-sm",
                        dangerously_set_inner_html = &post.toc.get()
                    )
                }

            }
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

    let id = path.strip_prefix("posts/").unwrap();
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
    Template::new("posts")
        .template(post_page)
        .head(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
}
