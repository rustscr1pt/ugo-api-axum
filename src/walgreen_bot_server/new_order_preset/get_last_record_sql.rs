use std::fmt::Display;
use std::sync::Arc;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use rustelebot::types::{BotInstance};
use tokio::sync::{Mutex, MutexGuard};
use crate::structs::structs::BasicPartGetAll;
use crate::walgreen_bot_server::bot_send_message_async::bot_send_message_async;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;
use crate::walgreen_bot_server::new_order_preset::format_new_order_message::format_new_order_message;
use crate::walgreen_bot_server::new_order_preset::format_sql_query_walgreen_ugo::format_sql_query_walgreen_ugo;
pub async fn get_last_record_sql(connection : &mut MutexGuard<'_, PooledConn>, base_type : BaseSelector, bot : Arc<Mutex<BotInstance>>) -> () {
    match connection.query_map(format_sql_query_walgreen_ugo(&base_type),
                               |(id, request_status, customer_name, customer_email, customer_self_description, date_time_added)| {
                             BasicPartGetAll {
                                 id,
                                 request_status,
                                 customer_name,
                                 customer_email,
                                 customer_self_description,
                                 date_time_added,
                             }
                         }
    ) {
        Ok(value) => {
            match value.get(0) {
                None => {
                    println!("Error when trying to get the last element from messages array")
                }
                Some(object) => {
                    println!("{:?}", object);
                    match bot_send_message_async(Arc::clone(&bot), format_new_order_message(object, base_type)).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Error when trying to send a message : \n{}", err)
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("Couldn't make a query to mysql base.\n{}", e)
        }
    }
}

