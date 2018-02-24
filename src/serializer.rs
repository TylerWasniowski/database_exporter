use rusqlite;
use rusqlite::{Connection, Error};
use serde_json;

use std::collections::HashMap;

fn map_data<T, F>(conn: &Connection, f: F) -> Vec<HashMap<String, T>>
    where F: Fn(rusqlite::types::Value) -> T
{
    let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
    let stmt2 = conn.prepare("SELECT * FROM users").unwrap();
    let column_names = &stmt2.column_names()[..];
    let mut rows = stmt.query(&[]).unwrap();

    let mut res: Vec<HashMap<String, T>> = Vec::new();
    while let Some(Ok(row)) = rows.next() {
        let mut res_row: HashMap<String, T> = HashMap::new();
        for i in 0..row.column_count() {
            res_row.insert(column_names.get(i as usize).unwrap().to_string(),
                           f(row.get(i)));
        }
        res.push(res_row);
    }
    res
}

pub fn to_json(conn: &Connection) -> String {
    serde_json::to_string(&map_data(conn, |cell| {
        match cell {
            rusqlite::types::Value::Null => "Null".to_string(),
            rusqlite::types::Value::Integer(n) => n.to_string(),
            rusqlite::types::Value::Real(n) => n.to_string(),
            rusqlite::types::Value::Text(s) => s,
            rusqlite::types::Value::Blob(data) => serde_json::to_string(&data).unwrap(),
        }
    })).unwrap()
}
