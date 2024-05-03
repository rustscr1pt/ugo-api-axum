use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StealthAuthToken {
    pub token : String
}
