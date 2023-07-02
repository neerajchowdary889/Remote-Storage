pub fn User(TotalHash: String){
    println!("Choose an option:");
    println!("1. Upload File");
    println!("2. Download File");
    println!("3. View Files");
    let choice = read_user_input();

    // Call the appropriate function based on the user's choice
    match choice {
        1 => UploadFile().await?,
        2 => DownloadFile().await?,
        3 => view_files().await?,
        _ => println!("Invalid choice"),
    }
    ok()
}
fn read_user_input() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}