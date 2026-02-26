#ifndef SQLITE3_UTILS_H
#define SQLITE3_UTILS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CDbType {
  Memory = 0,
  File = 1,
} CDbType;

typedef struct DbConnection DbConnection;

typedef struct CFieldDescription {
  const char *name;
  const char *data_type;
  bool is_primary;
  bool is_auto_inc;
  bool has_default;
  const char *default_val;
} CFieldDescription;

// Open the database from C.
// Returns a raw pointer to DbConnection. Returns null if failed.
struct DbConnection *sqlite3_utils_open_db(enum CDbType db_type, const char *path);

// Close the database and free the memory allocated by Rust.
// It is CRITICAL to call this to prevent memory leaks.
uint32_t sqlite3_utils_close_db(struct DbConnection *conn_ptr);

uint32_t sqlite3_utils_create_table(const struct DbConnection *conn_ptr,
                                    const char *table_name,
                                    const struct CFieldDescription *fields_ptr,
                                    uintptr_t fields_len);

#endif  /* SQLITE3_UTILS_H */
