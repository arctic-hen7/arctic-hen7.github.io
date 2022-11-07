use serde::{Serialize, Deserialize};
#[cfg(target_arch = "wasm32")]
use sycamore::prelude::*;

/// A function responsible for displaying any errors that occur while fetching shortforms.
///
/// This will be used in place of a shortform on either the error page or the single page.
#[cfg(target_arch = "wasm32")]
#[component]
pub fn ShortformErrorView<G: Html>(cx: Scope, err: ShortformError) -> View<G> {
    view! { cx,
        // TODO Styling
        div(class = "bg-red-400") {
            span(class = "text-white") { (format!("An error occurred: {}", err.to_string())) }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, thiserror::Error)]
pub enum ShortformError {
    /// Couldn't fetch the posts in the first place.
    #[error("couldn't fetch posts (are you online?)")]
    FetchFailed,
    /// Couldn't deserialize one of the posts.
    #[error("couldn't parse post (try reloading the page, otherwise this is a problem on our end)")]
    DeserFailed,
    /// The signature on one of the posts was invalid.
    ///
    /// TODO If this is recorded, ping an error logging
    /// server to let me know that someone has access to
    /// the ice floes repo.
    #[error("signature invalid (this post may have been made by an imposter!!!)")]
    SigInvalid,
    /// The hash component of the URL references a post that doesn't exist.
    #[error("no such post")]
    HashInvalid
}
