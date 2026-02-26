use criterion::{criterion_group, criterion_main, Criterion};
use sqlite3_utils::*;

fn bench_inserts(c: &mut Criterion) {
    let db = open_db(DbType::Memory, None, None).unwrap();
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
            name: "payload".to_string(),
            data_type: "TEXT".to_string(),
            is_primary: false,
            is_auto_inc: false,
            has_default: false,
            default_val: None,
        },
    ];
    db.create_table("bench_table", table_desc);

    c.bench_function("insert_single_row", |b| {
        b.iter(|| {
            let data = vec![FieldData {
                name: "payload".to_string(),
                data: "benchmark_data".to_string(),
            }];
            db.insert_data("bench_table", data);
        })
    });
}

criterion_group!(benches, bench_inserts);
criterion_main!(benches);
