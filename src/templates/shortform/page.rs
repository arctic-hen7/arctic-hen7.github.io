use sycamore::prelude::*;
use super::*;
use super::skeleton::*;
use crate::container::Container;

// This would use incremental generation if we had a server, but we don't
/// The shortform *page*. Whether or not this resolves to the root page, or
/// the page for a specific shortform, is dependent on the hash component of
/// the URL.
///
/// This takes the state of a list of shortforms, which will be empty on the
/// engine-side, and then filled up on the client-side. This can then be cached
/// in the PSS< preventing the need to reload everything each time the user navigates
/// back to the previous page.
///
/// On the engine-side, this will produce a skeleton, which will then be filled out
/// on the browser-side.
#[cfg(not(target_arch = "wasm32"))]
#[perseus::template_rx]
pub fn shortform_page<'rx, G: Html>(cx: Scope<'rx>, _shortform_list: ShortformListRx<'rx>) -> View<G> {
    view! { cx,
        Container(offset_top = true) {
            ShortformSkeleton {}
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[perseus::template_rx]
pub fn shortform_page<'rx, G: Html>(cx: Scope<'rx>, shortform_list: ShortformListRx<'rx>) -> View<G> {
    use super::single::SingleShortform;
    use super::root::ShortformRootPage;
    use super::error::ShortformErrorView;

    // We need to make sure the shortforms aren't already filled out, for when we re-render (and everything is in the PSS!)
    #[cfg(target_arch = "wasm32")]
    if shortform_list.list.get().is_none() {
        // TODO Error handling! (if we have critical errors, make the map empty, not nonexistent)
        perseus::spawn_local_scoped(cx, async {
            let full_list = gloo_net::http::Request::get("https://raw.githubusercontent.com/arctic-hen7/the-ice-floes/main/posts")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            // Split the list into posts, line-by-line
            let posts = full_list.split("\n");
            let mut list = HashMap::new();
            for post in posts {
                let res = serde_json::from_str::<Shortform>(post).map_err(|_| ShortformError::DeserFailed);
                match res {
                    Ok(post) => { list.insert(post.id.to_string(), post); },
                    // If deserialization failed, we don't even know the ID, so add this to the top-level errors
                    Err(err) => shortform_list.errors.modify().push(err)
                };
            }

            // Completely override whatever else was already in the list (even if there were all errors
            // and we have nothing, this still shouldn't be `None`)
            shortform_list.list.set(Some(list));
        });
    }

    let location = web_sys::window()
        .unwrap()
        .location();
    let hash = location.hash().unwrap();
    let hash = if hash.is_empty() {
        None
    } else {
        // If it does exist, it will have a `#` in front of it
        Some(hash.strip_prefix("#").unwrap().to_string())
    };

    // This provides the single shortform to render if there's a
    // hash component to the URL. If there isn't, it will be `Root`,
    // and we'll render the root page.
    let single_shortform = create_memo(cx, move || {
        let shortforms = &*shortform_list.list.get();
        if let Some(shortforms) = shortforms {
            if let Some(hash) = &hash {
                // We have the list of shortforms along with the hash itself, so get the right one
                let shortform = shortforms.get(hash).cloned();
                match shortform {
                    Some(shortform) => ShortformStatus::Single(shortform),
                    None => ShortformStatus::SingleNotFound
                }
            } else {
                ShortformStatus::Root
            }
        } else {
            ShortformStatus::Loading
        }
    });

    let view = create_memo(cx, move || {
        match &*single_shortform.get() {
            ShortformStatus::Single(shortform) => {
                let shortform = shortform.clone();
                view! { cx,
                        Container(offset_top = true) {
                            SingleShortform(shortform)
                        }
                }
            },
            ShortformStatus::SingleNotFound => view! { cx,
                                                       Container(offset_top = true) {
                                                           ShortformErrorView(ShortformError::HashInvalid)
                                                       }
            },
            ShortformStatus::Root => {
                let shortform_list = shortform_list.clone();
                view! { cx,
                        Container(offset_top = true) {
                            // By this point, we can guarantee that there's some stuff here
                            ShortformRootPage(shortform_list)
                        }
                }
            },
            ShortformStatus::Loading => view! { cx,
                                                    Container(offset_top = true) {
                                                        ShortformSkeleton {}
                                                    }
            }
        }
    });

    view! { cx,
        (*view.get())
    }
}

/// The status of the shortform page. This is necessary because hash routing is handled manually
/// on the client-side, which is a product of my wanting to run this as an exported app, rather
/// than using a server (which could use revalidation and incremental generation to make this
/// a piece of cake).
#[cfg(target_arch = "wasm32")]
enum ShortformStatus {
    /// There's no hash component, the root page should be rendered.
    Root,
    /// There is a valid hash component, the
    /// shortform it references should be rendered.
    Single(Shortform),
    /// There is a hash component that was invalid (the only error that can occur
    /// with a single shortform).
    SingleNotFound,
    /// We're still loading the shortforms.
    Loading,
}
