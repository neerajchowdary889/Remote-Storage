#![allow(warnings)]
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::thread;
use tokio::task;
use tokio::runtime::Runtime;
mod verifyCredits;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:5050").await?;
    println!("Server listening on port 5050");

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Client connected: {:?}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            if let Ok(size) = stream.read(&mut buffer).await {
                let received_data = String::from_utf8_lossy(&buffer[..size]);
                println!("Received message: {}", received_data);

                // Parse the received JSON data
                let json_data = match serde_json::from_str::<serde_json::Value>(&received_data) {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("Error parsing JSON data: {}", err);
                        return;
                    }
                };

                // Process the JSON data

                let json_data_copy = json_data.clone();

                // tokio::spawn(async move {
                //     let task = task::spawn(async move {
                //         verifyCredits::check_user_credentials(json_data_copy).await;
                //     });
                //     task.await.unwrap();
                // }).await.unwrap();

                let response = match verifyCredits::check_user_credentials::<Box<dyn Error>>(json_data_copy).await {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("Error processing JSON data: {}", err);
                        return;
                    }
                };
                let status = response["status"].as_str().unwrap();

                let response = match process_json_data(json_data, status.to_string()) {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("Error processing JSON data: {}", err);
                        return;
                    }
                };

                // Send the response back to the client
                let response_data = serde_json::to_string(&response).unwrap();
                if let Err(err) = stream.write_all(response_data.as_bytes()).await {
                    eprintln!("Error writing to socket: {}", err);
                }
            } else {
                eprintln!("Error reading from socket");
            }
        });
    }
}

fn process_json_data(data: serde_json::Value, status: String) -> Result<serde_json::Value, Box<dyn Error>> {

    Ok(serde_json::json!({
        "status": "success",
        "User": status,
        "message": "JSON data processed successfully"
    }))
    
}
