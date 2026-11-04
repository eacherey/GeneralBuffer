mod config;
mod error;

use crate::config::Config;

fn main() {
    Config::load();
}
