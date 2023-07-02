use std::process::Command;

fn main() {
    let command = if cfg!(target_os = "windows") {
        "start cmd.exe"
    } else if cfg!(target_os = "macos") {
        "open -a Terminal"
    } else {
        "x-terminal-emulator"
    };

    Command::new(command)
        .spawn()
        .expect("Failed to open new terminal window");
}