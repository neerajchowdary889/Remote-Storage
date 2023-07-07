use rusqlite::{params,Connection, Result};
use std::collections::HashMap;
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    totalhash: String,
}

struct UserFiles{
    filename: String,
    cid: String
}


fn main() -> Result<()> {

    let totalhash = "43779aafe3d07dcddefa257eb32b9752b2cd5193".to_string();

    println!("Choose an option:");
    println!("1. See Users");
    println!("2. See Users files");

    let str = read_user_input();
    if str == 1 {
        readUserTable()?;
    }
    else{
        let map = read_user_files(&totalhash);
        println!("{:?}", map);
    
    }
    let filename = "ComputerNetworks-Abstract.pdf".to_string(); // Replace with the desired filename

    match search_cid_by_filename(filename.clone(), &totalhash) {
        Ok(Some(cid)) => {
            println!("CID for file '{}' found: {}", filename, cid);
        }
        Ok(None) => {
            println!("CID not found for file '{}'", filename);
        }
        Err(err) => {
            eprintln!("Error searching for CID: {}", err);
        }
    }

    Ok(())
}


fn readUserTable() -> Result<()>{
    let conn = Connection::open("credits.db")?;

    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            totalhash: row.get(3)?,
        })
    })?;


    for user in users {
        let user = user.unwrap();
        println!("id = {}, username = {}, password = {}, totalhash = {}", user.id, user.username, user.password, user.totalhash);

    }

    Ok(())
}

pub fn read_user_files(totalhash: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let conn = Connection::open("credits.db")?;

    let mut map: HashMap<String, String> = HashMap::new();

    let mut stmt = conn.prepare(&format!("SELECT filename, cid FROM '{}'", totalhash))?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    // Process the retrieved data
    for row in rows {
        let (filename, cid) = row?;
        map.insert(filename, cid);
    }

    Ok(map)
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

struct FileEntry {
    filename: String,
    cid: String,
}

fn search_cid_by_filename(
    filename: String,
    totalhash: &str,
) -> Result<Option<String>> {
    let conn = Connection::open("credits.db")?;
    let mut stmt = conn.prepare(&format!("SELECT cid FROM '{}' WHERE filename = ?", totalhash))?;
    let mut rows = stmt.query(params![filename])?;

    if let Some(row) = rows.next()? {
        let cid: String = row.get(0)?;
        Ok(Some(cid))
    } else {
        Ok(None)
    }
}
