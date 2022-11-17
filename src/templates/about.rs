use crate::container::{Container, CurrentRoute};
use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::template_rx]
fn about_page<'rx, G: Html>(
    cx: Scope<'rx>,
    AboutPropsRx { content }: AboutPropsRx<'rx>,
) -> View<G> {
    view! { cx,
        Container(offset_top = true, route = CurrentRoute::About) {
            div(class = "flex flex-col md:flex-row justify-center items-center md:items-start") {
                img(class = "rounded-full h-52 w-52 md:mr-2 mb-4", src = "https://github.com/arctic-hen7.png", alt = "My GitHub profile picture") {}
                div(class = "md:ml-2 p-4 styled-prose max-w-prose", dangerously_set_inner_html = &content.get()) {}
            }
        }

    }
}

#[make_rx(AboutPropsRx)]
struct AboutProps {
    /// The HTML contents of the about page.
    content: String,
}

#[perseus::head]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

#[build_state]
fn get_build_state(_path: String, _locale: String) -> RenderFnResultWithCause<AboutProps> {
    use pulldown_cmark::{html, Options, Parser};

    let md_content = std::fs::read_to_string("about.md")?;

    let md_opts = Options::all();
    let parser = Parser::new_ext(&md_content, md_opts);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    Ok(AboutProps {
        content: html_content,
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about")
        .template(about_page)
        .head(head)
        .build_state_fn(get_build_state)
}
