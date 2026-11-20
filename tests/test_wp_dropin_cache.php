<?php
define('ABSPATH', __DIR__ . '/');

require_once __DIR__ . '/../dropins/object-cache.php';

$GLOBALS['wp_object_cache'] = new WP_Object_Cache();
$wp_object_cache = $GLOBALS['wp_object_cache'];

wp_cache_set('wp_user_1', ['id' => 1, 'user_login' => 'admin', 'display_name' => 'Matin Sanei'], 'users');

$user = wp_cache_get('wp_user_1', 'users');
echo "Retrieved user: " . $user['display_name'] . "\n";
assert($user['display_name'] === 'Matin Sanei');

global $wp_object_cache;
$stats = $wp_object_cache->stats();
echo "Cache Backend: " . $stats['engine'] . "\n";
echo "Active Items in Rust RAM: " . $stats['items'] . "\n";

echo ">>> WORDPRESS OBJECT-CACHE DROP-IN VERIFIED SUCCESSFULLY! <<<\n";
