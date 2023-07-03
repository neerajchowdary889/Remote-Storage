use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:21")?;
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