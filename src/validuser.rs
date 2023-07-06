use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use serde_json::json;
use tokio::fs;
use std::process::Command;

pub fn User(TotalHash: String){
    println!("Choose an option:");
    println!("1. Upload File");
    println!("2. Download File");
    println!("3. View Files");
    let choice = read_user_input();

    // Call the appropriate function based on the user's choice
    if choice == 1 {
        ServerUpload(TotalHash);
    }
    else if choice == 2 {
        println!("Download File");
    }
    else if choice == 3 {
        println!("View Files");
    }
    else {
        println!("Invalid choice");
    }
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
    let filename = "ComputerNetworks-Abstract.pdf";
    let output = Command::new("python3")
    .arg("send.py")
    .arg("sendfile")
    .arg(total_hash)
    .arg(filename)
    .output()
    .expect("Failed to execute command");

    if output.status.success() {
        let stdout = std::str::from_utf8(&output.stdout)?;
        let result = stdout.trim();

        println!("Result: {}", result);
    } else {
        let stderr = std::str::from_utf8(&output.stderr)?;
        println!("Error: {}", stderr);
    }

    Ok(())

}

fn Download(total_hash: String) -> Result<(), Box<dyn Error>>{
    let (username, password, total_hash) = clientLogin::Login();

    let mut stream = TcpStream::connect("127.0.0.1:5150").await?;
    // let mut stream = TcpStream::connect("0.tcp.in.ngrok.io:17554").await?;
    println!("Connected to server");

    let json_data = json!({
        "command": 1,
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
    Ok(())
}
