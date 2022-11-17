use sycamore::prelude::*;

static COPYRIGHT_YEARS: &str = "2021-2022";

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    let menu_open = create_signal(cx, false);
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    view! { cx,
        header(
            class = "sm:p-2 w-full mb-20 backdrop-blur-lg {}",
        ) {
            div(class = "flex justify-between xl:justify-center items-center") {
                a(class = "justify-self-start self-center m-3 ml-5 text-lg sm:text-xl text-bold title-font", href = "/") {
                    "The Arctic Site"
                }
                // Carefully chosen to avoid glitches
                div(class = "xl:w-[38rem] 2xl:w-[55rem] 3xl:w-[70rem]") {}
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
                            class = "tham-inner bg-white"
                        ) {}
                    }
                }
                // This displays the navigation links on desktop
                nav(class = "hidden md:flex") {
                    ul(class = "mr-5 flex") {
                        NavLinks(props.route)
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
                    NavLinks(props.route)
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
            class = "w-full flex justify-center py-5 bg-black"
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
    route: CurrentRoute,
}

#[component]
fn NavLinks<G: Html>(cx: Scope, curr_route: CurrentRoute) -> View<G> {
    view! { cx,
        (
            if curr_route != CurrentRoute::Home {
                view! { cx,
                    li(class = "m-3 p-1") {
                        a(href = "") { "Home" }
                    }
                }
            } else {
                View::empty()
            }
        )
        (
            if curr_route != CurrentRoute::About {
                view! { cx,
                    li(class = "m-3 p-1") {
                        a(href = "about") { "About" }
                    }
                }
            } else {
                View::empty()
            }
        )
        // We always want to be able to go back to the index of posts
        li(class = "m-3 p-1") {
            a(href = "posts") { "The Arctic Circle" }
        }
        (
            if curr_route != CurrentRoute::Shortform {
                view! { cx,
                    li(class = "m-3 p-1") {
                        a(href = "shortform") { "The Ice Floes" }
                    }
                }
            } else {
                View::empty()
            }
        )
    }
}

/// The templates in the app, just so we know which link to skip in the header.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CurrentRoute {
    Home,
    About,
    BlogPost,
    Shortform,
    Series,
    Tag,
    Contact,
}
