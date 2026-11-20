<?php
define('ABSPATH', __DIR__ . '/wordpress/');
define('WP_SETUP_CONFIG', true);

$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/wp-admin/setup-config.php';
$_SERVER['HTTP_HOST'] = 'localhost';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = 'localhost';
$_SERVER['PHP_SELF'] = '/wp-admin/setup-config.php';

echo "=== Loading WordPress wp-settings.php ===\n";

require_once ABSPATH . 'wp-includes/plugin.php';
add_filter('wp_redirect', function($location, $status) {
    echo ">>> WORDPRESS REDIRECT TRIGGERED TO: $location (HTTP status $status) <<<\n";
    debug_print_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS);
    return false;
}, 1, 2);

require_once ABSPATH . 'wp-settings.php';
echo "=== wp-settings.php LOADED SUCCESSFULLY! ===\n";
