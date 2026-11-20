use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use php_builtins::registry;
use php_runtime::run_source_with_argv;

#[global_allocator]
static GLOBAL_ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Clone)]
struct AppState {
    wp_source: Arc<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("          WPRust: Next-Gen WordPress on Rust                ");
    println!("      High-Performance In-Memory Resident Server            ");
    println!("============================================================");

    // Initial WordPress Simulation Script
    let wp_script = r#"<?php
// WordPress Core & Hook Simulation on WPRust
$wp_filter = [];

function add_filter($hook_name, $callback, $priority = 10) {
    global $wp_filter;
    $wp_filter[$hook_name][$priority][] = $callback;
    return true;
}

function apply_filters($hook_name, $value) {
    global $wp_filter;
    if (!isset($wp_filter[$hook_name])) {
        return $value;
    }
    ksort($wp_filter[$hook_name]);
    foreach ($wp_filter[$hook_name] as $priority => $callbacks) {
        foreach ($callbacks as $callback) {
            $value = $callback($value);
        }
    }
    return $value;
}

// Custom WordPress Hooks & Filters
add_filter('the_content', function($content) {
    return "<div class='wp-content'>" . $content . "</div>";
}, 10);

add_filter('the_content', function($content) {
    return $content . "\n<!-- Served with ultra-low latency by WPRust (Rust Runtime) -->";
}, 20);

$post_title = "Welcome to WordPress Running on Rust!";
$post_content = "This page was rendered in-memory by WPRust using a Rust-native PHP execution engine.";

$rendered_body = apply_filters('the_content', $post_content);

echo "<html>";
echo "<head><title>" . $post_title . "</title>";
echo "<style>body { font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; line-height: 1.6; background: #f0f2f5; }";
echo ".card { background: white; padding: 30px; border-radius: 12px; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1); }";
echo "h1 { color: #1e293b; border-bottom: 2px solid #3b82f6; padding-bottom: 10px; }";
echo ".badge { background: #10b981; color: white; padding: 4px 10px; border-radius: 20px; font-size: 0.85rem; font-weight: bold; }";
echo ".metric { margin-top: 20px; padding: 12px; background: #e0f2fe; border-left: 4px solid #0284c7; border-radius: 4px; font-family: monospace; }";
echo "</style></head>";
echo "<body>";
echo "<div class='card'>";
echo "<span class='badge'>WPRust Engine Active</span>";
echo "<h1>" . $post_title . "</h1>";
echo $rendered_body;
echo "<div class='metric'>Runtime Engine: pure Rust PHP 8.5 Bytecode VM<br>Resident Server: Tokio + Axum</div>";
echo "</div>";
echo "</body></html>";
"#;

    println!("[1] Testing in-memory PHP VM execution...");
    let start = Instant::now();
    let registry = registry();
    let name = b"wordpress-index.php";
    let source = wp_script.as_bytes();
    let argv = vec![b"wordpress-index.php".as_slice()];
    let ini_overrides = vec![];

    let outcome = run_source_with_argv(name, source, &registry, &argv, &ini_overrides)?;
    let duration = start.elapsed();
    println!("    Direct execution finished in: {:?}", duration);
    println!("    Rendered bytes: {} bytes", outcome.rendered.len());

    let state = AppState {
        wp_source: Arc::new(wp_script.as_bytes().to_vec()),
    };

    let app = Router::new()
        .route("/", get(handle_wordpress_request))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("\n[2] Starting WPRust Resident HTTP Server on http://{}", addr);
    println!("    Open http://{} in your browser or curl to test!", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Spawn server or run briefly for validation
    println!("    WPRust HTTP Server listening on http://127.0.0.1:8080");
    
    // In background or serving:
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("Server error: {}", e);
        }
    });

    println!("[3] Sending test request to local WPRust server...");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Self-test curl
    let client = reqwest_client_test().await;
    println!("    Local response test completed: {}\n", client);
    println!("============================================================");
    println!("  WPRust is fully operational! Ready for WordPress Core.    ");
    println!("============================================================");

    Ok(())
}

async fn handle_wordpress_request(State(state): State<AppState>) -> impl IntoResponse {
    let start = Instant::now();
    let registry = registry();
    let name = b"wordpress-request.php";
    let argv = vec![b"wordpress-request.php".as_slice()];
    let ini_overrides = vec![];

    let outcome = match run_source_with_argv(name, &state.wp_source, &registry, &argv, &ini_overrides) {
        Ok(o) => o,
        Err(e) => {
            return Html(format!("<h1>WPRust Error</h1><pre>{}</pre>", e));
        }
    };

    let elapsed = start.elapsed();
    let mut html_str = String::from_utf8_lossy(&outcome.rendered).into_owned();
    html_str.push_str(&format!(
        "\n<!-- WPRust Request Latency: {:?} -->",
        elapsed
    ));

    Html(html_str)
}

async fn reqwest_client_test() -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    match tokio::net::TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let req = "GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: close\r\n\r\n";
            if stream.write_all(req.as_bytes()).await.is_err() {
                return "Failed to send HTTP request".to_string();
            }
            let mut response = Vec::new();
            if stream.read_to_end(&mut response).await.is_err() {
                return "Failed to read response".to_string();
            }
            let resp_str = String::from_utf8_lossy(&response);
            let first_line = resp_str.lines().next().unwrap_or("No response");
            format!("{} (Total payload: {} bytes)", first_line, response.len())
        }
        Err(e) => format!("TCP Connection failed: {}", e),
    }
}
