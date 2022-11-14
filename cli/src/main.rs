use std::path::Path;

use crate::cli::CliState;

mod import_post;
mod post;
mod list_posts;
mod parse_post;
mod shortform;
mod cli;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load environment variables");

    // Preamble
    // Source: https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Bloody&text=Delilah
    println!(r#"
▓█████▄ ▓█████  ██▓     ██▓ ██▓    ▄▄▄       ██░ ██
▒██▀ ██▌▓█   ▀ ▓██▒    ▓██▒▓██▒   ▒████▄    ▓██░ ██▒
░██   █▌▒███   ▒██░    ▒██▒▒██░   ▒██  ▀█▄  ▒██▀▀██░
░▓█▄   ▌▒▓█  ▄ ▒██░    ░██░▒██░   ░██▄▄▄▄██ ░▓█ ░██
░▒████▓ ░▒████▒░██████▒░██░░██████▒▓█   ▓██▒░▓█▒░██▓
 ▒▒▓  ▒ ░░ ▒░ ░░ ▒░▓  ░░▓  ░ ▒░▓  ░▒▒   ▓▒█░ ▒ ░░▒░▒
 ░ ▒  ▒  ░ ░  ░░ ░ ▒  ░ ▒ ░░ ░ ▒  ░ ▒   ▒▒ ░ ▒ ░▒░ ░
 ░ ░  ░    ░     ░ ░    ▒ ░  ░ ░    ░   ▒    ░  ░░ ░
   ░       ░  ░    ░  ░ ░      ░  ░     ░  ░ ░  ░  ░
 ░
    "#);

    let search_dir = std::env::var("SEARCH_DIR").expect("No search directory provided");

    println!("Hey there, I'm Delilah! I'm your personal blog assistant, you can ask me to import blog posts automatically from your Zettelkasten, or any arbitrary file. Many of the commands you use here will produce a numbered list, which can be acted on using operators like `1, 2 , 3`, and ranges like `1-4, 5-6`.");

    let res = CliState::new("../.blog", search_dir).map(|mut state| state.start());
    if let Err(err) = res {
        eprintln!("{:#?}", err);
    }
    println!("Have a lovely day! 😊");
}
