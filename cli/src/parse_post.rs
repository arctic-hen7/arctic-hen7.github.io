use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A representation of an Org Mode file, fully parsed for checking if it's a post.
pub struct OrgFile {
    pub properties: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
    pub contents: String,
    pub path: PathBuf,
}
impl OrgFile {
    /// Parses the given file into this form.
    pub fn new(file: &Path) -> Result<Self> {
        let contents_str = fs::read_to_string(file).context("Failed to read original Org file")?;
        // Iterate over the lines to break the document into metadata and non-metadata
        let mut properties: HashMap<String, String> = HashMap::new(); // Stuff in the `:PROPERTIES:` drawer at the top
        let mut metadata: HashMap<String, String> = HashMap::new(); // Stuff with `#+` at the start
        let mut contents = Vec::new(); // Everything else
                                       // We always start with the properties drawer
        let mut file_loc = FileLocation::Properties;
        for line in contents_str.lines() {
            // If we've moved on to the contents, then we should do no further processing
            if file_loc == FileLocation::Contents {
                contents.push(line);
            }
            // Every other part of this conditional is non-content
            // `:PROPERTIES:` will only appear in the file head as the first line
            else if line.is_empty() || line == ":PROPERTIES:" {
                continue;
            }
            // Handle the end of the properties drawer (which is clearly delimited)
            else if line == ":END:" {
                // Move on to the metadata
                file_loc = FileLocation::Metadata;
            }
            // Deal with any comments in the properties/metadata
            else if line.starts_with("# ") {
                continue;
            } else if file_loc == FileLocation::Properties {
                // Form: `:KEY:  VALUE`
                let mut parts = line.split(':').collect::<Vec<_>>();
                parts.remove(0); // Meaningless
                let key = parts.remove(0);
                let val = parts.join(":");
                properties.insert(key.to_string(), val.trim().to_string());
            } else if file_loc == FileLocation::Metadata {
                // Handle the possibility that we've finished with the metadata
                if !line.starts_with("#+") {
                    file_loc = FileLocation::Contents;
                    continue;
                }
                // Form: `#+key: value`
                let mut parts = line.split(':').collect::<Vec<_>>();
                let key = parts.remove(0).strip_prefix("#+").unwrap();
                let val = parts.join(":").to_string();
                metadata.insert(key.to_string(), val.trim().to_string());
            }
        }
        // Make sure the file ended up being valid (i.e. that we had properties, metadata, and contents)
        if file_loc != FileLocation::Contents {
            bail!("Properties/metadata of '{}' never terminated (please review the file's formatting)", file.to_string_lossy());
        }

        Ok(Self {
            properties,
            metadata,
            contents: contents.join("\n"),
            path: file.to_path_buf(),
        })
    }
    /// Checks if this file is a blog post for publication.
    pub fn is_post(&self) -> bool {
        // Blog posts all have the `blog` file tag
        let filetags = match self.metadata.get("filetags") {
            Some(filetags) => filetags,
            None => return false,
        };
        // File tags are of the form `:tag1:tag2:tag3:`
        // We want a blog post that isn't a work-in-progress
        filetags.contains(":blog:") && !filetags.contains(":wip:")
    }
}

/// A representation of the different parts of a file, for tracking where we are in processing.
#[derive(PartialEq, Eq)]
enum FileLocation {
    Properties,
    Metadata,
    Contents,
}
