use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}

pub enum Role {
    Admin,
    User
}
// 为Role 实现 Display trait
impl Display for Role {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Administrator"),
            Role::User => write!(f, "User"),
        }
    }
}

// cargo add serde --features derive
// cargo add serde_json
// Default 定义解构体内容的默认值
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct  Movie {
    pub disc: usize,
    pub year: String,
    pub title: String,
    pub remark: Option<String>
}

impl PartialEq for Movie {
    fn eq(&self,other:&Self) -> bool {
        self.disc == other.disc
            && self.year == other.year
            && self.title == other.title
            && self.remark == other.remark
    }
}