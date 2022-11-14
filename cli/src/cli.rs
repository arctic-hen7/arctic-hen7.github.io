use std::{io::{self, Write}, path::{Path, PathBuf}};
use anyhow::Result;
use futures::executor::block_on;

use crate::{list_posts::PostsList, shortform::{create_shortform, delete_shortform, list_shortforms}};

static HELP_MENU: &str = r#"You can run any of the following commands:
help                    Prints this help page
blog detect new         Detects any new blog posts
blog detect changes     Detects any changed blog posts
blog import file        Imports and arbitrary file as a blog post
shortform new           Creates a new shortform post
shortform delete        Deletes an existing shortform post
shortform list          Lists all shortform posts

(To leave, just press C-C.)
"#;

/// The current state of the CLI.
pub struct CliState {
    /// An internal list of posts for actually working with the blog post system.
    posts_list: PostsList,
    /// The directory to search for posts in.
    search_dir: PathBuf,
}
impl CliState {
    /// Instantiates a new CLI state.
    pub fn new(blog_dir: impl AsRef<Path>, search_dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            posts_list: PostsList::new(blog_dir.as_ref())?,
            search_dir: search_dir.as_ref().to_path_buf()
        })
    }
    /// Starts the CLI off with an initial prompt.
    pub fn start(&mut self) -> Result<()> {
        let input = self.prompt("command")?;
        // This will start a cycle that will terminate once the command is complete, in which case we should restart
        self.parse_cmd(&input)?;
        self.start()
    }
    /// Present the given prompt to the user and waits for their input, returning it.
    pub fn prompt(&mut self, prompt: &str) -> Result<String> {
        print!("{} > ", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)?;
        let input = input.trim();
        Ok(input.to_string())

    }
    /// Parses the given user input as a command. This should only be called when no other operation is in progress.
    fn parse_cmd(&mut self, cmd: &str) -> Result<()> {
        match cmd {
            // No continued conversation from help
            "help" => Self::show_help(),
            "blog detect new" => {
                let new = self.posts_list.detect_new(&self.search_dir)?;
                let list = new
                    .into_iter()
                    .map(|(display, path)| (display, ListData::Path(path)))
                    .collect::<Vec<_>>();
                let selected = self.list(list, true)?;
                for data in selected.into_iter() {
                    let path = match data {
                        ListData::Path(path) => path,
                        // We know what we put in
                        _ => unreachable!(),
                    };
                    self.posts_list.add_post(&path)?;
                }
            },
            "blog detect changes" => {
                let new = self.posts_list.detect_changes()?;
                let list = new
                    .into_iter()
                    // We can use the display as an index for updating
                    .map(|display| (display.to_string(), ListData::String(display)))
                    .collect::<Vec<_>>();
                let selected = self.list(list, true)?;
                for data in selected.into_iter() {
                    let display = match data {
                        ListData::String(display) => display,
                        // We know what we put in
                        _ => unreachable!(),
                    };
                    self.posts_list.update_post(&display)?;
                }
            },
            "blog import file" => {
                let path = self.prompt("path")?;
                self.posts_list.add_post(Path::new(&path))?;
            },
            "shortform new" => {
                // Prompt the user for contents (Markdown)
                let contents = self.prompt("markdown contents")?;
                let author = self.prompt("author (default: you)")?;
                let author_form = if author.is_empty() {
                    "arctic-hen7".to_string()
                } else {
                    let input = self.prompt("github username or home url")?;
                    if input.starts_with("http") {
                        // We have a home URL, so we'll need a profile picture as well
                        let profile_pic_url = self.prompt("profile picture url (default: none)")?;
                        if profile_pic_url.is_empty() {
                            format!("{} ({})", &author, &input)
                        } else {
                            format!("{} ({}\\{})", &author, &input, &profile_pic_url)
                        }
                    } else {
                        // We have a GH username
                        format!("{} ({})", &author, &input)
                    }
                };
                block_on(create_shortform(
                    &contents,
                    &author_form,
                    // "Arctic Hen (https://example.com\\https://github.com/arctic-hen7.png)"
                ))?;
            },
            "shortform list" => {
                let (list, _sha) = block_on(list_shortforms())?;
                let list = list
                    .into_iter()
                    .map(|shortform| (format!("{}", shortform.content), ListData::String(shortform.id.to_string())))
                    .collect::<Vec<_>>();
                self.list(list, false)?;
            },
            "shortform delete" => {
                let (list, _sha) = block_on(list_shortforms())?;
                let list = list
                    .into_iter()
                    .map(|shortform| (format!("{}", shortform.content), ListData::String(shortform.id.to_string())))
                    .collect::<Vec<_>>();
                let selected = self.list(list, true)?;
                for id in selected {
                    let id = match id {
                        ListData::String(id) => id,
                        // We know what we put in
                        _ => unreachable!(),
                    };
                    block_on(delete_shortform(&id))?;
                }
            },
            _ => eprintln!("Invalid command '{}', type 'help' to see the available commands.", cmd)
        };
        Ok(())
    }
    /// Present the given list to the user, updating the internal cache. This will await the user's choices.
    pub fn list(&mut self, list: Vec<(String, ListData)>, select: bool) -> Result<Vec<ListData>> {
        let mut idx = 1;
        for (display, _) in list.iter() {
            println!("{}) {}", idx, display);
            idx += 1;
        }
        if !select {
            return Ok(Vec::new());
        }
        println!("Please select the posts from the above list that you'd like to import.");
        let raw_selection = self.prompt("selection")?;
        // The user will likely have selected multiple items
        let selection_parts = raw_selection
            .split(',')
            .map(|s| s.trim()) // Handle whitespace
            .filter(|s| !s.is_empty());
        let mut selected = Vec::new();
        for part in selection_parts {
            // Individual parts can be of the form `x` or `x-y`, where y>x.
            let components = part.split('-').collect::<Vec<_>>();
            if components.len() == 2 {
                // It's a range, make sure it's valid
                let lower = match components[0].parse::<usize>() {
                    Ok(lower) => lower,
                    Err(_) => {
                        eprintln!("Selections must use integers only, please try again.");
                        return self.list(list, true)
                    }
                };
                let upper = match components[1].parse::<usize>() {
                    Ok(lower) => lower,
                    Err(_) => {
                        eprintln!("Selections must use integers only, please try again.");
                        return self.list(list, true)
                    }
                };
                if upper > lower {
                    // We need `(upper + 1)` to get the right indices, just as a product of how the `..` syntax works (viz. up to, not including)
                    for idx in lower..(upper + 1) {
                        // We subtract one to translate 1-indexing back to 0-indexing
                        let list_data = match list.get(idx - 1) {
                            Some(tuple) => tuple.1.clone(),
                            None => {
                                eprintln!("Please select elements within the bounds of the list.");
                                return self.list(list, true)
                            }
                        };
                        selected.push(list_data);
                    }
                } else {
                    eprintln!("Selection ranges must go from a smaller number to a larger one, please try again.");
                    // Recurse until we get something valid
                    return self.list(list, true)
                }
            } else if components.len() == 1 {
                // It's a single index, make sure it's valid
                let idx = match components[0].parse::<usize>() {
                    Ok(lower) => lower,
                    Err(_) => {
                        eprintln!("Selections must use integers only, please try again.");
                        return self.list(list, true)
                    }
                };
                // We subtract one to translate 1-indexing back to 0-indexing
                let list_data = match list.get(idx - 1) {
                    Some(tuple) => tuple.1.clone(),
                    None => {
                        eprintln!("Please select elements within the bounds of the list.");
                        return self.list(list, true)
                    }
                };
                selected.push(list_data);
            } else {
                eprintln!("Selection parts cannot have more than one dash separator, please try again.");
                // Recurse until we get something valid
                return self.list(list, true)
            }
        }

        Ok(selected)
    }
    /// Displays the hardcoded help menu.
    fn show_help() {
        println!("{}", HELP_MENU);
    }
}

/// The types that can back up each entry in a list displayed to the user.
#[derive(Clone)]
pub enum ListData {
    String(String),
    Path(PathBuf),
}
