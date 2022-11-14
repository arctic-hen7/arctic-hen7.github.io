use std::{io::{self, Write}, path::{Path, PathBuf}};
use anyhow::{Result, bail};

use crate::list_posts::PostsList;

// TODO Docs
static HELP_MENU: &str = r#"You can run any of the following commands:
help
blog detect new
blog detect changes
blog detect all
blog import file
shortform new
shortform delete
shortform list

(To leave, just press C-C.)
"#;

/// The current state of the CLI.
pub struct CliState {
    /// The list of options most recently presented to the user.
    last_list: Vec<(String, ListData)>,
    /// The current operation.
    curr_op: Operation,
    /// An internal list of posts for actually working with the blog post system.
    posts_list: PostsList,
    /// The directory to search for posts in.
    search_dir: PathBuf,
}
impl CliState {
    pub fn new(blog_dir: impl AsRef<Path>, search_dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            last_list: Vec::new(),
            curr_op: Operation::default(),
            posts_list: PostsList::new(blog_dir.as_ref())?,
            search_dir: search_dir.as_ref().to_path_buf()
        })
    }
    /// Starts the CLI off with an initial prompt.
    pub fn start(&mut self) -> Result<()> {
        let input = self.prompt("command")?;
        // If we have a currently active operation, handle that, otherwise parse the input as a command
        if let Operation::None = self.curr_op {
            // This will start a cycle that will terminate once the command is complete, in which case we should restart
            self.parse_cmd(&input)?;
            self.start()
        } else {
            todo!()
        }
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
                let selected = self.list(list)?;
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
                let selected = self.list(list)?;
                for data in selected.into_iter() {
                    let display = match data {
                        ListData::String(display) => display,
                        // We know what we put in
                        _ => unreachable!(),
                    };
                    self.posts_list.update_post(&display)?;
                }
            },
            "blog detect changed" => todo!(),
            _ => eprintln!("Invalid command '{}', type 'help' to see the available commands.", cmd)
        };
        Ok(())
    }
    /// Present the given list to the user, updating the internal cache. This will await the user's choices.
    pub fn list(&mut self, list: Vec<(String, ListData)>) -> Result<Vec<ListData>> {
        let mut idx = 1;
        for (display, _) in list.iter() {
            println!("{}) {}", idx, display);
            idx += 1;
        }
        self.last_list = list.clone();
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
                        return self.list(list)
                    }
                };
                let upper = match components[1].parse::<usize>() {
                    Ok(lower) => lower,
                    Err(_) => {
                        eprintln!("Selections must use integers only, please try again.");
                        return self.list(list)
                    }
                };
                if upper > lower {
                    for idx in lower..upper {
                        // We subtract one to translate 1-indexing back to 0-indexing
                        let list_data = match list.get(idx - 1) {
                            Some(tuple) => tuple.1.clone(),
                            None => {
                                eprintln!("Please select elements within the bounds of the list.");
                                return self.list(list)
                            }
                        };
                        selected.push(list_data);
                    }
                } else {
                    eprintln!("Selection ranges must go from a smaller number to a larger one, please try again.");
                    // Recurse until we get something valid
                    return self.list(list)
                }
            } else if components.len() == 1 {
                // It's a single index, make sure it's valid
                let idx = match components[0].parse::<usize>() {
                    Ok(lower) => lower,
                    Err(_) => {
                        eprintln!("Selections must use integers only, please try again.");
                        return self.list(list)
                    }
                };
                // We subtract one to translate 1-indexing back to 0-indexing
                let list_data = match list.get(idx - 1) {
                    Some(tuple) => tuple.1.clone(),
                    None => {
                        eprintln!("Please select elements within the bounds of the list.");
                        return self.list(list)
                    }
                };
                selected.push(list_data);
            } else {
                eprintln!("Selection parts cannot have more than one dash separator, please try again.");
                // Recurse until we get something valid
                return self.list(list)
            }
        }

        Ok(selected)
    }
    /// Set the current operation to the given value.
    pub fn set_op(&mut self, op: Operation) {
        self.curr_op = op;
    }
    /// Clear the current state to leave the user with a blank state.
    pub fn clear(&mut self) {
        self.last_list = Vec::new();
        self.curr_op = Operation::default();
    }
    /// Displays the hardcoded help menu.
    fn show_help() {
        println!("{}", HELP_MENU);
    }
}

/// The operations that the CLI can execute. This is stored in the current state so the CLI can keep a kind of primitive conversation with the user.
pub enum Operation {
    /// There is no currently active operation, and we're waiting for a command from the user.
    None,
    /// A list has been displayed to the user, from which they will select the posts they want to import.
    ImportPostsFromList,
}
impl Default for Operation {
    fn default() -> Self { Self::None }
}

enum NextAction {
    Confirm(String),
    YesNo(String),
    ParseList,
}

/// The types that can back up each entry in a list displayed to the user.
#[derive(Clone)]
pub enum ListData {
    String(String),
    Path(PathBuf),
}
