use std::fs;
use rusqlite::Connection;

pub mod db_control;
pub mod ffi;

// --- Enums & Structs ---

pub enum DbType {
    Memory,
    File,
}

pub struct DbConfig {
    pub file_mode: Option<u32>,
    pub sync_mode: Option<String>,
}

pub struct CloseConfig {
    pub force_sync: bool,
}

pub struct FieldDescription {
    pub name: String,
    pub data_type: String,
    pub is_primary: bool,
    pub is_auto_inc: bool,
    pub has_default: bool,
    pub default_val: Option<String>,
}

pub struct FieldUpdateDescription {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldData {
    pub name: String,
    pub data: String, // Stored as string for simplicity, parsed according to schema
}

pub struct FieldId {
    pub col_name: String,
    pub id_val: String,
}

pub enum OrderDirection {
    Asc,
    Desc,
}

pub struct OrderDescript {
    pub col_name: String,
    pub direction: OrderDirection,
}

// Represents the database connection wrapper
pub struct DbConnection {
    pub(crate) conn: Connection,
}

// --- Root Methods ---

/// Open or create a database. Returns Option<DbConnection>.
pub fn open_db(db_type: DbType, path: Option<&str>, config: Option<DbConfig>) -> Option<DbConnection> {
    let conn = match db_type {
        DbType::Memory => Connection::open_in_memory().ok()?,
        DbType::File => {
            let p = path?;
            Connection::open(p).ok()?
        }
    };

    if let Some(cfg) = config {
        if let Some(sync) = cfg.sync_mode {
            let pragma = format!("PRAGMA synchronous = {};", sync);
            let _ = conn.execute(&pragma, []);
        }
    }

    Some(DbConnection { conn })
}

/// Delete a database file. Returns 0 for success, other for error.
pub fn delete_db(path: &str) -> u32 {
    match fs::remove_file(path) {
        Ok(_) => 0,
        Err(_) => 1, // File not found or permission denied
    }
}

// --- Helper Functions ---

/// Convert basic Rust types to Sqlite string representation
pub fn to_sqlite_type(rust_type: &str) -> String {
    match rust_type.to_lowercase().as_str() {
        "i32" | "i64" | "u32" | "usize" => "INTEGER".to_string(),
        "f32" | "f64" => "REAL".to_string(),
        "bool" => "INTEGER".to_string(), // SQLite uses 0/1 for booleans
        "string" | "str" => "TEXT".to_string(),
        _ => "BLOB".to_string(),
    }
}

/// Convert Sqlite string representation back to Rust type concept (simplified)
pub fn from_sqlite_type(sqlite_type: &str) -> String {
    match sqlite_type.to_uppercase().as_str() {
        "INTEGER" => "i64".to_string(),
        "REAL" => "f64".to_string(),
        "TEXT" => "String".to_string(),
        "BLOB" => "Vec<u8>".to_string(),
        _ => "String".to_string(),
    }
}
