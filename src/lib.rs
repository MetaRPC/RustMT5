//! Asynchronous Rust SDK for MetaTrader 5 over the MRPC gRPC gateway.
//!
//! Source layout:
//! - [`proto`] — the generated gRPC/protobuf contract (vendored);
//! - [`mt5`] — the three SDK layers: [`MT5Account`] (low-level), [`MT5Service`] (facade),
//!   [`MT5Sugar`] (high-level);
//! - [`helpers`] — auxiliary modules: the [`MT5Error`] type and the ready-made [`strategies`].

pub mod proto;
pub mod mt5;
pub mod helpers;

// Re-export the modules at the crate root so `crate::account` / `crate::error` paths resolve.
pub use helpers::config;
pub use helpers::error;
pub use helpers::strategies;
pub use mt5::account;
pub use mt5::service;
pub use mt5::sugar;

// The primary public types.
pub use helpers::config::Settings;
pub use helpers::error::MT5Error;
pub use mt5::account::MT5Account;
pub use mt5::service::MT5Service;
pub use mt5::sugar::MT5Sugar;
