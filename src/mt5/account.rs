//! `MT5Account` — the low-level async gRPC client over the MRPC MT5 gateway.
//!
//! A faithful Rust port of the Python `MetaRpcMT5.MT5Account`: the same method set, the same
//! metadata (`apikey` + terminal `id`) on every call, and the same "return the reply's `data`
//! or an [`MT5Error`]" contract. Streaming and the full method surface are added incrementally.

use crate::error::MT5Error;
use crate::proto::mt5_term_api as pb;
use pb::account_helper_client::AccountHelperClient;
use pb::account_information_client::AccountInformationClient;
use pb::connection_client::ConnectionClient;
use pb::market_info_client::MarketInfoClient;
use pb::trade_functions_client::TradeFunctionsClient;
use pb::trading_helper_client::TradingHelperClient;
use tonic::transport::{Channel, ClientTlsConfig};

/// Unwrap a `…Reply { response: Option<oneof{ Data, Error }> }` into `Result<Data, MT5Error>`.
///
/// `$module` is the generated reply module, e.g. `account_summary_reply`.
macro_rules! take_data {
    ($reply:expr, $module:ident) => {{
        use $crate::proto::mt5_term_api::$module as m;
        match $reply.response {
            Some(m::Response::Data(d)) => Ok(d),
            Some(m::Response::Error(e)) => Err($crate::error::MT5Error::from(e)),
            None => Err($crate::error::MT5Error::EmptyResponse),
        }
    }};
}

/// A unary RPC: build the named client from the channel, attach auth metadata, call `$m`, unwrap.
macro_rules! unary {
    ($self:ident, $client:ident, $m:ident, $req:expr, $mod:ident) => {{
        let mut c = $client::new($self.channel()?);
        let reply = c.$m($self.authed($req)?).await?.into_inner();
        take_data!(reply, $mod)
    }};
}

/// Some replies use a flat `{ data: Option<Data>, error: Option<Error> }` shape instead of a oneof.
macro_rules! take_data_b {
    ($reply:expr) => {{
        match ($reply.data, $reply.error) {
            (Some(d), _) => Ok(d),
            (None, Some(e)) => Err($crate::error::MT5Error::from(e)),
            (None, None) => Err($crate::error::MT5Error::EmptyResponse),
        }
    }};
}
macro_rules! unary_b {
    ($self:ident, $client:ident, $m:ident, $req:expr) => {{
        let mut c = $client::new($self.channel()?);
        let reply = c.$m($self.authed($req)?).await?.into_inner();
        take_data_b!(reply)
    }};
}

/// A server-streaming RPC on `SubscriptionServiceClient`: returns a stream of `Result<Data, MT5Error>`.
macro_rules! stream_rpc {
    ($self:ident, $m:ident, $req:expr, $mod:ident) => {{
        let mut c = pb::subscription_service_client::SubscriptionServiceClient::new($self.channel()?);
        let inner = c.$m($self.authed($req)?).await?.into_inner();
        Ok(::tokio_stream::StreamExt::map(inner, |item| match item {
            Ok(reply) => take_data!(reply, $mod),
            Err(status) => Err($crate::error::MT5Error::from(status)),
        }))
    }};
}

/// Low-level MT5 account client. Build with [`MT5Account::new`], connect, then call methods.
#[allow(dead_code)]
pub struct MT5Account {
    user: u64,
    password: String,
    grpc_server: String,
    api_key: String,

    /// Terminal session id, assigned by the gateway on connect and sent with every later call.
    id: Option<String>,
    server_name: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    connect_timeout_seconds: u32,
    terminal_name: Option<String>,

    channel: Option<Channel>,
}

impl MT5Account {
    /// Create a client. `grpc_server` defaults to `mt5.mrpc.pro:443`; the API key is taken from
    /// `MRPC_API_KEY` or falls back to `"TRIAL"`.
    pub fn new(user: u64, password: impl Into<String>, grpc_server: Option<String>) -> Self {
        let grpc_server = grpc_server.unwrap_or_else(|| "mt5.mrpc.pro:443".to_string());
        let api_key = std::env::var("MRPC_API_KEY")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "TRIAL".to_string());
        Self {
            user,
            password: password.into(),
            grpc_server,
            api_key,
            id: None,
            server_name: None,
            host: None,
            port: None,
            connect_timeout_seconds: 30,
            terminal_name: None,
            channel: None,
        }
    }

    /// The terminal session id, once connected.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// The broker server name used for the last `connect_by_server_name`, if any.
    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    // ===== internals =====

    /// Ensure the TLS gRPC channel to the gateway exists.
    async fn ensure_channel(&mut self) -> Result<(), MT5Error> {
        if self.channel.is_some() {
            return Ok(());
        }
        let uri = format!("https://{}", self.grpc_server);
        let channel = Channel::from_shared(uri)
            .map_err(|e| MT5Error::Connection(e.to_string()))?
            .tls_config(ClientTlsConfig::new())
            .map_err(|e| MT5Error::Connection(e.to_string()))?
            .connect()
            .await?;
        self.channel = Some(channel);
        Ok(())
    }

    fn channel(&self) -> Result<Channel, MT5Error> {
        self.channel.clone().ok_or(MT5Error::NotConnected)
    }

    /// Wrap a message in a request carrying the `apikey` (+ `id` once connected) metadata.
    fn authed<T>(&self, msg: T) -> Result<tonic::Request<T>, MT5Error> {
        let mut req = tonic::Request::new(msg);
        let md = req.metadata_mut();
        let key = tonic::metadata::MetadataValue::try_from(self.api_key.as_str())
            .map_err(|_| MT5Error::InvalidArgument("api_key".into()))?;
        md.insert("apikey", key);
        if let Some(id) = &self.id {
            let idv = tonic::metadata::MetadataValue::try_from(id.as_str())
                .map_err(|_| MT5Error::InvalidArgument("id".into()))?;
            md.insert("id", idv);
        }
        Ok(req)
    }

    // ===== connection =====

    /// Connect by broker server/cluster name (the recommended path). Stores the terminal `id`.
    pub async fn connect_by_server_name(
        &mut self,
        server_name: &str,
        timeout_seconds: u32,
    ) -> Result<(), MT5Error> {
        self.server_name = Some(server_name.to_string());
        self.connect_timeout_seconds = timeout_seconds;
        self.ensure_channel().await?;

        let mut client = ConnectionClient::new(self.channel()?);
        let req = self.authed(pb::ConnectExRequest {
            user: self.user,
            password: self.password.clone(),
            mt_cluster_name: server_name.to_string(),
            timeout_seconds: Some(timeout_seconds),
            name: self.terminal_name.clone(),
            ..Default::default()
        })?;
        let reply = client.connect_ex(req).await?.into_inner();
        let data = take_data!(reply, connect_ex_reply)?;
        self.id = Some(data.terminal_instance_guid);
        Ok(())
    }

    /// Connect by terminal host/port (an explicit gateway-reachable MT5 terminal address).
    pub async fn connect_by_host_port(
        &mut self,
        host: &str,
        port: u16,
        timeout_seconds: u32,
    ) -> Result<(), MT5Error> {
        self.host = Some(host.to_string());
        self.port = Some(port);
        self.connect_timeout_seconds = timeout_seconds;
        self.ensure_channel().await?;

        let mut client = ConnectionClient::new(self.channel()?);
        let req = self.authed(pb::ConnectRequest {
            user: self.user,
            password: self.password.clone(),
            host: host.to_string(),
            port: port as i32,
            timeout_seconds: Some(timeout_seconds),
            name: self.terminal_name.clone(),
            ..Default::default()
        })?;
        let reply = client.connect(req).await?.into_inner();
        let data = take_data!(reply, connect_reply)?;
        self.id = Some(data.terminal_instance_guid);
        Ok(())
    }

    /// Re-establish the session using the last connect parameters.
    pub async fn reconnect(&mut self) -> Result<(), MT5Error> {
        self.channel = None;
        if let Some(name) = self.server_name.clone() {
            self.connect_by_server_name(&name, self.connect_timeout_seconds).await
        } else if let (Some(h), Some(p)) = (self.host.clone(), self.port) {
            self.connect_by_host_port(&h, p, self.connect_timeout_seconds).await
        } else {
            Err(MT5Error::NotConnected)
        }
    }

    /// Release the session (drops the channel; the terminal id is forgotten).
    pub async fn disconnect(&mut self) -> Result<(), MT5Error> {
        self.channel = None;
        self.id = None;
        Ok(())
    }

    // ===== 1. Account information =====

    /// Full account snapshot — balance, equity, margin, leverage, currency, server time.
    pub async fn account_summary(&self) -> Result<pb::AccountSummaryData, MT5Error> {
        let mut client = AccountHelperClient::new(self.channel()?);
        let req = self.authed(pb::AccountSummaryRequest {})?;
        let reply = client.account_summary(req).await?.into_inner();
        take_data!(reply, account_summary_reply)
    }

    pub async fn account_info_double(&self, request: pb::AccountInfoDoubleRequest) -> Result<pb::AccountInfoDoubleData, MT5Error> {
        unary!(self, AccountInformationClient, account_info_double, request, account_info_double_reply)
    }
    pub async fn account_info_integer(&self, request: pb::AccountInfoIntegerRequest) -> Result<pb::AccountInfoIntegerData, MT5Error> {
        unary!(self, AccountInformationClient, account_info_integer, request, account_info_integer_reply)
    }
    pub async fn account_info_string(&self, request: pb::AccountInfoStringRequest) -> Result<pb::AccountInfoStringData, MT5Error> {
        unary!(self, AccountInformationClient, account_info_string, request, account_info_string_reply)
    }

    // ===== 2. Symbols =====

    pub async fn symbols_total(&self, request: pb::SymbolsTotalRequest) -> Result<pb::SymbolsTotalData, MT5Error> {
        unary!(self, MarketInfoClient, symbols_total, request, symbols_total_reply)
    }
    pub async fn symbol_exist(&self, request: pb::SymbolExistRequest) -> Result<pb::SymbolExistData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_exist, request, symbol_exist_reply)
    }
    pub async fn symbol_name(&self, request: pb::SymbolNameRequest) -> Result<pb::SymbolNameData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_name, request, symbol_name_reply)
    }
    pub async fn symbol_select(&self, request: pb::SymbolSelectRequest) -> Result<pb::SymbolSelectData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_select, request, symbol_select_reply)
    }
    pub async fn symbol_is_synchronized(&self, request: pb::SymbolIsSynchronizedRequest) -> Result<pb::SymbolIsSynchronizedData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_is_synchronized, request, symbol_is_synchronized_reply)
    }
    pub async fn symbol_info_double(&self, request: pb::SymbolInfoDoubleRequest) -> Result<pb::SymbolInfoDoubleData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_double, request, symbol_info_double_reply)
    }
    pub async fn symbol_info_integer(&self, request: pb::SymbolInfoIntegerRequest) -> Result<pb::SymbolInfoIntegerData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_integer, request, symbol_info_integer_reply)
    }
    pub async fn symbol_info_string(&self, request: pb::SymbolInfoStringRequest) -> Result<pb::SymbolInfoStringData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_string, request, symbol_info_string_reply)
    }
    pub async fn symbol_info_margin_rate(&self, request: pb::SymbolInfoMarginRateRequest) -> Result<pb::SymbolInfoMarginRateData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_margin_rate, request, symbol_info_margin_rate_reply)
    }
    pub async fn symbol_info_tick(&self, request: pb::SymbolInfoTickRequest) -> Result<pb::MrpcMqlTick, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_tick, request, symbol_info_tick_request_reply)
    }
    pub async fn symbol_info_session_quote(&self, request: pb::SymbolInfoSessionQuoteRequest) -> Result<pb::SymbolInfoSessionQuoteData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_session_quote, request, symbol_info_session_quote_reply)
    }
    pub async fn symbol_info_session_trade(&self, request: pb::SymbolInfoSessionTradeRequest) -> Result<pb::SymbolInfoSessionTradeData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_info_session_trade, request, symbol_info_session_trade_reply)
    }
    pub async fn symbol_params_many(&self, request: pb::SymbolParamsManyRequest) -> Result<pb::SymbolParamsManyData, MT5Error> {
        unary!(self, AccountHelperClient, symbol_params_many, request, symbol_params_many_reply)
    }
    pub async fn symbol_list(&self, request: pb::SymbolListRequest) -> Result<pb::SymbolListData, MT5Error> {
        unary!(self, MarketInfoClient, symbol_list, request, symbol_list_reply)
    }

    // ===== 3. Quotes / market data =====

    pub async fn price_history(&self, request: pb::PriceHistoryRequest) -> Result<pb::PriceHistoryData, MT5Error> {
        unary!(self, MarketInfoClient, price_history, request, price_history_reply)
    }
    pub async fn tick_value_with_size(&self, request: pb::TickValueWithSizeRequest) -> Result<pb::TickValueWithSizeData, MT5Error> {
        unary_b!(self, AccountHelperClient, tick_value_with_size, request)
    }

    // ===== 4. Market Book (DOM) =====

    pub async fn market_book_add(&self, request: pb::MarketBookAddRequest) -> Result<pb::MarketBookAddData, MT5Error> {
        unary!(self, MarketInfoClient, market_book_add, request, market_book_add_reply)
    }
    pub async fn market_book_get(&self, request: pb::MarketBookGetRequest) -> Result<pb::MarketBookGetData, MT5Error> {
        unary!(self, MarketInfoClient, market_book_get, request, market_book_get_reply)
    }
    pub async fn market_book_release(&self, request: pb::MarketBookReleaseRequest) -> Result<pb::MarketBookReleaseData, MT5Error> {
        unary!(self, MarketInfoClient, market_book_release, request, market_book_release_reply)
    }

    // ===== 5. Positions & orders (read) =====

    pub async fn positions_total(&self) -> Result<pb::PositionsTotalData, MT5Error> {
        unary!(self, TradeFunctionsClient, positions_total, (), positions_total_reply)
    }
    pub async fn opened_orders(&self, request: pb::OpenedOrdersRequest) -> Result<pb::OpenedOrdersData, MT5Error> {
        unary!(self, AccountHelperClient, opened_orders, request, opened_orders_reply)
    }
    pub async fn opened_orders_tickets(&self, request: pb::OpenedOrdersTicketsRequest) -> Result<pb::OpenedOrdersTicketsData, MT5Error> {
        unary!(self, AccountHelperClient, opened_orders_tickets, request, opened_orders_tickets_reply)
    }

    // ===== 6. History =====

    pub async fn order_history(&self, request: pb::OrderHistoryRequest) -> Result<pb::OrdersHistoryData, MT5Error> {
        unary!(self, AccountHelperClient, order_history, request, order_history_reply)
    }
    pub async fn positions_history(&self, request: pb::PositionsHistoryRequest) -> Result<pb::PositionsHistoryData, MT5Error> {
        unary!(self, AccountHelperClient, positions_history, request, positions_history_reply)
    }

    // ===== 7. Trading =====

    pub async fn order_send(&self, request: pb::OrderSendRequest) -> Result<pb::OrderSendData, MT5Error> {
        unary!(self, TradingHelperClient, order_send, request, order_send_reply)
    }
    pub async fn order_modify(&self, request: pb::OrderModifyRequest) -> Result<pb::OrderModifyData, MT5Error> {
        unary!(self, TradingHelperClient, order_modify, request, order_modify_reply)
    }
    pub async fn order_close(&self, request: pb::OrderCloseRequest) -> Result<pb::OrderCloseData, MT5Error> {
        unary!(self, TradingHelperClient, order_close, request, order_close_reply)
    }
    pub async fn order_check(&self, request: pb::OrderCheckRequest) -> Result<pb::OrderCheckData, MT5Error> {
        unary!(self, TradeFunctionsClient, order_check, request, order_check_reply)
    }
    pub async fn order_calc_margin(&self, request: pb::OrderCalcMarginRequest) -> Result<pb::OrderCalcMarginData, MT5Error> {
        unary!(self, TradeFunctionsClient, order_calc_margin, request, order_calc_margin_reply)
    }
    pub async fn order_calc_profit(&self, request: pb::OrderCalcProfitRequest) -> Result<pb::OrderCalcProfitData, MT5Error> {
        unary!(self, TradeFunctionsClient, order_calc_profit, request, order_calc_profit_reply)
    }

    // ===== 8. Streaming (server streams) =====

    /// Live tick updates for the requested symbols.
    pub async fn on_symbol_tick(
        &self,
        request: pb::OnSymbolTickRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnSymbolTickData, MT5Error>>, MT5Error> {
        stream_rpc!(self, on_symbol_tick, request, on_symbol_tick_reply)
    }

    /// Trade events (orders/positions/deals changes).
    pub async fn on_trade(
        &self,
        request: pb::OnTradeRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeData, MT5Error>>, MT5Error> {
        stream_rpc!(self, on_trade, request, on_trade_reply)
    }

    /// Live per-position floating profit updates.
    pub async fn on_position_profit(
        &self,
        request: pb::OnPositionProfitRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionProfitData, MT5Error>>, MT5Error> {
        stream_rpc!(self, on_position_profit, request, on_position_profit_reply)
    }

    /// Live open-position and pending-order ticket lists.
    pub async fn on_positions_and_pending_orders_tickets(
        &self,
        request: pb::OnPositionsAndPendingOrdersTicketsRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionsAndPendingOrdersTicketsData, MT5Error>>, MT5Error> {
        stream_rpc!(self, on_positions_and_pending_orders_tickets, request, on_positions_and_pending_orders_tickets_reply)
    }

    /// Trade-transaction events (the MT5 `OnTradeTransaction` feed).
    pub async fn on_trade_transaction(
        &self,
        request: pb::OnTradeTransactionRequest,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeTransactionData, MT5Error>>, MT5Error> {
        stream_rpc!(self, on_trade_transaction, request, on_trade_transaction_reply)
    }
}
