use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct Message { // An easy answer to show a result of some action
    pub is_succeed: bool,
    pub message : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WriteToBaseNewCustomer { // Represents the struct which is written inside mySQL about the customer
pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
}