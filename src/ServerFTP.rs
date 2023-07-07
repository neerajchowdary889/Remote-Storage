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


#![allow(warnings)]
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::thread;
use tokio::task;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use serde_json::{json, Value};
use std::process::Command;
use rusqlite::{params, Connection, Result};

// mod clientRequests;
mod parseDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:5500").await?;
    println!("Server listening on port 5500");

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
                let response = match check_requests(json_data_copy).await {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("Error processing JSON data: {}", err);
                        return;
                    }
                };

                let response = match process_json_data(json_data, &response) {
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

fn process_json_data(data: serde_json::Value, Response: &str) -> Result<serde_json::Value, Box<dyn Error>> {

    println!("156");
    Ok(serde_json::json!({

        "status": "success",
        "response": Response
    }))
    
}
// use std::process::Command;
// use std::error::Error;
// fn main() -> Result<(), Box<dyn Error>> {

//     let output = Command::new("python3")
//     .arg("receiver.py")
//     .arg("StartServer")
//     .output()
//     .expect("Failed to execute command");

//     if output.status.success() {
//         let stdout = std::str::from_utf8(&output.stdout)?;
//         let result = stdout.trim();

//         println!("Result: {}", result);
//     } else {
//         let stderr = std::str::from_utf8(&output.stderr)?;
//         println!("Error: {}", stderr);
//     }

//     Ok(())

// }

// pub async fn check_requests(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    
//     let request = data["request"].as_str().ok_or("Missing request number")?;
//     let totalhash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

//     if request == "1" {
//         let response = true;
//         println!("Download files");
//         Ok(json!({ "response": response }))
//     } 
//     else if request == "2" {
//         // let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();

//         let map = parseDB::read_user_files(totalhash);
//         // Add more key-value pairs as needed

//         let map_value = serde_json::to_value(&map)?;
//         Ok(json!({"response": map_value}))
//     } 
//     else {
//         let err = "Invalid request number";
//         println!("Error: {}", err);
//         Ok(json!({ "error": err }))
//     }
// }
// pub async fn check_requests(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
//     let request = data["request"].as_str().ok_or("Missing request number")?;
//     let totalhash = data["TotalHash"].as_str().ok_or("Missing total hash")?;
//     println!("{}, {}",request, totalhash);

//     if request == "1" {
//         let response = true;
//         println!("Download files");
//         Ok(json!({ "response": response }))
//     } else if request == "2" {
//         println!("221");
//         let map = match parseDB::read_user_files(totalhash) {
//             Ok(map) => map,
//             Err(err) => {
//                 let error_msg = err.to_string();
//                 return Ok(json!({ "error": error_msg }));
//             }
//         };

//         println!("231");
//         Ok(json!({ "response": map }))
//     } else {
//         let err = "Invalid request number";
//         println!("Error: {}", err);
//         Ok(json!({ "error": err }))
//     }
// }
// pub async fn check_requests(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
//     let request = data["request"].as_str().ok_or("Missing request number")?;
//     let totalhash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

//     if request == "1" {
//         let response = true;
//         println!("Download files");
//         Ok(json!({ response }))
//     } else if request == "2" {
//         let map = match parseDB::read_user_files(totalhash) {
//             Ok(map) => map,
//             Err(err) => {
//                 let error_msg = err.to_string();
//                 return Ok(json!({ "error": error_msg }));
//             }
//         };

//         let map_value = serde_json::to_value(&map)?;
//         Ok(json!({ "response": map_value }))
//     } else {
//         let err = "Invalid request number";
//         println!("Error: {}", err);
//         Ok(json!({ "error": err }))
//     }
// }
pub async fn check_requests(data: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let request = data["request"].as_str().ok_or("Missing request number")?;
    let totalhash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

    if request == "1" {
        let response = "Downloaded files".to_string();

        let filename = data["Filename"].as_str().ok_or("Missing request number")?;
        let IP = data["IP"].as_str().ok_or("Missing total hash")?;

        let cid = match get_cid(filename, totalhash) {
            Ok(Some(cid)) => cid,
            Ok(None) => return Err("CID not found".into()),
            Err(err) => return Err(err.into()),
        };
        println!("cid: {:?}", cid);

        let output_path = format!("UserFolders/{}/{}", totalhash, filename);
        // download_from_ipfs(cid, &output_path);

        let output = Command::new("python3")
        .arg("serverSendFile.py")
        .arg("sendfile")
        .arg(totalhash)
        .arg(filename)
        .arg(IP)
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
    
        Ok(response)
    } else if request == "2" {
        let map = match parseDB::read_user_files(totalhash) {
            Ok(map) => map,
            Err(err) => {
                let error_msg = err.to_string();
                return Ok(error_msg);
            }
        };

        let map_str = serde_json::to_string(&map)?;
        Ok(map_str)
    } else {
        let err = "Invalid request number".to_string();
        println!("Error: {}", err);
        Ok(err)
    }
}


fn get_cid(filename: &str, total_hash: &str) -> Result<Option<String>> {
    let conn = Connection::open("credits.db")?;
    let sql = format!("SELECT cid FROM {} WHERE filename = ?", total_hash);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![filename])?;

    if let Some(row) = rows.next()? {
        let cid = row.get(0)?;
        Ok(Some(cid))
    } else {
        Ok(None)
    }
}

fn download_from_ipfs(cid: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ipfs")
        .arg("get")
        .arg(cid)
        .arg("-o")
        .arg(output_path)
        .output();

    if output.as_ref().map_or(false, |o| o.status.success()) {
        let stderr = String::from_utf8(output.as_ref().unwrap().stderr.clone())?;
        return Err(stderr.into());
    }

    Ok(())
}