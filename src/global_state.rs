use perseus::{state::GlobalStateCreator, RenderFnResult};
use std::collections::HashMap;
use crate::BLOG_DIR;
use crate::post::*;

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

/// The global state for the app, which mostly involves
#[perseus::make_rx(AppStateRx)]
pub struct AppState {
    /// A map of each tag to all the posts that include it.
    pub tags_map: HashMap<String, Vec<String>>,
    // /// A map of each post series to all the posts that are a part of it, in order.
    // pub series_map: HashMap<String, Vec<String>>
}

#[perseus::global_build_state]
pub async fn get_build_state() -> RenderFnResult<AppState> {
    use std::fs;
    use anyhow::Context;

    let mut tags_map: HashMap<String, Vec<String>> = HashMap::new();
    // Get everything in the blog directory (which just has flat files, indexed by Org ID)
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path()).context("Failed to read file in blog index")?;
        let full_post: FullPost = serde_json::from_str(&contents).context("Failed to deserialize file in blog index")?;
        let post = full_post.post; // We don't need any of the filesystem data

        // Go through all the tags, and add this post to each one in the tags map
        for tag in post.tags.into_iter() {
            if let Some(posts_for_tag) = tags_map.get_mut(&tag) {
                // There are already some other posts for this tag
                posts_for_tag.push(post.title.clone())
            } else {
                // No other post has this tag yet
                tags_map.insert(tag, vec![ post.title.clone() ]);
            }
        }
    };

    Ok(AppState {
        tags_map,
    })
}
