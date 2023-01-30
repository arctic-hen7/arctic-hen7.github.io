mod error;
mod page;
#[cfg(client)]
mod root;
#[cfg(client)]
mod single;
mod skeleton;

use chrono::{DateTime, Utc};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(engine)]
use sycamore::prelude::*;
use uuid::Uuid;

use error::ShortformError;

use crate::post::PostAuthor;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "ShortformListRx")]
pub struct ShortformList {
    /// A list of shortforms, indexed by their IDs.
    ///
    /// This will be `None` until it's populated by the client.
    /// In a serverful setup, this could be populated from the server-side.
    list: Option<HashMap<String, Shortform>>,
    /// Errors applying to the fetching of all posts. These will be displayed
    /// in error boxes above the posts themselves.
    errors: Vec<ShortformError>,
}

/// A representation of a single shortform message.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Shortform {
    /// A UUID for the post, generated as a version 4 UUID from random data.
    pub id: Uuid,
    /// The actual contents of the post, in HTML.
    pub content: String,
    /// The time at which the post was made, in UTC time.
    pub time: DateTime<Utc>,
    /// The writer of the post.
    pub author: PostAuthor,
    // TODO Other fields, such as hashtags and @ references
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { ("The Ice Floes | The Arctic Site") }
    }
}

// This exists solely to initialise the shortform list as empty
#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> ShortformList {
    ShortformList {
        list: None,
        errors: Vec::new(),
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("shortform")
        .view_with_state(page::shortform_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}
