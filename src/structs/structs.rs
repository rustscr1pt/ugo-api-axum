use std::fmt::Debug;
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
    pub time_remaining : u16
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedObject {
    pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub date_time_added : String,
    pub notes : Vec<NoteObjectNotation>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteObjectNotation {
    pub id : u16,
    pub text_info : String,
    pub date_time : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicPartGetAll { // Get first part without json on get all orders request
pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub date_time_added : String
}

#[derive(Debug)]
pub struct RowsGetter {
    pub count : u16
}

#[derive(Debug, Serialize)]
pub struct EmptyStruct {
    pub null : bool
}

#[derive(Debug, Serialize)]
pub struct ReplyWithStruct<T>
    where T : Debug + Serialize
{
    pub is_succeed : bool,
    pub message : String,
    pub reply : Vec<T>
}