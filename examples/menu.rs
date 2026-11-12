/*══════════════════════════════════════════════════════════════════════════════
 FILE: examples/menu.rs - INTERACTIVE EXAMPLE LAUNCHER
 PURPOSE:
   A small menu over the other examples. Pick a number and it shells out to
   `cargo run --example <name>` for that demo; `0` / `q` quits.

 🎯 WHO SHOULD USE THIS:
   • Anyone browsing the examples without memorising the --example names
   • A quick entry point to run any demo from one place

 📚 WHAT THIS DEMO COVERS (dispatches to):
      quickstart                            - connect, read account, one trade
      lowlevel_market_data / _trading / _streaming (MT5Account)
      service_demo                          - typed facade (MT5Service)
      sugar_basics / _trading / _positions / _history (MT5Sugar)
      strategies_demo                       - orchestrators
      protobuf_inspector                    - explore proto types (offline)

 ℹ️  SAFE: launcher only - it runs no trades itself; the demo it launches may.
     Connection settings come from examples/0_common/settings.json and/or the
     MT5_LOGIN / MT5_PASSWORD / MT5_SERVER / MRPC_API_KEY env vars (env wins).

 USAGE:
   cargo run --example menu
══════════════════════════════════════════════════════════════════════════════*/

use std::io::{self, Write};
use std::process::Command;

// region: EXAMPLE CATALOG
// ══════════════════════════════════════════════════════════════════
// EXAMPLE CATALOG - menu label -> cargo --example name
// ══════════════════════════════════════════════════════════════════

/// (menu label, `--example` name), grouped by layer via the labels.
const ITEMS: &[(&str, &str)] = &[
    ("Quickstart — connect, read account, place one trade", "quickstart"),
    ("Low-level · market data (MT5Account)", "lowlevel_market_data"),
    ("Low-level · trading (MT5Account)", "lowlevel_trading"),
    ("Low-level · streaming (MT5Account)", "lowlevel_streaming"),
    ("Service · typed facade demo (MT5Service)", "service_demo"),
    ("Sugar · basics — balances, prices, info", "sugar_basics"),
    ("Sugar · trading — orders with SL/TP", "sugar_trading"),
    ("Sugar · positions", "sugar_positions"),
    ("Sugar · history & statistics", "sugar_history"),
    ("Strategies · orchestrators demo", "strategies_demo"),
    ("Tools · Protobuf Inspector (explore proto types — offline)", "protobuf_inspector"),
];
// endregion: EXAMPLE CATALOG

// region: MENU LOOP
// ══════════════════════════════════════════════════════════════════
// MENU LOOP - print the list, read a choice, launch the example
// ══════════════════════════════════════════════════════════════════

// ──────────────────────────────────────────────────────────────────
// main()
//   Loop: print the numbered catalog, read a selection, and spawn
//   `cargo run --example <name>` for the chosen demo until 0/q/EOF.
// ──────────────────────────────────────────────────────────────────
fn main() {
    loop {
        println!("\n╔══════════════════════════════════════════════╗");
        println!("║            RustMT5 — examples menu           ║");
        println!("╚══════════════════════════════════════════════╝");
        for (i, (label, _)) in ITEMS.iter().enumerate() {
            println!("  {:>2}. {label}", i + 1);
        }
        println!("   0. quit");
        print!("\nselect [0-{}]: ", ITEMS.len());
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
            break; // EOF (e.g. piped input ended)
        }
        let choice = line.trim();
        if choice.is_empty() || choice == "0" || choice.eq_ignore_ascii_case("q") {
            println!("bye");
            break;
        }

        let n: usize = match choice.parse() {
            Ok(n) if (1..=ITEMS.len()).contains(&n) => n,
            _ => {
                println!("→ enter a number between 0 and {}", ITEMS.len());
                continue;
            }
        };

        let name = ITEMS[n - 1].1;
        println!("\n▶ cargo run --example {name}\n");
        match Command::new("cargo")
            .args(["run", "--quiet", "--example", name])
            .status()
        {
            Ok(s) if s.success() => println!("\n✓ {name} finished"),
            Ok(s) => println!("\n✗ {name} exited with {s}"),
            Err(e) => println!("\n✗ failed to launch cargo: {e}"),
        }
    }
}
// endregion: MENU LOOP
