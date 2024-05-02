use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChangeOrderStatusBody {
    pub order_id : String,
    pub new_status : String
}

pub struct InsertStruct {
    pub id : u16,
    pub new_status : String
}