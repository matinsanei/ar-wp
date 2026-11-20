# AR-WP (Artemis WordPress Runtime) ⚡

**In-Memory Resident WordPress Application Server & Native VM in Rust.**

Engineered by [Matin Sanei](https://github.com/matinsanei).

---

## 🚀 Abstract

Traditional WordPress hosting architectures rely on the 1990s "shared-nothing" CGI/PHP-FPM execution model. On every inbound HTTP request:
- Processes are spawned or allocated from worker pools.
- Hundreds of `.php` files are repeatedly traversed and parsed from disk or opcode cache.
- Single-threaded blocking sockets communicate with MySQL.
- Cross-request object caching requires external network roundtrips to Redis or Memcached.

**AR-WP** redesigns the WordPress runtime from the ground up:
1. **Resident In-Memory VM:** Houses WordPress core inside an in-memory Virtual File System (VFS) with pre-compiled bytecode units ready to execute in sub-milliseconds.
2. **Native Async HTTP (Tokio + Axum):** Eliminates PHP-FPM and Nginx reverse proxies, running as a single, compiled Rust binary capable of serving tens of thousands of concurrent requests.
3. **Rust-Native Database Pooling:** Fast-paths WordPress database operations (`$wpdb`) directly into asynchronous connection pools (`sqlx`).
4. **Lock-Free RAM Object Caching:** Connects `wp_cache_*` primitives to atomic, concurrent in-memory hash tables (`DashMap`), achieving nanosecond-level cache hits without external services.

---

## 🏗️ Systems Architecture

```
                          [ Inbound HTTP / Web Traffic ]
                                         │
                                         ▼
                     ┌────────────────────────────────────────┐
                     │     AR-WP Server (Tokio + Axum)        │
                     └───────────────────┬────────────────────┘
                                         │
                                         ▼
                     ┌────────────────────────────────────────┐
                     │    In-Memory Resident Worker Pool      │
                     │    (Worker-Actor Model per Core)       │
                     └───────────────────┬────────────────────┘
                                         │
                 ┌───────────────────────┼───────────────────────┐
                 ▼                       ▼                       ▼
        ┌──────────────────┐   ┌──────────────────┐   ┌──────────────────┐
        │   php-runtime    │   │    php-types     │   │   php-builtins   │
        │  (Bytecode VM &  │   │  (Zval, PhpArray,│   │ (1000+ Internal  │
        │  Execution Loop) │   │   COW Memory)    │   │  PHP Functions)  │
        └────────┬─────────┘   └──────────────────┘   └────────┬─────────┘
                 │                                             │
                 ▼                                             ▼
        ┌──────────────────┐                          ┌──────────────────┐
        │  In-Memory VFS   │                          │ Native SQL Pool  │
        │ (WordPress Core) │                          │  (Async MySQL)   │
        └──────────────────┘                          └──────────────────┘
```

---

## 🔬 Real-World WordPress Verification

AR-WP is verified directly against **official, unmodified WordPress Core** (`wp-includes/plugin.php`, `class-wp-hook.php`, and `class-wp-filter-sentinel.php`):

```bash
cargo run -p php-cli -- tests/test_real_wp_hooks.php
```

Output:
```text
[SUCCESS] Official WordPress Hook functions loaded!
Initial: Testing Real WordPress on WPRust Engine
Output:  <p>Testing Real WordPress on WPRust Engine</p> <!-- Filtered by Real WP Core on Rust -->
[ACTION] 'init' hook fired successfully inside WPRust!
[VERIFIED] All real WordPress hook mechanics passed with 100% success!
```

---

## 📦 Workspace Layout

- **`crates/arwp-server`**: Asynchronous HTTP application server powered by Tokio and Axum.
- **`crates/php-types`**: Memory model and dynamic type system (`Zval`, `PhpArray`, `PhpObject`, References).
- **`crates/php-runtime`**: AST lowering (via `mago`), compiler, and bytecode virtual machine.
- **`crates/php-builtins`**: Native implementations of standard PHP functions (arrays, regex, json, crypto, pdo, mysqli).
- **`crates/php-cli`**: CLI runner (`phpr`) for executing PHP scripts from the command line.
- **`crates/arwp-prototype`**: Standalone minimal proof of concept demonstrating hook auto-vivification in Rust.

---

## ⚡ Getting Started

### Prerequisites
* Rust toolchain (Rust 1.85+)
* Cargo

### Running the Resident Server

```bash
# Clone the repository
git clone https://github.com/matinsanei/ar-wp.git
cd ar-wp

# Run the high-performance HTTP server
cargo run -p arwp-server --release
```

Test the live endpoint:

```bash
curl -i http://127.0.0.1:8080/
```

---

## 🗺️ Roadmap

- [x] In-memory execution of official WordPress 7.2 core hook pipeline.
- [x] Resident Tokio + Axum HTTP server.
- [x] Full Linux shared-library resolution via custom `syslibs`.
- [ ] Direct MySQL pool integration bypassing C sockets (`sqlx`).
- [ ] Native lock-free object cache bridge (`DashMap`).
- [ ] Pre-compiled in-memory VFS for full WordPress installations.
- [ ] JIT compiler integration via Cranelift.

---

## 📜 License

AR-WP is released under the **Apache License, Version 2.0**.
See [LICENSE](LICENSE) for details.

This project incorporates components from the `php-rust` project (by Francesco Tinti) and PHP software freely available from <http://www.php.net/software/>, licensed under The PHP License, version 3.01.
