# WPRust 🚀

**High-Performance In-Memory WordPress Application Server & Native Runtime in Rust.**

WPRust bridges the gap between WordPress's massive plugin ecosystem and Rust's world-class systems performance. Rather than replacing WordPress with yet another CMS, **WPRust executes real, unmodified WordPress PHP core, hooks, themes, and plugins inside an in-memory resident Rust runtime.**

---

## 🌟 The Vision

Traditional WordPress deployments suffer from architectural bottlenecks that have remained unchanged for two decades:
1. **PHP-FPM Fork & I/O Overhead:** Every HTTP request triggers cold boots, scanning hundreds of `.php` files from disk, and parsing them into memory.
2. **Synchronous Blocking Sockets:** Database queries to MySQL block worker processes on single-threaded C sockets.
3. **External Cache Latency:** Tools like Redis or Memcached require network hops or Unix domain sockets for every single `wp_cache_get` call.

**WPRust fundamentally redesigns this stack:**
* **Resident in Memory:** WordPress files reside in an in-memory Virtual File System (VFS) with pre-compiled bytecode units ready to execute in sub-milliseconds.
* **Native Async HTTP (Tokio + Axum):** An embedded, single-binary application server capable of handling tens of thousands of concurrent connections.
* **Direct Database Bypass:** Connects `$wpdb` directly to Rust's asynchronous MySQL connection pool.
* **Nanosecond In-Memory Caching:** Connects `wp_cache_*` directly to lock-free concurrent hash maps in RAM (`DashMap`), bypassing network latency completely.

---

## 🏗️ Architecture

```
                                [ Incoming HTTP Requests ]
                                            │
                                            ▼
                          ┌───────────────────────────────────┐
                          │   WPRust Server (Tokio + Axum)    │
                          └─────────────────┬─────────────────┘
                                            │
                                            ▼
                          ┌───────────────────────────────────┐
                          │   In-Memory Resident Worker Pool   │
                          │   (Worker-Actor Model per Thread) │
                          └─────────────────┬─────────────────┘
                                            │
                      ┌─────────────────────┼─────────────────────┐
                      ▼                     ▼                     ▼
             ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
             │   php-runtime   │   │   php-types     │   │  php-builtins   │
             │ (Bytecode VM &  │   │ (Zval, PhpArray,│   │ (1000+ Internal │
             │  Execution Loop)│   │  Cow Memory)    │   │  PHP Functions) │
             └────────┬────────┘   └─────────────────┘   └────────┬────────┘
                      │                                           │
                      ▼                                           ▼
             ┌─────────────────┐                         ┌─────────────────┐
             │ In-Memory VFS   │                         │ Native SQL Pool │
             │ (WordPress Core)│                         │ (Async MySQL)   │
             └─────────────────┘                         └─────────────────┘
```

---

## 📦 Workspace Crates

- **`crates/wprust-server`**: The primary asynchronous HTTP application server powered by Axum and Tokio.
- **`crates/php-types`**: The core PHP type system (`Zval`, `PhpArray`, `PhpObject`, References, and arithmetic type juggling).
- **`crates/php-runtime`**: The complete bytecode virtual machine, AST lowering (via `mago`), compiler, and execution dispatch loop.
- **`crates/php-builtins`**: Over 1,000 native implementations of standard PHP functions (arrays, strings, regex, json, hash, streams, PDO, mysqli).
- **`crates/php-cli`**: CLI drop-in runner for executing PHP scripts directly from the terminal.
- **`crates/wprust-prototype`**: Minimal standalone educational prototype demonstrating WordPress hook priority ordering in Rust.

---

## ⚡ Quick Start

### Prerequisites
* Rust 1.85+ (Edition 2021)
* Cargo

### Build and Run the Server

```bash
# Clone the repository
git clone https://github.com/matinsanei/wprust.git
cd wprust

# Run the resident WordPress HTTP server
cargo run -p wprust-server
```

Once running, navigate to `http://127.0.0.1:8080` in your browser or test with curl:

```bash
curl -i http://127.0.0.1:8080/
```

### Run the Hook Engine Prototype

To test the standalone WordPress hook prioritization proof of concept:

```bash
cargo run -p wprust-prototype
```

---

## ⚖️ License & Attribution

WPRust incorporates code and architecture from the `php-rust` project by Francesco Tinti, licensed under **The PHP License, version 3.01**.

- Copyright (c) 2026 Matin Sanei
- Copyright (c) 2026 Francesco Tinti
- Copyright (c) 1999 - 2024 The PHP Group. All rights reserved.

> *This product includes PHP software, freely available from <http://www.php.net/software/>.*

For full licensing details, see [LICENSE](LICENSE).
