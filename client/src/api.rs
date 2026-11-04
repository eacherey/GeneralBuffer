use common::{Clip, CreateClip};

use crate::{config::Config, error::ClientResult};

pub fn send_clip(config: &Config, text: String) -> ClientResult<()> {
    let server_url = format!("{}/api/v1/clips", config.server_url);

    let create_clip = CreateClip { content: text };

    ureq::post(&server_url)
        .header("x-api-key", &config.api_key)
        .send_json(create_clip)?;

    Ok(())
}

pub fn fetch_latest_clip(config: &Config) -> ClientResult<Clip> {
    let server_url = format!("{}/api/v1/clips/latest", config.server_url);

    let mut clip = ureq::get(&server_url)
        .header("x-api-key", &config.api_key)
        .call()?;

    let json_clip: Clip = clip.body_mut().read_json::<Clip>()?;

    Ok(json_clip)
}
