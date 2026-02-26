// In main.c
#include <stdio.h>
#include "sqlite3_utils.h"

int main() {
    // 1. Open database
    DbConnection* conn = sqlite3_utils_open_db(File, "./test.db");
    if (!conn) {
        printf("Failed to open DB\n");
        return 1;
    }

    // 2. Do operations (create table, insert, etc...)
    CFieldDescription id_field = {
        .name = "id",
        .data_type = "INTEGER",
        .is_primary = true,
        .is_auto_inc = true,
        .has_default = false,
        .default_val = NULL
    };

    CFieldDescription name_field = {
        .name = "username",
        .data_type = "TEXT",
        .is_primary = false,
        .is_auto_inc = false,
        .has_default = true,
        .default_val = "'guest'" // Note that SQLite default value strings require single quotes
    };

    CFieldDescription fields[] = { id_field, name_field };

    // 3. Call the Rust table creation function
    printf("Creating table 'users'...\n");
    uint32_t status = sqlite3_utils_create_table(conn, "users", fields, 2);

    if (status == 0) {
        printf("Table created successfully!\n");
    } else {
        printf("Failed to create table, error code: %u\n", status);
    }
    
    // 4. Close database and free memory
    uint32_t result = sqlite3_utils_close_db(conn);
    if (result == 0) {
        printf("DB closed successfully\n");
    }
    
    return 0;
}
