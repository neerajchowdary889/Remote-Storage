use rusqlite::{Connection, Result};

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    totalhash: String,
}

fn main() -> Result<()> {
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