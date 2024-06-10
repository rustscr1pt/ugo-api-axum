use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleLogObject {
    pub id : u16,
    pub contents : String,
    pub date_time : String
}