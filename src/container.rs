use sycamore::prelude::*;

static COPYRIGHT_YEARS: &str = "2021-2022";

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    let menu_open = create_signal(cx, false);
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    view! { cx,
        header(
            class = "shadow-md sm:p-2 w-full mb-20 backdrop-blur-lg {}",
        ) {
            div(class = "flex justify-between items-center") {
                a(class = "justify-self-start self-center m-3 ml-5 text-md sm:text-2xl text-bold title-font text-white", href = "/") {
                    "The Arctic Circle"
                }
                // The button for opening/closing the hamburger menu on mobile
                // This is done by a Tailwind module
                div(
                    class = format!(
                        "md:hidden m-3 mr-5 tham tham-e-spin tham-w-6 {}",
                        if *menu_open.get() {
                            "tham-active"
                        } else {
                            ""
                        }
                    ),
                    on:click = toggle_menu
                ) {
                    div(class = "tham-box") {
                        div(
                            class = "tham-inner"
                        ) {}
                    }
                }
                // This displays the navigation links on desktop
                nav(class = "hidden md:flex") {
                    ul(class = "mr-5 flex") {
                        NavLinks()
                    }
                }
            }
            // This displays the navigation links when the menu is opened on mobile
            // TODO Click-away event
            nav(
                id = "mobile_nav_menu",
                class = format!(
                    "md:hidden w-full text-center justify-center {}",
                    if *menu_open.get() {
                        "flex flex-col"
                    } else {
                        "hidden"
                    }
                )
            ) {
                ul(class = "mr-5") {
                    NavLinks()
                }
            }
        }
        main(
            class = if props.offset_top {
                "mt-14 xs:mt-16 sm:mt-20 lg:mt-24"
            } else { "" }
        ) {
            (children)
        }
        footer(
            class = "w-full flex justify-center py-5 bg-black text-white"
        ) {
            p(class = "mx-5 text-center") {
                (format!("© arctic-hen7 {}", COPYRIGHT_YEARS))
            }
        }
    }
}

#[derive(Prop)]
pub struct ContainerProps<'a, G: Html> {
    children: Children<'a, G>,
    offset_top: bool,
}

#[component]
fn NavLinks<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        li(class = "m-3 p-1") {
            a(href = "about") { "About" }
        }
    }
}
