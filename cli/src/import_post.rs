use crate::parse_post::{is_post_public, parse_frontmatter};
use crate::post::*;
use anyhow::{anyhow, bail, Context, Result};
use chrono::NaiveDate;
use katex::Opts;
use regex::{Captures, Regex};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl FullPost {
    /// Imports a single post from my blog posts directory and converts it to HTML. This returns the post
    /// itself, and the file that contains its information in the blog index.
    ///
    /// This will add the post to the index on-disk!
    pub fn new(path: &Path, blog_dir: &Path, search_dir: &Path) -> Result<(Self, PathBuf)> {
        let contents = fs::read_to_string(path).with_context(|| "failed to read post file")?;
        // Parse the entire post
        let (metadata, html_contents) = parse_frontmatter(contents, true);
        if !is_post_public(&metadata) {
            bail!(
                "File '{}' is not a blog post ready for publication",
                path.to_string_lossy()
            );
        }

        // We know there will always be a title, description, and date
        let title = metadata.get("title").ok_or(anyhow!(
            "File '{}' had no `title` metadatum.",
            path.to_string_lossy()
        ))?;
        let description = metadata.get("description").ok_or(anyhow!(
            "File '{}' had no `description` metadatum.",
            path.to_string_lossy()
        ))?;
        let date = metadata.get("date").ok_or(anyhow!(
            "File '{}' had no `date` metadatum.",
            path.to_string_lossy()
        ))?;

        // Construct the unique identifier from the title (lowercase alphanumeric with hyphens instead of whitespace)
        let id = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace("_", "-"); // Preference
        let id = id.strip_suffix(".md").unwrap_or(&id);
        let id = id.to_string();

        // Now parse the date
        let date = {
            // Dates are of the form `<year>-<month>-<day>`
            let date_parts = date.split('-').collect::<Vec<_>>();
            if date_parts.len() != 3 {
                bail!(
                    "File '{}' had an invalid date (must be of the form `<year>-<month>-<day>`)",
                    path.to_string_lossy()
                );
            }
            NaiveDate::from_ymd_opt(
                date_parts[0].parse::<i32>().context("Invalid year")?,
                date_parts[1].parse::<u32>().context("Invalid month")?,
                date_parts[2].parse::<u32>().context("Invalid day")?,
            )
            .ok_or(anyhow!(
                "File '{}' had an invalid date (must be of the form `<year>-<month>-<day>`)",
                path.to_string_lossy()
            ))?
        };

        // Parse the author
        let author = match metadata.get("author") {
            Some(author) => PostAuthor::from_metadata(author)?,
            // I am the default
            None => PostAuthor::Me,
        };

        // Get the tags
        let tags = metadata.get("tags").ok_or(anyhow!(
            "File '{}' had no blog tags (these should be specified under the `tags` metadatum)",
            path.to_string_lossy()
        ))?;
        let tags = tags
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        // Get the series (if there is one)
        let series: Option<Result<(String, usize)>> = metadata.get("series").map(|series| {
            // Series are of the form `Series name(<index>)`
            let mut series_parts = series.split('(').collect::<Vec<_>>();
            let series_idx = series_parts
                .remove(series_parts.len() - 1)
                .strip_suffix(")")
                .ok_or(anyhow!(
                    "File '{}' has an invalid series specification (must include the index)",
                    path.to_string_lossy()
                ))?;
            let series_name = series_parts.join("(");
            let series_idx = series_idx.parse::<usize>().context(anyhow!(
                "Invalid series index for file '{}'",
                path.to_string_lossy()
            ))?;

            Ok((series_name, series_idx))
        });
        let series = match series {
            Some(v) => Some(v?),
            None => None,
        };

        // TODO Parse links to other posts

        // Parse hash links (since the `<base>` tag will interfere)
        let html_contents = Regex::new(r#"href="\#(.*?)""#)
            .unwrap()
            .replace_all(&html_contents, format!(r#"href="post/{}#$1""#, &id));
        // Process equations with KaTeX on the server-side so they're delivered immediately
        let display_opts = Opts::builder().display_mode(true).build()?;
        let html_contents = Regex::new(r#"(?s)\$\$\n(.*?)\n\$\$"#).unwrap().replace_all(
            &html_contents,
            |caps: &Captures| {
                // We use the whole statement here, otherwise `&` is invalid
                let display_eqn = caps.get(1).unwrap().as_str();
                // We can't handle errors here properly...
                katex::render_with_opts(display_eqn, &display_opts).unwrap()
            },
        );
        let html_contents = Regex::new(r#"\\\((.*?)\\\)"#).unwrap().replace_all(
            &html_contents,
            |caps: &Captures| {
                let inline_eqn = caps.get(1).unwrap().as_str();
                // We can't handle errors here properly...
                katex::render(inline_eqn).unwrap()
            },
        );
        let html_contents =
            Regex::new(r#"\$(.*?)\$"#)
                .unwrap()
                .replace_all(&html_contents, |caps: &Captures| {
                    let inline_eqn = caps.get(1).unwrap().as_str();
                    // We can't handle errors here properly...
                    katex::render(inline_eqn).unwrap()
                });

        // Now construct the post itself
        let post = Post {
            title: title.to_string(),
            id: id.to_string(),
            author,
            description: description.to_string(),
            contents: html_contents.to_string(),
            tags,
            series,
            // TODO Table of contents!
            toc: String::new(),
            date,
        };
        // Store the path relative to the search dir (avoids leaking information)
        // If we got this far, we definitely have a search directory
        // We have a default because we can import from arbitrary paths
        let relative_path = path.strip_prefix(search_dir).unwrap_or(path);
        let full_post = FullPost {
            post,
            source: relative_path.to_path_buf(),
            mtime: fs::metadata(&path)
                .context("Failed to get metadata for source of post file")?
                .modified()
                .context("Failed to get modification time for source of post file")?,
        };
        // This really shouldn't fail...
        let full_post_str = serde_json::to_string(&full_post).unwrap();

        // We'll write into the given directory, with the ID as the file name
        let dest = blog_dir.join(format!("{id}.json"));
        fs::write(&dest, full_post_str).context("Failed to write processed post")?;

        Ok((full_post, dest.to_path_buf()))
    }
}
