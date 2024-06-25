use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AdminsData { // Represents the admin which is added in the admins stack of mySQL
    pub id : u32,
    pub user_login : String,
    pub user_password : String
}

#[derive(Debug, Clone)]
pub struct Token { // Represents one active login token in storage. With timing before disconnection
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
    pub id : u32,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedObject {
    pub id : u32,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub date_time_added : String,
    pub notes : Vec<NoteObjectNotation>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteObjectNotation {
    pub id : u32,
    pub text_info : String,
    pub date_time : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicPartGetAll { // Get first part without json on get all orders request
    pub id : u32,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub date_time_added : String
}

#[derive(Debug)]
pub struct RowsGetter {
    pub count : u32
}

#[derive(Debug, Serialize)]
pub struct EmptyStruct {
    pub null : bool
}

#[derive(Debug, Serialize)]
pub struct ReplyWithStruct<T> // Represents a generic reply for request with data as answer.
    where T : Debug + Serialize
{
    pub is_succeed : bool,
    pub message : String,
    pub reply : Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminAccountTemplate {
    pub id : u32,
    pub user_login : String,
    pub user_password : String,
    pub created : String
}

#[derive(Debug, Deserialize)]
pub struct GetPhoneAndName { // walgreen_web_routes => get_phone_and_name
    pub customer_name : String,
    pub customer_phone_email : String,
    pub customer_comment : String
}

#[derive(Debug, Deserialize)]
pub struct AddNoteToOrder { // ugo_vape
    pub order_id : String,
    pub note_to_add : String
}

pub struct InsertStruct {
    pub id : u32,
    pub note : String,
    pub new_status: String,
}

pub struct InsertStructIdNote {
    pub id : u32,
    pub note : String
}

pub struct InsertStructNewStatus {
    pub id : u32,
    pub new_status : String
}

#[derive(Debug, Deserialize)]
pub struct ChangeOrderStatusBody {
    pub order_id : String,
    pub new_status : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRequestFiltered {
    pub rows_per_page : String,
    pub page_number : String,
    pub filter_type : String,
    pub filter_query : String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRequest {
    pub rows_per_page : String,
    pub page_number : String
}

#[derive(Debug, Deserialize)]
pub struct RemoveObjectByID {
    pub id : String
}

#[derive(Debug, Deserialize)]
pub struct WriteDataBody { // A data which passed in by user from the site.
    pub email : String,
    pub name : String,
    pub about_customer : String
}

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