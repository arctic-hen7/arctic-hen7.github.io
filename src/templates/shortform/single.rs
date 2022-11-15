use sycamore::prelude::*;
use super::Shortform;
use crate::post::PostAuthor;

static ANONYMOUS_USER_PIC_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24"><g><rect fill="none" height="24" width="24"/></g><g><g><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM7.35 18.5C8.66 17.56 10.26 17 12 17s3.34.56 4.65 1.5c-1.31.94-2.91 1.5-4.65 1.5s-3.34-.56-4.65-1.5zm10.79-1.38C16.45 15.8 14.32 15 12 15s-4.45.8-6.14 2.12C4.7 15.73 4 13.95 4 12c0-4.42 3.58-8 8-8s8 3.58 8 8c0 1.95-.7 3.73-1.86 5.12z"/><path d="M12 6c-1.93 0-3.5 1.57-3.5 3.5S10.07 13 12 13s3.5-1.57 3.5-3.5S13.93 6 12 6zm0 5c-.83 0-1.5-.67-1.5-1.5S11.17 8 12 8s1.5.67 1.5 1.5S12.83 11 12 11z"/></g></g></svg>"#;

/// The page that displays a single shortform, providing all the replies thereto.
///
/// It is impossible for errors to occur with the management of a single shortform,
/// since any errors would prevent knowing its ID, and such errors coudl only
/// be listed on the root page as top-level errors.
#[component]
pub fn SingleShortform<G: Html>(cx: Scope, shortform: Shortform) -> View<G> {
    let post_link = format!("shortform#{}", shortform.id);

    let (author_name, author_home_url, author_profile_pic) = shortform.author.parse(cx);

    let local = shortform.time.with_timezone(&chrono::Local);
    // See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    let time = local.format("%_I:%M%P");
    let date = local.format("%A %x");

    view! { cx,
        a(class = "rounded-md p-6 bg-neutral-900 block m-2 max-w-prose", href = post_link) {
            div(class = "flex justify-between items-center mb-1") {
                div(class = "flex items-center") {
                    (author_profile_pic)
                    object(class = "inline-flex items-center max-h-[0.1rem]", type = "invalid/mime-type") {
                        a(class = "ml-2 font-bold", href = author_home_url, target = "blank") { (author_name) }
                    }
                }
                span(class = "italic text-neutral-400") { (format!("at {} on {}", time, date)) }
            }
            // TODO Is this unacceptably risky?
            div(class = "styled-prose", dangerously_set_inner_html = &shortform.content)
        }

    }
}
