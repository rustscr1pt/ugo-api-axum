use std::num::ParseIntError;
use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::{MutexGuard, RwLockReadGuard};
use crate::structs::structs::{BasicPartGetAll, FormattedObject, NoteObjectNotation, Token};

// Extract u32 from string
pub fn extract_u32(value : String) -> Result<u32, ParseIntError> {
    match value.parse::<u32>() {
        Ok(value) => {return Ok(value)}
        Err(err) => {return Err(err)}
    }
}

// Collect notes for a user request with related id from the db. (ugo-vape)
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

// Collect notes for a user request with related id from the db. (walgreen)
pub fn collect_walgreen_notes(unlocked : &mut MutexGuard<PooledConn>, object : &BasicPartGetAll) -> mysql::Result<FormattedObject, Error> {
    match unlocked.query_map(format!("SELECT id, text_info, date_time FROM walgreen_order_notes WHERE related_id = {}", object.id),
        |(id, text_info, date_time)| {
            NoteObjectNotation {
                id,
                text_info,
                date_time
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

pub fn release_string_uuid() -> String { // Release a UUID string for placing in React
    return String::from(uuid::Uuid::new_v4())
}

// check if required token is in the pool and return true / false
pub fn token_check_before_action(readable_pool : RwLockReadGuard<Vec<Token>>, token : String) -> bool
{
    if readable_pool.iter().any(|object| object.token == token) {
        return true
    }
    return false
}