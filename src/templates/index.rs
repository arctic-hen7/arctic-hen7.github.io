use crate::container::{Container, CurrentRoute};
use crate::global_state::AppStateRx;
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    let global_state: &AppStateRx = Reactor::<G>::from_cx(cx).get_global_state(cx);

    view! { cx,
        Container(offset_top = false, route = CurrentRoute::Home) {
            // This page is a beautiful example of extreme manual spacing control
            div(class = "flex flex-col justify-center items-center h-[90vh]") {
                div(class = "uppercase font-bold text-4xl xs:text-5xl sm:text-7xl md:text-8xl xl:text-[10rem] flex flex-col justify-center items-center mt-6") {
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

                div(class = "italic bg-gray-900 rounded-lg p-2 px-4 text-center text-lg max-w-3xl mt-6") {
                    span(class = "font-bold") { "Motd: " }
                    span { (global_state.motd.get()) }
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

#[engine_only_fn]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Home | The Arctic Site" }
        meta(name = "description", content = "The landing page, where you can learn about who I am and what I do.")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
