// use std::error::Error;
// use std::path::PathBuf;
// use tokio::net::TcpListener;
// // use tokio::prelude::*;
// use tokio::fs::File as TokioFile;
// use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, ErrorKind};
// use tokio::fs::OpenOptions;
// use tokio::net::TcpStream;
// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // let listener = TcpListener::bind("0.0.0.0:5100").await?;
//     let listener = TcpListener::bind("127.0.0.1:5100").await?;
//     println!("FTP server listening on port 5100");

//     loop {

//         let (mut socket, _) = listener.accept().await?;
//         // let socket_copy = Arc::new(socket);
//         // let socket_copy_clone = socket_copy.clone();

//         tokio::spawn(async move {
//             // let mut reader = BufReader::new(socket_copy);
//             // let mut writer = BufWriter::new(socket);
//             let (reader, mut writer) = socket.split();
//             let mut reader = BufReader::new(reader);

//             // Read the file name from the client
//             let mut buffer = [0; 4096];
//             let size = reader.read(&mut buffer).await.unwrap();
//             let mut file_name = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
//             // println!("File name: {}", file_name);

//             // Read the string from the client
//             let size = reader.read(&mut buffer).await.unwrap();
//             let string_data = String::from_utf8_lossy(&buffer[..size]).trim().to_string();

//             // Create a new file with the received file name in the Downloads folder
//             let len = file_name.len();
//             let hash_File = file_name.split_off(len - 40);
//             let FTP_File_name = file_name;

//             // println!("Hash: {} and File: {}", hash_File, FTP_File_name);
//             // let mut file = format!("UserFolders/{}", hash_File.to_string());
//             // println!("File: {}", file);
//             // let current_dir = env::current_dir().unwrap();
//             let mut file_path = PathBuf::new();
        
//             // env::set_current_dir(&file).unwrap();
//             // println!("File2: {}", file);
//             file_path.push("UserFolders");
//             // println!("File3: {}", file);
//             file_path.push(&hash_File);
//             // println!("File4: {}", file);
//             file_path.push(&FTP_File_name);
//             // println!("File5: {}", file_path.display());
//             let mut file = OpenOptions::new().create(true).write(true).open(file_path).await.unwrap();
//             println!("Line 59");
//             // Read the file data from the client and write it to the file
//             let mut total_bytes = 0;
//             loop {
//                 let size = reader.read(&mut buffer).await.unwrap();
//                 if size == 0 {
//                     break;
//                 }
//                 total_bytes += size;
//                 file.write_all(&buffer[..size]).await.unwrap();
//             }
//             // loop {
//             //     let size = reader.read(&mut buffer).await.unwrap();
//             //     if size == 0 {
//             //         break;
//             //     }
//             //     file.write_all(&buffer[..size]).await.unwrap();
//             // }

//             // Send a response back to the client
//             let response = "File received successfully";
//             writer.write_all(response.as_bytes()).await.unwrap();
//         });
//     }
// }

// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

// use rocket::Data;
// use rocket::DataPart;
// use rocket::http::ContentType;
// use rocket::http::Status;
// use rocket::response::status;
// use std::io::{self, Read};
// use std::fs::{self, File};

// #[post("/", data = "<data>")]
// fn upload_file(data: Data, name: String) -> io::Result<String> {
//     // Create a folder for the user if it doesn't exist
//     let user_folder = format!("UserFolders/{}", name);
//     if !user_folder.exists() {
//         fs::create_dir_all(&user_folder)?;
//     }

//     // Read the data from the request
//     let mut buffer = Vec::new();
//     data.open().read_to_end(&mut buffer)?;

//     // Decode the base64-encoded data
//     let decoded_data = base64::decode(&buffer)?;

//     // Save the PDF file in the user's folder
//     let file_path = format!("{}/uploaded.pdf", user_folder);
//     let mut file = File::create(&file_path)?;
//     file.write_all(&decoded_data)?;

//     Ok(String::from("File uploaded successfully!"))
// }

// fn main() {
//     rocket::ignite().mount("/", routes![upload_file]).launch().port(5100);
// }

#![allow(warnings)]
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::thread;
use tokio::task;
use tokio::runtime::Runtime;
mod verifyFiledata;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:5100").await?;
    println!("Server listening on port 5100");

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Client connected: {:?}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            if let Ok(size) = stream.read(&mut buffer).await {
                let received_data = String::from_utf8_lossy(&buffer[..size]);
                println!("Received message: {}", received_data);

                let json_data = match serde_json::from_str::<serde_json::Value>(&received_data) {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("Error parsing JSON data: {}", err);
                        return;
                    }
                };


                let json_data_copy = json_data.clone();

                let response = match verifyFiledata::CreateFile::<Box<dyn Error>>(json_data_copy).await {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("Error processing JSON data: {}", err);
                        return;
                    }
                };
                let status = response["status"].as_str().unwrap();

                let response = match process_json_data(json_data) {
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

fn process_json_data(data: serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {

    Ok(serde_json::json!({
        "status": "success",
        "message": "JSON data processed successfully"
    }))
    
}