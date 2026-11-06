# 3. Positions & Orders — Overview

> What you hold now (open positions), what's resting (pending orders), their ticket lists, the closed history, and a P/L helper.

In MT5, **positions** and **orders** are distinct: an order is an instruction (market or pending); a position is the resulting open exposure. Deals are the fills recorded in history. These methods are **read-only**.

---

## Methods in this group

### Current state

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`positions_total()`](./positions_total.md) | `PositionsTotalData` | How many positions are open right now. |
| [`opened_orders(req)`](./opened_orders.md) | `OpenedOrdersData` | Open positions **and** active pending orders, with full detail. |
| [`opened_orders_tickets(req)`](./opened_orders_tickets.md) | `OpenedOrdersTicketsData` | Just the ticket numbers of open orders and positions. |

### History

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`order_history(req)`](./order_history.md) | `OrdersHistoryData` | Historical orders and deals in a time range (paged). |
| [`positions_history(req)`](./positions_history.md) | `PositionsHistoryData` | Closed positions in a time range (paged). |

### P/L helper

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`tick_value_with_size(req)`](./tick_value_with_size.md) | `TickValueWithSizeData` | Tick value/size and contract size for symbols — the inputs for money-per-point P/L. |

---

## When to use which

* **"How many positions do I have?"** → `positions_total()` (a single count).
* **"Show me everything open, with prices and P/L"** → `opened_orders()` — returns both `position_infos` and `opened_orders` (pendings).
* **"I just need the tickets to iterate"** → `opened_orders_tickets()` (lighter payload).
* **"What happened last week?"** → `order_history()` (orders + deals) or `positions_history()` (closed positions).
* **"How much is a point worth per lot?"** → `tick_value_with_size()`.

> `opened_orders()` is the workhorse: its `position_infos` carry live `profit`, `price_current`, SL/TP, swap and more. `MT5Sugar` builds its position helpers on top of it.

---

## ⚠️ Reply-shape note

Most methods here use the standard `oneof { data, error }` reply. **`tick_value_with_size` is different** — its reply carries flat `data` / `error` fields (handled internally by a separate unwrap). You still just get `Result<TickValueWithSizeData, MT5Error>`.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
