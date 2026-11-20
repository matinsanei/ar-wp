<?php
$_SERVER['REQUEST_METHOD'] = 'GET';
$_SERVER['REQUEST_URI'] = '/wp-admin/index.php';
$_SERVER['HTTP_HOST'] = '127.0.0.1:8080';
$_SERVER['SERVER_PROTOCOL'] = 'HTTP/1.1';
$_SERVER['SERVER_NAME'] = '127.0.0.1';
$_SERVER['SCRIPT_NAME'] = '/wp-admin/index.php';
$_SERVER['PHP_SELF'] = '/wp-admin/index.php';

require_once __DIR__ . '/wordpress/wp-admin/index.php';
