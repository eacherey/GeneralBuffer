use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub server_url: String,
}

impl Config {
    pub fn load() -> Self {
        let config: Self = confy::load("generalbuffer", None).unwrap();

        config
    }
}
