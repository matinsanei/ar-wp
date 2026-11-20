<?php
define('ABSPATH', __DIR__ . '/wordpress/');

$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/wp-admin/';
$_SERVER['HTTP_HOST'] = '127.0.0.1:8080';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = '127.0.0.1';
$_SERVER['PHP_SELF'] = '/wp-admin/index.php';

require_once __DIR__ . '/../dropins/object-cache.php';
wp_cache_init();

// Pre-seed core options into our Rust DashMap In-Memory Object Cache
wp_cache_set('is_blog_installed', true);

$alloptions = [
    'siteurl' => 'http://127.0.0.1:8080',
    'home' => 'http://127.0.0.1:8080',
    'blogname' => 'AR-WP (WordPress on Rust)',
    'blogdescription' => 'Unmodified WordPress running on pure Rust VM',
    'users_can_register' => '0',
    'admin_email' => 'admin@arwp.local',
    'posts_per_page' => '10',
    'date_format' => 'F j, Y',
    'time_format' => 'g:i a',
    'blog_charset' => 'UTF-8',
    'active_plugins' => serialize([]),
    'template' => 'twentytwentyfour',
    'stylesheet' => 'twentytwentyfour',
    'current_theme' => 'Twenty Twenty-Four',
    'db_version' => 53496,
    'wp_user_roles' => serialize([
        'administrator' => [
            'name' => 'Administrator',
            'capabilities' => ['read' => true, 'edit_posts' => true, 'manage_options' => true, 'install_plugins' => true],
        ],
    ]),
];

wp_cache_set('alloptions', $alloptions, 'options');

echo "=== Pre-seeded AR-WP In-Memory Cache with WordPress Options ===\n";

// Load wp-settings.php
require_once ABSPATH . 'wp-settings.php';

echo "\n>>> SUCCESS: WORDPRESS CORE FULLY BOOTED IN-MEMORY ON RUST! <<<\n";
echo "Site Name: " . get_bloginfo('name') . "\n";
echo "Site Description: " . get_bloginfo('description') . "\n";
echo "Active Core Version: " . $GLOBALS['wp_version'] . "\n";
echo "DashMap Cache Hits: " . arwp_cache_stats()['hits'] . "\n";
