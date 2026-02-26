use crate::*;
use rusqlite::params_from_iter;

impl DbConnection {
    /// Close the database connection with config.
    pub fn close(self, config: Option<CloseConfig>) -> u32 {
        if let Some(cfg) = config {
            if cfg.force_sync {
                let _ = self.conn.execute("PRAGMA wal_checkpoint(TRUNCATE);", []);
            }
        }
        match self.conn.close() {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Create a table based on FieldDescriptions.
    pub fn create_table(&self, table_name: &str, table_desc: Vec<FieldDescription>) -> u32 {
        let mut primary_count = 0;
        let mut columns = Vec::new();

        for field in table_desc {
            if field.is_primary {
                primary_count += 1;
            }
            if primary_count > 1 {
                return 2; // Error 2: Multiple primary keys abort
            }

            let mut col_def = format!("{} {}", field.name, field.data_type);

            if field.is_primary {
                col_def.push_str(" PRIMARY KEY");
            }
            if field.is_auto_inc {
                col_def.push_str(" AUTOINCREMENT");
            }
            if field.has_default {
                if let Some(val) = field.default_val {
                    col_def.push_str(&format!(" DEFAULT {}", val));
                }
            }
            columns.push(col_def);
        }

        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns.join(", "));
        match self.conn.execute(&sql, []) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Drop a table.
    pub fn drop_table(&self, table_name: &str) -> u32 {
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        match self.conn.execute(&sql, []) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Update table schema (drop columns, rename columns, add columns).
    pub fn update_table(
        &self,
        table_name: &str,
        remove_field: Vec<String>,
        update_field: Vec<FieldUpdateDescription>,
        new_field: Vec<FieldDescription>,
    ) -> u32 {
        // Drop columns (requires SQLite >= 3.35.0)
        for col in remove_field {
            let sql = format!("ALTER TABLE {} DROP COLUMN {}", table_name, col);
            if self.conn.execute(&sql, []).is_err() {
                return 1; // Assuming dropping primary key triggers native SQLite error or unsupported
            }
        }

        // Rename columns (requires SQLite >= 3.25.0)
        for update in update_field {
            let sql = format!(
                "ALTER TABLE {} RENAME COLUMN {} TO {}",
                table_name, update.old_name, update.new_name
            );
            if self.conn.execute(&sql, []).is_err() { return 1; }
        }

        // Add new columns
        for field in new_field {
            if field.is_primary {
                return 3; // Error 3: Cannot add a primary key column to existing table
            }
            let mut col_def = format!("{} {}", field.name, field.data_type);
            if field.has_default {
                if let Some(val) = field.default_val {
                    col_def.push_str(&format!(" DEFAULT {}", val));
                }
            }
            let sql = format!("ALTER TABLE {} ADD COLUMN {}", table_name, col_def);
            if self.conn.execute(&sql, []).is_err() { return 1; }
        }

        0
    }

    /// Insert data into the table.
    pub fn insert_data(&self, table_name: &str, data: Vec<FieldData>) -> u32 {
        let cols: Vec<String> = data.iter().map(|d| d.name.clone()).collect();
        let placeholders: Vec<String> = data.iter().map(|_| "?".to_string()).collect();
        let values: Vec<String> = data.into_iter().map(|d| d.data).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            cols.join(", "),
            placeholders.join(", ")
        );

        match self.conn.execute(&sql, params_from_iter(values)) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Update data in the table.
    pub fn update_data(&self, table_name: &str, data: Vec<FieldData>, id: FieldId) -> u32 {
        let set_clause: Vec<String> = data.iter().map(|d| format!("{} = ?", d.name)).collect();
        let mut values: Vec<String> = data.into_iter().map(|d| d.data).collect();

        let sql = format!(
            "UPDATE {} SET {} WHERE {} = ?",
            table_name,
            set_clause.join(", "),
            id.col_name
        );

        values.push(id.id_val); // Add ID value for the WHERE clause

        match self.conn.execute(&sql, params_from_iter(values)) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Delete data from the table.
    pub fn delete_data(&self, table_name: &str, id: FieldId) -> u32 {
        let sql = format!("DELETE FROM {} WHERE {} = ?", table_name, id.col_name);
        match self.conn.execute(&sql, [&id.id_val]) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    /// Query data from the table.
    pub fn query_data(
        &self,
        table_name: &str,
        id: FieldId,
        limit: u32,
        orders: Vec<OrderDescript>,
    ) -> Option<Vec<Vec<FieldData>>> {
        let mut sql = format!("SELECT * FROM {} WHERE {} = ?", table_name, id.col_name);

        if !orders.is_empty() {
            let order_clauses: Vec<String> = orders.iter().map(|o| {
                let dir = match o.direction {
                    OrderDirection::Asc => "ASC",
                    OrderDirection::Desc => "DESC",
                };
                format!("{} {}", o.col_name, dir)
            }).collect();
            sql.push_str(" ORDER BY ");
            sql.push_str(&order_clauses.join(", "));
        }

        sql.push_str(&format!(" LIMIT {}", limit));

        let mut stmt = self.conn.prepare(&sql).ok()?;
        let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();

        let rows = stmt.query_map([&id.id_val], |row| {
            let mut row_data = Vec::new();
            for (i, name) in column_names.iter().enumerate() {
                // Read everything as string for generalized return type
                let val: String = row.get(i).unwrap_or_else(|_| "NULL".to_string());
                row_data.push(FieldData {
                    name: name.clone(),
                    data: val,
                });
            }
            Ok(row_data)
        }).ok()?;

        let mut result = Vec::new();
        for r in rows {
            if let Ok(data) = r {
                result.push(data);
            }
        }

        Some(result)
    }
}
