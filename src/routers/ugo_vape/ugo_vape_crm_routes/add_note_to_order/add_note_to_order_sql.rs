use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::generic_replies::generic_log_writer::generic_log_writer;
use crate::structs::structs::{InsertStructIdNote, NoteObjectNotation};

fn format_notes_parsing(id : u32) -> String {
    return format!("SELECT id, text_info, date_time FROM order_notes WHERE related_id = {}", id)
}

pub fn add_note_to_order_sql(unlocked : &mut MutexGuard<PooledConn>, id : u32, note : String) -> mysql::Result<Vec<NoteObjectNotation>, Error> {
    let mut to_insert : Vec<InsertStructIdNote> = Vec::with_capacity(1);
    to_insert.push(InsertStructIdNote {
        id,
        note : note.clone(),
    });
    match unlocked.exec_batch(r"INSERT INTO order_notes VALUES (0, :related_id, :text_info, NOW())",
                              to_insert.iter().map(|value| params! {
            "related_id" => value.id,
            "text_info" => &value.note
        })
    ) {
        Ok(_) => {
            match generic_log_writer(format!("Добавлена пометка для заявки номер : {} -> '{}'", id, note),unlocked) {
                Ok(_) => {
                    match unlocked.query_map(format_notes_parsing(id),
                                             |(id, text_info, date_time)| {
                                                 NoteObjectNotation {
                                                     id,
                                                     text_info,
                                                     date_time
                                                 }
                                             })
                    {
                        Ok(value) => {
                            return Ok(value)
                        }
                        Err(e) => {return Err(e)}
                    }
                }
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Err(e) => {return Err(e)}
    }
}