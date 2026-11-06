//! `MT5Service` — the mid-level facade over [`MT5Account`].
//!
//! Port of Python `pymt5.mt5_service.MT5Service`: the same operations, but with friendly arguments
//! (plain property enums, symbols, sides) and clean return values — simple getters hand back the
//! inner `f64`/`i64`/`String` instead of the protobuf `…Data` wrapper. Richer helpers (risk sizing,
//! one-liners) live one layer up in `MT5Sugar`.

use crate::account::MT5Account;
use crate::error::MT5Error;
use crate::proto::mt5_term_api as pb;

pub struct MT5Service {
    acc: MT5Account,
}

impl MT5Service {
    /// Wrap an account (connected or not yet).
    pub fn new(acc: MT5Account) -> Self {
        Self { acc }
    }

    /// Borrow the underlying low-level account.
    pub fn account(&self) -> &MT5Account {
        &self.acc
    }
    /// Mutably borrow the underlying low-level account.
    pub fn account_mut(&mut self) -> &mut MT5Account {
        &mut self.acc
    }

    // ===== Connection (delegates to MT5Account) =====

    pub async fn connect_by_server_name(&mut self, server_name: &str, timeout_seconds: u32) -> Result<(), MT5Error> {
        self.acc.connect_by_server_name(server_name, timeout_seconds).await
    }
    pub async fn connect_by_host_port(&mut self, host: &str, port: u16, timeout_seconds: u32) -> Result<(), MT5Error> {
        self.acc.connect_by_host_port(host, port, timeout_seconds).await
    }
    pub async fn reconnect(&mut self) -> Result<(), MT5Error> {
        self.acc.reconnect().await
    }
    pub async fn disconnect(&mut self, _delete: bool) -> Result<(), MT5Error> {
        self.acc.disconnect().await
    }

    // ===== 1. Account information =====

    /// Full account snapshot (balance, equity, margin, leverage, currency, server time).
    pub async fn account_summary(&self) -> Result<pb::AccountSummaryData, MT5Error> {
        self.acc.account_summary().await
    }

    /// One account `double` property (e.g. `ACCOUNT_BALANCE`, `ACCOUNT_EQUITY`) as an `f64`.
    pub async fn account_double(&self, property_id: pb::AccountInfoDoublePropertyType) -> Result<f64, MT5Error> {
        Ok(self
            .acc
            .account_info_double(pb::AccountInfoDoubleRequest { property_id: property_id as i32 })
            .await?
            .requested_value)
    }

    /// One account `integer` property (e.g. `ACCOUNT_LEVERAGE`, `ACCOUNT_LOGIN`) as an `i64`.
    pub async fn account_integer(&self, property_id: pb::AccountInfoIntegerPropertyType) -> Result<i64, MT5Error> {
        Ok(self
            .acc
            .account_info_integer(pb::AccountInfoIntegerRequest { property_id: property_id as i32 })
            .await?
            .requested_value)
    }

    /// One account `string` property (e.g. `ACCOUNT_CURRENCY`, `ACCOUNT_COMPANY`) as a `String`.
    pub async fn account_string(&self, property_id: pb::AccountInfoStringPropertyType) -> Result<String, MT5Error> {
        Ok(self
            .acc
            .account_info_string(pb::AccountInfoStringRequest { property_id: property_id as i32 })
            .await?
            .requested_value)
    }

    // ===== 2. Symbols =====

    /// Number of symbols (all, or only Market Watch when `selected_only`).
    pub async fn symbols_total(&self, selected_only: bool) -> Result<i32, MT5Error> {
        Ok(self.acc.symbols_total(pb::SymbolsTotalRequest { mode: selected_only }).await?.total)
    }
    /// Whether a symbol exists (and whether it is custom).
    pub async fn symbol_exist(&self, name: &str) -> Result<pb::SymbolExistData, MT5Error> {
        self.acc.symbol_exist(pb::SymbolExistRequest { name: name.to_string() }).await
    }
    /// Symbol name at an index (optionally within Market Watch only).
    pub async fn symbol_name(&self, index: i32, selected_only: bool) -> Result<String, MT5Error> {
        Ok(self.acc.symbol_name(pb::SymbolNameRequest { index, selected: selected_only }).await?.name)
    }
    /// Add/remove a symbol in Market Watch; returns whether it succeeded.
    pub async fn symbol_select(&self, symbol: &str, select: bool) -> Result<bool, MT5Error> {
        Ok(self.acc.symbol_select(pb::SymbolSelectRequest { symbol: symbol.to_string(), select }).await?.success)
    }
    /// Whether the symbol's data is synchronized with the server.
    pub async fn symbol_is_synchronized(&self, symbol: &str) -> Result<bool, MT5Error> {
        Ok(self.acc.symbol_is_synchronized(pb::SymbolIsSynchronizedRequest { symbol: symbol.to_string() }).await?.synchronized)
    }
    /// One symbol `double` property (Bid, Ask, Point, …) as an `f64`.
    pub async fn symbol_double(&self, symbol: &str, property: pb::SymbolInfoDoubleProperty) -> Result<f64, MT5Error> {
        Ok(self.acc.symbol_info_double(pb::SymbolInfoDoubleRequest { symbol: symbol.to_string(), r#type: property as i32 }).await?.value)
    }
    /// One symbol `integer` property (Digits, Spread, …) as an `i64`.
    pub async fn symbol_integer(&self, symbol: &str, property: pb::SymbolInfoIntegerProperty) -> Result<i64, MT5Error> {
        Ok(self.acc.symbol_info_integer(pb::SymbolInfoIntegerRequest { symbol: symbol.to_string(), r#type: property as i32 }).await?.value)
    }
    /// One symbol `string` property (Description, Currency, …) as a `String`.
    pub async fn symbol_string(&self, symbol: &str, property: pb::SymbolInfoStringProperty) -> Result<String, MT5Error> {
        Ok(self.acc.symbol_info_string(pb::SymbolInfoStringRequest { symbol: symbol.to_string(), r#type: property as i32 }).await?.value)
    }
    /// Margin rates for a symbol and order type.
    pub async fn symbol_margin_rate(&self, symbol: &str, order_type: pb::EnumOrderType) -> Result<pb::SymbolInfoMarginRateData, MT5Error> {
        self.acc.symbol_info_margin_rate(pb::SymbolInfoMarginRateRequest { symbol: symbol.to_string(), order_type: order_type as i32 }).await
    }
    /// The latest tick for a symbol.
    pub async fn symbol_tick(&self, symbol: &str) -> Result<pb::MrpcMqlTick, MT5Error> {
        self.acc.symbol_info_tick(pb::SymbolInfoTickRequest { symbol: symbol.to_string() }).await
    }
    /// Quote-session times for a symbol on a given weekday / session index.
    pub async fn symbol_session_quote(&self, symbol: &str, day: pb::DayOfWeek, session_index: u32) -> Result<pb::SymbolInfoSessionQuoteData, MT5Error> {
        self.acc.symbol_info_session_quote(pb::SymbolInfoSessionQuoteRequest { symbol: symbol.to_string(), day_of_week: day as i32, session_index }).await
    }
    /// Trade-session times for a symbol on a given weekday / session index.
    pub async fn symbol_session_trade(&self, symbol: &str, day: pb::DayOfWeek, session_index: u32) -> Result<pb::SymbolInfoSessionTradeData, MT5Error> {
        self.acc.symbol_info_session_trade(pb::SymbolInfoSessionTradeRequest { symbol: symbol.to_string(), day_of_week: day as i32, session_index }).await
    }
    /// Full parameters for one symbol (`Some(name)`) or many (`None`), with optional paging.
    pub async fn symbol_params_many(&self, symbol_name: Option<String>, page_number: Option<i32>, items_per_page: Option<i32>) -> Result<pb::SymbolParamsManyData, MT5Error> {
        self.acc.symbol_params_many(pb::SymbolParamsManyRequest { symbol_name, sort_type: None, page_number, items_per_page }).await
    }

    // ===== 3. Positions & orders =====

    /// Number of open positions.
    pub async fn positions_total(&self) -> Result<i32, MT5Error> {
        Ok(self.acc.positions_total().await?.total_positions)
    }
    /// All opened orders (market + pending), in the requested sort order.
    pub async fn opened_orders(&self, sort_mode: pb::Bmt5EnumOpenedOrderSortType) -> Result<pb::OpenedOrdersData, MT5Error> {
        self.acc.opened_orders(pb::OpenedOrdersRequest { input_sort_mode: sort_mode as i32 }).await
    }
    /// The ticket ids of all opened orders (lightweight).
    pub async fn opened_orders_tickets(&self) -> Result<pb::OpenedOrdersTicketsData, MT5Error> {
        self.acc.opened_orders_tickets(pb::OpenedOrdersTicketsRequest::default()).await
    }
    /// Closed-order history over a window, with sort and paging.
    pub async fn order_history(
        &self,
        sort_mode: pb::Bmt5EnumOrderHistorySortType,
        from: Option<prost_types::Timestamp>,
        to: Option<prost_types::Timestamp>,
        page_number: i32,
        items_per_page: i32,
    ) -> Result<pb::OrdersHistoryData, MT5Error> {
        self.acc
            .order_history(pb::OrderHistoryRequest {
                input_from: from,
                input_to: to,
                input_sort_mode: sort_mode as i32,
                page_number,
                items_per_page,
                ..Default::default()
            })
            .await
    }
    /// Closed-position history over a window, with sort and paging.
    pub async fn positions_history(
        &self,
        sort_type: pb::AhEnumPositionsHistorySortType,
        from: Option<prost_types::Timestamp>,
        to: Option<prost_types::Timestamp>,
        page_number: Option<i32>,
        items_per_page: Option<i32>,
    ) -> Result<pb::PositionsHistoryData, MT5Error> {
        self.acc
            .positions_history(pb::PositionsHistoryRequest {
                sort_type: sort_type as i32,
                position_open_time_from: from,
                position_open_time_to: to,
                page_number,
                items_per_page,
                ..Default::default()
            })
            .await
    }

    // ===== 4. Market Depth (DOM) =====

    /// Subscribe to a symbol's market book (DOM).
    pub async fn subscribe_market_depth(&self, symbol: &str) -> Result<pb::MarketBookAddData, MT5Error> {
        self.acc.market_book_add(pb::MarketBookAddRequest { symbol: symbol.to_string() }).await
    }
    /// Release a symbol's market-book subscription.
    pub async fn unsubscribe_market_depth(&self, symbol: &str) -> Result<pb::MarketBookReleaseData, MT5Error> {
        self.acc.market_book_release(pb::MarketBookReleaseRequest { symbol: symbol.to_string() }).await
    }
    /// Current market-book (DOM) snapshot for a symbol.
    pub async fn market_depth(&self, symbol: &str) -> Result<pb::MarketBookGetData, MT5Error> {
        self.acc.market_book_get(pb::MarketBookGetRequest { symbol: symbol.to_string() }).await
    }

    // ===== 6. Streaming =====

    /// Live tick stream for the given symbols.
    pub async fn stream_ticks(
        &self,
        symbols: Vec<String>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnSymbolTickData, MT5Error>>, MT5Error> {
        self.acc.on_symbol_tick(pb::OnSymbolTickRequest { symbol_names: symbols }).await
    }
    /// Live trade-event stream.
    pub async fn stream_trade_updates(
        &self,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeData, MT5Error>>, MT5Error> {
        self.acc.on_trade(pb::OnTradeRequest::default()).await
    }
    /// Live per-position profit stream, emitted every `timer_period_ms`.
    pub async fn stream_position_profits(
        &self,
        timer_period_ms: i32,
        ignore_empty: bool,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionProfitData, MT5Error>>, MT5Error> {
        self.acc
            .on_position_profit(pb::OnPositionProfitRequest {
                timer_period_milliseconds: timer_period_ms,
                ignore_empty_data: ignore_empty,
            })
            .await
    }
    /// Live open-position & pending-order ticket stream, emitted every `timer_period_ms`.
    pub async fn stream_opened_tickets(
        &self,
        timer_period_ms: i32,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnPositionsAndPendingOrdersTicketsData, MT5Error>>, MT5Error> {
        self.acc
            .on_positions_and_pending_orders_tickets(pb::OnPositionsAndPendingOrdersTicketsRequest {
                timer_period_milliseconds: timer_period_ms,
            })
            .await
    }
    /// Live trade-transaction stream.
    pub async fn stream_transactions(
        &self,
    ) -> Result<impl tokio_stream::Stream<Item = Result<pb::OnTradeTransactionData, MT5Error>>, MT5Error> {
        self.acc.on_trade_transaction(pb::OnTradeTransactionRequest::default()).await
    }

    // ===== 5. Trading =====
    //
    // Market/pending order construction (symbol/side/volume → OrderSendRequest) is the job of
    // `MT5Sugar`; at this layer `place_order`/`modify_order`/`close_order`/`check_order` take the
    // request directly, mirroring the Python facade.

    /// Send a market or pending order.
    pub async fn place_order(&self, request: pb::OrderSendRequest) -> Result<pb::OrderSendData, MT5Error> {
        self.acc.order_send(request).await
    }
    /// Modify an existing order's SL/TP/price/expiration.
    pub async fn modify_order(&self, request: pb::OrderModifyRequest) -> Result<pb::OrderModifyData, MT5Error> {
        self.acc.order_modify(request).await
    }
    /// Close a position (opposite market order) or delete a pending order.
    pub async fn close_order(&self, request: pb::OrderCloseRequest) -> Result<pb::OrderCloseData, MT5Error> {
        self.acc.order_close(request).await
    }
    /// Validate a trade request before sending it.
    pub async fn check_order(&self, request: pb::OrderCheckRequest) -> Result<pb::OrderCheckData, MT5Error> {
        self.acc.order_check(request).await
    }
    /// Required margin for an order type / symbol / volume at a given price.
    pub async fn calculate_margin(
        &self,
        symbol: &str,
        order_type: pb::EnumOrderTypeTf,
        volume: f64,
        open_price: f64,
    ) -> Result<pb::OrderCalcMarginData, MT5Error> {
        self.acc
            .order_calc_margin(pb::OrderCalcMarginRequest {
                symbol: symbol.to_string(),
                order_type: order_type as i32,
                volume,
                open_price,
            })
            .await
    }
    /// Expected profit of a hypothetical trade between open and close prices.
    pub async fn calculate_profit(
        &self,
        symbol: &str,
        order_type: pb::EnumOrderTypeTf,
        volume: f64,
        open_price: f64,
        close_price: f64,
    ) -> Result<pb::OrderCalcProfitData, MT5Error> {
        self.acc
            .order_calc_profit(pb::OrderCalcProfitRequest {
                order_type: order_type as i32,
                symbol: symbol.to_string(),
                volume,
                open_price,
                close_price,
            })
            .await
    }
}
