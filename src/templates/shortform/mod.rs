mod skeleton;
#[cfg(target_arch = "wasm32")]
mod root;
#[cfg(target_arch = "wasm32")]
mod single;
mod error;
mod page;

use chrono::{DateTime, Utc};
use perseus::prelude::*;
use sycamore::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

use error::ShortformError;

#[perseus::make_rx(ShortformListRx)]
pub struct ShortformList {
    /// A list of shortforms, indexed by their IDs.
    ///
    /// This will be `None` until it's populated by the client.
    /// In a serverful setup, this could be populated from the server-side.
    list: Option<HashMap<String, Shortform>>,
    /// Errors applying to the fetching of all posts. These will be displayed
    /// in error boxes above the posts themselves.
    errors: Vec<ShortformError>
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
    // TODO Other fields, such as hashtags and @ references
}

#[perseus::head]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { ("The Ice Floes | The Arctic Circle") }
    }
}

// This exists solely to initialise the shortform list as empty
#[perseus::build_state]
fn get_build_state(_: String, _: String) -> RenderFnResultWithCause<ShortformList> {
    Ok(ShortformList {
        list: None,
        errors: Vec::new()
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("shortform")
        .template(page::shortform_page)
        .head(head)
        .build_state_fn(get_build_state)
}
