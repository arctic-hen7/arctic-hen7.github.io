use crate::container::{Container, CurrentRoute};
#[cfg(engine)]
use crate::Error;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use self::resume::Time;

#[auto_scope]
fn about_page<G: Html>(cx: Scope, AboutStateRx { resume }: &AboutStateRx) -> View<G> {
    // When iterating over a reactive collection, you'll usually need to do this
    // to integrate all the lifetimes together
    let areas = create_ref(cx, resume.areas.get());
    let resume = View::new_fragment(
        areas.iter()
            .map(|area| {
                let projects = create_ref(cx, area.projects.get());
                let projects = View::new_fragment(
                    projects.iter()
                        .map(|project| {
                            let achievements = create_ref(cx, project.achievements.get());
                            let achievements = View::new_fragment(
                                achievements.iter()
                                    .map(|achievement| {
                                        view! { cx,
                                            // These can have styling, and they all come from
                                            // a trusted source
                                            li(class = "ml-7") {
                                                span(dangerously_set_inner_html = &achievement.get()) {}
                                            }
                                        }
                                    })
                                    .collect()
                            );

                            view! { cx,
                                div(class = "flex justify-between items-center") {
                                    a(class = "text-xl text-blue-300 hover:text-blue-200 transition-colors duration-150", href = project.link.get()) {
                                        span { ((project.name.get())) }
                                        span(class = "italic") { (format!(" ({})", project.status.get())) }
                                    }
                                    p(class = "ml-1") { Time(&project.time.get()) }
                                }
                                h4(class = "text-lg text-neutral-300 italic") { (project.role.get()) }
                                p(class = "", dangerously_set_inner_html = &project.description.get()) {}
                                ul(class = "list-disc") {
                                    (achievements)
                                }
                            }
                        })
                        .collect()
                );
                view! { cx,
                    div(class = "my-3") {
                        // Header
                        div(class = "flex justify-between items-end border-b border-white px-4") {
                            h3(class = "text-2xl") {
                                (area.name.get())
                            }
                            p(class = "ml-1") { Time(&area.time.get()) }
                        }
                        // Wrapper to maintain the spacing so the broder continues out
                        div(class = "px-4") {
                            h4(class = "text-neutral-300 italic text-xl") { (area.role.get()) }
                            p(class = "", dangerously_set_inner_html = &area.description.get()) {}

                            // Projects
                            div {
                                (projects)
                            }
                        }
                    }
                }
            })
            .collect()
    );

    view! { cx,
        Container(offset_top = true, route = CurrentRoute::About) {
            div(class = "flex flex-col md:flex-row justify-center items-center md:items-start") {
                img(class = "rounded-full h-52 w-52 ml-4 md:mr-2 mb-4", src = "https://github.com/arctic-hen7.png", alt = "My GitHub profile picture") {}
                div(class = "md:ml-2 p-4 max-w-prose") {
                    p {
                        "Hey there! I'm "
                        b { i { "Sam Brew" } }
                        " ("
                        i { "arctic-hen7" }
                        ") a "
                        b { "fullstack web developer" }
                        ", "
                        b { "distributed systems designer" }
                        ", and budding "
                        b { "economist" }
                        "! This page is where you can learn more about me, and the things I've done so far in my short life on this Earth. I provide my full resumé externally to this website as required."
                    }
                    div(class = "mt-4") {
                        (resume)
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "AboutStateRx")]
struct AboutState {
    /// My resumé.
    #[rx(nested)]
    resume: resume::Resume,
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Me | The Arctic Site" }
    }
}

#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> Result<AboutState, BlamedError<Error>> {
    let resume = resume::Resume::from_directory("resume").map_err(Error::from)?;

    Ok(AboutState { resume })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("about")
        .view_with_state(about_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

mod resume {
    #[cfg(engine)]
    use crate::Error;
    use perseus::prelude::*;
    use perseus::state::rx_collections::{RxVec, RxVecNested};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use sycamore::prelude::*;

    #[derive(Serialize, Deserialize, Clone, ReactiveState)]
    pub struct Resume {
        #[rx(nested)]
        pub areas: RxVecNested<ResumeArea>,
    }

    #[derive(Serialize, Deserialize, Clone, ReactiveState)]
    pub struct ResumeArea {
        pub name: String,
        pub role: String,
        pub description: String,
        pub time: (ResumeTime, ResumeTime),
        #[rx(nested)]
        pub projects: RxVecNested<ResumeProject>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum ResumeTime {
        // This has to be a string because of TOML parsing requirements unfortunately
        Year(String),
        Present,
    }

    #[component]
    pub fn Time<G: Html>(cx: Scope, time: &(ResumeTime, ResumeTime)) -> View<G> {
        let start = match &time.0 {
            ResumeTime::Present => "Present".to_string(),
            ResumeTime::Year(year) => year.to_string(),
        };
        let end = match &time.1 {
            ResumeTime::Present => "Present".to_string(),
            ResumeTime::Year(year) => year.to_string(),
        };

        view! { cx,
            span {
                (start)
                span(dangerously_set_inner_html = "&ndash;") {}
                (end)
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, ReactiveState)]
    pub struct ResumeProject {
        pub name: String,
        pub role: String,
        pub status: String,
        pub time: (ResumeTime, ResumeTime),
        pub link: String,
        pub description: String,
        #[rx(nested)]
        pub achievements: RxVec<String>,
    }

    #[cfg(engine)]
    impl Resume {
        // This will not examine subdirectories
        pub fn from_directory(path: &str) -> Result<Self, Error> {
            let mut areas = Vec::new();

            for entry in fs::read_dir(path).map_err(Error::from)? {
                let entry = entry.map_err(Error::from)?;

                if entry.metadata().map_err(Error::from)?.is_file()
                    && entry.file_name().to_string_lossy().ends_with(".toml")
                {
                    let contents = fs::read_to_string(entry.path()).map_err(Error::from)?;
                    let parsed: ResumeArea = toml::from_str(&contents).map_err(Error::from)?;
                    areas.push(parsed);
                }
            }

            Ok(Self {
                areas: areas.into(),
            })
        }
    }
}
