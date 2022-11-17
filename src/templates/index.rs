use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};
use crate::container::{Container, CurrentRoute};

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Container(offset_top = false, route = CurrentRoute::Home) {
            // This page is a beautiful example of extreme manual spacing control
            div(class = "flex flex-col justify-center items-center h-[90vh] uppercase font-bold text-4xl xs:text-5xl sm:text-7xl md:text-8xl xl:text-[10rem]") {
                div {
                    span(class = "spaced-letter") { "S" }
                    span(class = "spaced-letter") { "a" }
                    span { "m" }
                }
                div(class = "my-12 border border-netural-200 w-[85%] max-w-[70rem]") {}
                div {
                    span(class = "spaced-letter") { "B" }
                    span(class = "spaced-letter") { "r" }
                    span(class = "spaced-letter") { "e" }
                    span { "w" }
                }
            }
            div(class = "flex justify-center items-center h-[90vh] text-center text-xl xs:text-2xl sm:text-3xl lg:text-4xl font-mono w-full") {
                div(class = "") {
                    p(class = "") { "Developer" }
                    div(class = "my-36") {}
                    p(class = "") { "Philosopher" }
                }
                div(class = "w-20 xs:w-36 sm:w-52 md:w-64")
                div(class = "") {
                    p(class = "") { "Economist" }
                    div(class = "my-36") {}
                    p(class = "") { "Musician" }
                }
                div(class = "flex justify-center items-center h-full w-full absolute text-center font-mono text-4xl lg:text-5xl") {
                    span(class = "underline") { "Thinker" }
                }
            }
            div(class = "flex flex-col justify-center items-center h-[90vh]") {
                div(class = "flex flex-col md:flex-row justify-center") {
                    a(
                        class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-6 m-4 my-2 max-w-md flex flex-col justify-center",
                        href = "posts"
                    ) {
                        span(class = "text-2xl") { "📖 The Arctic Circle" }
                        span { "Longform blog posts about almost anything. Some worthy of framing, some worthy of burning. Some both." }
                    }
                    a(
                        class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-6 m-4 my-2 max-w-md flex flex-col justify-center",
                        href = "shortform"
                    ) {
                        span(class = "text-2xl") { "🗫 The Ice Floes " }
                        span { "Random short musings, the hermit's Twitter." }
                    }
                }
                a(
                    class = "block border-4 border-neutral-800 hover:bg-neutral-800 transition-colors duration-150 rounded-lg p-6 m-4 my-2 max-w-md flex flex-col",
                    href = "about"
                ) {
                    span(class = "text-2xl") { "👋 About Me" }
                    span { "Learn more about me, and what I'm working on. If you'd like to contact me, this is the place to go." }
                }
                a(href = "contact", class = "underline text-blue-400 hover:text-blue-500 transition-colors duration-150 mt-8") { "📨 Contact Me" }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Home | The Arctic Site" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
