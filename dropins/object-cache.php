<?php
/**
 * AR-WP Native In-Memory Shared Object Cache Drop-in for WordPress.
 *
 * Plug this file into `wp-content/object-cache.php`.
 * Powered by pure-Rust lock-free concurrent DashMap in AR-WP.
 * Zero Redis/Memcached socket overhead. Sub-microsecond response latency.
 *
 * @package AR-WP
 * @author Matin Sanei <saneimatin33@gmail.com>
 * @license Apache-2.0
 */

defined('ABSPATH') || exit;

/**
 * Sets up Object Cache Global and assigns it.
 */
function wp_cache_init() {
    $GLOBALS['wp_object_cache'] = new WP_Object_Cache();
}

/**
 * Adds data to the cache, if the cache key doesn't already exist.
 */
function wp_cache_add($key, $data, $group = '', $expire = 0) {
    global $wp_object_cache;
    return $wp_object_cache->add($key, $data, $group, (int) $expire);
}

/**
 * Sets data in the cache, regardless of whether it exists.
 */
function wp_cache_set($key, $data, $group = '', $expire = 0) {
    global $wp_object_cache;
    return $wp_object_cache->set($key, $data, $group, (int) $expire);
}

/**
 * Retrieves the cache contents from the cache.
 */
function wp_cache_get($key, $group = '', $force = false, &$found = null) {
    global $wp_object_cache;
    return $wp_object_cache->get($key, $group, $force, $found);
}

/**
 * Removes the cache contents matching key and group.
 */
function wp_cache_delete($key, $group = '') {
    global $wp_object_cache;
    return $wp_object_cache->delete($key, $group);
}

/**
 * Increments numeric cache item's value.
 */
function wp_cache_incr($key, $offset = 1, $group = '') {
    global $wp_object_cache;
    return $wp_object_cache->incr($key, $offset, $group);
}

/**
 * Decrements numeric cache item's value.
 */
function wp_cache_decr($key, $offset = 1, $group = '') {
    global $wp_object_cache;
    return $wp_object_cache->decr($key, $offset, $group);
}

/**
 * Removes all cache items.
 */
function wp_cache_flush() {
    global $wp_object_cache;
    return $wp_object_cache->flush();
}

/**
 * Closes the cache.
 */
function wp_cache_close() {
    return true;
}

function wp_cache_add_non_persistent_groups($groups) {
    global $wp_object_cache;
    if (isset($wp_object_cache)) {
        return $wp_object_cache->add_non_persistent_groups($groups);
    }
    return true;
}

function wp_cache_add_global_groups($groups) {
    global $wp_object_cache;
    if (isset($wp_object_cache)) {
        return $wp_object_cache->add_global_groups($groups);
    }
    return true;
}

function wp_cache_supports($feature) {
    return true;
}

function wp_cache_switch_to_blog($blog_id) {
    return true;
}

/**
 * Core class used to implement the AR-WP Native Object Cache.
 */
class WP_Object_Cache {
    /**
     * Holds the cache stats.
     */
    public $cache_hits = 0;
    public $cache_misses = 0;

    public function __construct() {}

    public function add($key, $data, $group = 'default', $expire = 0) {
        if (function_exists('arwp_cache_add')) {
            return arwp_cache_add($key, $data, $group, $expire);
        }
        return false;
    }

    public function set($key, $data, $group = 'default', $expire = 0) {
        if (function_exists('arwp_cache_set')) {
            return arwp_cache_set($key, $data, $group, $expire);
        }
        return false;
    }

    public function get($key, $group = 'default', $force = false, &$found = null) {
        if (function_exists('arwp_cache_get')) {
            $val = arwp_cache_get($key, $group, $force, $found);
            if ($val !== false) {
                $this->cache_hits++;
                $found = true;
                return $val;
            }
        }
        $this->cache_misses++;
        $found = false;
        return false;
    }

    public function delete($key, $group = 'default') {
        if (function_exists('arwp_cache_delete')) {
            return arwp_cache_delete($key, $group);
        }
        return false;
    }

    public function incr($key, $offset = 1, $group = 'default') {
        if (function_exists('arwp_cache_incr')) {
            return arwp_cache_incr($key, $offset, $group);
        }
        return false;
    }

    public function decr($key, $offset = 1, $group = 'default') {
        if (function_exists('arwp_cache_decr')) {
            return arwp_cache_decr($key, $offset, $group);
        }
        return false;
    }

    public function flush() {
        if (function_exists('arwp_cache_flush')) {
            return arwp_cache_flush();
        }
        return true;
    }

    public function add_global_groups($groups) {
        return true;
    }

    public function add_non_persistent_groups($groups) {
        return true;
    }

    public function switch_to_blog($blog_id) {
        return true;
    }

    public function reset() {
        return true;
    }

    public function stats() {
        if (function_exists('arwp_cache_stats')) {
            return arwp_cache_stats();
        }
        return ['engine' => 'fallback'];
    }
}
