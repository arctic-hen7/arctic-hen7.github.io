use super::single::SingleShortform;
use super::*;
use sycamore::prelude::*;

pub static ERROR_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" height="24" viewBox="0 0 24 24" width="24"><path d="M11 15h2v2h-2v-2zm0-8h2v6h-2V7zm.99-5C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z"/></svg>"#;

/// The root page that displays many shortforms.
///
/// The shortforms this receives are guaranteed to have already been loaded.
#[component]
pub fn ShortformRootPage<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    shortform_list: &'a ShortformListRx,
) -> View<G> {
    let shortforms = create_memo(cx, move || {
        let list = &*shortform_list.list.get();
        let list = list.as_ref().unwrap(); // Guaranteed by caller
        let mut posts = list.values().collect::<Vec<_>>();
        // Sort the posts by their time of creation
        posts.sort_by(|a, b| b.time.partial_cmp(&a.time).unwrap());
        View::new_fragment(
            posts
                .into_iter()
                .map(|post| {
                    let post = post.clone();
                    view! { cx,
                        li {
                            SingleShortform(post)
                        }
                    }
                })
                .collect::<Vec<_>>(),
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
                            li(class = "ml-8") { (error) }
                    }
                })
                .collect::<Vec<_>>(),
        );
        // We don't use the system for displaying a single error when we're displaying many
        if errors_list.is_empty() {
            View::empty()
        } else {
            view! { cx,
                div(class = "bg-red-300 p-6 max-w-prose rounded-md text-red-900 flex mx-4") {
                    span(dangerously_set_inner_html = ERROR_ICON, class = "fill-red-900") {}
                    div(class = "pl-2") {
                        p { "One or more errors occurred while fetching the posts:" }
                        ul(class = "list-disc") { (errors) }
                    }
                }
            }
        }
    });

    view! { cx,
        div(class = "flex flex-col items-center") {
            (*list_errors.get())
            ul(class = "") {
                (*shortforms.get())
            }
        }
    }
}
