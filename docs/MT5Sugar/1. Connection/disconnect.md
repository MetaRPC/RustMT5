# Disconnect (`disconnect`)

> **Sugar method:** Close the terminal connection.

**API Information:**

* **Method:** `sugar.disconnect(delete)`
* **Returns:** `Result<(), MT5Error>`
* **Layer:** HIGH (MT5Sugar) — delegates to `MT5Service` → `MT5Account`

---

## Method Signature

```rust
pub async fn disconnect(&mut self, delete: bool) -> Result<(), MT5Error>
```

---

## ℹ️ Input / Output

| Input | Type | Description |
|-------|------|-------------|
| `delete` | `bool` | Mirrors the Python facade's flag. Accepted for API parity; the low-level `disconnect()` takes no argument, so the channel is simply dropped. |

| Output | Type | Description |
|--------|------|-------------|
| `()` | — | Unit on success. |

---

## 🏛️ Essentials

* **What it is:** Ends the session and releases the channel.
* **Why you need it:** Clean shutdown at the end of a program/task.
* **Note:** `delete` is kept only for signature parity with the Python Sugar; it does not change behavior here.

---

## ⚡ Under the Hood

```text
MT5Sugar.disconnect(delete)
  → MT5Service.disconnect(delete)   // delete ignored
    → MT5Account.disconnect()       // drops the channel
```

---

## When to Use

* **On shutdown** of a script or worker.
* Before re-creating a connection from scratch.

---

## 🔗 Usage Examples

### 1) Clean shutdown

```rust
sugar.disconnect(false).await?;
```

### 2) In a run function

```rust
async fn run(sugar: &mut MT5Sugar) -> Result<(), MT5Error> {
    // ... do work ...
    sugar.disconnect(false).await
}
```

### 3) Ignore errors on teardown

```rust
let _ = sugar.disconnect(true).await;  // best-effort cleanup
```

---

## Related Methods

* [connect_by_server_name](connect_by_server_name.md) · [connect_by_host_port](connect_by_host_port.md)
* [ping](ping.md)

---

## ⚠️ Pitfalls

* **Needs `&mut`.**
* **`delete` is a no-op here** — don't rely on it to delete anything server-side.

---

## 📚 See Also

* [ping](ping.md) · [connect_by_server_name](connect_by_server_name.md)
