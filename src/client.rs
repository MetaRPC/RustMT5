//! High-level async client over the MetaRPC MT5 gRPC gateway.
//!
//! Every call attaches two metadata headers: `id` (the terminal instance id — pass a **stable**
//! one per account so re-runs reuse the same terminal) and `apikey`.
//!
//! All methods take `&self`; the client is `Send + Sync` and can be shared behind an `Arc`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc;
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Streaming};

use crate::error::MT5Error;
use crate::models::*;
use crate::pb::mt5_term_api as pb;
use pb::account_helper_client::AccountHelperClient;
use pb::connection_client::ConnectionClient;
use pb::market_info_client::MarketInfoClient;
use pb::subscription_service_client::SubscriptionServiceClient;
use pb::trade_functions_client::TradeFunctionsClient;
use pb::trading_helper_client::TradingHelperClient;

/// MQL5 `ENUM_TIMEFRAMES` values accepted by `price_history`.
pub mod timeframe {
    pub const M1: i32 = 1;
    pub const M2: i32 = 2;
    pub const M3: i32 = 3;
    pub const M5: i32 = 5;
    pub const M10: i32 = 10;
    pub const M15: i32 = 15;
    pub const M30: i32 = 30;
    pub const H1: i32 = 16385;
    pub const H2: i32 = 16386;
    pub const H4: i32 = 16388;
    pub const H12: i32 = 16396;
    pub const D1: i32 = 16408;
    pub const W1: i32 = 32769;
    pub const MN1: i32 = 49153;

    /// Parse "M1".."MN1" (case-insensitive).
    pub fn parse(s: &str) -> Option<i32> {
        Some(match s.to_ascii_uppercase().as_str() {
            "M1" => M1,
            "M2" => M2,
            "M3" => M3,
            "M5" => M5,
            "M10" => M10,
            "M15" => M15,
            "M30" => M30,
            "H1" => H1,
            "H2" => H2,
            "H4" => H4,
            "H12" => H12,
            "D1" => D1,
            "W1" => W1,
            "MN1" => MN1,
            _ => return None,
        })
    }
}

/// What `connect_ex` was called with; kept for `reconnect()`.
#[derive(Debug, Clone)]
pub struct ConnectParams {
    pub login: u64,
    pub password: String,
    pub server: String,
    pub name: Option<String>,
    pub timeout_seconds: u32,
}

pub struct MT5Client {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
    /// Terminal instance id sent as the `id` header. Set it **before** `connect_ex` to a stable
    /// value (see [`MT5Client::stable_terminal_id`]); if left `None` a random UUID v4 is used and
    /// a fresh terminal is created on every run.
    pub id: Option<String>,
    /// Deadline applied to every unary call (default 60 s; ConnectEx uses its own timeout + 30 s).
    pub call_timeout: Duration,
    /// Max retries for transient transport errors / terminal-not-found (default 3).
    pub max_retries: u32,
    channel: Mutex<Option<Channel>>,
    terminal_guid: Mutex<Option<String>>,
    params: Mutex<Option<ConnectParams>>,
    connected: AtomicBool,
    /// Broker server time − UTC, in seconds (`utc_timezone_server_time_shift_minutes × 60`),
    /// learned from the first `account_summary()`. History windows and the timestamps the
    /// gateway returns are in **broker server time**, not UTC.
    server_utc_shift: Mutex<Option<i64>>,
}

fn sha256_bytes(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut msg = input.to_vec();
    let bit_len = (msg.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() + 8) % 64 != 0 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, v) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&v.to_be_bytes());
    }
    out
}

/// Unwrap a `XxxReply { response: Some(Data|Error) }` oneof into `Result<Data, MT5Error>`.
macro_rules! unwrap_reply {
    ($reply:expr, $module:ident) => {
        match $reply.into_inner().response {
            Some(pb::$module::Response::Data(d)) => Ok(d),
            Some(pb::$module::Response::Error(e)) => Err(MT5Error::from_api(e)),
            None => Err(MT5Error::Protocol(concat!("empty ", stringify!($module)).to_string())),
        }
    };
}

pub fn timestamp(t: SystemTime) -> prost_types::Timestamp {
    let d = t.duration_since(UNIX_EPOCH).unwrap_or_default();
    prost_types::Timestamp { seconds: d.as_secs() as i64, nanos: d.subsec_nanos() as i32 }
}

pub fn unix_secs(t: &Option<prost_types::Timestamp>) -> i64 {
    t.as_ref().map(|x| x.seconds).unwrap_or(0)
}

/// Shift a UTC instant by `shift_secs` (broker server time = UTC + shift).
pub fn shift_time(t: SystemTime, shift_secs: i64) -> SystemTime {
    if shift_secs >= 0 {
        t + Duration::from_secs(shift_secs as u64)
    } else {
        t - Duration::from_secs((-shift_secs) as u64)
    }
}

impl MT5Client {
    // ----------------------------------------------------------------- construction / ids

    /// Legacy deterministic id = sha256("login:password") folded into a GUID. Prefer
    /// [`MT5Client::stable_terminal_id`] which does not depend on the password.
    pub fn compute_deterministic_id(user: u64, password: &str) -> String {
        let input = format!("{}:{}", user, password);
        let hash = sha256_bytes(input.as_bytes());
        let mut le = [0u8; 16];
        le[0] = hash[3];
        le[1] = hash[2];
        le[2] = hash[1];
        le[3] = hash[0];
        le[4] = hash[5];
        le[5] = hash[4];
        le[6] = hash[7];
        le[7] = hash[6];
        le[8..16].copy_from_slice(&hash[8..16]);
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            le[0], le[1], le[2], le[3], le[4], le[5], le[6], le[7], le[8], le[9], le[10], le[11], le[12],
            le[13], le[14], le[15]
        )
    }

    /// UUID v5 (namespace DNS) of a label such as `"S042"` or `"fleet-audit-RustMT5"`. The same
    /// label always yields the same terminal id, so restarts reuse the terminal instead of
    /// orphaning a new one (the gateway caps terminals per key).
    pub fn stable_terminal_id(label: &str) -> String {
        uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, label.as_bytes()).to_string()
    }

    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self::build(host.into(), port, std::env::var("MRPC_API_KEY").ok(), None)
    }

    pub fn with_api_key(host: impl Into<String>, port: u16, api_key: impl Into<String>) -> Self {
        Self::build(host.into(), port, Some(api_key.into()), None)
    }

    fn build(host: String, port: u16, api_key: Option<String>, id: Option<String>) -> Self {
        MT5Client {
            host,
            port,
            api_key,
            id,
            call_timeout: Duration::from_secs(60),
            max_retries: 3,
            channel: Mutex::new(None),
            terminal_guid: Mutex::new(None),
            params: Mutex::new(None),
            connected: AtomicBool::new(false),
            server_utc_shift: Mutex::new(None),
        }
    }

    /// Broker server time − UTC in seconds, once known (after `account_summary()`).
    pub fn server_utc_shift_secs(&self) -> Option<i64> {
        *self.server_utc_shift.lock().unwrap()
    }

    /// Convert a gateway timestamp (broker server time) to unix UTC seconds.
    pub fn to_utc_secs(&self, server_ts: &Option<prost_types::Timestamp>) -> i64 {
        unix_secs(server_ts) - self.server_utc_shift_secs().unwrap_or(0)
    }

    async fn ensure_shift(&self) -> Result<i64, MT5Error> {
        if let Some(s) = self.server_utc_shift_secs() {
            return Ok(s);
        }
        self.account_summary().await?;
        Ok(self.server_utc_shift_secs().unwrap_or(0))
    }

    pub fn with_auth(mut self, api_key: impl Into<String>, id: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self.id = Some(id.into());
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_call_timeout(mut self, t: Duration) -> Self {
        self.call_timeout = t;
        self
    }

    /// Legacy helper: sets `id` from login+password and returns it.
    pub async fn get_id(&mut self, user: u64, password: &str) -> Result<String, MT5Error> {
        let id_str = Self::compute_deterministic_id(user, password);
        self.id = Some(id_str.clone());
        Ok(id_str)
    }

    /// The id the gateway knows this terminal by (returned `terminalInstanceGuid`, else `id`).
    pub fn terminal_id(&self) -> Option<String> {
        self.terminal_guid.lock().unwrap().clone().or_else(|| self.id.clone())
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    pub fn connect_params(&self) -> Option<ConnectParams> {
        self.params.lock().unwrap().clone()
    }

    // ----------------------------------------------------------------- transport

    fn channel(&self) -> Result<Channel, MT5Error> {
        let mut guard = self.channel.lock().unwrap();
        if let Some(ch) = guard.as_ref() {
            return Ok(ch.clone());
        }
        let uri = format!("https://{}:{}", self.host, self.port);
        let endpoint = Endpoint::from_shared(uri)
            .map_err(|e| MT5Error::Transport(e.to_string()))?
            .tls_config(ClientTlsConfig::new().with_webpki_roots().domain_name(self.host.clone()))
            .map_err(|e| MT5Error::Transport(e.to_string()))?
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .http2_keep_alive_interval(Duration::from_secs(30))
            .keep_alive_timeout(Duration::from_secs(20))
            .keep_alive_while_idle(true)
            .connect_timeout(Duration::from_secs(20));
        let ch = endpoint.connect_lazy();
        *guard = Some(ch.clone());
        Ok(ch)
    }

    /// Wrap a protobuf message in a `tonic::Request` with `id` + `apikey` metadata and a deadline.
    pub fn request<T>(&self, msg: T, timeout: Option<Duration>) -> Result<Request<T>, MT5Error> {
        let mut req = Request::new(msg);
        if let Some(t) = timeout {
            req.set_timeout(t);
        }
        let md = req.metadata_mut();
        if let Some(id) = self.terminal_id() {
            md.insert("id", id.parse().map_err(|_| MT5Error::Protocol("bad id".into()))?);
        }
        let key = self.api_key.clone().unwrap_or_else(|| "TRIAL".to_string());
        md.insert("apikey", key.parse().map_err(|_| MT5Error::Protocol("bad api key".into()))?);
        Ok(req)
    }

    fn req<T>(&self, msg: T) -> Result<Request<T>, MT5Error> {
        self.request(msg, Some(self.call_timeout))
    }

    /// Run `op`, retrying transient transport errors with back-off and re-`ConnectEx`-ing once
    /// per attempt on `TERMINAL_INSTANCE_NOT_FOUND`. `InvalidLogin` is never retried.
    pub async fn call<T>(&self, op: impl AsyncFn() -> Result<T, MT5Error>) -> Result<T, MT5Error> {
        let mut attempt: u32 = 0;
        loop {
            match op().await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    attempt += 1;
                    if attempt > self.max_retries || !e.is_retryable() {
                        return Err(e);
                    }
                    if e.is_terminal_not_found() {
                        if let Err(re) = self.reconnect().await {
                            if matches!(re, MT5Error::InvalidLogin(_)) {
                                return Err(re);
                            }
                        }
                    }
                    let backoff = Duration::from_millis(500 * (1u64 << attempt.min(5)));
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    // ----------------------------------------------------------------- connection

    /// `Connection/ConnectEx` by broker server name (`mt_cluster_name`, e.g. `"FBS-Demo"`).
    /// Blocks until the terminal has logged in (up to `timeout_seconds`, default 120).
    /// Never retries an invalid login.
    pub async fn connect_ex(
        &self,
        login: u64,
        password: &str,
        server: &str,
        name: Option<&str>,
        timeout_seconds: u32,
    ) -> Result<pb::ConnectData, MT5Error> {
        let params = ConnectParams {
            login,
            password: password.to_string(),
            server: server.to_string(),
            name: name.map(|s| s.to_string()),
            timeout_seconds,
        };
        *self.params.lock().unwrap() = Some(params.clone());
        self.do_connect(&params).await
    }

    /// Convenience: `connect_ex(login, password, server, None, 120)`.
    pub async fn connect(&self, login: u64, password: &str, server: &str) -> Result<(), MT5Error> {
        self.connect_ex(login, password, server, None, 120).await.map(|_| ())
    }

    async fn do_connect(&self, p: &ConnectParams) -> Result<pb::ConnectData, MT5Error> {
        if self.id.is_none() {
            // No stable id given: fall back to a random one (new terminal each run).
            let fresh = uuid::Uuid::new_v4().to_string();
            *self.terminal_guid.lock().unwrap() = Some(fresh);
        }
        let timeout = if p.timeout_seconds == 0 { 120 } else { p.timeout_seconds };
        let msg = pb::ConnectExRequest {
            user: p.login,
            password: p.password.clone(),
            mt_cluster_name: p.server.clone(),
            experts_to_add: vec![],
            timeout_seconds: Some(timeout),
            name: p.name.clone(),
            expiration: None,
        };
        let req = self.request(msg, Some(Duration::from_secs(timeout as u64 + 30)))?;
        let mut client = ConnectionClient::new(self.channel()?);
        let reply = client.connect_ex(req).await?;
        let data: pb::ConnectData = unwrap_reply!(reply, connect_ex_reply)?;
        if !data.terminal_instance_guid.is_empty() {
            *self.terminal_guid.lock().unwrap() = Some(data.terminal_instance_guid.clone());
        }
        self.connected.store(true, Ordering::SeqCst);
        Ok(data)
    }

    /// Re-run `ConnectEx` with the stored parameters (same terminal id).
    pub async fn reconnect(&self) -> Result<(), MT5Error> {
        let p = self.params.lock().unwrap().clone().ok_or(MT5Error::NotConnected)?;
        self.do_connect(&p).await.map(|_| ())
    }

    /// `Connection/CheckConnect` — cheap liveness probe of the terminal.
    pub async fn check_connect(&self) -> Result<pb::CheckConnectData, MT5Error> {
        let mut c = ConnectionClient::new(self.channel()?);
        let r = c.check_connect(self.req(pb::CheckConnectRequest {})?).await?;
        unwrap_reply!(r, check_connect_reply)
    }

    /// `Connection/Disconnect`. `delete = true` removes the terminal instance permanently,
    /// `false` just stops it (it can be reused by a later `ConnectEx` with the same id).
    pub async fn disconnect(&self, delete: bool) -> Result<pb::DisconnectData, MT5Error> {
        let mut c = ConnectionClient::new(self.channel()?);
        let msg = pb::DisconnectRequest { reason: Some("client disconnect".into()), delete: Some(delete) };
        let r = c.disconnect(self.req(msg)?).await?;
        self.connected.store(false, Ordering::SeqCst);
        unwrap_reply!(r, disconnect_reply)
    }

    pub async fn disconnect_default(&self) -> Result<pb::DisconnectData, MT5Error> {
        self.disconnect(false).await
    }

    // ----------------------------------------------------------------- account

    /// `AccountHelper/AccountSummary` — login, balance, equity, currency, leverage, trade mode.
    pub async fn account_summary(&self) -> Result<pb::AccountSummaryData, MT5Error> {
        self.call(async || {
            let mut c = AccountHelperClient::new(self.channel()?);
            let r = c.account_summary(self.req(pb::AccountSummaryRequest {})?).await?;
            let d: pb::AccountSummaryData = unwrap_reply!(r, account_summary_reply)?;
            *self.server_utc_shift.lock().unwrap() = Some(d.utc_timezone_server_time_shift_minutes * 60);
            Ok(d)
        })
        .await
    }

    /// `account_summary` decoded into [`AccountInfo`] (with `trade_mode`).
    pub async fn get_account_info(&self) -> Result<AccountInfo, MT5Error> {
        let d = self.account_summary().await?;
        let server = self.params.lock().unwrap().as_ref().map(|p| p.server.clone()).unwrap_or_default();
        Ok(AccountInfo::from_summary(&d, &server))
    }

    // ----------------------------------------------------------------- symbols / quotes

    /// `MarketInfo/SymbolSelect` — add (`true`) / remove (`false`) a symbol in Market Watch.
    pub async fn symbol_select(&self, symbol: &str, select: bool) -> Result<bool, MT5Error> {
        self.call(async || {
            let mut c = MarketInfoClient::new(self.channel()?);
            let msg = pb::SymbolSelectRequest { symbol: symbol.to_string(), select };
            let r = c.symbol_select(self.req(msg)?).await?;
            unwrap_reply!(r, symbol_select_reply).map(|d| d.success)
        })
        .await
    }

    pub async fn symbol_exist(&self, symbol: &str) -> Result<bool, MT5Error> {
        self.call(async || {
            let mut c = MarketInfoClient::new(self.channel()?);
            let msg = pb::SymbolExistRequest { name: symbol.to_string() };
            let r = c.symbol_exist(self.req(msg)?).await?;
            unwrap_reply!(r, symbol_exist_reply).map(|d| d.exists)
        })
        .await
    }

    pub async fn symbol_is_synchronized(&self, symbol: &str) -> Result<bool, MT5Error> {
        self.call(async || {
            let mut c = MarketInfoClient::new(self.channel()?);
            let msg = pb::SymbolIsSynchronizedRequest { symbol: symbol.to_string() };
            let r = c.symbol_is_synchronized(self.req(msg)?).await?;
            unwrap_reply!(r, symbol_is_synchronized_reply).map(|d| d.synchronized)
        })
        .await
    }

    /// `MarketInfo/SymbolInfoTick` — the authoritative bid/ask. (Bulk `SymbolParamsMany` prices
    /// are intermittently stale.)
    pub async fn symbol_info_tick(&self, symbol: &str) -> Result<pb::MrpcMqlTick, MT5Error> {
        self.call(async || {
            let mut c = MarketInfoClient::new(self.channel()?);
            let msg = pb::SymbolInfoTickRequest { symbol: symbol.to_string() };
            let r = c.symbol_info_tick(self.req(msg)?).await?;
            unwrap_reply!(r, symbol_info_tick_request_reply)
        })
        .await
    }

    pub async fn quote(&self, symbol: &str) -> Result<Quote, MT5Error> {
        let t = self.symbol_info_tick(symbol).await?;
        Ok(Quote::from_tick(symbol, &t))
    }

    /// Select the symbol, then wait until it is synchronized and publishes a two-sided quote.
    pub async fn ensure_symbol(&self, symbol: &str, wait: Duration) -> Result<Quote, MT5Error> {
        self.symbol_select(symbol, true).await?;
        let deadline = tokio::time::Instant::now() + wait;
        let mut last: Option<Quote> = None;
        loop {
            let synced = self.symbol_is_synchronized(symbol).await.unwrap_or(false);
            if synced {
                if let Ok(q) = self.quote(symbol).await {
                    if q.is_usable() {
                        return Ok(q);
                    }
                    last = Some(q);
                }
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(MT5Error::Protocol(format!(
                    "{symbol}: no usable two-sided quote within {:?} (last={:?})",
                    wait, last
                )));
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    /// `AccountHelper/SymbolParamsMany`. `None` = every symbol in one page (paging on the gateway
    /// does not advance the window — use a single unfiltered call).
    pub async fn symbol_params_many(&self, name_filter: Option<&str>) -> Result<pb::SymbolParamsManyData, MT5Error> {
        self.call(async || {
            let mut c = AccountHelperClient::new(self.channel()?);
            let msg = pb::SymbolParamsManyRequest {
                symbol_name: name_filter.map(|s| s.to_string()),
                sort_type: None,
                page_number: None,
                items_per_page: None,
            };
            let r = c.symbol_params_many(self.req(msg)?).await?;
            unwrap_reply!(r, symbol_params_many_reply)
        })
        .await
    }

    /// Contract specs of one symbol (digits, volume_min/step/max, contract size, tick size/value,
    /// currencies, trade_mode). Exact-name match.
    pub async fn symbol_params(&self, symbol: &str) -> Result<pb::SymbolParameters, MT5Error> {
        let d = self.symbol_params_many(Some(symbol)).await?;
        d.symbol_infos
            .into_iter()
            .find(|s| s.name == symbol)
            .ok_or_else(|| MT5Error::Protocol(format!("symbol {symbol} not in SymbolParamsMany reply")))
    }

    // ----------------------------------------------------------------- bars

    /// `MarketInfo/PriceHistory` for the newest `max_bars` bars of `timeframe` (see [`timeframe`]).
    pub async fn price_history(&self, symbol: &str, timeframe: i32, max_bars: i32) -> Result<Vec<pb::PriceBar>, MT5Error> {
        self.price_history_range(symbol, timeframe, 0, 0, max_bars).await
    }

    /// `PriceHistory` with explicit unix-second bounds (both 0 = newest `max_bars`).
    pub async fn price_history_range(
        &self,
        symbol: &str,
        timeframe: i32,
        from_time: i64,
        to_time: i64,
        max_bars: i32,
    ) -> Result<Vec<pb::PriceBar>, MT5Error> {
        self.call(async || {
            let mut c = MarketInfoClient::new(self.channel()?);
            let msg = pb::PriceHistoryRequest {
                symbol: symbol.to_string(),
                timeframe,
                from_time,
                to_time,
                max_bars,
            };
            let r = c.price_history(self.req(msg)?).await?;
            unwrap_reply!(r, price_history_reply).map(|d| d.bars)
        })
        .await
    }

    pub async fn bars(&self, symbol: &str, timeframe: i32, max_bars: i32) -> Result<Vec<Bar>, MT5Error> {
        Ok(self.price_history(symbol, timeframe, max_bars).await?.iter().map(Bar::from).collect())
    }

    // ----------------------------------------------------------------- streams

    /// Raw `SubscriptionService/OnSymbolTick` server stream. Drop the `Streaming` to unsubscribe
    /// (the gRPC call is cancelled). No deadline is applied to the stream.
    pub async fn on_symbol_tick(&self, symbols: &[&str]) -> Result<Streaming<pb::OnSymbolTickReply>, MT5Error> {
        let mut c = SubscriptionServiceClient::new(self.channel()?);
        let msg = pb::OnSymbolTickRequest { symbol_names: symbols.iter().map(|s| s.to_string()).collect() };
        let r = c.on_symbol_tick(self.request(msg, None)?).await?;
        Ok(r.into_inner())
    }

    /// Tick stream mapped to [`Quote`]s on an mpsc channel. The background task ends (and the
    /// receiver closes) on the first stream error or when the receiver is dropped.
    pub async fn subscribe_quotes(&self, symbols: Vec<String>) -> Result<mpsc::Receiver<Quote>, MT5Error> {
        let refs: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
        let mut stream = self.on_symbol_tick(&refs).await?;
        let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            loop {
                match stream.message().await {
                    Ok(Some(reply)) => match reply.response {
                        Some(pb::on_symbol_tick_reply::Response::Data(d)) => {
                            if let Some(t) = d.symbol_tick {
                                if tx.send(Quote::from_subscription_tick(&t)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Some(pb::on_symbol_tick_reply::Response::Error(_)) | None => break,
                    },
                    Ok(None) | Err(_) => break,
                }
            }
        });
        Ok(rx)
    }

    /// Raw `SubscriptionService/OnTrade` stream (order/position/deal events).
    pub async fn on_trade(&self) -> Result<Streaming<pb::OnTradeReply>, MT5Error> {
        let mut c = SubscriptionServiceClient::new(self.channel()?);
        let r = c.on_trade(self.request(pb::OnTradeRequest {}, None)?).await?;
        Ok(r.into_inner())
    }

    // ----------------------------------------------------------------- trading

    /// `TradingHelper/OrderSend` with the raw request (`operation` = `Tmt5EnumOrderType as i32`,
    /// `expert_id` = magic).
    pub async fn order_send_raw(&self, msg: pb::OrderSendRequest) -> Result<pb::OrderSendData, MT5Error> {
        // Trade calls are *not* blindly retried on transient errors: a timed-out OrderSend may
        // have filled. Only terminal-not-found is handled by the caller via `call` if they wish.
        let mut c = TradingHelperClient::new(self.channel()?);
        let r = c.order_send(self.req(msg)?).await?;
        unwrap_reply!(r, order_send_reply)
    }

    /// High-level `OrderSend`. Returns `TradeError(retcode, description)` unless the retcode is
    /// 10008 / 10009 / 10010.
    pub async fn order_send(&self, req: OrderRequest) -> Result<OrderResult, MT5Error> {
        let d = self.order_send_raw(req.to_proto()).await?;
        let res = OrderResult::from_proto(d);
        if res.is_ok() {
            Ok(res)
        } else {
            Err(MT5Error::TradeError(res.retcode, res.retcode_description.clone()))
        }
    }

    /// `TradingHelper/OrderModify` — change SL/TP of a **position** (ticket = position ticket) or
    /// price/SL/TP of a pending order. Passing `None` leaves the field as the gateway's default
    /// (0 = remove).
    pub async fn order_modify(&self, ticket: u64, stop_loss: Option<f64>, take_profit: Option<f64>) -> Result<pb::OrderModifyData, MT5Error> {
        let msg = pb::OrderModifyRequest {
            ticket,
            stop_loss,
            take_profit,
            price: None,
            expiration_time_type: None,
            expiration_time: None,
            stop_limit: None,
        };
        self.order_modify_raw(msg).await
    }

    pub async fn order_modify_raw(&self, msg: pb::OrderModifyRequest) -> Result<pb::OrderModifyData, MT5Error> {
        let mut c = TradingHelperClient::new(self.channel()?);
        let r = c.order_modify(self.req(msg)?).await?;
        let d: pb::OrderModifyData = unwrap_reply!(r, order_modify_reply)?;
        if retcode_ok(d.returned_code) {
            Ok(d)
        } else {
            Err(MT5Error::TradeError(d.returned_code, d.returned_code_description.clone()))
        }
    }

    /// `TradingHelper/OrderClose` — close a position (market) or delete a pending order.
    /// `volume` = lots to close (full position volume for a complete close), `slippage` in points.
    pub async fn order_close(&self, ticket: u64, volume: f64, slippage: i32) -> Result<pb::OrderCloseData, MT5Error> {
        let mut c = TradingHelperClient::new(self.channel()?);
        let msg = pb::OrderCloseRequest { ticket, volume, slippage };
        let r = c.order_close(self.req(msg)?).await?;
        let d: pb::OrderCloseData = unwrap_reply!(r, order_close_reply)?;
        if retcode_ok(d.returned_code) {
            Ok(d)
        } else {
            Err(MT5Error::TradeError(d.returned_code, d.returned_code_description.clone()))
        }
    }

    // ----------------------------------------------------------------- open state

    /// `AccountHelper/OpenedOrders` — pending orders **and** open positions in one reply.
    pub async fn opened_orders(&self) -> Result<pb::OpenedOrdersData, MT5Error> {
        self.call(async || {
            let mut c = AccountHelperClient::new(self.channel()?);
            let msg = pb::OpenedOrdersRequest { input_sort_mode: 0 };
            let r = c.opened_orders(self.req(msg)?).await?;
            unwrap_reply!(r, opened_orders_reply)
        })
        .await
    }

    pub async fn positions(&self) -> Result<Vec<pb::PositionInfo>, MT5Error> {
        Ok(self.opened_orders().await?.position_infos)
    }

    /// Open positions whose `magic_number == magic` — the per-strategy filter.
    pub async fn positions_by_magic(&self, magic: i64) -> Result<Vec<pb::PositionInfo>, MT5Error> {
        Ok(self.positions().await?.into_iter().filter(|p| p.magic_number == magic).collect())
    }

    pub async fn positions_total(&self) -> Result<i32, MT5Error> {
        self.call(async || {
            let mut c = TradeFunctionsClient::new(self.channel()?);
            // PositionsTotal takes google.protobuf.Empty, which prost maps to `()`.
            let r = c.positions_total(self.req(())?).await?;
            unwrap_reply!(r, positions_total_reply).map(|d| d.total_positions)
        })
        .await
    }

    // ----------------------------------------------------------------- history

    /// `AccountHelper/OrderHistory` — closed orders **and deals** between `from` and `to`
    /// (`history_data[i].history_order` / `.history_deal`). Up to `items_per_page` = 1000.
    ///
    /// `from`/`to` are **UTC**; the SDK shifts them into broker server time (the gateway's
    /// `inputFrom`/`inputTo` are server time). Returned `setup_time`/`done_time`/`time` are
    /// server time — use [`MT5Client::to_utc_secs`] to convert back.
    pub async fn order_history(&self, from: SystemTime, to: SystemTime) -> Result<pb::OrdersHistoryData, MT5Error> {
        let shift = self.ensure_shift().await?;
        self.order_history_server_time(shift_time(from, shift), shift_time(to, shift)).await
    }

    /// `OrderHistory` with the window already in broker server time.
    pub async fn order_history_server_time(&self, from: SystemTime, to: SystemTime) -> Result<pb::OrdersHistoryData, MT5Error> {
        self.call(async || {
            let mut c = AccountHelperClient::new(self.channel()?);
            let msg = pb::OrderHistoryRequest {
                input_from: Some(timestamp(from)),
                input_to: Some(timestamp(to)),
                input_sort_mode: pb::Bmt5EnumOrderHistorySortType::Bmt5SortByCloseTimeDesc as i32,
                page_number: 0,
                items_per_page: 1000,
            };
            let r = c.order_history(self.req(msg)?).await?;
            unwrap_reply!(r, order_history_reply)
        })
        .await
    }

    /// `AccountHelper/PositionsHistory` — closed positions opened between `from` and `to`
    /// (open/close price, profit, commission, swap, magic, comment). `from`/`to` are UTC and
    /// are shifted to broker server time like `order_history`.
    pub async fn positions_history(&self, from: SystemTime, to: SystemTime) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        let shift = self.ensure_shift().await?;
        self.positions_history_server_time(shift_time(from, shift), shift_time(to, shift)).await
    }

    /// `PositionsHistory` with the window already in broker server time.
    pub async fn positions_history_server_time(&self, from: SystemTime, to: SystemTime) -> Result<Vec<pb::PositionHistoryInfo>, MT5Error> {
        self.call(async || {
            let mut c = AccountHelperClient::new(self.channel()?);
            let msg = pb::PositionsHistoryRequest {
                sort_type: pb::AhEnumPositionsHistorySortType::AhPositionOpenTimeDesc as i32,
                position_open_time_from: Some(timestamp(from)),
                position_open_time_to: Some(timestamp(to)),
                page_number: None,
                items_per_page: None,
            };
            let r = c.positions_history(self.req(msg)?).await?;
            unwrap_reply!(r, positions_history_reply).map(|d| d.history_positions)
        })
        .await
    }
}
