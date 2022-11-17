use crate::container::{Container, CurrentRoute};
use crate::post::*;
use crate::BLOG_DIR;
use perseus::{RenderFnResult, RenderFnResultWithCause, Template};
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

// Note: when Perseus has islands, this will be a component with state, which can be included on the page of each post in a series.
#[perseus::template_rx]
pub fn series_page<'rx, G: Html>(cx: Scope<'rx>, series: SeriesRx<'rx>) -> View<G> {
    let posts = series.posts_in_series.get();
    let posts_view = View::new_fragment(posts.iter().cloned().enumerate().map(|(idx, post)| {
        let (author_name, author_home_url, author_profile_pic) = post.author.parse(cx);
        view! { cx,
                li(class = "inline-block text-left") {
                    a(
                        class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-6 m-4 max-w-md flex flex-col",
                        href = format!("post/{}", post.id)
                    ) {
                        div(class = "text-xl font-bold flex flex-row items-center") {
                            span(class = "p-2 px-4 rounded-full bg-neutral-700 mr-3") { (idx + 1) }
                            span { (post.title) }
                        }
                        div(class = "inline-flex items-center my-2") {
                            (author_profile_pic)
                            // Nested links aren't allowed under the HTML spec, but this works
                            // Source: https://kizu.dev/nested-links/
                                object(class = "inline-flex items-center max-h-[0.1rem]", type = "invalid/mime-type") {
                                    a(class = "ml-2 font-semibold", href = author_home_url, target = "blank") { (author_name) }
                                }
                        }
                        span(class = "") { (post.description) }
                    }
                }
        }
    }).collect::<Vec<_>>());

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::Series) {
            div(class = "flex flex-col justify-center items-center") {
                h1(class = "text-4xl p-4 text-center") { (format!("Posts in series '{}'", series.name.get())) }
                ul(class = "max-w-[80%] text-center") {
                    (posts_view)
                }
            }
        }
    }
}

#[perseus::make_rx(SeriesRx)]
struct Series {
    name: String,
    posts_in_series: Vec<SlimPost>,
}

#[perseus::head]
pub fn head(cx: Scope, series: Series) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} (Series) | The Arctic Circle", series.name)) }
    }
}

#[perseus::build_state]
fn get_build_state(path: String, _: String) -> RenderFnResultWithCause<Series> {
    use std::fs;

    let series = path.strip_prefix("series/").unwrap();

    // Get all the blog posts, and figure out which ones are in this series
    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut posts_in_series = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path())?;
        let post: FullPost = serde_json::from_str(&contents)?;

        if post.post.series.is_some() && post.post.series.as_ref().unwrap().0 == series {
            posts_in_series.push(SlimPost {
                id: post.post.id,
                title: post.post.title,
                author: post.post.author,
                description: post.post.description,
                series: post.post.series,
            });
        }
    }

    // Now order those by their indices to make sure we display the right order
    posts_in_series.sort_by(|a, b| {
        a.series
            .as_ref()
            .unwrap()
            .1
            .partial_cmp(&b.series.as_ref().unwrap().1)
            .unwrap()
    });

    Ok(Series {
        posts_in_series,
        name: series.to_string(),
    })
}

#[perseus::build_paths]
fn get_build_paths() -> RenderFnResult<Vec<String>> {
    use anyhow::Context;
    use std::fs;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut series_list = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents =
            fs::read_to_string(entry.path()).context("Failed to read file in blog index")?;
        let post: FullPost =
            serde_json::from_str(&contents).context("Failed to deserialize file in blog index")?;

        // If this post is in a series, add that series to our list
        if let Some(series) = post.post.series {
            if !series_list.contains(&series.0) {
                series_list.push(series.0);
            }
        }
    }

    Ok(series_list)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("series")
        .template(series_page)
        .head(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
}
