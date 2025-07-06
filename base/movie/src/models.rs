pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}

pub enum Role {
    Admin,
    User
}

