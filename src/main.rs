mod container;
mod error_pages;
mod post;
mod rss;
mod templates;

use perseus::{plugins::Plugins, Html, PerseusApp, PerseusRoot};

// Relative to the root of the project
static BLOG_DIR: &str = "./.blog";

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::post::get_template)
        .template(crate::templates::posts::get_template)
        .template(crate::templates::tag::get_template)
        .template(crate::templates::series::get_template)
        .template(crate::templates::shortform::get_template)
        .template(crate::templates::contact::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|cx| sycamore::view! { cx,
            html(class = "light") {
                head {
                    meta(charset = "UTF-8") {}
                    meta(name = "viewport", content = "width=device-width, initial-scale=1.0") {}
                    link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                }
                body(class = "bg-black text-white") {
                    PerseusRoot()
                }
            }
        })
        .plugins(Plugins::new().plugin(
                rss::get_rss_plugin,
                (),
        ))
}
