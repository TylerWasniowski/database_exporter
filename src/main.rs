#![feature(test)]
extern crate test;
extern crate serde_json;
#[macro_use]
extern crate fake;
extern crate rusqlite;

mod generate_records;
mod serializer;

fn main() {
    let conn = generate_records::create_conn("./db.sqlite".to_string());
//    generate_records::insert_generated_records(&conn, 2);
    println!("{}", serializer::to_json(&conn));
}
