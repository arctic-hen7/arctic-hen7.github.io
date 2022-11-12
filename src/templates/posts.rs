use perseus::prelude::*;
use sycamore::prelude::*;
use crate::post::*;
use crate::container::{Container, CurrentRoute};
use crate::BLOG_DIR;

#[perseus::template_rx]
pub fn post_page<'rx, G: Html>(cx: Scope<'rx>, post: PostRx<'rx>) -> View<G> {
    // Whenever we load a new post, reload MathJax
    // #[cfg(target_arch = "wasm32")]
    // {
    //     js_sys::eval(r#"MathJax.Hub.Queue(["Typeset",MathJax.Hub]);"#);
    // }

    let tags = post.tags.get();
    let tags_view = View::new_fragment(tags.iter().cloned().map(|tag| {
        let tag_link = format!("tag/{}", &tag);
        view! { cx,
            li { a(href = tag_link) { (tag) } }
        }
    }).collect::<Vec<_>>());

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::BlogPost) {
            div(class = "flex flex-col lg:flex-row-reverse justify-center items-center w-full px-4 sm:px-8 lg:px-10") {
                // This is present above the table of contents, but we don't want it interfering with the row layout on larger screens
                h1(class = "text-4xl my-3 lg:hidden") { (post.title.get()) }
                // The table of contents will be sticky-aligned to the side of the page
                nav(
                    // Yes, this is 63vh high
                    // I use a larger width here than in the child `<div>` to size things nicely while avoiding a horizontal scrollbar
                    // Bottom padding is to offset the upper padding on the heading
                    class = "styled-prose lg:sticky self-start top-14 xs:top-16 sm:top-20 lg:top-24 max-w-md max-h-[63vh] overflow-y-auto border-l border-neutral-600 pl-4 lg:mx-4 py-3"
                ) {
                    div(
                        class = "max-w-sm",
                        dangerously_set_inner_html = &post.toc.get()
                    )
                }
                div(class = "min-w-0") {
                    // Opposite role of the above `h1`
                    h1(class = "text-4xl my-3 text-center hidden lg:block") { (post.title.get()) }
                    div(class = "styled-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl min-w-0 min-h-0", dangerously_set_inner_html = &post.contents.get()) {}
                    ul {
                        (tags_view)
                    }
                }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope, post: Post) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | The Arctic Circle", post.title)) }
        // We only need the KaTeX stylesheet, since everything has been prerendered on the server-side!
        link(rel = "stylesheet", href = "https://cdn.jsdelivr.net/npm/katex@0.16.3/dist/katex.min.css") {}
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
