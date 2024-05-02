use std::num::ParseIntError;
use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::{BasicPartGetAll, EmptyStruct, FormattedObject, NoteObjectNotation};

pub fn extract_u16(value : String) -> Result<u16, ParseIntError> {
    match value.parse::<u16>() {
        Ok(value) => {return Ok(value)}
        Err(err) => {return Err(err)}
    }
}

pub fn collect_group_notes(unlocked : &mut MutexGuard<PooledConn>, object : &BasicPartGetAll) -> mysql::Result<FormattedObject, Error> {
    match unlocked.query_map(format!("SELECT id, text_info, date_time FROM order_notes WHERE related_id = {}", object.id),
                             |(id, text_info, date_time)| {
                                 NoteObjectNotation {
                                     id,
                                     text_info,
                                     date_time,
                                 }
                             }
    ) {
        Ok(value) => {
            return Ok(FormattedObject {
                id: object.id,
                request_status: object.request_status.clone(),
                customer_name: object.customer_name.clone(),
                customer_email: object.customer_email.clone(),
                customer_self_description: object.customer_self_description.clone(),
                date_time_added: object.date_time_added.clone(),
                notes: value,
            })
        }
        Err(e) => {
            return Err(e)
        }
    }
}

pub fn release_empty_vec() -> Vec<EmptyStruct> {
    return vec![]
}

pub fn release_string_uuid() -> String { // Release a UUID string for placing in React
    return String::from(uuid::Uuid::new_v4())
}