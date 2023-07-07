use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use serde_json::json;
use tokio::fs;
use std::process::Command;
use serde_json::Value;

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
        Download(TotalHash);
    }
    else if choice == 3 {
        viewFiles(TotalHash);
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

    let total_hash_clone = total_hash.clone();
    viewFiles(total_hash_clone);

    let status = Command::new("sh")
        .arg("/path/to/script.sh")
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait for command");

    if status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed");
    }

    // let mut stream = TcpStream::connect("198.168.118.50:5150")?;
    let mut stream = TcpStream::connect("127.0.0.1:5500")?;
    // let mut stream = TcpStream::connect("0.tcp.in.ngrok.io:17554").await?;
    println!("Connected to server");

    println!("Enter the IP address of your local pc: ");

    let mut IP = String::new();
    io::stdin().read_line(&mut IP).expect("Failed to read line");
    println!("IP: {}", IP);

    println!("Enter the FileName to retrive: ");

    let mut Retrivename = String::new();
    io::stdin().read_line(&mut Retrivename).expect("Failed to read line");
    println!("IP: {}", IP);
    
    let json_data = json!({
        "request": "1",
        "IP": IP,
        "Filename": Retrivename,
        "TotalHash": total_hash
    });

    let data = serde_json::to_string(&json_data)?;
    stream.write_all(data.as_bytes())?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    let response_data = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response_data);

    let response_json = serde_json::from_str::<serde_json::Value>(&response_data)?;
    let user = response_json.get("User").and_then(|v| v.as_str()).unwrap_or("Unknown User");
    Ok(())
}
fn viewFiles(total_hash: String) -> Result<(), Box<dyn Error>>{
    let mut stream = TcpStream::connect("127.0.0.1:5500")?;
    // let mut stream = TcpStream::connect("198.168.229.50:5150")?;
    // let mut stream = TcpStream::connect("0.tcp.in.ngrok.io:17554").await?;
    println!("Connected to server");

    let json_data = json!({
        "request": "2",
        "TotalHash": total_hash
    });

    let data = serde_json::to_string(&json_data)?;
    stream.write_all(data.as_bytes())?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    let response_data = String::from_utf8_lossy(&buffer[..size]);
    // println!("Received response: {}", response_data);

    let parsed_response: Value = serde_json::from_str(&response_data).unwrap();

    // Check the status field
    if parsed_response["status"] == "success" {
        // Access the response field
        let inner_response = parsed_response["response"].as_str().unwrap();

        // Parse the inner response
        let parsed_inner_response: Value = serde_json::from_str(inner_response).unwrap();

        // Iterate over the parsed inner response
        for (name, value) in parsed_inner_response.as_object().unwrap() {
            println!("FileName: {} ===> CID: {}", name, value);
        }
    }

    let response_json = serde_json::from_str::<serde_json::Value>(&response_data)?;
    let user = response_json.get("User").and_then(|v| v.as_str()).unwrap_or("Unknown User");
    Ok(())
}