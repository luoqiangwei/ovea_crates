use sqlite3_utils::*;

#[test]
fn test_database_lifecycle() {
    // 1. Open memory DB
    let db = open_db(DbType::Memory, None, None).expect("Failed to open DB");

    // 2. Create Table
    let table_desc = vec![
        FieldDescription {
            name: "id".to_string(),
            data_type: "INTEGER".to_string(),
            is_primary: true,
            is_auto_inc: true,
            has_default: false,
            default_val: None,
        },
        FieldDescription {
            name: "username".to_string(),
            data_type: "TEXT".to_string(),
            is_primary: false,
            is_auto_inc: false,
            has_default: false,
            default_val: None,
        },
    ];
    let res = db.create_table("users", table_desc);
    assert_eq!(res, 0);

    // 3. Insert Data
    let data = vec![
        FieldData { name: "username".to_string(), data: "Alice".to_string() }
    ];
    let res = db.insert_data("users", data);
    assert_eq!(res, 0);

    // 4. Query Data
    let id_field = FieldId { col_name: "id".to_string(), id_val: "1".to_string() };
    let results = db.query_data("users", id_field, 10, vec![]);
    assert!(results.is_some());
    let rows = results.unwrap();
    assert_eq!(rows.len(), 1);

    // Check returned username
    let username_field = rows[0].iter().find(|f| f.name == "username").unwrap();
    assert_eq!(username_field.data, "Alice");

    // 5. Delete Data
    let del_id = FieldId { col_name: "id".to_string(), id_val: "1".to_string() };
    let res = db.delete_data("users", del_id);
    assert_eq!(res, 0);

    // 6. Close DB
    let res = db.close(None);
    assert_eq!(res, 0);
}

// rust call c/cpp
use std::os::raw::c_int;

// 1. Declare the external interface
unsafe extern "C" {
    // Match the signature in my_math.h
    unsafe fn fast_add(a: c_int, b: c_int) -> c_int;
}

#[test]
fn test_c_math_library() {
    let x = 10;
    let y = 20;

    // 2. Wrap the unsafe C call
    let result = unsafe { fast_add(x, y) };

    println!("Result from C: {}", result);
    assert_eq!(result, 30);
}

