use sycamore::prelude::*;
use super::Shortform;

/// The page that displays a single shortform, providing all the replies thereto.
///
/// It is impossible for errors to occur with the management of a single shortform,
/// since any errors would prevent knowing its ID, and such errors coudl only
/// be listed on the root page as top-level errors.
#[component]
pub fn SingleShortform<G: Html>(cx: Scope, shortform: Shortform) -> View<G> {
    view! { cx,
            // TODO Is this unacceptably risky?
            div(dangerously_set_inner_html = &shortform.content)
    }
}
