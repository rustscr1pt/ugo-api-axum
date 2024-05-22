use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GetPhoneAndName {
    pub customer_name : String,
    pub customer_phone_email : String,
    pub customer_comment : String
}

pub