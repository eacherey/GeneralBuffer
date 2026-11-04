use serde::{Deserialize, Serialize};

use crate::error::ClientResult;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub server_url: String,
}

impl Config {
    pub fn load() -> ClientResult<Self> {
        let config: Self = confy::load("generalbuffer", None)?;

        Ok(config)
    }
}
