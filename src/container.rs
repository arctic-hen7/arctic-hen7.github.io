use crate::global_state::AppStateRx;
use perseus::reactor::Reactor;
use sycamore::prelude::*;

static COPYRIGHT_YEARS: &str = "2021-2022";

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    let menu_open = create_signal(cx, false);
    let toggle_menu = |_| menu_open.set(!*menu_open.get());

    // Global state isn't accessible in error views, but we sometimes use global state there
    let show_gong = !if props.route != CurrentRoute::NotFound {
        let global_state: &AppStateRx = Reactor::<G>::from_cx(cx).get_global_state(cx);
        *global_state.woke_up_on_time.get()
    } else {
        // Never display the gong in error views (the pages will cover my humiliation enough)
        false
    };

    // The gong message takes up some room in the header, so our offset styles have to be adjusted a little
    let offset_style = if show_gong {
        "mt-24"
    } else {
        "mt-14 xs:mt-16 sm:mt-20 lg:mt-24"
    };

    view! { cx,
        header(
            class = "w-full mb-20 backdrop-blur-lg {}",
        ) {
            // (if show_gong {
            //     view! {
            //         cx,
            //         div(class = "bg-red-400 text-white w-full flex justify-center p-1 px-2 font-bold font-mono") {
            //             p {
            //                 "Error: I did not wake up on time today! "
            //                 a(
            //                     href = "post/4726b90d-613e-4a45-b419-44cd6ec887a4",
            //                     class = "underline"
            //                 ) { "Learn more" }
            //                 "."
            //             }
            //         }
            //     }
            // } else {
            //     View::empty()
            // })

            div(class = "sm:p-2 flex justify-between xl:justify-center items-center") {
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
                    "sm:p-2 md:hidden w-full text-center justify-center {}",
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
                offset_style
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
    NotFound,
}
