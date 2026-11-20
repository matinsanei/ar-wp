<?php
error_reporting(E_ALL);
ini_set('display_errors', 1);

$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/wp-admin/setup-config.php';
$_SERVER['HTTP_HOST'] = 'localhost';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = 'localhost';
$_SERVER['SCRIPT_NAME'] = '/wp-admin/setup-config.php';
$_SERVER['PHP_SELF'] = '/wp-admin/setup-config.php';

define('ABSPATH', __DIR__ . '/wordpress/');
define('WPINC', 'wp-includes');

echo "[1] Loading wp-includes/load.php...\n";
require_once ABSPATH . 'wp-includes/load.php';

echo "[2] Loading wp-includes/default-constants.php...\n";
require_once ABSPATH . 'wp-includes/default-constants.php';

echo "[3] Initializing constants...\n";
wp_initial_constants();

echo "[4] Checking PHP/MySQL versions...\n";
wp_check_php_mysql_versions();

echo "[5] Fixing server vars...\n";
wp_fix_server_vars();

echo "[6] Loading wp-includes/functions.php...\n";
require_once ABSPATH . 'wp-includes/functions.php';

echo "[7] Loading wp-includes/plugin.php...\n";
require_once ABSPATH . 'wp-includes/plugin.php';

echo "[8] Loading wp-settings.php...\n";
define('WP_SETUP_CONFIG', true);
require_once ABSPATH . 'wp-settings.php';

echo "WordPress Setup Config loaded successfully!\n";
