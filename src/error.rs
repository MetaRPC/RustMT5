use thiserror::Error;

#[derive(Error, Debug)]
pub enum MT5Error {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Authentication error for account: {0}")]
    AuthError(u64),
    #[error("Trade error with retcode {0}: {1}")]
    TradeError(u32, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
