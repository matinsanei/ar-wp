mod ast;
mod engine;
mod parser;
mod val;

use engine::Engine;
use parser::{Lexer, Parser};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!("   WPRust Prototype: Executing WordPress Hooks in Rust       ");
    println!("============================================================");

    let php_code = r#"
<?php
// Initialize WordPress hook registry
$wp_filter = array();

// Core WordPress hook registration function
function add_filter($hook_name, $callback, $priority = 10) {
    global $wp_filter;
    $wp_filter[$hook_name][$priority][] = $callback;
    return true;
}

// Core WordPress hook execution function
function apply_filters($hook_name, $value) {
    global $wp_filter;
    if (!isset($wp_filter[$hook_name])) {
        return $value;
    }

    // Sort priority levels numerically (e.g. 5, 10, 20)
    ksort($wp_filter[$hook_name]);

    foreach ($wp_filter[$hook_name] as $priority => $callbacks) {
        foreach ($callbacks as $callback) {
            $value = $callback($value);
        }
    }
    return $value;
}

// Plugin / Theme callbacks
function hook_early($content) {
    return "[EARLY_5: " . $content . "]";
}

function hook_normal($content) {
    return "[NORMAL_10: " . $content . "]";
}

function hook_late($content) {
    return "[LATE_20: " . $content . "]";
}

// Register hooks with different priorities
add_filter('the_content', 'hook_normal', 10);
add_filter('the_content', 'hook_late', 20);
add_filter('the_content', 'hook_early', 5);

// Execute the filter pipeline
$initial_content = "Hello from pure WordPress PHP";
$final_output = apply_filters('the_content', $initial_content);

echo "Initial: " . $initial_content . "\n";
echo "Output:  " . $final_output . "\n";
"#;

    println!("[1] Tokenizing PHP code...");
    let mut lexer = Lexer::new(php_code);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;
    println!("    Generated {} tokens successfully.", tokens.len());

    println!("[2] Parsing into AST...");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().map_err(|e| format!("Parser error: {}", e))?;
    println!("    Parsed {} top-level statements.", ast.len());

    println!("[3] Executing on WPRust Engine...");
    let mut engine = Engine::new();
    let start = Instant::now();
    engine.execute(&ast).map_err(|e| format!("Runtime error: {}", e))?;
    let duration = start.elapsed();
    println!("    Execution completed in: {:?}", duration);

    // Verify global state
    println!("\n[4] Inspecting Internal State:");
    let wp_filter = engine.get_var("wp_filter");
    println!("    $wp_filter = {}", wp_filter);

    let final_output = engine.get_var("final_output");
    println!("    $final_output = {}", final_output);

    // Assert priority ordering worked as WordPress expects:
    // hook_early (5) -> hook_normal (10) -> hook_late (20)
    let expected = "[LATE_20: [NORMAL_10: [EARLY_5: Hello from pure WordPress PHP]]]";
    if final_output.to_string_lossy() == expected {
        println!("\n>>> SUCCESS! Hook priority execution order matches WordPress 100%! <<<");
    } else {
        println!("\n>>> MISMATCH: Expected {}, got {}", expected, final_output);
    }

    Ok(())
}
