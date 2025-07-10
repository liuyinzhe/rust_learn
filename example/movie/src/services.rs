use core::option::Option::None;
use core::option::Option::Some;
use core::result::Result::Ok;
//use core::result::Result;
use std::error::Error;
use crate::models::Movie;
use crate::models::User;
use crate::models::Role;
use std::fs;
use std::io;

pub fn get_users() -> Vec<User>{
    vec![
        User{
            username:"Admin".to_string(),
            password:"admin".to_string(),
            role:Role::Admin,
        },
        User{
            username:"Dave".to_string(),
            password:"Mustaine".to_string(),
            role:Role::User,
        },
        User {
            username:"Nick".to_string(),
            password:"Carter".to_string(),
            role:Role::User,
        }
    ]

}

pub fn login_success(role: &Role) -> Result<(),Box<dyn Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_role() -> Result<Option<Role>,Box<dyn Error>> {
    let role = fs::read_to_string(".session")?; 
    match role.as_str() {
        "Administrator" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None)
    }
}

pub  fn logout() {
    fs::remove_file(".session").unwrap_or_else(|_|{
        println!("No user is logged in.");
    });
}

pub fn read_from_json() -> Result<Vec<Movie>,Box<dyn Error>> {
    let file = fs::File::open("Movies.json")?; // Result? -> <T>
    let reader = io::BufReader::new(file);
    let movies: Vec<Movie> = serde_json::from_reader(reader)?;
    Ok(movies)
}