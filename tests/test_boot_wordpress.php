<?php
// Test WordPress Boot sequence on AR-WP
$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/';
$_SERVER['HTTP_HOST'] = 'localhost';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = 'localhost';

echo "=== Testing WordPress Load Sequence on AR-WP ===\n";

require_once __DIR__ . '/wordpress/wp-load.php';

echo "WordPress Booted!\n";
