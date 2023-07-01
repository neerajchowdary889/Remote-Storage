use rusqlite::{params, Connection, Result};
use serde_json::json;
use std::error::Error;
use std::env;
use std::fs;

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
fn create_UserFolder(total_hash: &str) -> bool{
    if let Err(e) = env::set_current_dir("UserFolders") {
        eprintln!("Failed to change directory: {}", e);
        return false;
    }
    if let Err(e) = fs::create_dir(total_hash) {
        eprintln!("Failed to create directory: {}", e);
        return false;
    }
    true
}

pub async fn Start(data: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let conn = establish_connection()?;

    create_users_table(&conn)?;

    let username = data["name"].as_str().unwrap();
    let password = data["password"].as_str().unwrap();
    let total_hash = data["TotalHash"].as_str().unwrap();

    insert_user(&conn, username, password, total_hash)?;

    if create_UserFolder(total_hash){
        println!("{} User folder created",total_hash);
    }
    else{
        println!("Failed to create user folder");
    }

    Ok(())
}
