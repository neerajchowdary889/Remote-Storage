use std::error::Error;
use std::path::PathBuf;
use tokio::net::TcpListener;
// use tokio::prelude::*;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, ErrorKind};
use tokio::fs::OpenOptions;
use tokio::net::TcpStream;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5100").await?;
    println!("FTP server listening on port 21");

    loop {

        let (mut socket, _) = listener.accept().await?;
        // let socket_copy = Arc::new(socket);
        // let socket_copy_clone = socket_copy.clone();

        tokio::spawn(async move {
            // let mut reader = BufReader::new(socket_copy);
            // let mut writer = BufWriter::new(socket);
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            // Read the file name from the client
            let mut buffer = [0; 1024];
            let size = reader.read(&mut buffer).await.unwrap();
            let file_name = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
            println!("Received file name: {}", file_name);

            // Read the string from the client
            let size = reader.read(&mut buffer).await.unwrap();
            let string_data = String::from_utf8_lossy(&buffer[..size]).trim().to_string();

            // Create a new file with the received file name in the Downloads folder
            let mut file_path = PathBuf::new();
            file_path.push(std::env::var("HOME").unwrap());
            file_path.push("Documents");
            file_path.push("Remote_Storage");
            file_path.push("src");
            file_path.push("UserFolders");
            file_path.push(&string_data);
            file_path.push(&file_name);
            let mut file = OpenOptions::new().create(true).write(true).open(file_path).await.unwrap();

            // Read the file data from the client and write it to the file
            loop {
                let size = reader.read(&mut buffer).await.unwrap();
                if size == 0 {
                    break;
                }
                file.write_all(&buffer[..size]).await.unwrap();
            }

            // Send a response back to the client
            let response = "File received successfully";
            writer.write_all(response.as_bytes()).await.unwrap();
        });
    }
}