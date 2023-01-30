use crate::container::{Container, CurrentRoute};
use crate::post::*;
#[cfg(engine)]
use crate::{Error, BLOG_DIR};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[auto_scope]
fn post_page<G: Html>(cx: Scope, posts: &PostsRx) -> View<G> {
    let posts = posts.posts.get();
    let posts = View::new_fragment(
        posts
            .iter()
            .cloned()
            .map(|post| {
                let (author_name, author_home_url, author_profile_pic) = post.author.parse(cx);
                view! { cx,
                        li(class = "inline-block text-left") {
                            a(
                                class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-6 m-4 max-w-md flex flex-col",
                                href = format!("post/{}", post.id)
                            ) {
                                span(class = "text-xl font-bold") { (post.title) }
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
            })
            .collect::<Vec<_>>()
    );

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::BlogPost) {
            div(class = "flex justify-center") {
                ul(class = "max-w-[80%] text-center") {
                    (posts)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PostsRx")]
struct Posts {
    posts: Vec<SlimPost>,
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "The Arctic Circle | The Arctic Site" }
    }
}

#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> Result<Posts, BlamedError<Error>> {
    use std::fs;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut posts = Vec::new();
    for entry in fs::read_dir(BLOG_DIR).map_err(Error::from)? {
        let entry = entry.map_err(Error::from)?;

        let contents = fs::read_to_string(entry.path()).map_err(Error::from)?;
        let post: FullPost = serde_json::from_str(&contents).map_err(Error::from)?;

        posts.push(SlimPost {
            id: post.post.id,
            title: post.post.title,
            author: post.post.author,
            description: post.post.description,
            series: post.post.series,
            date: post.post.date,
        });
    }

    posts.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());

    Ok(Posts { posts })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("posts")
        .view_with_state(post_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}
