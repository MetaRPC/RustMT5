//! Connection settings — load from a JSON file and/or environment variables.
//!
//! [`Settings::load`] reads a `settings.json` (if present) and then overlays environment
//! variables on top (env wins), so examples and apps can configure the connection either way.
//!
//! Resolution order for the file path:
//! 1. the `MRPC_SETTINGS` environment variable, if set;
//! 2. `examples/0_common/settings.json` (the bundled template location);
//! 3. `settings.json` in the current directory.
//!
//! Environment overlay (each overrides the file when set):
//! `MT5_LOGIN`, `MT5_PASSWORD`, `MT5_SERVER`, `MT5_SYMBOL`, `MRPC_API_KEY`.
//!
//! If `api_key` ends up set (from the file) and `MRPC_API_KEY` is not already in the
//! environment, it is exported so the SDK's metadata layer picks it up.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::MT5Error;

/// Connection settings for an MT5 account.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// MT5 account login number.
    pub user: u64,
    /// MT5 account password.
    #[serde(default)]
    pub password: String,
    /// gRPC endpoint (`host:port`); `None` → the SDK default (`mt5.mrpc.pro:443`).
    #[serde(default)]
    pub grpc_server: Option<String>,
    /// Broker server / cluster name (e.g. `"MetaQuotes-Demo"`).
    #[serde(default = "default_server_name")]
    pub server_name: String,
    /// MetaRPC API key; when present it is exported as `MRPC_API_KEY` for the SDK.
    #[serde(default)]
    pub api_key: Option<String>,
    /// A default symbol for examples (e.g. `"EURUSD"`).
    #[serde(default = "default_symbol")]
    pub test_symbol: String,
}

fn default_server_name() -> String {
    "MetaQuotes-Demo".to_string()
}
fn default_symbol() -> String {
    "EURUSD".to_string()
}

impl Settings {
    /// Load settings from the resolved `settings.json` (if any) and overlay environment variables.
    pub fn load() -> Result<Self, MT5Error> {
        let explicit = std::env::var("MRPC_SETTINGS").ok().map(PathBuf::from);
        Self::from_sources(explicit)
    }

    /// Load settings from a specific JSON file, then overlay environment variables.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, MT5Error> {
        Self::from_sources(Some(path.as_ref().to_path_buf()))
    }

    fn from_sources(explicit: Option<PathBuf>) -> Result<Self, MT5Error> {
        // 1) Start from a JSON object read from the first existing candidate file (if any).
        let candidates: Vec<PathBuf> = match explicit {
            Some(p) => vec![p],
            None => vec![
                PathBuf::from("examples/0_common/settings.json"),
                PathBuf::from("settings.json"),
            ],
        };

        let mut value = serde_json::Value::Object(Default::default());
        for p in &candidates {
            if p.exists() {
                let txt = std::fs::read_to_string(p)
                    .map_err(|e| MT5Error::InvalidArgument(format!("reading {}: {e}", p.display())))?;
                value = serde_json::from_str(&txt)
                    .map_err(|e| MT5Error::InvalidArgument(format!("parsing {}: {e}", p.display())))?;
                break;
            }
        }

        let obj = value
            .as_object_mut()
            .ok_or_else(|| MT5Error::InvalidArgument("settings.json must be a JSON object".to_string()))?;

        // 2) Overlay environment variables (env wins over the file).
        if let Ok(x) = std::env::var("MT5_LOGIN") {
            let n: u64 = x
                .parse()
                .map_err(|_| MT5Error::InvalidArgument("MT5_LOGIN must be a number".to_string()))?;
            obj.insert("user".to_string(), serde_json::json!(n));
        }
        if let Ok(x) = std::env::var("MT5_PASSWORD") {
            obj.insert("password".to_string(), serde_json::json!(x));
        }
        if let Ok(x) = std::env::var("MT5_SERVER") {
            obj.insert("server_name".to_string(), serde_json::json!(x));
        }
        if let Ok(x) = std::env::var("MT5_SYMBOL") {
            obj.insert("test_symbol".to_string(), serde_json::json!(x));
        }
        if let Ok(x) = std::env::var("MRPC_API_KEY") {
            obj.insert("api_key".to_string(), serde_json::json!(x));
        }

        // 3) Deserialize into the typed struct.
        let settings: Settings = serde_json::from_value(value).map_err(|e| {
            MT5Error::InvalidArgument(format!(
                "invalid settings (need at least `user`; set it in settings.json or MT5_LOGIN): {e}"
            ))
        })?;

        // 4) Make the API key visible to the SDK's metadata layer.
        if let Some(key) = &settings.api_key {
            if std::env::var("MRPC_API_KEY").is_err() {
                std::env::set_var("MRPC_API_KEY", key);
            }
        }

        Ok(settings)
    }
}
