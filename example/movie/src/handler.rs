use std::error::Error;
use crate::services::{get_users, login_success,logout};
// cargo add rpassword
pub fn handle_loging(username: &str) -> Result<(),Box<dyn Error>> {
    println!("username:{username}");
    if let Some(user) = get_users()
        .iter()
        .find(|u|u.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter the password:");
        // let mut pw = String::new();
        // if io::stdin().read_line(&mut pw).is_ok(){
        //     println!("Log in successfully");
        // }else{
        //     println!("Failed to read password");
        // }
        match rpassword::read_password(){
            Ok(password) => {
                if user.password == password {
                    // println!("Password: {password}");
                    login_success(&user.role)?;
                    println!("Log in successfully");
                }else{
                    println!("Incorrect password.");
                    println!("Your password:{}",user.password);
                }
            }
            Err(_) => {
                println!("Failed to read password.");
            }
        }
    }else{
        println!("User not found.");
    }
    Ok(())
}

pub fn handle_logout() {
    logout();
    println!("Logged out successfully.");
}