use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StealthAuthToken {
    pub token : String
}