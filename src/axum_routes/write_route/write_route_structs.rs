use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WriteDataBody { // A data which passed in by user from the site.
pub email : String,
    pub name : String,
    pub about_customer : String
}