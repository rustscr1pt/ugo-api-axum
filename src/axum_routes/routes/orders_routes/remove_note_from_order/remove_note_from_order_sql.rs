use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::NoteObjectNotation;

pub fn remove_note_from_order_sql(note_id: u16, related_id : u16, pool: &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<NoteObjectNotation>, Error> {
    match pool.query_drop(formatter(note_id)) {
        Ok(_) => {
            match get_related_notes(related_id, pool) {
                Ok(value) => {
                    return Ok(value)
                }
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Err(e) => {
            return Err(e)
        }
    }
}

fn formatter(id : u16) -> String {
    return format!("DELETE FROM order_notes WHERE id = {}", id)
}

fn get_related_notes(related_id : u16, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<NoteObjectNotation>, Error> {
    match pool.query_map(format!("SELECT id, text_info, date_time FROM order_notes WHERE related_id = {}", related_id),
                         |(id, text_info, date_time)| {
                             NoteObjectNotation {
                                 id,
                                 text_info,
                                 date_time,
                             }
                         }
    ) {
        Ok(vector) => {return Ok(vector)}
        Err(err) => {return Err(err)}
    }
}