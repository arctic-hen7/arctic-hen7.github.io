use std::{fs, path::{PathBuf, Path}};
use anyhow::{Context, Result, anyhow, bail};
use katex::Opts;
use regex::{Captures, Regex};
use std::process::Command;
use std::io::Write;
use crate::{parse_post::OrgFile, post::*};

impl FullPost {
    /// Imports a single post from my Zettelkasten in Org Roam and converts it to HTML. This function could easily take quite a while.
    /// This returns the post itself, and the file that contains its information in the blog index.
    ///
    /// This will add the post to the index on-disk!
    pub fn new(file: &OrgFile, blog_dir: &Path) -> Result<(Self, PathBuf)> {
        if !file.is_post() {
            bail!("File '{}' is not a blog post ready for publication", file.path.to_string_lossy());
        }

        // We know there will always be an ID property and a title metadatum
        let id = file.properties.get("ID").ok_or(anyhow!("File '{}' had no `ID` property.", file.path.to_string_lossy()))?;
        let title = file.metadata.get("title").ok_or(anyhow!("File '{}' had no `title` metadatum.", file.path.to_string_lossy()))?;

        // Parse the author
        let author = match file.metadata.get("author") {
            Some(author) => PostAuthor::from_metadata(author)?,
            // I am the default
            None => PostAuthor::Me,
        };

        // Get the tags
        let tags = file.properties.get("BLOG_TAGS").ok_or(anyhow!("File '{}' had no blog tags (these should be specified under the `BLOG_TAGS` property)", file.path.to_string_lossy()))?;
        let tags = tags.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        // Get the series (if there is one)
        let series: Option<Result<(String, usize)>> = file.properties.get("BLOG_SERIES").map(|series| {
            // Series are of the form `Series name(<index>)`
            let mut series_parts = series.split('(').collect::<Vec<_>>();
            let series_idx = series_parts.remove(series_parts.len() - 1).strip_suffix(")").ok_or(anyhow!("File '{}' has an invalid series specification (must include the index)", file.path.to_string_lossy()))?;
            let series_name = series_parts.join("(");
            let series_idx = series_idx.parse::<usize>().context(anyhow!("Invalid series index for file '{}'", file.path.to_string_lossy()))?;

            Ok((series_name, series_idx))
        });
        let series = match series {
            Some(v) => Some(v?),
            None => None,
        };

        // Now process the contents into HTML using Org Mode (this will take a while!)
        #[cfg(unix)]
        let shell_exec = "sh";
        #[cfg(windows)]
        let shell_exec = "powershell";
        #[cfg(unix)]
        let shell_param = "-c";
        #[cfg(windows)]
        let shell_param = "-command";
        // This will NOT pipe output/errors to the console
        let command = format!(
            // WARNING: You will need this in your `$PATH`!
            r#"export-blog-post "{}""#,
            file.path.canonicalize().unwrap().to_string_lossy()
        );
        let output = Command::new(shell_exec)
            .args([shell_param, &command])
            .output()
            .context("Failed to execute Org HTML export command")?;

        let exit_code = match output.status.code() {
            Some(exit_code) => exit_code,         // If we have an exit code, use it
            None if output.status.success() => 0, /* If we don't, but we know the command succeeded, */
            // return 0 (success code)
            None => 1, /* If we don't know an exit code but we know that the command failed, return 1
             * (general error code) */
        };
        // Print `stderr` and `stdout` only if there's something therein and the exit
        // code is non-zero
        if !output.stderr.is_empty() && exit_code != 0 {
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
        if !output.stdout.is_empty() && exit_code != 0 {
            std::io::stdout().write_all(&output.stdout).unwrap();
        }

        // Get the expected name of the HTML file
        let mut html_file = file.path.to_path_buf();
        html_file.set_extension("html");

        let html_contents = fs::read_to_string(&html_file).context("Failed to read converted HTML file")?;
        // We don't need any of the `<head>` in that generated file, so just grab the body
        let html_parts = html_contents.trim().split(r#"<div id="content" class="content">"#).collect::<Vec<_>>(); // Yes, we need this `.trim()`!
        let body = html_parts[1].strip_suffix("</div>\n</body>\n</html>").unwrap().trim();
        // Parse links
        // Links are of the form `<a href="[filename]#ID-[post-id]">[title]</a>`
        let body = Regex::new(r#"<a(.*?)href=".*?#ID-(.*?)"(.*?)>"#)
            .unwrap()
            .replace_all(&body, r#"<a${1}href="posts/$2"$3>"#);
        // Parse hash links (since the `<base>` tag will interfere)
        let body = Regex::new(r#"href="\#(.*?)""#)
            .unwrap()
            .replace_all(&body, format!(r#"href="posts/{}#$1""#, &id));
        // And take the table of contents out
        let toc = Regex::new(r#"(?s)<div id="table-of-contents".*?</ul>\n</div>\n</div>"#)
            .unwrap()
            .find(&body);
        let (toc, body) = match toc {
            Some(toc) => {
                let toc = toc.as_str();
                // Delete it from the current body text
                (toc, body.replace(toc, ""))
            },
            // If there were no sections, that's fine, we just won't have a table of contents
            None => ("<div></div>", body.to_string())
        };
        // Remove the erroneous title from the table of contents (its positioning makes its role clear)
        let toc = toc.replace("<h2>Table of Contents</h2>", "");
        // Remove the erroneous title
        let body = Regex::new(r#"(?s)<h1 class="title">.*</h1>"#)
            .unwrap()
            .replace_all(&body, "");
        // Process equations with KaTeX on the server-side so they're delivered immediately
        let display_opts = Opts::builder()
            .display_mode(true)
            .build()?;
        let body = Regex::new(r#"(?s)\\begin\{align\*\}\n(.*?)\n\\end\{align\*\}"#)
            .unwrap()
            .replace_all(&body, |caps: &Captures| {
                // We use the whole statement here, otherwise `&` is invalid
                let display_eqn = caps.get(0).unwrap().as_str();
                // We can't handle errors here properly...
                katex::render_with_opts(display_eqn, &display_opts).unwrap()
            });
        let body = Regex::new(r#"\\\((.*?)\\\)"#)
            .unwrap()
            .replace_all(&body, |caps: &Captures| {
                let inline_eqn = caps.get(1).unwrap().as_str();
                // We can't handle errors here properly...
                katex::render(inline_eqn).unwrap()
            });
        let body = Regex::new(r#"\$(.*?)\$"#)
            .unwrap()
            .replace_all(&body, |caps: &Captures| {
                let inline_eqn = caps.get(1).unwrap().as_str();
                // We can't handle errors here properly...
                katex::render(inline_eqn).unwrap()
            });
        // There will probably be some leftover newlines at the start or end
        let body = body.trim();

        // Delete the HTMl file so they don't glut up my Zettelkasten folder (which is inside a Git repo)
        fs::remove_file(html_file).context("Failed to remove converted HTML file")?;

        // Now construct the post itself
        let post = Post {
            title: title.to_string(),
            id: id.to_string(),
            author,
            contents: body.to_string(),
            tags,
            series,
            toc: toc.to_string(),
        };
        let full_post = FullPost {
            post,
            source: file.path.to_path_buf(),
            mtime: fs::metadata(&file.path).context("Failed to get metadata for source of post file")?.modified().context("Failed to get modification time for source of post file")?
        };
        // This really shouldn't fail...
        let full_post_str = serde_json::to_string(&full_post).unwrap();

        // We'll write into the given directory, with the Org ID as the file name
        let dest = blog_dir.join(format!("{id}.json"));
        fs::write(&dest, full_post_str).context("Failed to write processed post")?;

        Ok((full_post, dest.to_path_buf()))
    }
}
