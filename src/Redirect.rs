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

fn main() {
    let mut user_folders_path = PathBuf::new();
    user_folders_path.push("UserFolders");

    let entries = fs::read_dir(user_folders_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.display());
        }
    }
}