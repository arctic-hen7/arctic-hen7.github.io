use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod create_shortform;
mod delete_shortform;
mod list_shortform;

pub use create_shortform::create_shortform;

use uuid::Uuid;

/// A representation of a single shortform message.
#[derive(Serialize, Deserialize)]
pub struct Shortform {
    /// A UUID for the post, generated as a version 4 UUID from random data.
    pub id: Uuid,
    /// The actual contents of the post, in HTML.
    pub content: String,
    /// The time at which the post was made, in UTC time.
    pub time: DateTime<Utc>,
    // TODO Other fields, such as hashtags and @ references
}

use std::env;
use anyhow::{Result, Context};
use octocrab::Octocrab;

/// Initializes the GitHub client with authentication. Once this is called, it
/// will insert itself into a static store. This should only be called once.
pub async fn get_client() -> Result<()> {
    let token = env::var("GITHUB_TOKEN")
        .context(
            "Personal access token for GitHub must be specified through `GITHUB_TOKEN` environment variable (expected to be in `.env`)"
        )?;

    let octocrab = Octocrab::builder()
        .personal_token(token);

    octocrab::initialise(octocrab)?;

    Ok(())
}

