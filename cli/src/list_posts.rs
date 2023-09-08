use crate::{
    parse_post::{is_post_public, parse_frontmatter},
    post::*,
};
use anyhow::{anyhow, bail, Context, Result};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

/// A list of all the currently indexed posts for publication.
pub struct PostsList {
    posts: HashMap<String, PathBuf>,
    blog_dir: PathBuf,
}
// Note that functions that take a 'title' here are actually taking a string that has any guest authors in brackets
impl PostsList {
    /// Creates a new list of posts from the given blog directory.
    pub fn new(blog_dir: &Path) -> Result<Self> {
        let mut posts = HashMap::new();
        // We know the blog directory is a flat file system, with each file named by its Org ID
        for entry in fs::read_dir(blog_dir)? {
            let entry = entry?;
            let contents =
                fs::read_to_string(entry.path()).context("Failed to read file in blog index")?;
            let post: FullPost = serde_json::from_str(&contents)
                .context("Failed to deserialize file in blog index")?;

            posts.insert(
                get_post_display(&post.post.title, &post.post.author),
                entry.path(),
            );
        }

        Ok(Self {
            posts,
            blog_dir: blog_dir.to_path_buf(),
        })
    }
    /// Detects any changes to currently indexed posts, returning a list of them for the user to choose to act on
    /// individually.
    pub fn detect_changes(&self, search_dir: &Path) -> Result<Vec<String>> {
        let mut modified = Vec::new();

        for (display, path) in self.posts.iter() {
            // Get the full post details
            let contents = fs::read_to_string(path).context("Failed to read file in blog index")?;
            let post: FullPost = serde_json::from_str(&contents)
                .context("Failed to deserialize file in blog index")?;

            // Now check if the source file has changed on disk by modification time
            // Some posts are legacy from the old system, we won't check those
            if post.source != Path::new(".") {
                let full_path = search_dir.join(&post.source);
                let curr_mtime = fs::metadata(&full_path)
                    .context("Failed to get metadata for source of post file")?
                    .modified()
                    .context("Failed to get modification time for source of post file")?;
                let last_known_mtime = post.mtime;
                if last_known_mtime < curr_mtime {
                    modified.push(display.to_string());
                }
            }
        }

        Ok(modified)
    }
    /// Detects any new posts in the given directory, which is assumed to be a flat file system.
    ///
    /// This returns a list of display names for posts, and their source file paths. This function parses all
    /// the Org files in the gioven search directory, but does not attempt to convert any to posts.
    pub fn detect_new(&self, search_dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        let mut new = Vec::new();
        for entry in fs::read_dir(search_dir)? {
            let entry = entry?;
            // Rule out any directories or non-Org files
            if entry
                .file_type()
                .context("Failed to get file type of original markdown file")?
                .is_dir()
                || entry.path().extension() != Some(OsStr::new("md"))
            {
                continue;
            }
            let contents =
                fs::read_to_string(&entry.path()).with_context(|| "failed to read post file")?;
            // Parse the frontmatter (not the content)
            let (metadata, _) = parse_frontmatter(contents, false);
            // Check if it's ready for publication
            if is_post_public(&metadata) {
                let title = metadata.get("title").ok_or(anyhow!(
                    "Post file '{}' has no title",
                    entry.path().to_string_lossy()
                ))?;
                let author = match metadata.get("author") {
                    Some(author) => PostAuthor::from_metadata(author)?,
                    // I am the default
                    None => PostAuthor::Me,
                };
                let display = get_post_display(&title, &author);

                // Now make sure we don't already have it
                if !self.posts.contains_key(&display) {
                    new.push((display, entry.path()))
                }
            }
        }

        Ok(new)
    }
    /// Removes a given post by its title.
    ///
    /// This will NOT alter the source of the post, it will only delete it from the index.
    pub fn remove_post(&mut self, title: &str) -> Result<()> {
        // Get the path and remove it from this list at the same time (if it exists)
        let path = self.posts.remove(title);
        if let Some(path) = path {
            // Remove the actual file
            fs::remove_file(&path).context("Failed to remove specified post")?;

            Ok(())
        } else {
            bail!("No post exists in the current posts list by the title '{}' (filesystem modification?)", title);
        }
    }
    /// Adds the post at the given path on the filesystem.
    pub fn add_post(&mut self, path: &Path, search_dir: &Path) -> Result<()> {
        // Just import the file
        let (post, index_path) = FullPost::new(path, &self.blog_dir, search_dir)?;
        let display = get_post_display(&post.post.title, &post.post.author);
        self.posts.insert(display, index_path);
        Ok(())
    }
    /// Updates a given post by its title by re-importing it (this may lead to the title changing).
    pub fn update_post(&mut self, title: &str, search_dir: &Path) -> Result<()> {
        // Get the post itself (removing it, since we'll re-add it completely, potentially with a new title)
        let index_path = self.posts.remove(title);
        if let Some(index_path) = index_path {
            let contents =
                fs::read_to_string(index_path).context("Failed to read file in blog index")?;
            let post: FullPost = serde_json::from_str(&contents)
                .context("Failed to deserialize file in blog index")?;

            // And now add it back as usual!
            self.add_post(&post.source, search_dir)
        } else {
            bail!("No post exists in the current posts list by the title '{}' (filesystem modification?)", title);
        }
    }
}
// For showing the list to the user in the terminal
impl std::fmt::Display for PostsList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let titles = self.posts.keys().map(|s| s.as_str()).collect::<Vec<&str>>();
        let titles = titles.join("\n");
        write!(f, "{}", titles)
    }
}

/// Gets the string to display to the user about a post.
fn get_post_display(title: &str, author: &PostAuthor) -> String {
    format!(
        "{}{}",
        title,
        // Only display the author if it's not me
        if let PostAuthor::Guest { name, .. } = &author {
            format!(" ({})", name)
        } else {
            String::new()
        }
    )
}
