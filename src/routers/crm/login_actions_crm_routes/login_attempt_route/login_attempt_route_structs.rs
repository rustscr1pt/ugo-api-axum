use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequestData { // A body which arrives when the login request is made.
pub login : String,
    pub password : String
}