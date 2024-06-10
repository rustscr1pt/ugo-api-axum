use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRequestFiltered {
    pub rows_per_page : String,
    pub page_number : String,
    pub filter_type : String,
    pub filter_query : String
}