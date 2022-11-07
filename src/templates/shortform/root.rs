use sycamore::prelude::*;
use super::*;
use super::single::SingleShortform;

/// The root page that displays many shortforms.
///
/// The shortforms this receives are guaranteed to have already been loaded.
#[component]
pub fn ShortformRootPage<'rx, G: Html>(cx: Scope<'rx>, shortform_list: ShortformListRx<'rx>) -> View<G> {
    // TODO Does this need to be in a memo?
    let shortforms = create_memo(cx, move || {
        let list = &*shortform_list.list.get();
        let list = list.as_ref().unwrap(); // Guaranteed by caller
        View::new_fragment(
            list
                .iter()
                .map(|(_id, post)| {
                    let post = post.clone();
                    view! { cx,
                        SingleShortform(post)
                    }
                })
                .collect::<Vec<_>>()
        )
    });
    // Any errors affecting the whole list
    let list_errors = create_memo(cx, move || {
        let errors_list = shortform_list.errors.get();
        let errors = View::new_fragment(
            errors_list
                .iter()
                .map(|err| {
                    let error = err.to_string();
                    view! { cx,
                            li(class = "") { (error) }
                    }
                })
                .collect::<Vec<_>>()
        );
        // We don't use the system for displaying a single error when we're displaying many
        if errors_list.is_empty() {
            View::empty()
        } else {
            view! { cx,
                    div(class = "bg-red") {
                        p { "One or more errors occurred while fetching the posts." }
                        ul { (errors) }
                    }
            }
        }
    });



    view! { cx,
        div {
            (*list_errors.get())
        }
        ul {
            (*shortforms.get())
        }
    }
}
