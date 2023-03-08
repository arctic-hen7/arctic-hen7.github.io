use crate::container::{Container, CurrentRoute};
use crate::post::*;
#[cfg(engine)]
use crate::{Error, BLOG_DIR};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[auto_scope]
fn tag_page<G: Html>(cx: Scope, tag: &TagRx) -> View<G> {
    let posts = tag.posts_with_tag.get();
    let posts_view = View::new_fragment(posts.iter().cloned().map(|post| {
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
                                object(class = "inline-flex items-center", type = "invalid/mime-type") {
                                    a(class = "ml-2 font-semibold", href = author_home_url, target = "blank") { (author_name) }
                                }
                        }
                        span(class = "") { (post.description) }
                    }
                }
        }
    }).collect::<Vec<_>>());

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::Tag) {
            div(class = "flex flex-col justify-center items-center") {
                h1(class = "text-4xl p-4 text-center") { (format!("Posts tagged '{}'", tag.name.get())) }
                ul(class = "max-w-[80%] text-center") {
                    (posts_view)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "TagRx")]
struct Tag {
    name: String,
    posts_with_tag: Vec<SlimPost>,
}

#[engine_only_fn]
fn head(cx: Scope, tag: Tag) -> View<SsrNode> {
    let tag = create_ref(cx, tag);
    view! { cx,
        title { (format!("Posts tagged '{}' | The Arctic Circle", tag.name)) }
        meta(name = "description", content = format!("A list of posts in the '{}' tag.", &tag.name)) {}
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<Tag, BlamedError<Error>> {
    use std::fs;

    // Get all the blog posts, and figure out which ones have this tag
    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut posts_with_tag = Vec::new();
    for entry in fs::read_dir(BLOG_DIR).map_err(Error::from)? {
        let entry = entry.map_err(Error::from)?;

        let contents = fs::read_to_string(entry.path()).map_err(Error::from)?;
        let post: FullPost = serde_json::from_str(&contents).map_err(Error::from)?;

        if post.post.tags.iter().any(|t| t == &path) {
            posts_with_tag.push(SlimPost {
                id: post.post.id,
                title: post.post.title,
                author: post.post.author,
                description: post.post.description,
                series: post.post.series,
                date: post.post.date,
            });
        }
    }

    posts_with_tag.sort_by(|a, b| b.date.partial_cmp(&a.date).unwrap());

    Ok(Tag {
        posts_with_tag,
        name: path,
    })
}

#[engine_only_fn]
async fn get_build_paths() -> Result<BuildPaths, Error> {
    use std::fs;

    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    let mut tags = Vec::new();
    for entry in fs::read_dir(BLOG_DIR).map_err(Error::from)? {
        let entry = entry.map_err(Error::from)?;

        let contents = fs::read_to_string(entry.path()).map_err(Error::from)?;
        let post: FullPost = serde_json::from_str(&contents).map_err(Error::from)?;

        // Add all the tags of this post to the list of paths, making sure we don't have any duplications
        for tag in post.post.tags.into_iter() {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
    }

    Ok(BuildPaths {
        paths: tags,
        extra: ().into(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("tag")
        .view_with_state(tag_page)
        .head_with_state(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .build()
}
