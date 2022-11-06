mod templates;
mod post;
mod container;
mod global_state;

use perseus::{Html, PerseusApp, PerseusRoot};

// Relative to the root of the project
static BLOG_DIR: &str = "./.blog";

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::post::get_template)
        .template(crate::templates::tag::get_template)
        .template(crate::templates::series::get_template)
        // .global_state_creator(crate::global_state::get_global_state_creator())
        .index_view(|cx| sycamore::view! { cx,
            html(class = "light") {
                head {
                    meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css")
                }
                body(class = "bg-black text-white") {
                    PerseusRoot()
                }
            }
        })
}
