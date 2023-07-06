// use rusqlite::{params, Connection};
// use std::error::Error;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use serde_json::Value;
// mod parseDB;

// pub async fn check_requests<E>(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
//     let conn = Connection::open("credits.db")?;
    
//     let request = data["request"].as_str().ok_or("Missing request number")?;
//     let total_hash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

//    if request == "1" {
//         let response = true;
//         println!("Download files");
//         Ok(response)
//     } else if request == "2" {
//         let response = false;

//         parseDB::readUserFiles(total_hash.to_string());

//         Ok(response)
//     }
//     else{
//         let err = Error;
//         println!("Error");
//         OK(err)
//     }
// }

use std::collections::HashMap;
use serde_json::{json, Value};


pub async fn check_requests(data: serde_json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    
    let request = data["request"].as_str().ok_or("Missing request number")?;
    let totalhash = data["TotalHash"].as_str().ok_or("Missing total hash")?;

    if request == "1" {
        let response = true;
        println!("Download files");
        Ok(json!({ "response": response }))
    } 
    else if request == "2" {
        // let mut key_value_pairs: HashMap<&str, &str> = HashMap::new();

        let map = parseDB::readUserFiles(totalhash);
        // Add more key-value pairs as needed

        Ok(json!({"response": map}))
    } 
    else {
        let err = "Invalid request number";
        println!("Error: {}", err);
        Ok(json!({ "error": err }))
    }
}