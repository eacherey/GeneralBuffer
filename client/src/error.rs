use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Config error: {0}")]
    Config(#[from] confy::ConfyError),

    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),

    #[error("Json error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Server error: {status}, {message}")]
    Api { status: u16, message: String },

    #[error("Clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),
}

pub type ClientResult<T> = Result<T, ClientError>;
