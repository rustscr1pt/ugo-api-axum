use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRequest {
    pub rows_per_page : String,
    pub page_number : String
}