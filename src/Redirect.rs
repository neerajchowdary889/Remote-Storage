// use std::process::{Command, Stdio};

// fn print_to_new_terminal(output: &str) {
//     if cfg!(target_os = "windows") {
//         Command::new("cmd")
//             .arg("/c")
//             .arg("start")
//             .arg("cmd")
//             .arg("/k")
//             .arg("echo")
//             .arg(output)
//             .spawn()
//             .expect("Failed to execute command");
//     } else if cfg!(target_os = "macos") {
//         Command::new("open")
//             .arg("-a")
//             .arg("Terminal")
//             .arg("-e")
//             .arg("echo")
//             .arg(output)
//             .spawn()
//             .expect("Failed to execute command");
//     } else {
//         // Linux or other Unix-like systems
//         Command::new("gnome-terminal")
//             .arg("--")
//             .arg("sh")
//             .arg("-c")
//             .arg("echo")
//             .arg(output)
//             .spawn()
//             .expect("Failed to execute command");
//     }
// }

// fn main() {
//     // Example usage
//     print_to_new_terminal("Hello, world!");
// }

use std::fs;
use std::path::PathBuf;
use std::env;
fn main() {
    // let mut user_folders_path = PathBuf::new();
    // user_folders_path.push("UserFolders");

    // let entries = fs::read_dir(user_folders_path).unwrap();
    // for entry in entries {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_file() {
    //         println!("{}", path.display());
    //     }
    //     let current_dir = env::current_dir().unwrap();
    //     println!("Current directory: {}", current_dir.display());
    // }
    let mut my_string = "cr.txt43779aafe3d07dcddefa257eb32b9752b2cd5193".to_string();
    let len = my_string.len();
    let hash_File = my_string.split_off(len - 40);
    let FTP_File_name = my_string;
    println!("First part: {}", FTP_File_name);
    println!("Second part: {}", hash_File);
    let mut file = format!("UserFolders/{}", &hash_File);
    print!("File: {}", file);
    let current_dir = env::current_dir().unwrap();
    let mut file_path = PathBuf::new();

    env::set_current_dir(&file).unwrap();
    file_path.push("UserFolders");
    file_path.push(&hash_File);
    // file_path.push(&file_name);
    // let mut file = OpenOptions::new().create(true).write(true).open(file_path).await.unwrap();
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());
}