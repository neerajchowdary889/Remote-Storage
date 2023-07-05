use rusqlite::{params, Connection};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

pub async fn CreateFile<E>(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    
    let Filename = data["Filename"].as_str().ok_or("Missing username")?;
    let TotalHash = data["TotalHash"].as_str().ok_or("Missing password")?;
    let Content = data["Content"].as_str().ok_or("Missing total hash")?;

    let mut file_path = PathBuf::new();
        
    file_path.push("UserFolders");
    file_path.push(&TotalHash);
    file_path.push(&Filename);
    let mut file = OpenOptions::new().create(true).write(true).open(file_path)?;
    file.write_all(Content.as_bytes())?;
Ok(().into())
  
}
