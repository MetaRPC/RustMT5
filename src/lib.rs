//! MetaRPC MT5 Rust SDK.
//!
//! * [`MT5Client`] — high-level async client (ConnectEx, account, symbols, bars, tick stream,
//!   OrderSend / OrderModify / OrderClose, history) talking gRPC+TLS to `mt5.mrpc.pro:443`.
//! * [`pb`] — the raw prost/tonic generated types (`pb::mt5_term_api::*`) for anything the
//!   high-level wrappers do not cover.

pub mod client;
pub mod error;
pub mod models;
pub mod pb;

pub use client::{timeframe, ConnectParams, MT5Client};
pub use error::MT5Error;
pub use models::*;
pub use pb::mt5_term_api;
