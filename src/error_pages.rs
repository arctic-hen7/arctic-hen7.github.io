use perseus::{i18n::Translator, ErrorPages, Html};
use std::rc::Rc;
use sycamore::prelude::*;
use crate::container::{Container, CurrentRoute};

// This site will be exported statically, so we only have control over 404 pages
// for broken links in the site itself
pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(
        |cx, url, status, err, _| {
            view! { cx,
                p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
            }
        },
        |cx, _, _, _, _| {
            view! { cx,
                title { "Error" }
            }
        },
    );
    error_pages.add_page(404, not_found_page, |cx, _, _, _, _| {
        view! { cx,
            title { "Not Found" }
        }
    });

    error_pages
}

fn not_found_page<G: Html>(
    cx: Scope,
    _url: String,
    _status: u16,
    _err: String,
    _translator: Option<Rc<Translator>>,
) -> View<G> {
    view! { cx,
        Container(offset_top = false, route = CurrentRoute::NotFound) {
            div(class = "flex flex-col justify-center items-center h-screen") {
                main(class = "flex flex-col border-2 border-neutral-800 rounded-lg max-w-xl m-4") {
                    h3(class = "text-2xl font-bold w-full pb-4 border-b-2 border-neutral-800 my-4") {
                        span(class = "font-mono pl-4") { "404: Page not found!" }
                    }
                    div(class = "p-4 pt-0 my-4") {
                        span { "That page doesn't seem to exist. If you came here from a link elsewhere on the site, I'm terribly sorry, I clearly can't type properly. If you came here another website, or a search engine, this page probably existed once, but has since been moved. Here are some pages you might like to try instead:" }
                        ul(class = "pl-6 mt-4 w-full list-disc") {
                            li {
                                a(class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150", href = "") { "Home" }
                            }
                            li {
                                a(class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150", href = "about") { "About Me" }
                            }
                            li {
                                a(class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150", href = "posts") { "The Arctic Circle" }
                            }
                            li {
                                a(class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150", href = "shortform") { "The Ice Floes" }
                            }
                        }
                    }
                }
            }
        }
    }
}
