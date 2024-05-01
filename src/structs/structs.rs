#[derive(Debug)]
pub struct AdminsData { // Represents the admin which is added in the admins stack of mySQL
    pub id : u32,
    pub user_login : String,
    pub user_password : String
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token : String,
    pub time_remaining : u32
}