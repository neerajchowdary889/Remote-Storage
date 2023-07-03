#![allow(warnings)]
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde_json::json;
mod CreateAccount;
mod clientLogin;
mod validuser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Prompt the user to choose between creating an account and logging in
    println!("Choose an option:");
    println!("1. Create account");
    println!("2. Log in");
    let choice = read_user_input();

    // Call the appropriate function based on the user's choice
    match choice {
        1 => create_account().await?,
        2 => log_in().await?,
        _ => println!("Invalid choice"),
    }

    Ok(())
}

async fn create_account() -> Result<(), Box<dyn Error>> {
    let (username, password, total_hash) = CreateAccount::CreateAccount();

    // let mut stream = TcpStream::connect("127.0.0.1:5000").await?;
    let mut stream = TcpStream::connect("0.tcp.in.ngrok.io:18096").await?;
    println!("Connected to server");

    let json_data = json!({
        "name": username,
        "password": password,
        "TotalHash": total_hash
    });

    let data = serde_json::to_string(&json_data)?;
    stream.write_all(data.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let response_data = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response_data);

    Ok(())
}

async fn log_in() -> Result<(), Box<dyn Error>> {
    let (username, password, total_hash) = clientLogin::Login();

    let mut stream = TcpStream::connect("127.0.0.1:5050").await?;
    // let mut stream = TcpStream::connect("0.tcp.in.ngrok.io:17554").await?;
    println!("Connected to server");

    let json_data = json!({
        "name": username,
        "password": password,
        "TotalHash": total_hash
    });

    let data = serde_json::to_string(&json_data)?;
    stream.write_all(data.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let response_data = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response_data);

    let response_json = serde_json::from_str::<serde_json::Value>(&response_data)?;
    let user = response_json.get("User").and_then(|v| v.as_str()).unwrap_or("Unknown User");

    match user{
        "Unknown User" => println!("User not found"),
        "Invalid User" => println!("Invalid Password"),
        "Valid User" => validuser::User(total_hash),
        _ => println!("Unknown Error"),
    }

    Ok(())
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}