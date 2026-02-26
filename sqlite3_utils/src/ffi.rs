use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::{open_db, DbType, DbConfig, DbConnection};

// --- FFI Enums ---
// Use #[repr(C)] to ensure C-compatible memory layout
#[repr(C)]
pub enum CDbType {
    Memory = 0,
    File = 1,
}

// --- FFI Methods ---

/// Open the database from C.
/// Returns a raw pointer to DbConnection. Returns null if failed.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_utils_open_db(
    db_type: CDbType,
    path: *const c_char,
) -> *mut DbConnection {
    let r_db_type = match db_type {
        CDbType::Memory => DbType::Memory,
        CDbType::File => DbType::File,
    };

    let r_path = if path.is_null() {
        None
    } else {
        // Convert C string to Rust str safely
        let c_str = unsafe { CStr::from_ptr(path) };
        match c_str.to_str() {
            Ok(s) => Some(s),
            Err(_) => return std::ptr::null_mut(), // Invalid UTF-8
        }
    };

    // Call the original Rust function (ignoring config for simplicity here)
    match open_db(r_db_type, r_path, None) {
        Some(conn) => {
            // Box the connection and leak it to pass ownership to C
            Box::into_raw(Box::new(conn))
        }
        None => std::ptr::null_mut(),
    }
}

/// Close the database and free the memory allocated by Rust.
/// It is CRITICAL to call this to prevent memory leaks.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_utils_close_db(conn_ptr: *mut DbConnection) -> u32 {
    if conn_ptr.is_null() {
        return 1; // Error: null pointer
    }

    // Re-construct the Box to automatically drop and free memory
    let conn = unsafe { Box::from_raw(conn_ptr) };
    
    // Call the actual close method
    conn.close(None)
}

#[repr(C)]
pub struct CFieldDescription {
    pub name: *const c_char,
    pub data_type: *const c_char,
    pub is_primary: bool,
    pub is_auto_inc: bool,
    pub has_default: bool,
    pub default_val: *const c_char,
}

use crate::FieldDescription; // Ensure this is imported

#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_utils_create_table(
    conn_ptr: *const DbConnection,
    table_name: *const c_char,
    fields_ptr: *const CFieldDescription,
    fields_len: usize,
) -> u32 {
    if conn_ptr.is_null() || table_name.is_null() || fields_ptr.is_null() {
        return 1; 
    }

    // 1. Safely borrow the DbConnection
    let conn = unsafe { &*conn_ptr };
    
    // 2. Convert table_name from C string
    let r_table_name = unsafe { CStr::from_ptr(table_name) }.to_string_lossy();

    // 3. Explicitly type the Vector to satisfy the compiler
    let mut r_fields: Vec<FieldDescription> = Vec::with_capacity(fields_len);

    // 4. Convert C array to Rust slice
    let c_fields = unsafe { std::slice::from_raw_parts(fields_ptr, fields_len) };

    for c_field in c_fields {
        // Safety: Ensure these pointers aren't null before converting
        if c_field.name.is_null() || c_field.data_type.is_null() {
            return 4; // Error code for invalid field data
        }

        let name = unsafe { CStr::from_ptr(c_field.name) }.to_string_lossy().into_owned();
        let data_type = unsafe { CStr::from_ptr(c_field.data_type) }.to_string_lossy().into_owned();
        
        let default_val = if !c_field.default_val.is_null() {
            Some(unsafe { CStr::from_ptr(c_field.default_val) }.to_string_lossy().into_owned())
        } else {
            None
        };

        r_fields.push(FieldDescription {
            name,
            data_type,
            is_primary: c_field.is_primary,
            is_auto_inc: c_field.is_auto_inc,
            has_default: c_field.has_default,
            default_val,
        });
    }

    // 5. Call the native Rust method now that types are clear
    conn.create_table(&r_table_name, r_fields)
}
