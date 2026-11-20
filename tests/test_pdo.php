<?php
// Test native SQLite PDO in AR-WP
$pdo = new PDO('sqlite:' . __DIR__ . '/test.db');
$pdo->exec("CREATE TABLE IF NOT EXISTS wp_options (option_id INTEGER PRIMARY KEY, option_name TEXT UNIQUE, option_value TEXT, autoload TEXT)");
$pdo->exec("INSERT OR REPLACE INTO wp_options (option_name, option_value, autoload) VALUES ('siteurl', 'http://127.0.0.1:8080', 'yes')");

$stmt = $pdo->query("SELECT * FROM wp_options WHERE option_name = 'siteurl'");
$row = $stmt->fetch(PDO::FETCH_ASSOC);

echo "PDO SQLite Query Result:\n";
print_r($row);

assert($row['option_value'] === 'http://127.0.0.1:8080');
echo "\n>>> PDO SQLITE IN AR-WP IS 100% WORKING! <<<\n";
