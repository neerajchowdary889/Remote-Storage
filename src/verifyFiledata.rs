use rusqlite::{params, Connection};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

pub async fn CreateFile<E>(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    
  println!("{}",data);
  Ok(data)
}
