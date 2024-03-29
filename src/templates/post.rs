use crate::container::{Container, CurrentRoute};
use crate::post::*;
#[cfg(engine)]
use crate::{Error, BLOG_DIR};
use perseus::prelude::*;
use sycamore::prelude::*;

#[auto_scope]
fn post_page<G: Html>(cx: Scope, post: &PostRx) -> View<G> {
    let tags = post.tags.get();
    let tags_view = View::new_fragment(tags.iter().cloned().map(|tag| {
        let tag_link = format!("tag/{}", &tag);
        view! { cx,
            li(class = "inline-block") {
                a(
                    class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-2 px-4 m-2 font-mono flex items-center",
                    href = tag_link
                ) { (tag) }
            }
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
                    div(class = "styled-prose lg:max-w-3xl xl:max-w-4xl 2xl:max-w-5xl min-w-0 min-h-0 mb-6", dangerously_set_inner_html = &post.contents.get()) {}
                }
            }

            div(class = "flex flex-col items-center w-full") {
                // The top margin on this is compounded by the `m-2` on each list item
                ul(class = "flex justify-center text-center mt-4") {
                    (tags_view)
                }
                // Giscus container
                div(class = "max-w-prose px-4 mt-8 giscus") {}
            }

            // Giscus loader
            script(
                src = "https://giscus.app/client.js",
                data-repo = "arctic-hen7/arctic-hen7.github.io",
                data-repo-id = "MDEwOlJlcG9zaXRvcnkzNjc4MTc5Njk=",
                data-category = "Comments",
                data-category-id = "DIC_kwDOFex08c4CSjuT",
                data-mapping = "pathname",
                data-strict = "0",
                data-reactions-enabled = "1",
                data-emit-metadata = "0",
                data-input-position = "top",
                data-theme = "dark_high_contrast",
                data-lang = "en",
                data-loading = "lazy",
                crossorigin = "anonymous",
                async = true,
            ) {}
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, post: Post) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} | The Arctic Circle", post.title)) }
        // We only need the KaTeX stylesheet, since everything has been prerendered on the server-side!
        link(rel = "stylesheet", href = "https://cdn.jsdelivr.net/npm/katex@0.16.3/dist/katex.min.css") {}
        link(rel = "stylesheet", href = ".perseus/static/org.css") {}
        meta(name = "description", content = post.description) {}
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<Post, BlamedError<Error>> {
    use std::fs;
    use std::path::Path;

    let filename = format!("{path}.json");
    let file_path = Path::new(BLOG_DIR).join(filename);

    let contents = fs::read_to_string(file_path).map_err(Error::from)?;
    let post: FullPost = serde_json::from_str(&contents).map_err(Error::from)?;

    Ok(post.post)
}

#[engine_only_fn]
async fn get_build_paths() -> Result<BuildPaths, Error> {
    use std::fs;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut paths = Vec::new();
    for entry in fs::read_dir(BLOG_DIR).map_err(Error::from)? {
        let entry = entry.map_err(Error::from)?;

        let contents = fs::read_to_string(entry.path()).map_err(Error::from)?;
        let post: FullPost = serde_json::from_str(&contents).map_err(Error::from)?;

        paths.push(post.post.id)
    }

    Ok(BuildPaths {
        paths,
        extra: ().into(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("post")
        .view_with_state(post_page)
        .head_with_state(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .build()
}
