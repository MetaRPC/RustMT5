# Connect By Server Name (`connect_by_server_name`)

> **Sugar method:** Connect to the terminal gateway by broker **cluster name** — the recommended way.

**API Information:**

* **Method:** `sugar.connect_by_server_name(server_name, timeout_seconds)`
* **Returns:** `Result<(), MT5Error>`
* **Layer:** HIGH (MT5Sugar) — delegates straight to `MT5Service` → `MT5Account`

---

## Method Signature

```rust
pub async fn connect_by_server_name(
    &mut self,
    server_name: &str,
    timeout_seconds: u32,
) -> Result<(), MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `server_name` | `&str` | Broker server/cluster name (e.g. `"MetaQuotes-Demo"`). |
| `timeout_seconds` | `u32` | How long to wait for the connection. |

| Output | Type | Description |
|--------|------|-------------|
| `()` | — | Unit on success; `MT5Error` on failure. |

---

## 🏛️ Essentials

* **What it is:** Resolves the broker cluster by name and opens the terminal session.
* **Why you need it:** It's the first call — every other method needs an open connection.
* **Recommended:** Prefer this over host/port unless you have a specific address.

---

## ⚡ Under the Hood

```text
MT5Sugar.connect_by_server_name()
  → MT5Service.connect_by_server_name()
    → MT5Account.connect_by_server_name()  (ConnectExRequest → terminal session id)
```

`&mut self` — connecting mutates the wrapped service/account (stores the session id).

---

## When to Use

* **At startup**, before any balance/price/trade call.
* When you know the **broker cluster name** (most common for demo/live logins).

---

## 🔗 Usage Examples

### 1) Basic connect

```rust
let mut sugar = MT5Sugar::new(MT5Service::new(account));
sugar.connect_by_server_name("MetaQuotes-Demo", 30).await?;
println!("connected");
```

### 2) With a default symbol set

```rust
let mut sugar = MT5Sugar::new(MT5Service::new(account)).with_default_symbol("EURUSD");
sugar.connect_by_server_name("MetaQuotes-Demo", 60).await?;
println!("bid {:.5}", sugar.bid(None).await?);  // uses the default symbol
```

### 3) Handle a connection error

```rust
if let Err(e) = sugar.connect_by_server_name("Wrong-Server", 15).await {
    eprintln!("connect failed: {e}");
}
```

---

## Related Methods

* [connect_by_host_port](connect_by_host_port.md) — connect by explicit address
* [ping](ping.md) — check the connection is alive
* [disconnect](disconnect.md) — close it

---

## ⚠️ Pitfalls

* **Needs `&mut`.** The `sugar` binding must be `mut`.
* **Wrong cluster name** → connection error; names are broker-specific.
* **Too-short timeout** on a cold terminal can fail; 30–60 s is safe.

---

## 📚 See Also

* [connect_by_host_port](connect_by_host_port.md) · [ping](ping.md)
* Low-level: [MT5Account — Constructor & connection](../../MT5Account/MT5Account.Master.Overview.md)
