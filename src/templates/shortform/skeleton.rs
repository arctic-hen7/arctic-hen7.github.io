use sycamore::prelude::*;

/// The skeleton displayed before the shortform page has been loaded.
///
/// We don't differentiate between the one for single shortforms and the
/// one for the root page because the server will always load the root one,
/// and differentiation would lead to a nasty flicker.
#[component]
pub fn ShortformSkeleton<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class = "flex flex-col items-center") {
            ul(class = "max-w-4xl") {
                li(class = "") {
                    span { "Loading..." }
                }
            }
        }
    }
}
