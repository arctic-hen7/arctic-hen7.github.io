use anyhow::{anyhow, Result};

use crate::cli::CliState;

mod cli;
mod import_post;
mod list_posts;
mod parse_post;
mod post;
mod shortform;

#[tokio::main]
async fn main() {
    // Preamble
    // Source: https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Bloody&text=Delilah
    println!(
        r#"
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
    "#
    );

    println!("Hey there, I'm Delilah! I'm your personal blog assistant, you can ask me to import blog posts automatically from your Zettelkasten, or any arbitrary file. Many of the commands you use here will produce a numbered list, which can be acted on using operators like `1, 2 , 3`, and ranges like `1-4, 5-6`.");

    // 1. Initialise a shortform client
    // 2. Instantiate a new CLI state
    // 3. Start the CLI
    let res = core().await;
    if let Err(err) = res {
        println!("{:#?}", err);
    }
}

async fn core() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let search_dir = args.get(1).ok_or(anyhow!("no search directory provided"))?;

    // We'll find our dotenv config file from another environment variable
    let dotenv_location = std::env::var("DELILAH_CONF").unwrap_or(".env".to_string());
    dotenv::from_filename(dotenv_location).expect("Failed to load environment variables");

    shortform::get_client().await?;
    let mut state = CliState::new("../.blog", search_dir)?;
    state.start()
}
