use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    name: String,
    password: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:5000").await?;
    println!("Connected to server");

    // Create a message struct to send to the server
    let message = Message {
        name: "Neeraj".to_string(),
        password: 25267
    };

    // Serialize the message to a JSON string and send it to the server
    let json_data = serde_json::to_string(&message)?;
    stream.write_all(json_data.as_bytes()).await?;

    // Read the response from the server
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let response_data = String::from_utf8_lossy(&buffer[..size]).to_string();
    println!("Received response: {}", response_data);

    // Serialize the message to a byte array and send it to the server
    let data = serialize(&message)?;
    stream.write_all(&data).await?;

    // Read the response from the server
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;
    let response_data = deserialize(&buffer[..size])?;
    println!("Received response: {:?}", response_data);

    Ok(())
}