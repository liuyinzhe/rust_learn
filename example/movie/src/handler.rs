use std::error::Error;
use crate::models::{Role,Movie};
use crate::services::{self, get_logged_in_role, get_users, login_success, logout};
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
        match rpassword::read_password(){ // Option match 
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

pub fn handle_list() -> Result<(),Box<dyn Error>>{
    // 检查用户登录
    match get_logged_in_role()? {
        Some(_) => {
            let movies = services::read_from_json()?;
            services::list_movies(&movies);
            //println!("{movies:#?}");
        }
        None => {
            println!("you need to log in to view the movies");
        }
        
    }
    Ok(())

}

pub fn handle_add(disc: usize, year: &str, title: &str, remark: &Option<String>) -> Result<(),Box<dyn Error>> {
    // 限制登陆用户角色
    match get_logged_in_role()?{
        Some(Role::Admin) => {
            let mut movies = services::read_from_json()?;
            let new_moive = Movie{
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_moive);
            services::write_to_json(&movies)?;
            println!("Movie added.");
        }
        _ => {
            println!("You new to log in as Admin to add a movie");
        }
    }

    Ok(())
}

pub fn handle_delete(disc: &usize,index: &usize) -> Result<(),Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()? {
        let movies = services::read_from_json()?;
        if let Some(movie) = movies
            .iter()
            .filter(|m|m.disc == *disc)
            .enumerate()
            .find(|(i,_)| i == index)
            .map(|(_,m)|m.clone())
        {
            let left_movies = movies
            .into_iter()
            .filter(|m|*m != movie)
            .collect::<Vec<Movie>>();
            
            services::write_to_json(&left_movies)?;
            println!("Movie deleted.")
        }
    }else{
        println!("You need to log in as admin to delete a movie.");
    }
    Ok(())
}