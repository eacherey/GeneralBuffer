mod api;
mod cli;
mod clipboard;
mod config;
mod error;

use clap::Parser;

use crate::error::ClientError;
use crate::{
    cli::{Cli, Command},
    clipboard::ClipBoardManager,
    config::Config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let mut clipboard = ClipBoardManager::new()?;

    let cli = Cli::parse();
    match &cli.command {
        Command::Push => {
            let text = clipboard
                .get_clipboard()
                .ok_or(ClientError::EmptyClipboard)?;

            api::push_clip(&config, text.clone())?;
            println!("Push {}", text);
        }
        Command::Pull => {
            let clip = api::pull_latest_clip(&config)?;

            clipboard.set_clipboard(&clip.content)?;
            println!("[latest] {}", clip.content);

        }
        Command::Get { id } => {
            let clip = api::get_clip_by_id(&config, *id)?;

            clipboard.set_clipboard(&clip.content)?;
            println!("[{}] {}", clip.id, clip.content);
        }
        Command::Delete { id } => {
            let clip = api::delete_clip_by_id(&config, *id)?;

            println!("Deleted {}", clip.content);
        }
        Command::List => {
            let clips = api::get_list_clips(&config)?;

            for clip in &clips {
                println!("[{}] {}", clip.id, clip.content);
            }
        }
    }

    Ok(())
}
