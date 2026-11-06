# Server UTC Shift (`server_utc_shift_secs`)

> **Sugar method:** Broker **server** time minus **UTC**, in seconds — for converting gateway timestamps to UTC.

**API Information:**

* **Method:** `sugar.server_utc_shift_secs()`
* **Returns:** `Result<i64, MT5Error>`
* **Layer:** HIGH (MT5Sugar)

---

## Method Signature

```rust
pub async fn server_utc_shift_secs(&self) -> Result<i64, MT5Error>
```

Paired free functions (module `metarpc_mt5::sugar`):

```rust
pub fn unix_secs(ts: &Option<prost_types::Timestamp>) -> i64;
pub fn to_utc_secs(server_ts: &Option<prost_types::Timestamp>, server_utc_shift_secs: i64) -> i64;
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| *None* | — | No parameters. |

| Output | Type | Description |
|--------|------|-------------|
| `i64` | — | `utc_timezone_server_time_shift_minutes × 60` from the account summary. |

---

## 🏛️ Essentials

* **What it is:** The server-to-UTC offset in seconds (derived from `account_summary`).
* **Why you need it:** Gateway timestamps are in **broker server time**; subtract this shift to get UTC.
* **Fetch once, reuse:** the shift is stable for a session — read it once and pass it to `to_utc_secs`.

---

## ⚡ Under the Hood

```text
MT5Sugar.server_utc_shift_secs()
  → MT5Service.account_summary().utc_timezone_server_time_shift_minutes * 60
```

---

## 🔗 Usage Examples

### 1) Convert a position's open time to UTC

```rust
use metarpc_mt5::sugar::to_utc_secs;

let shift = sugar.server_utc_shift_secs().await?;
if let Some(p) = sugar.get_open_positions().await?.first() {
    let utc = to_utc_secs(&p.open_time, shift);
    println!("opened at {utc} (unix UTC seconds)");
}
```

### 2) Raw unix seconds (no shift)

```rust
use metarpc_mt5::sugar::unix_secs;

let s = unix_secs(&None); // 0 when absent
println!("{s}");
```

### 3) To a chrono DateTime

```rust
use metarpc_mt5::sugar::to_utc_secs;
use chrono::{TimeZone, Utc};

let shift = sugar.server_utc_shift_secs().await?;
let p = sugar.get_open_positions().await?;
if let Some(pos) = p.first() {
    let dt = Utc.timestamp_opt(to_utc_secs(&pos.open_time, shift), 0).single();
    println!("opened {dt:?}");
}
```

---

## Related Methods

* [get_account_info](get_account_info.md) · low-level [account_summary](../../MT5Account/1.%20Account_Information/account_summary.md)

---

## ⚠️ Pitfalls

* **Server time vs UTC.** History/position timestamps are server time — convert before comparing with `Utc::now()`.
* **Fetch the shift once.** Don't call it per timestamp; it costs an `account_summary` round-trip.

---

## 📚 See Also

* [get_account_info](get_account_info.md) · [get_deals](../8.%20History_Statistics/get_deals.md)
