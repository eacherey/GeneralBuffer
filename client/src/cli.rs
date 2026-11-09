use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Push,
    Pull,
    Get { id: Uuid },
    Delete { id: Uuid },
    List,
    Hotkey,
}
