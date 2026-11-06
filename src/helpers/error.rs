//! Error type for the MT5 SDK.

use crate::proto::mt5_term_api as pb;

/// Everything the SDK can fail with. Mirrors the Python `ApiExceptionMT5` / connection errors,
/// collapsed into one idiomatic Rust enum returned as `Result<_, MT5Error>`.
#[derive(Debug, thiserror::Error)]
pub enum MT5Error {
    /// The server answered with a structured error (its `data`/`error` oneof carried `error`).
    #[error("MT5 API error {code}: {message} (mql {mql_code})")]
    Api {
        code: String,
        message: String,
        /// Numeric MQL5 error code (`mql_error_int_code`), 0 when not a trade/MQL error.
        mql_code: i32,
    },

    /// A reply arrived but its `response` oneof was empty (neither data nor error).
    #[error("empty response from server")]
    EmptyResponse,

    /// A method was called before connecting.
    #[error("not connected: call connect_by_server_name / connect_by_host_port first")]
    NotConnected,

    /// Connecting / handshake failed, or a timeout waiting for the terminal.
    #[error("connection error: {0}")]
    Connection(String),

    /// gRPC transport failure (channel, DNS, TLS).
    #[error("transport error: {0}")]
    Transport(String),

    /// gRPC call returned a non-OK status.
    #[error("grpc status: {0}")]
    Status(String),

    /// Invalid argument supplied to a method (e.g. a non-ASCII header value).
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    /// An operation did not complete within its deadline.
    #[error("timeout: {0}")]
    Timeout(String),
}

impl From<pb::Error> for MT5Error {
    fn from(e: pb::Error) -> Self {
        MT5Error::Api {
            code: e.error_code,
            message: if e.error_message.is_empty() { e.mql_error_description.clone() } else { e.error_message },
            mql_code: e.mql_error_int_code,
        }
    }
}

impl From<tonic::Status> for MT5Error {
    fn from(s: tonic::Status) -> Self {
        MT5Error::Status(format!("{}: {}", s.code(), s.message()))
    }
}

impl From<tonic::transport::Error> for MT5Error {
    fn from(e: tonic::transport::Error) -> Self {
        MT5Error::Transport(e.to_string())
    }
}
