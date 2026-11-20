<?php
$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/wp-admin/setup-config.php';
$_SERVER['HTTP_HOST'] = 'localhost';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = 'localhost';
$_SERVER['SCRIPT_NAME'] = '/wp-admin/setup-config.php';
$_SERVER['PHP_SELF'] = '/wp-admin/setup-config.php';

require_once __DIR__ . '/wordpress/wp-admin/setup-config.php';
