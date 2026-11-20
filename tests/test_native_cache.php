<?php
// Test AR-WP Native DashMap Shared Object Cache

echo "=== AR-WP Native Cache Test ===\n";

// 1. Primitive scalar caching
$set_res = wp_cache_set('site_title', 'My Awesome WordPress on Rust', 'options');
echo "wp_cache_set string: " . ($set_res ? "SUCCESS" : "FAILED") . "\n";

$title = wp_cache_get('site_title', 'options');
echo "wp_cache_get string: " . $title . "\n";
assert($title === 'My Awesome WordPress on Rust');

// 2. Complex array caching (e.g. alloptions in WP)
$options = [
    'siteurl' => 'https://rustpress.io',
    'active_plugins' => ['woocommerce', 'elementor'],
    'users_can_register' => 0,
    'posts_per_page' => 10,
];

$set_arr = wp_cache_set('alloptions', $options, 'options');
echo "wp_cache_set array: " . ($set_arr ? "SUCCESS" : "FAILED") . "\n";

$cached_options = wp_cache_get('alloptions', 'options');
echo "wp_cache_get siteurl: " . $cached_options['siteurl'] . "\n";
echo "wp_cache_get plugin 0: " . $cached_options['active_plugins'][0] . "\n";
assert($cached_options['active_plugins'][1] === 'elementor');

// 3. Counter operations (incr / decr)
wp_cache_set('view_counter', 100, 'stats');
$c1 = wp_cache_incr('view_counter', 5, 'stats');
echo "wp_cache_incr (+5): " . $c1 . "\n";
assert($c1 === 105);

$c2 = wp_cache_decr('view_counter', 10, 'stats');
echo "wp_cache_decr (-10): " . $c2 . "\n";
assert($c2 === 95);

// 4. Object caching (e.g. stdClass / WP_Post simulation)
$post = new stdClass();
$post->ID = 42;
$post->post_title = "Hello World from Rust RAM";
$post->post_status = "publish";

wp_cache_set('post_42', $post, 'posts');
$cached_post = wp_cache_get('post_42', 'posts');
echo "wp_cache_get Object post_title: " . $cached_post->post_title . "\n";
assert($cached_post->ID === 42);

// 5. Cache stats directly from Rust
$stats = arwp_cache_stats();
echo "Cache Engine: " . $stats['engine'] . "\n";
echo "Total Cached Items: " . $stats['items'] . "\n";
echo "Total Hits: " . $stats['hits'] . "\n";
echo "Total Sets: " . $stats['sets'] . "\n";
assert($stats['hits'] === 3);

echo "\n>>> ALL NATIVE DASHMAP OBJECT CACHE TESTS PASSED! <<<\n";
