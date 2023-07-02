use std::io;
extern crate colored;
use colored::Colorize;
use sha1::{Digest, Sha1};


pub fn Login() -> (String, String, String){
    println!("{}","++  Login Portal ++".bold().green().italic());

    println!("Enter Username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Wrong input");
    let mut input = healthy_input(&username);

    if input == false{
        return Login()
    }

    println!("Enter Password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Wrong input");
    input = healthy_input(&password);

    if input == false{
        return Login()
    }

    let(Username,Password, TotalHash) = convert_to_sha(username.trim().to_string(), password.trim().to_string());
    // println!("{}\n{}\n{}",Username,Password,TotalHash);  // This is the final output
    return(Username,Password,TotalHash)
  
}

fn convert_to_sha(username: String, password: String) -> (String, String, String){

    // println!("{},{}", username, password);

    let mut hasher = Sha1::new();
    let mut hasher_password = Sha1::new();
    let mut TotalHash = Sha1::new();

    hasher.update(&username);
    let result = hasher.finalize();
    let result_str = format!("{:x}", result);

    hasher_password.update(&password);
    let result_password = hasher_password.finalize();
    let result_password_str = format!("{:x}", result_password);

    TotalHash.update(&(result_str.to_string() + &result_password_str));
    let TotalHash = TotalHash.finalize();
    let TotalHash = format!("{:x}", TotalHash);
    // println!("{}\n{}",result_str, result_password_str);

    // println!("{}","\n++ Account Created ++".bold().green().italic());
    return(result_str, result_password_str, TotalHash)
}


fn healthy_input(input: &String) -> bool{
    if input.trim().is_empty(){
        println!("{}", "Wrong input... TryAgain".red().bold());
        false
    }
    else{
        true
    }
}