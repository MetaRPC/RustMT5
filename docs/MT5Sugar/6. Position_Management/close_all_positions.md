# Close All Positions (`close_all_positions`)

> **Sugar method:** Close every open position (optionally only on one symbol); returns how many closed.

**API Information:**

* **Method:** `sugar.close_all_positions(symbol)`
* **Returns:** `Result<usize, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn close_all_positions(&self, symbol: Option<&str>) -> Result<usize, MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `symbol` | `Option<&str>` | Limit to this symbol, or `None` for **all** positions. |

| Output | Type | Description |
|--------|------|-------------|
| `usize` | — | Number of positions successfully closed. |

---

## 🏛️ Essentials

* **What it is:** Iterates open positions and closes each (filtered by symbol if given).
* **Why you need it:** Panic button / end-of-session flatten.
* **Counting:** only positions the server confirmed closed are counted.

> Note: here `symbol` is `Option<&str>` as a direct filter — it is **not** the default-symbol fallback. `None` means "all symbols".

---

## ⚡ Under the Hood

```text
MT5Sugar.close_all_positions(symbol)
  → get_open_positions()                       // MT5Service.opened_orders(...).position_infos
  → for each (optionally matching symbol): close_position(ticket)
  → returns count of successful closes
```

---

## When to Use

* **Flatten everything** at session end or on a risk trigger.
* **Flatten one symbol** before reversing.

---

## 🔗 Usage Examples

### 1) Close everything

```rust
let n = sugar.close_all_positions(None).await?;
println!("closed {n} positions");
```

### 2) Close only EURUSD

```rust
let n = sugar.close_all_positions(Some("EURUSD")).await?;
```

### 3) Risk trigger

```rust
if sugar.floating_profit().await? <= -200.0 {
    sugar.close_all_positions(None).await?;   // cut the day
}
```

---

## Related Methods

* [close_position](close_position.md) · [get_open_positions](../7.%20Position_Information/get_open_positions.md) · [count_open_positions](../7.%20Position_Information/count_open_positions.md)

---

## ⚠️ Pitfalls

* **`None` = ALL.** Unlike price methods, `None` is not the default symbol here — it means every position.
* **Partial failures** are skipped (not counted), not surfaced as errors; compare the returned count with [count_open_positions](../7.%20Position_Information/count_open_positions.md) if it matters.

---

## 📚 See Also

* [close_position](close_position.md) · [get_open_positions](../7.%20Position_Information/get_open_positions.md)
