# 4. Market Depth (DOM) — Overview

> Level-II order book: the queue of buy/sell orders at each price. Subscribe, read snapshots, release.

The Depth of Market (DOM) is a **subscription**, not a one-shot read. The lifecycle is:

1. **[`market_book_add`](./market_book_add.md)** — subscribe to a symbol's book.
2. **[`market_book_get`](./market_book_get.md)** — read the current book snapshot (repeat as needed).
3. **[`market_book_release`](./market_book_release.md)** — unsubscribe when done.

---

## Methods in this group

| Method | Returns | One-liner |
|--------|---------|-----------|
| [`market_book_add(req)`](./market_book_add.md) | `MarketBookAddData` | Subscribe to a symbol's order book (`opened_successfully`). |
| [`market_book_get(req)`](./market_book_get.md) | `MarketBookGetData` | Current book: a vector of price levels with side & volume. |
| [`market_book_release(req)`](./market_book_release.md) | `MarketBookReleaseData` | Unsubscribe (`closed_successfully`). |

---

## When to use which

* **Starting out** → `market_book_add(symbol)` once, check `opened_successfully`.
* **Each refresh** → `market_book_get(symbol)` → iterate `mql_book_infos`.
* **Cleanup** → `market_book_release(symbol)` to stop the feed.

> DOM is only meaningful for instruments the broker provides depth for (typically exchange-traded). Many retail FX feeds return an empty or shallow book.

---

**Back to:** [MT5Account — Master Overview](../MT5Account.Master.Overview.md)
