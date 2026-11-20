<?php
/**
 * AR-WP WordPress Configuration File.
 */
define( 'DB_NAME', 'wordpress' );
define( 'DB_USER', 'root' );
define( 'DB_PASSWORD', '' );
define( 'DB_HOST', 'localhost' );
define( 'DB_CHARSET', 'utf8mb4' );
define( 'DB_COLLATE', '' );

define( 'AUTH_KEY',         'arwp-secure-auth-key-rust-01' );
define( 'SECURE_AUTH_KEY',  'arwp-secure-auth-key-rust-02' );
define( 'LOGGED_IN_KEY',    'arwp-secure-auth-key-rust-03' );
define( 'NONCE_KEY',        'arwp-secure-auth-key-rust-04' );
define( 'AUTH_SALT',        'arwp-secure-auth-salt-rust-01' );
define( 'SECURE_AUTH_SALT', 'arwp-secure-auth-salt-rust-02' );
define( 'LOGGED_IN_SALT',   'arwp-secure-auth-salt-rust-03' );
define( 'NONCE_SALT',       'arwp-secure-auth-salt-rust-04' );

$table_prefix = 'wp_';

define( 'WP_HOME', 'http://127.0.0.1:8080' );
define( 'WP_SITEURL', 'http://127.0.0.1:8080' );

define( 'WP_DEBUG', false );
define( 'WP_CACHE', true );

if ( ! defined( 'ABSPATH' ) ) {
    define( 'ABSPATH', __DIR__ . '/' );
}

require_once ABSPATH . 'wp-settings.php';
