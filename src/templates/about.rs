use crate::container::{Container, CurrentRoute};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[auto_scope]
fn about_page<G: Html>(cx: Scope, AboutStateRx { content }: &AboutStateRx) -> View<G> {
    view! { cx,
        Container(offset_top = true, route = CurrentRoute::About) {
            div(class = "flex flex-col md:flex-row justify-center items-center md:items-start") {
                img(class = "rounded-full h-52 w-52 md:mr-2 mb-4", src = "https://github.com/arctic-hen7.png", alt = "My GitHub profile picture") {}
                div(class = "md:ml-2 p-4 styled-prose max-w-prose", dangerously_set_inner_html = &content.get()) {}
            }
        }

    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "AboutStateRx")]
struct AboutState {
    /// The HTML contents of the about page.
    content: String,
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Me | The Arctic Site" }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _: StateGeneratorInfo<()>,
) -> Result<AboutState, BlamedError<std::io::Error>> {
    use pulldown_cmark::{html, Options, Parser};

    let md_content = std::fs::read_to_string("about.md")?;

    let md_opts = Options::all();
    let parser = Parser::new_ext(&md_content, md_opts);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    Ok(AboutState {
        content: html_content,
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about")
        .view_with_state(about_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}
