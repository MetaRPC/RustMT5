//! Interactive Protobuf Inspector for the MT4 gRPC API (offline).
//!
//! Explore the `mt5_term_api` protobuf messages, fields and enums without a terminal connection.
//! Use it to answer "what is the exact field name?", "what proto type is it?", "which enum value
//! do I pass?" — straight from the generated code.
//!
//! Unlike the C#/Go/Java inspectors, `prost` keeps **no** runtime descriptors and this crate ships
//! no `.proto` files — only the pre-generated `package/mt5_term_api.rs`. So instead of reflecting,
//! this tool parses that generated file (embedded at compile time with `include_str!`). Every field
//! carries its `#[prost(..)]` attribute, so the output includes the proto wire type and field tag —
//! a little more than reflection alone would give.
//!
//! Run:  `cargo run --example protobuf_inspector`   (or pick it from `cargo run --example menu`)
//!
//! ```text
//!   COMMAND            DESCRIPTION
//!   ─────────────────────────────────────────────────────────────
//!   list, ls           list all message types
//!   <TypeName>         inspect a message — its fields and types
//!   search <text>      find types whose name contains text (alias: find)
//!   field <name>       find messages that contain a field matching name
//!   enum <name>        show every value of an enum
//!   enums              list all enum types
//!   help, ?            command reference
//!   exit, quit, q      leave
//! ```

use std::collections::BTreeMap;
use std::io::{self, Write};

/// The generated prost code, embedded verbatim. Path is relative to THIS source file.
const GEN: &str = include_str!("../../package/mt5_term_api.rs");

#[derive(Clone)]
struct Field {
    name: String,
    tag: i32,
    /// Human-readable proto type, e.g. `double`, `repeated OpenedOrder`, `MrpcEnumAccountTradeMode`.
    type_label: String,
}

#[derive(Clone)]
struct Message {
    name: String,
    fields: Vec<Field>,
}

#[derive(Clone)]
struct EnumDef {
    name: String,
    values: Vec<(String, i64)>,
}

struct Model {
    messages: BTreeMap<String, Message>, // keyed by lowercase name
    enums: BTreeMap<String, EnumDef>,    // keyed by lowercase name
}

fn main() {
    let model = parse(GEN);
    print_header(&model);
    print_quick_start();

    let stdin = io::stdin();
    loop {
        print!("\n> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            return; // EOF
        }
        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        let (cmd, arg) = split_once_ws(input);
        match cmd.to_lowercase().as_str() {
            "exit" | "quit" | "q" => {
                println!("\n👋 Goodbye!");
                return;
            }
            "help" | "?" => print_help(),
            "list" | "ls" => list_types(&model),
            "enums" => list_enums(&model),
            "search" | "find" => {
                if arg.is_empty() {
                    println!("❌ Usage: search <text>");
                } else {
                    search_types(&model, arg);
                }
            }
            "field" => {
                if arg.is_empty() {
                    println!("❌ Usage: field <fieldname>");
                } else {
                    find_field(&model, arg);
                }
            }
            "enum" => {
                if arg.is_empty() {
                    println!("❌ Usage: enum <enumname>");
                } else {
                    inspect_enum(&model, arg);
                }
            }
            _ => inspect_type(&model, input),
        }
    }
}

// ──────────────────────────────────────────────────────────────────
// Commands
// ──────────────────────────────────────────────────────────────────

fn list_types(m: &Model) {
    println!("\n📦 {} message types:\n", m.messages.len());
    for msg in m.messages.values() {
        let tag = if msg.name.ends_with("Data") {
            "[Data]   "
        } else if msg.name.ends_with("Request") {
            "[Request]"
        } else if msg.name.ends_with("Reply") {
            "[Reply]  "
        } else {
            "[Message]"
        };
        println!("  {tag} {}", msg.name);
    }
    println!("\n💡 Type a name to inspect it (e.g. 'AccountSummaryData'). 'enums' lists enums.");
}

fn list_enums(m: &Model) {
    println!("\n🔢 {} enum types:\n", m.enums.len());
    for e in m.enums.values() {
        println!("  [Enum] {}  ({} values)", e.name, e.values.len());
    }
    println!("\n💡 'enum <Name>' shows every value.");
}

fn search_types(m: &Model, term: &str) {
    let needle = term.to_lowercase();
    println!("\n🔍 Types containing '{term}':\n");
    let mut found = 0;
    for msg in m.messages.values() {
        if msg.name.to_lowercase().contains(&needle) {
            println!("  [Message] {}", msg.name);
            found += 1;
        }
    }
    for e in m.enums.values() {
        if e.name.to_lowercase().contains(&needle) {
            println!("  [Enum]    {}", e.name);
            found += 1;
        }
    }
    if found == 0 {
        println!("  ❌ Nothing matched '{term}'");
    } else {
        println!("\n✅ {found} match(es). Type a name to inspect it.");
    }
}

fn find_field(m: &Model, field: &str) {
    let needle = field.to_lowercase();
    println!("\n🔍 Messages with a field matching '{field}':\n");
    let mut found = 0;
    for msg in m.messages.values() {
        let hits: Vec<&Field> = msg
            .fields
            .iter()
            .filter(|f| f.name.to_lowercase().contains(&needle))
            .collect();
        if !hits.is_empty() {
            println!("📦 {}:", msg.name);
            for f in hits {
                println!("   └─ {} : {}", f.name, f.type_label);
            }
            found += 1;
        }
    }
    if found == 0 {
        println!("  ❌ No message has a field containing '{field}'");
    } else {
        println!("\n✅ Found in {found} type(s).");
    }
}

fn inspect_enum(m: &Model, name: &str) {
    match m.enums.get(&name.to_lowercase()) {
        None => println!("\n❌ Enum '{name}' not found.  💡 Try: enum MrpcEnumAccountTradeMode"),
        Some(e) => {
            println!("\n╔═══ ENUM: {} ═══╗\n", e.name);
            for (vn, num) in &e.values {
                println!("  {vn:<48} = {num}");
            }
        }
    }
}

fn inspect_type(m: &Model, name: &str) {
    if let Some(msg) = m.messages.get(&name.to_lowercase()) {
        println!("\n╔═══ MESSAGE: {} ═══╗\n", msg.name);
        if msg.fields.is_empty() {
            println!("  (no fields)");
            return;
        }
        println!("📋 Fields ({}):\n", msg.fields.len());
        for f in &msg.fields {
            let tagcol = if f.tag > 0 { format!("#{}", f.tag) } else { String::new() };
            println!("  • {:<4} {:<40} : {}", tagcol, f.name, f.type_label);
        }
        println!("\n💡 For an enum field, run: enum <EnumName>");
    } else if m.enums.contains_key(&name.to_lowercase()) {
        inspect_enum(m, name);
    } else {
        println!("\n❌ Type '{name}' not found.");
        println!("💡 Try 'search {name}' or 'list'.");
    }
}

// ──────────────────────────────────────────────────────────────────
// Parser — reads the prost-generated source
// ──────────────────────────────────────────────────────────────────

fn parse(src: &str) -> Model {
    let lines: Vec<&str> = src.lines().collect();
    let mut messages = BTreeMap::new();
    let mut enums = BTreeMap::new();

    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();

        if let Some(rest) = t.strip_prefix("pub struct ") {
            if preceded_by(&lines, i, "::prost::Message") {
                let name = ident_head(rest);
                // Empty messages are generated on one line: `pub struct Foo {}` (or `;`).
                let empty = rest.contains("{}") || rest.trim_end().ends_with(';');
                if empty {
                    messages.insert(name.to_lowercase(), Message { name, fields: Vec::new() });
                    i += 1;
                } else {
                    let (fields, next) = parse_fields(&lines, i + 1);
                    messages.insert(name.to_lowercase(), Message { name, fields });
                    i = next;
                }
                continue;
            }
        }

        if let Some(rest) = t.strip_prefix("pub enum ") {
            if preceded_by(&lines, i, "::prost::Enumeration") {
                let name = ident_head(rest);
                let (values, next) = parse_variants(&lines, i + 1);
                enums.insert(name.to_lowercase(), EnumDef { name, values });
                i = next;
                continue;
            }
        }

        i += 1;
    }

    Model { messages, enums }
}

/// True if one of the few lines just above `i` carries the given prost derive marker.
fn preceded_by(lines: &[&str], i: usize, marker: &str) -> bool {
    let start = i.saturating_sub(5);
    for l in &lines[start..i] {
        let lt = l.trim();
        if lt.contains(marker) {
            return true;
        }
    }
    false
}

/// Parse struct fields starting at `start`, stopping after the closing `}`. Returns (fields, index-after).
fn parse_fields(lines: &[&str], start: usize) -> (Vec<Field>, usize) {
    let mut fields = Vec::new();
    let mut j = start;
    while j < lines.len() {
        let t = lines[j].trim();
        if t == "}" {
            return (fields, j + 1);
        }
        if let Some(attr) = extract_prost_attr(t) {
            // The field's declaration is the next line that starts with `pub `.
            let mut k = j + 1;
            while k < lines.len() && !lines[k].trim().starts_with("pub ") {
                if lines[k].trim() == "}" {
                    break;
                }
                k += 1;
            }
            if k < lines.len() && lines[k].trim().starts_with("pub ") {
                if let Some(f) = make_field(&attr, lines[k].trim()) {
                    fields.push(f);
                }
                j = k + 1;
                continue;
            }
        }
        j += 1;
    }
    (fields, j)
}

fn parse_variants(lines: &[&str], start: usize) -> (Vec<(String, i64)>, usize) {
    let mut values = Vec::new();
    let mut j = start;
    while j < lines.len() {
        let t = lines[j].trim();
        if t == "}" {
            return (values, j + 1);
        }
        // Variant form: `Name = 3,`
        if let Some(eq) = t.find('=') {
            let name = t[..eq].trim();
            let rest = t[eq + 1..].trim().trim_end_matches(',').trim();
            if is_ident(name) {
                if let Ok(num) = rest.parse::<i64>() {
                    values.push((name.to_string(), num));
                }
            }
        }
        j += 1;
    }
    (values, j)
}

/// Pull the inside of a `#[prost( ... )]` attribute line.
fn extract_prost_attr(line: &str) -> Option<String> {
    let s = line.trim();
    let start = s.find("#[prost(")? + "#[prost(".len();
    let end = s.rfind(")]")?;
    if end <= start {
        return None;
    }
    Some(s[start..end].to_string())
}

/// Build a Field from its prost attribute and the `pub name: Type,` declaration line.
fn make_field(attr: &str, decl: &str) -> Option<Field> {
    // field name: between `pub ` and `:`
    let after_pub = decl.strip_prefix("pub ")?;
    let colon = after_pub.find(':')?;
    let name = after_pub[..colon].trim().to_string();
    let rust_type = after_pub[colon + 1..].trim().trim_end_matches(',').trim();

    // Capture the quoted span that follows a key, e.g. `enumeration = "Foo"` → `Foo`,
    // `map = "string, string"` → `string, string` (handles commas inside the quotes).
    let quoted = |key: &str| -> Option<String> {
        let i = attr.find(key)?;
        let rest = &attr[i + key.len()..];
        let q1 = rest.find('"')?;
        let q2 = rest[q1 + 1..].find('"')?;
        Some(rest[q1 + 1..q1 + 1 + q2].to_string())
    };

    // The tag is a bare `tag = "N"` — match the exact key so `tags = "1, 2"` can't impersonate it.
    let tag = attr
        .split(',')
        .find_map(|p| {
            let mut kv = p.splitn(2, '=');
            if kv.next()?.trim() != "tag" {
                return None;
            }
            kv.next()?.trim().trim_matches('"').parse::<i32>().ok()
        })
        .unwrap_or(0);

    let kind = attr.split([',', '=']).next().unwrap_or("").trim();

    // A oneof is a synthetic field over several tags; show the oneof type and its tag list.
    if kind == "oneof" {
        let ref_name = quoted("oneof").map(|v| leaf_type(&v)).unwrap_or_else(|| "?".into());
        let tags = quoted("tags").unwrap_or_default();
        let label = if tags.is_empty() {
            format!("oneof {ref_name}")
        } else {
            format!("oneof {ref_name} (tags {tags})")
        };
        return Some(Field { name, tag: 0, type_label: label });
    }
    let repeated = attr.split(',').any(|p| p.trim() == "repeated");
    let optional = attr.split(',').any(|p| p.trim() == "optional");

    let base = match kind {
        "message" | "group" => leaf_type(rust_type),
        "enumeration" => quoted("enumeration").unwrap_or_else(|| "enum".to_string()),
        "map" => {
            // attr like: map = "string, string"
            let inner = quoted("map").unwrap_or_default();
            format!("map<{inner}>")
        }
        "bytes" => "bytes".to_string(),
        other => other.to_string(), // string, double, int32, int64, bool, uint64, ...
    };

    let type_label = if kind == "map" {
        base
    } else if repeated {
        format!("repeated {base}")
    } else if optional {
        format!("optional {base}")
    } else {
        base
    };

    Some(Field { name, tag, type_label })
}

/// Reduce a Rust type to its leaf proto type name: `Option<::prost_types::Timestamp>` → `Timestamp`,
/// `::prost::alloc::vec::Vec<OpenedOrder>` → `OpenedOrder`, `super::Error` → `Error`.
fn leaf_type(rust_type: &str) -> String {
    let mut s = rust_type;
    // Dive into the innermost generic argument.
    while let Some(lt) = s.rfind('<') {
        let rest = &s[lt + 1..];
        let end = rest.find('>').unwrap_or(rest.len());
        s = rest[..end].trim();
    }
    // Last path segment.
    let leaf = s.rsplit("::").next().unwrap_or(s).trim();
    leaf.trim_end_matches('>').to_string()
}

// ──────────────────────────────────────────────────────────────────
// Small helpers
// ──────────────────────────────────────────────────────────────────

fn split_once_ws(s: &str) -> (&str, &str) {
    match s.find(char::is_whitespace) {
        Some(i) => (s[..i].trim(), s[i..].trim()),
        None => (s, ""),
    }
}

/// Leading Rust identifier of a string, e.g. `Foo {}` → `Foo`, `Bar {` → `Bar`.
fn ident_head(s: &str) -> String {
    s.trim()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        && s.chars().next().map(|c| !c.is_ascii_digit()).unwrap_or(false)
}

// ──────────────────────────────────────────────────────────────────
// UI text
// ──────────────────────────────────────────────────────────────────

fn print_header(m: &Model) {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║        🔍 PROTOBUF INSPECTOR — mt5_term_api           ║");
    println!("║   Explore MT4 gRPC messages, fields & enums offline   ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!(
        "Parsed {} messages, {} enums from the generated code.",
        m.messages.len(),
        m.enums.len()
    );
}

fn print_quick_start() {
    println!("\n┌─ QUICK START ─────────────────────────────────────────┐");
    println!("│ list                list every message type           │");
    println!("│ AccountSummaryData  inspect a message's fields        │");
    println!("│ field balance       find who declares a field         │");
    println!("│ enum MrpcEnumAccountTradeMode   show enum values          │");
    println!("│ help                full command reference            │");
    println!("│ exit                quit                              │");
    println!("└───────────────────────────────────────────────────────┘");
}

fn print_help() {
    println!("\n═══ COMMAND REFERENCE ═══\n");
    println!("BROWSE");
    println!("  list, ls             list all message types");
    println!("  enums                list all enum types");
    println!("  <TypeName>           inspect a message (fields + types)");
    println!("\nSEARCH");
    println!("  search <text>        find types by name (alias: find)");
    println!("  field <name>         find messages containing a field");
    println!("  enum <name>          show enum values");
    println!("\nUTILITY");
    println!("  help, ?              this help");
    println!("  exit, quit, q        leave");
    println!("\nEXAMPLES");
    println!("  > list");
    println!("  > OpenedOrdersData");
    println!("  > search Order");
    println!("  > field ticket");
    println!("  > enum Bmt5EnumOrderType");
}
