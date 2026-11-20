<?php
/**
 * AR-WP High-Performance SQLite & In-Memory Database Bridge for WordPress.
 *
 * Replaces external MySQL with native SQLite (via Rust-bundled rusqlite / PDO).
 * Zero external servers required. Completely embedded and instant.
 *
 * @package AR-WP
 * @author Matin Sanei <saneimatin33@gmail.com>
 */

defined('ABSPATH') || exit;

class ARWP_DB extends wpdb {
    public $show_errors = false;
    public $suppress_errors = false;
    public $last_error = '';
    public $last_query = '';
    public $last_result = [];
    public $rows_affected = 0;
    public $num_rows = 0;
    public $insert_id = 0;
    public $prefix = 'wp_';
    public $base_prefix = 'wp_';
    public $ready = true;
    public $has_connected = true;
    public $is_mysql = false;
    public $charset = 'utf8mb4';
    public $collate = '';
    public $queries = [];

    protected $tables = ['posts', 'comments', 'links', 'options', 'postmeta', 'terms', 'term_taxonomy', 'term_relationships', 'termmeta', 'commentmeta', 'users', 'usermeta'];
    private $pdo;

    public function __construct($dbuser = '', $dbpassword = '', $dbname = '', $dbhost = '') {
        parent::__construct($dbuser, $dbpassword, $dbname, $dbhost);
        $this->db_connect();
        $this->init_schema();
    }

    public function db_connect($allow_bail = true) {
        if ($this->pdo) {
            return true;
        }
        $db_dir = WP_CONTENT_DIR . '/database';
        if (!is_dir($db_dir)) {
            @mkdir($db_dir, 0755, true);
        }
        $db_path = $db_dir . '/arwp_wordpress.sqlite';
        $this->pdo = new PDO('sqlite:' . $db_path);
        $this->pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_SILENT);
        $this->pdo->setAttribute(PDO::ATTR_DEFAULT_FETCH_MODE, PDO::FETCH_OBJ);
        return true;
    }

    private function init_schema() {
        $this->pdo->exec("
            CREATE TABLE IF NOT EXISTS wp_options (
                option_id INTEGER PRIMARY KEY AUTOINCREMENT,
                option_name TEXT UNIQUE,
                option_value TEXT,
                autoload TEXT DEFAULT 'yes'
            );
            CREATE TABLE IF NOT EXISTS wp_users (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                user_login TEXT UNIQUE,
                user_pass TEXT,
                user_nicename TEXT,
                user_email TEXT,
                user_url TEXT,
                user_registered TEXT,
                user_activation_key TEXT,
                user_status INTEGER DEFAULT 0,
                display_name TEXT
            );
            CREATE TABLE IF NOT EXISTS wp_usermeta (
                umeta_id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                meta_key TEXT,
                meta_value TEXT
            );
            CREATE TABLE IF NOT EXISTS wp_posts (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                post_author INTEGER DEFAULT 1,
                post_date TEXT,
                post_date_gmt TEXT,
                post_content TEXT,
                post_title TEXT,
                post_excerpt TEXT,
                post_status TEXT DEFAULT 'publish',
                comment_status TEXT DEFAULT 'open',
                ping_status TEXT DEFAULT 'open',
                post_password TEXT,
                post_name TEXT,
                to_ping TEXT,
                pinged TEXT,
                post_modified TEXT,
                post_modified_gmt TEXT,
                post_content_filtered TEXT,
                post_parent INTEGER DEFAULT 0,
                guid TEXT,
                menu_order INTEGER DEFAULT 0,
                post_type TEXT DEFAULT 'post',
                post_mime_type TEXT,
                comment_count INTEGER DEFAULT 0
            );
        ");

        // Seed default options if empty
        $count = $this->get_var("SELECT COUNT(*) FROM wp_options");
        if ((int)$count === 0) {
            $default_options = [
                'siteurl' => 'http://127.0.0.1:8080',
                'home' => 'http://127.0.0.1:8080',
                'blogname' => 'AR-WP (WordPress on Rust)',
                'blogdescription' => 'Supercharged In-Memory WordPress Runtime Powered by Rust',
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
                'db_version' => 64486,
                'wp_user_roles' => serialize([
                    'administrator' => [
                        'name' => 'Administrator',
                        'capabilities' => ['read' => true, 'edit_posts' => true, 'manage_options' => true, 'install_plugins' => true, 'level_10' => true],
                    ],
                ]),
            ];

            $stmt = $this->pdo->prepare("INSERT OR REPLACE INTO wp_options (option_name, option_value, autoload) VALUES (?, ?, 'yes')");
            foreach ($default_options as $k => $v) {
                $stmt->execute([$k, is_array($v) ? serialize($v) : $v]);
            }

            // Create admin user (id=1, user_login=admin)
            $this->pdo->exec("INSERT OR IGNORE INTO wp_users (ID, user_login, user_nicename, user_email, display_name) VALUES (1, 'admin', 'admin', 'admin@arwp.local', 'Administrator')");
            $this->pdo->exec("INSERT OR IGNORE INTO wp_usermeta (user_id, meta_key, meta_value) VALUES (1, 'wp_capabilities', 'a:1:{s:13:\"administrator\";b:1;}')");
            $this->pdo->exec("INSERT OR IGNORE INTO wp_usermeta (user_id, meta_key, meta_value) VALUES (1, 'wp_user_level', '10')");
        }
    }

    public function set_prefix($prefix, $set_table_names = true) {
        $this->prefix = $prefix;
        if ($set_table_names) {
            foreach ($this->tables as $table) {
                $this->$table = $this->prefix . $table;
            }
        }
        return $this->prefix;
    }

    public function suppress_errors($suppress = true) {
        $prev = $this->suppress_errors;
        $this->suppress_errors = (bool)$suppress;
        return $prev;
    }

    public function query($query) {
        $this->last_query = $query;
        $clean_query = $this->sanitize_query($query);
        $stmt = $this->pdo->query($clean_query);
        if ($stmt === false) {
            $err = $this->pdo->errorInfo();
            $this->last_error = $err[2] ?? 'SQLite query error';
            return false;
        }

        if (stripos(ltrim($clean_query), 'SELECT') === 0 || stripos(ltrim($clean_query), 'SHOW') === 0 || stripos(ltrim($clean_query), 'PRAGMA') === 0) {
            $this->last_result = $stmt->fetchAll(PDO::FETCH_OBJ);
            $this->num_rows = count($this->last_result);
            return $this->num_rows;
        }

        $this->rows_affected = $stmt->rowCount();
        $this->insert_id = (int)$this->pdo->lastInsertId();
        return $this->rows_affected;
    }

    public function get_var($query = null, $x = 0, $y = 0) {
        if ($query) {
            $this->query($query);
        }
        if (!empty($this->last_result) && isset($this->last_result[$y])) {
            $row = (array)$this->last_result[$y];
            $values = array_values($row);
            return $values[$x] ?? null;
        }
        return null;
    }

    public function get_row($query = null, $output = OBJECT, $y = 0) {
        if ($query) {
            $this->query($query);
        }
        if (!empty($this->last_result) && isset($this->last_result[$y])) {
            $row = $this->last_result[$y];
            if ($output === ARRAY_A) {
                return (array)$row;
            }
            return (object)$row;
        }
        return null;
    }

    public function get_col($query = null, $x = 0) {
        if ($query) {
            $this->query($query);
        }
        $col = [];
        if (!empty($this->last_result)) {
            foreach ($this->last_result as $row) {
                $values = array_values((array)$row);
                if (isset($values[$x])) {
                    $col[] = $values[$x];
                }
            }
        }
        return $col;
    }

    public function get_results($query = null, $output = OBJECT) {
        if ($query) {
            $this->query($query);
        }
        if ($output === ARRAY_A) {
            return array_map(function($item) { return (array)$item; }, $this->last_result);
        }
        return $this->last_result;
    }

    public function prepare($query, ...$args) {
        if (isset($args[0]) && is_array($args[0])) {
            $args = $args[0];
        }
        foreach ($args as $arg) {
            $escaped = is_numeric($arg) ? $arg : "'" . SQLite3::escapeString((string)$arg) . "'";
            $pos = strpos($query, '%s');
            if ($pos === false) {
                $pos = strpos($query, '%d');
            }
            if ($pos !== false) {
                $query = substr_replace($query, $escaped, $pos, 2);
            }
        }
        return $query;
    }

    public function _escape($data) {
        if (is_array($data)) {
            foreach ($data as $k => $v) {
                $data[$k] = $this->_escape($v);
            }
            return $data;
        }
        return str_replace("'", "''", (string)$data);
    }

    public function escape_by_ref(&$string) {
        $string = $this->_escape($string);
    }

    public function check_connection($allow_bail = true) {
        return true;
    }

    public function has_cap($cap) {
        return true;
    }

    public function db_version() {
        return '8.0.35-ARWP';
    }

    public function db_server_info() {
        return '8.0.35-ARWP';
    }

    private function sanitize_query($q) {
        // Convert incompatible MySQL syntax to SQLite
        $q = preg_replace('/\bSQL_CALC_FOUND_ROWS\b/i', '', $q);
        $q = preg_replace('/\bNOW\(\)/i', "datetime('now')", $q);
        $q = preg_replace('/\bAUTOLOAD\s+IN\s+\([^)]+\)/i', "autoload = 'yes'", $q);
        return $q;
    }
}

$GLOBALS['wpdb'] = new ARWP_DB(DB_USER, DB_PASSWORD, DB_NAME, DB_HOST);
$wpdb = $GLOBALS['wpdb'];
