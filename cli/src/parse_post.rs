use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;

/// Checks from the given post metadata if the post is intended for publication.
pub(crate) fn is_post_public(metadata: &HashMap<String, String>) -> bool {
    metadata
        .get("public")
        .map(|val| val == "true")
        .unwrap_or(false)
}

/// Parses a Markdown post file's frontmatter to get metadata about it.
pub(crate) fn parse_frontmatter(
    file_contents: String,
    parse_contents: bool,
) -> (HashMap<String, String>, String) {
    let mut frontmatter = HashMap::new();
    let mut contents = String::new();

    let mut lines = file_contents.lines();
    // Advance into the frontmatter, which must be at the top
    if lines.next() != Some("---") {
        panic!("no frontmatter found in post");
    }

    // Parse frontmatter
    let mut in_frontmatter = true;
    for line in lines {
        if line == "---" {
            in_frontmatter = false;
            continue;
        }

        if in_frontmatter {
            if line.contains(":") {
                let line = line.split(':').collect::<Vec<_>>();
                let key = line[0].trim().to_string();
                let val = line[1].trim().to_string();

                frontmatter.insert(key, val);
            }
        } else {
            // Remaining contents of file
            contents.push_str(line);
            contents.push('\n');
        }
    }

    let contents = if parse_contents {
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&contents, opts);
        let mut html_contents = String::new();
        html::push_html(&mut html_contents, parser);

        html_contents
    } else {
        String::new()
    };

    (frontmatter, contents)
}
