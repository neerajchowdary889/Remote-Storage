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
