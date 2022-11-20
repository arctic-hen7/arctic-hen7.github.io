use crate::container::{Container, CurrentRoute};
use crate::post::*;
use crate::BLOG_DIR;
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::template_rx]
pub fn post_page<'rx, G: Html>(cx: Scope<'rx>, posts: PostsRx<'rx>) -> View<G> {
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

#[make_rx(PostsRx)]
struct Posts {
    posts: Vec<SlimPost>,
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "The Arctic Circle | The Arctic Site" }
    }
}

#[perseus::build_state]
fn get_build_state(_: String, _: String) -> RenderFnResultWithCause<Posts> {
    use std::fs;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut posts = Vec::new();
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path())?;
        let post: FullPost = serde_json::from_str(&contents)?;

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
    Template::new("posts")
        .template(post_page)
        .head(head)
        .build_state_fn(get_build_state)
}
