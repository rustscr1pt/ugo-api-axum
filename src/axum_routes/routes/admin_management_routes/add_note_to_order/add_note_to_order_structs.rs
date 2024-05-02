use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddNoteToOrder {
    pub order_id : String,
    pub note_to_add : String
}

pub struct InsertStruct {
    pub id : u16,
    pub note : String
}