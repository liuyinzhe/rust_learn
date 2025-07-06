use crate::models::User;
use crate::models::Role;

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