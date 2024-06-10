use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::generic_replies::generic_log_writer::generic_log_writer;
use crate::structs::structs::NoteObjectNotation;

pub fn remove_note_walgreen_sql(note_id: u32, related_id : u32, pool: &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<NoteObjectNotation>, Error> {
    match pool.query_drop(formatter(note_id)) {
        Ok(_) => {
            match generic_log_writer(format!("Удалена пометка для заявки номер : {} => Walgreen", related_id), pool) {
                Ok(_) => {
                    match get_related_notes(related_id, pool) {
                        Ok(value) => {
                            return Ok(value)
                        }
                        Err(err) => {
                            return Err(err)
                        }
                    }
                }
                Err(err) => {return Err(err)}
            }
        }
        Err(err) => {return Err(err)}
    }
}

fn formatter(id : u32) -> String {
    return format!("DELETE FROM walgreen_order_notes WHERE id = {}", id)
}

fn get_related_notes(related_id : u32, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<NoteObjectNotation>, Error> {
    match pool.query_map(format!("SELECT id, text_info, date_time FROM walgreen_order_notes WHERE related_id = {}", related_id),
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