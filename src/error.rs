use thiserror::Error;

use crate::pb::mt5_term_api as pb;

/// Every failure the SDK can surface.
///
/// * `Grpc` — transport / HTTP2 level (`tonic::Status`). `Unavailable` and `DeadlineExceeded`
///   are retried by the client's `call` wrapper.
/// * `Api` — the gateway answered with its `Error` oneof (`error_code`, `error_message`,
///   MQL error / trade codes). `TERMINAL_INSTANCE_NOT_FOUND` triggers one reconnect + retry.
/// * `InvalidLogin` — `CONNECT_EX_INVALID_LOGIN_ERROR` and friends. **Never retried**: repeated
///   bad logins are what brokers lock accounts for.
/// * `TradeError` — `OrderSend` / `OrderModify` / `OrderClose` returned a retcode other than
///   10008 (placed) / 10009 (done).
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
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("MetaRPC API error {code}: {message} (mql={mql_code}/{mql_trade_code} {mql_desc})")]
    Api {
        code: String,
        message: String,
        mql_code: i32,
        mql_trade_code: i32,
        mql_desc: String,
    },
    #[error("invalid login (not retried): {0}")]
    InvalidLogin(String),
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("not connected — call connect_ex() first")]
    NotConnected,
}

impl MT5Error {
    pub fn from_api(e: pb::Error) -> Self {
        let code_up = e.error_code.to_ascii_uppercase();
        let msg_up = e.error_message.to_ascii_uppercase();
        if code_up.contains("INVALID_LOGIN")
            || code_up.contains("WRONG_PASSWORD")
            || code_up.contains("INVALID_PASSWORD")
            || msg_up.contains("INVALID LOGIN")
            || msg_up.contains("WRONG PASSWORD")
        {
            return MT5Error::InvalidLogin(format!("{}: {}", e.error_code, e.error_message));
        }
        let mql_desc = if !e.mql_error_trade_description.is_empty() {
            e.mql_error_trade_description
        } else {
            e.mql_error_description
        };
        MT5Error::Api {
            code: e.error_code,
            message: e.error_message,
            mql_code: e.mql_error_int_code,
            mql_trade_code: e.mql_error_trade_int_code,
            mql_desc,
        }
    }

    /// Gateway lost the terminal (pod restart, idle eviction). Re-`ConnectEx` with the same id
    /// and retry the call.
    pub fn is_terminal_not_found(&self) -> bool {
        match self {
            MT5Error::Api { code, .. } => {
                code.contains("TERMINAL_INSTANCE_NOT_FOUND")
                    || code.contains("TERMINAL_REGISTRY_TERMINAL_NOT_FOUND")
            }
            _ => false,
        }
    }

    /// Transient transport failure — safe to retry after a back-off.
    pub fn is_transient(&self) -> bool {
        match self {
            MT5Error::Grpc(s) => matches!(
                s.code(),
                tonic::Code::Unavailable | tonic::Code::DeadlineExceeded | tonic::Code::Cancelled
            ),
            MT5Error::Transport(_) => true,
            _ => false,
        }
    }

    pub fn is_retryable(&self) -> bool {
        self.is_transient() || self.is_terminal_not_found()
    }

    /// The gateway's error code string (empty for non-API errors).
    pub fn api_code(&self) -> &str {
        match self {
            MT5Error::Api { code, .. } => code,
            _ => "",
        }
    }
}
