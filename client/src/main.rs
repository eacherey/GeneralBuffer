mod config;

use crate::config::Config;

fn main() {
    Config::load();
}
