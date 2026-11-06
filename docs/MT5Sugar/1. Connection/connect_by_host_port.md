# Connect By Host/Port (`connect_by_host_port`)

> **Sugar method:** Connect to the terminal gateway by an explicit **host and port**.

**API Information:**

* **Method:** `sugar.connect_by_host_port(host, port, timeout_seconds)`
* **Returns:** `Result<(), MT5Error>`
* **Layer:** HIGH (MT5Sugar) — delegates to `MT5Service` → `MT5Account`

---

## Method Signature

```rust
pub async fn connect_by_host_port(
    &mut self,
    host: &str,
    port: u16,
    timeout_seconds: u32,
) -> Result<(), MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `host` | `&str` | Server address (e.g. `"mt5.broker.com"`). |
| `port` | `u16` | Port (often `443`). |
| `timeout_seconds` | `u32` | Connection timeout. |

| Output | Type | Description |
|--------|------|-------------|
| `()` | — | Unit on success; `MT5Error` otherwise. |

---

## 🏛️ Essentials

* **What it is:** Opens the terminal session to a specific address.
* **Why you need it:** For brokers/setups where you connect by host rather than cluster name.
* **Alternative:** [connect_by_server_name](connect_by_server_name.md) is usually simpler.

---

## ⚡ Under the Hood

```text
MT5Sugar.connect_by_host_port()
  → MT5Service.connect_by_host_port()
    → MT5Account.connect_by_host_port()  (ConnectRequest)
```

---

## When to Use

* You have an **explicit server address** (not just a cluster name).
* Testing against a specific gateway endpoint.

---

## 🔗 Usage Examples

### 1) Connect by address

```rust
let mut sugar = MT5Sugar::new(MT5Service::new(account));
sugar.connect_by_host_port("mt5.broker.com", 443, 30).await?;
```

### 2) Then trade

```rust
sugar.connect_by_host_port("mt5.broker.com", 443, 30).await?;
let bal = sugar.balance().await?;
println!("balance {bal:.2}");
```

### 3) Error handling

```rust
if let Err(e) = sugar.connect_by_host_port("bad.host", 443, 10).await {
    eprintln!("connect failed: {e}");
}
```

---

## Related Methods

* [connect_by_server_name](connect_by_server_name.md) — by cluster name (recommended)
* [ping](ping.md) · [disconnect](disconnect.md)

---

## ⚠️ Pitfalls

* **Needs `&mut`.**
* **Wrong host/port** → connection error.
* Prefer the cluster-name connect unless you specifically need an address.

---

## 📚 See Also

* [connect_by_server_name](connect_by_server_name.md) · [ping](ping.md)
* Low-level: [MT5Account — Constructor & connection](../../MT5Account/MT5Account.Master.Overview.md)
