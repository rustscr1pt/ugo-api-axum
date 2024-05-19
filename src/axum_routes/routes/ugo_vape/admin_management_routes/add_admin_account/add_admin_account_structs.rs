use serde::{Deserialize, Serialize};

pub struct InsertToTable {
    pub user_login: String,
    pub user_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAdminRequest {
    pub login : String,
    pub password : String,
    pub token : String
}