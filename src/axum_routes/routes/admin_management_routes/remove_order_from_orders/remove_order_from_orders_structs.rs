use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RemoveObjectByID {
    pub id : String
}