use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;

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
    let mut stream = TcpStream::connect("127.0.0.1:5100")?;
    println!("Connected to server");

    // Send the file name to the server
    let file_name = "cr.txt";
    stream.write_all(file_name.as_bytes())?;

    // Send the string to the server
    let string_data = total_hash;
    stream.write_all(string_data.as_bytes())?;

    // Open the file and send its contents to the server
    let mut file = std::fs::File::open(file_name)?;
    let mut buffer = [0; 1024];
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }
        stream.write_all(&buffer[..size])?;
    }

    // Wait for the server to respond
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("Server response: {}", response);

    Ok(())
}