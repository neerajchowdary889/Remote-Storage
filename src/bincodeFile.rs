// use std::error::Error;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};
// use bincode::{serialize, deserialize};

// #[derive(Debug)]
// struct Message {
//     name: String,
//     password: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let listener = TcpListener::bind("127.0.0.1:5000").await?;
//     println!("Server listening on port 5000");

//     loop {
//         let (mut stream, _) = listener.accept().await?;
//         println!("New client connected");

//         // Read the message from the client
//         let mut buffer = [0; 1024];
//         let size = stream.read(&mut buffer).await?;
//         let message_data = deserialize(&buffer[..size])?;
//         println!("Received message: {:?}", message_data);

//         // Create a response message
//         let response_message = Message {
//             name: "Server".to_string(),
//             password: 12345,
//         };

//         // Serialize the response message to a byte array and send it to the client
//         let data = serialize(&response_message)?;
//         stream.write_all(&data).await?;
//     }
// }
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    name: String,
    password: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    println!("Server listening on port 5000");

    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("New client connected");

        // Read the message from the client
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).await?;
        let message_data = deserialize(&buffer[..size])?;
        println!("Received message: {:?}", message_data);

        // Create a response message
        let response_message = Message {
            name: "Server".to_string(),
            password: 12345,
        };

        // Serialize the response message to a byte array and send it to the client
        let data = serialize(&response_message)?;
        stream.write_all(&data).await?;
    }
}