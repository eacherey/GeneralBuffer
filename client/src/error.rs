use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Config error: {0}")]
    Config(#[from] confy::ConfyError),

    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),

    #[error("Json error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("Clipboard empty")]
    EmptyClipboard,
}

pub type ClientResult<T> = Result<T, ClientError>;
