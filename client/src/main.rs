mod api;
mod clipboard;
mod config;
mod error;

use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    Ok(())
}
