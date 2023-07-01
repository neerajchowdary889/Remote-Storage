use rusqlite::{params, Connection, Result};
use serde_json::json;
use std::error::Error;

// pub fn Start(data: serde_json::Value) -> Result<(), Box<dyn Error>> {
//     let conn = establish_connection();
//     create_users_table(&conn)?;
//     let username = data["name"].as_str().unwrap();
//     let password = data["password"].as_str().unwrap();
//     let total_hash = data["TotalHash"].as_str().unwrap();
//     insert_user(&conn, username, password, total_hash)?;
//     Ok(())
// }

// fn establish_connection() -> &'static Connection {
//     Connection::open("users.db").unwrap()
// }

// fn create_users_table(conn: &Connection) -> Result<()> {
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS users (
//             id INTEGER PRIMARY KEY,
//             Username TEXT NOT NULL,
//             Password TEXT NOT NULL,
//             TotalHash TEXT NOT NULL
//         )",
//         [],
//     )?;
//     Ok(())
// }

// fn insert_user(conn: &Connection, username: &str, password: &str, total_hash: &str) -> Result<()> {
//     conn.execute(
//         "INSERT INTO users (username, password, total_hash) VALUES (?1, ?2, ?3)",
//         params![username, password, total_hash],
//     )?;
//     Ok(())
// }

fn establish_connection() -> Result<Connection> {
    Connection::open("credits.db")
}

fn create_users_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            Username TEXT NOT NULL,
            Password TEXT NOT NULL,
            TotalHash TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_user(conn: &Connection, username: &str, password: &str, total_hash: &str) -> Result<()> {

    conn.execute(
        "INSERT INTO users (Username, Password, TotalHash) VALUES (?1, ?2, ?3)",
        params![username, password, total_hash],
    )?;

    Ok(())
}

pub async fn Start(data: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let conn = establish_connection()?;
    create_users_table(&conn)?;
    let username = data["name"].as_str().unwrap();
    let password = data["password"].as_str().unwrap();
    let total_hash = data["TotalHash"].as_str().unwrap();
    insert_user(&conn, username, password, total_hash)?;
    Ok(())
}
