use std::collections::HashMap;

use perseus::Template;
use sycamore::{Prop, prelude::{view, Html, Scope, SsrNode, View}};
use crate::container::{Container, CurrentRoute};

#[perseus::template_rx]
pub fn contact_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Container(offset_top = true, route = CurrentRoute::Contact) {
            div(class = "flex flex-col justify-center items-center") {
                h1(class = "text-3xl mb-6 p-4 text-center") { "Here are some of the ways you might like to contact me..." }
                div(class = "flex flex-col lg:flex-row mb-16") {
                    div(class = "border-4 border-neutral-800 rounded-lg p-6 m-4 my-2 max-w-md") {
                        h3(class = "text-3xl mb-2") { "By project..." }
                        ul(class = "list-disc pl-5") {
                            ContactEntry(title = "Perseus", links = vec![
                                ( "GitHub", "https://github.com/framesurge/perseus" ),
                                ( "Discord", "https://discord.com/invite/GNqWYWNTdp" ),
                            ])
                            ContactEntry(title = "Andromeda", links = vec![
                                ( "GitHub", "https://github.com/framesurge/andromeda" ),
                            ])
                            ContactEntry(title = "Bonnie", links = vec![
                                ( "GitHub", "https://github.com/arctic-hen7/bonnie" ),
                            ])
                            ContactEntry(title = "Diana", links = vec![
                                ( "GitHub", "https://github.com/arctic-hen7/diana" ),
                            ])
                        }
                    }
                    div(class = "border-4 border-neutral-800 rounded-lg p-6 m-4 my-2 max-w-md") {
                        h3(class = "text-3xl mb-2") { "For business..." }
                        ul(class = "list-disc pl-5") {
                            ContactEntry(title = "Programming", links = vec![
                                ( "GitHub", "https://github.com/arctic-hen7" ),
                                ( "Email", "mailto:arctic.hen@pm.me" ),
                            ])
                            ContactEntry(title = "Economics", links = vec![
                                ( "GitHub", "https://github.com/arctic-hen7" ),
                                ( "Email", "mailto:arctic.hen@pm.me" ),
                            ])
                            ContactEntry(title = "Business", links = vec![
                                ( "Email", "mailto:arctic.hen@pm.me" ),
                            ])
                        }
                    }
                    div(class = "border-4 border-neutral-800 rounded-lg p-6 m-4 my-2 max-w-sm") {
                        h3(class = "text-3xl mb-2") { "For personal reasons..." }
                        span { "If I haven't given you my personal contact details, please send a legal waiver to my business email so that I can sign off on your sending a scent-tracking inter-continental carrier pigeon to find me. I will be more than happy to comply." }
                    }
                }
                div(class = "max-w-lg mx-4") {
                    h3(class = "text-3xl mb-2") { "Further details" }
                    p {
                        "I'm currently based in "
                        strong { "Sydney, Australia" }
                        ", with timezone AEDT (UTC+11)."

                    }
                    p {
                        "You can use "
                        a(href = "https://www.timeanddate.com/worldclock/converter.html?p1=240", class = "text-blue-400 hover:text-blue-600 transition-colors duration-150 underline") { "this tool" }
                        " to plan a meeting time with me. I'm generally available for meetings on weekdays from 9:00-11:00, 11:30-12:30, and 13:30-15:30. If you notify me that you'd like a meeting by 15:30 the day before, I'll usually be able to fit it in, though I can't make any guarantees, as my schedule is usually quite busy."
                    }
                }
            }
        }
    }
}

#[derive(Prop)]
struct ContactEntryProps<'a> {
    title: &'a str,
    links: Vec<(&'a str, &'a str)>
}

#[sycamore::component]
fn ContactEntry<'a, G: Html>(cx: Scope<'a>, props: ContactEntryProps<'a>) -> View<G> {
    let links_view = View::new_fragment(
        props
            .links
            .into_iter()
            .map(|(text, link)| {
                let text = text.to_string();
                let link = link.to_string();
                view! { cx,
                    a(href = link, class = "text-blue-400 hover:text-blue-600 transition-colors duration-150 underline mx-1") { (text) }
                }
            })
            .collect::<Vec<_>>()
    );

    view! { cx,
        li() {
            span {
                (format!("{}: ", props.title))
                (links_view)
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Contact Me | The Arctic Site" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("contact").template(contact_page).head(head)
}
