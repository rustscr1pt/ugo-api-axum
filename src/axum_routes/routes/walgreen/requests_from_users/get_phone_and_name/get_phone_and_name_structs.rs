use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetPhoneAndName {
    pub customer_name : String,
    pub customer_phone : String
}