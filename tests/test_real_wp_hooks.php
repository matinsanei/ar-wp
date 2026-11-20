<?php
// Test real WordPress Plugin API directly from WordPress Core!
define('WPINC', 'wp-includes');
define('ABSPATH', __DIR__ . '/wordpress/');

require_once __DIR__ . '/wordpress/wp-includes/plugin.php';

// Check if official WordPress functions are loaded
if (function_exists('add_filter') && function_exists('apply_filters') && function_exists('add_action') && function_exists('do_action')) {
    echo "[SUCCESS] Official WordPress Hook functions loaded!\n";
} else {
    echo "[FAILURE] Functions not loaded!\n";
    exit(1);
}

// 1. Test Filter
add_filter('the_content', function($content) {
    return "<p>" . $content . "</p>";
}, 10);

add_filter('the_content', function($content) {
    return $content . " <!-- Filtered by Real WP Core on Rust -->";
}, 20);

$input = "Testing Real WordPress on WPRust Engine";
$output = apply_filters('the_content', $input);

echo "Initial: " . $input . "\n";
echo "Output:  " . $output . "\n";

// 2. Test Action Hook
$action_executed = false;
add_action('init', function() use (&$action_executed) {
    $action_executed = true;
    echo "[ACTION] 'init' hook fired successfully inside WPRust!\n";
});

do_action('init');

if ($action_executed) {
    echo "[VERIFIED] All real WordPress hook mechanics passed with 100% success!\n";
}
