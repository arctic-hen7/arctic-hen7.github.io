use serde::{Deserialize, Serialize};
#[cfg(client)]
use sycamore::prelude::*;

/// A function responsible for displaying any errors that occur while fetching shortforms.
///
/// This will be used in place of a shortform on either the error page or the single page.
#[cfg(target_arch = "wasm32")]
#[component]
pub fn ShortformErrorView<G: Html>(cx: Scope, err: ShortformError) -> View<G> {
    use super::root::ERROR_ICON;

    view! { cx,
        div(class = "flex justify-center") {
            div(class = "bg-red-300 p-6 max-w-prose rounded-md text-red-900 flex mx-4") {
                span(dangerously_set_inner_html = ERROR_ICON, class = "fill-red-900") {}
                div(class = "pl-2") {
                    p { (err.to_string()) }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, thiserror::Error)]
pub enum ShortformError {
    /// Couldn't fetch the posts in the first place.
    #[error("Couldn't fetch posts (are you online?)")]
    FetchFailed,
    /// Couldn't deserialize one of the posts.
    #[error(
        "Couldn't parse post (try reloading the page, otherwise this is a problem on our end)"
    )]
    DeserFailed,
    /// The hash component of the URL references a post that doesn't exist.
    #[error("No such post")]
    HashInvalid,
}
