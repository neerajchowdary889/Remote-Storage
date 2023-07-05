use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use serde_json::json;
use tokio::fs;

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

async fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:5100")?;
    println!("Connected to server");
    
    // Send the file name to the server
    let file_name = "cr.txt";
    let file_contents = fs::read_to_string(file_name).await?;

    let json_data = json!({
        "Filename": file_name,
        "TotalHash": total_hash,
        "Content": file_contents
    });

    let data = serde_json::to_string(&json_data)?;
    stream.write_all(data.as_bytes())?;

    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;
    let response_data = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response_data);

    Ok(())

}

// fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
//     let mut stream = TcpStream::connect("127.0.0.1:5100")?;
//     println!("Connected to server");

//     // Send the file name to the server
//     let file_name = "cr.txt";
//     // let file_name = "4Blocks (1).pdf";
//     stream.write_all(file_name.as_bytes())?;

//     // Send the string to the server
//     let string_data = total_hash;
//     stream.write_all(string_data.as_bytes())?;

//     // Open the file and send its contents to the server
//     let mut file = std::fs::File::open(file_name)?;
//     let mut buffer = [0; 4096];
//     loop {
//         let size = file.read(&mut buffer)?;
//         if size == 0 {
//             break;
//         }
//         stream.write_all(&buffer[..size])?;
//     }

//     // Wait for the server to respond
//     let mut response = String::new();
//     stream.read_to_string(&mut response)?;
//     println!("Server response: {}", response);

//     Ok(())
// }


// fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
//     let mut stream = TcpStream::connect("127.0.0.1:5100")?;
//     println!("Connected to server");

//     // Send the file name to the server
//     let file_name = "cr.txt";
//     stream.write_all(file_name.as_bytes())?;

//     // Send the string to the server
//     let string_data = total_hash;
//     stream.write_all(string_data.as_bytes())?;

//     // Open the file and send its contents to the server
//     let mut file = File::open(file_name)?;
//     let mut buffer = [0; 4096]; // Increase buffer size
//     let mut total_bytes = 0;
//     loop {
//         let size = file.read(&mut buffer)?;
//         if size == 0 {
//             break;
//         }
//         stream.write_all(&buffer[..size])?;
//         total_bytes += size;
//     }

//     // Wait for the server to respond
//     let mut response = String::new();
//     stream.read_to_string(&mut response)?;
//     println!("Server response: {}", response);

//     Ok(())
// }





// use reqwest::blocking::multipart::{Form, Part};
// use reqwest::blocking::Client;
// use std::fs::File;
// use std::io::Read;
// use std::error::Error;

// fn ServerUpload(total_hash: String) -> Result<(), Box<dyn Error>> {
//     // Read the PDF file
//     let mut file = File::open("4Blocks (1).pdf")?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;

//     // Convert the PDF file to a reqwest Part
//     let part = Part::bytes(buffer)
//         .file_name(file)
//         .mime_str("application/pdf")?;

//     // Prepare the form data
//     let name = total_hash.to_string();
//     let form = Form::new()
//         .text("name", name.to_owned())
//         .part("file", part);

//     // Send the form data to the server
//     let client = Client::new();
//     let response = client
//         .post("http://localhost:5100")
//         .multipart(form)
//         .send()?;

//     // Check the response
//     if response.status().is_success() {
//         println!("File uploaded successfully!");
//     } else {
//         println!("Failed to upload the file.");
//     }

//     Ok(())
// }
