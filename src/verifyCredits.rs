use rusqlite::{params, Connection};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
// use ServerLogin;

pub async fn check_user_credentials<E>(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    let conn = Connection::open("credits.db")?;
    
    let username = data["name"].as_str().ok_or("Missing username")?;
    let password = data["password"].as_str().ok_or("Missing password")?;
    let total_hash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

    let mut stmt = conn.prepare("SELECT * FROM users WHERE Username = ? AND Password = ?")?;
    let mut rows = stmt.query(params![username, password])?;

    let mut is_authenticated = false;
    while let Ok(Some(row)) = rows.next() {
        is_authenticated = true;
    }

    if is_authenticated {
        // Send the response back to the client
        let response = valid(data)?;

        let status = response["status"].as_str().unwrap();
        let message = response["message"].as_str().unwrap();

        println!("Status: {}", status);
        println!("Message: {}", message);

        Ok(response)
    } else {
        let response = invalid(data)?;

        let status = response["status"].as_str().unwrap();
        let message = response["message"].as_str().unwrap();

        println!("Status: {}", status);
        println!("Message: {}\n", message);

        Ok(response)
    }
}

pub fn valid(data: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {

    Ok(serde_json::json!({
        "status": "Valid User",
        "message": "Thankyou for Logging in"
    }))
}
pub fn invalid(data: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {

    Ok(serde_json::json!({
        "status": "Invalid User",
        "message": "Create an account to login"
    }))
}