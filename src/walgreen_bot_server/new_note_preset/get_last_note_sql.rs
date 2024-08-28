use std::fmt::Display;
use std::sync::Arc;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use rustelebot::types::BotInstance;
use tokio::sync::{Mutex, MutexGuard};
use crate::structs::structs::NoteObjectNotationFull;
use crate::walgreen_bot_server::bot_send_message_async::bot_send_message_async;
use crate::walgreen_bot_server::new_note_preset::format_last_note_query::format_note_sql_query;
use crate::walgreen_bot_server::new_note_preset::format_new_note_message::format_new_note_message;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;

pub async fn get_last_note_sql(connection : &mut MutexGuard<'_,PooledConn>,
                               base_type : BaseSelector,
                               bot : Arc<Mutex<BotInstance>>) -> () {
    match connection.query_map(format_note_sql_query(&base_type),
        |(id, related_id, text_info, date_time)| {
            NoteObjectNotationFull {
                id,
                related_id,
                text_info,
                date_time
            }
        }
    ) {
        Ok(value) => {
            println!("{:?}", value);
            match value.get(0) {
                None => {println!("Error when trying to get the last element from messages array")}
                Some(object) => {
                    println!("{:?}", object);
                    match bot_send_message_async(Arc::clone(&bot), format_new_note_message(object, &base_type)).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Error when trying to send a message : \n{}", err)
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Couldn't make a query to mysql base.\n{}", e)
        }
    }
}