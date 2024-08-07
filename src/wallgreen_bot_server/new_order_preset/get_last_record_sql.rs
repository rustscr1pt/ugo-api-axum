use std::cmp::PartialEq;
use std::sync::Arc;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use rustelebot::types::{BotInstance};
use tokio::sync::{Mutex, MutexGuard};
use crate::structs::structs::BasicPartGetAll;
use crate::wallgreen_bot_server::new_order_preset::format_new_order_message::format_new_order_message;
use crate::wallgreen_bot_server::new_order_preset::get_last_record_sql::BaseSelector::Wallgreen;
#[derive(PartialEq)]
pub enum BaseSelector {
    Wallgreen,
    Ugo
}

fn format_walgreen_sql(selector : &BaseSelector) -> String {
    if selector == &Wallgreen {
        return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `walgreen_customers_request` WHERE id=(SELECT MAX(id) FROM `walgreen_customers_request`)")
    }
    else {
        return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `ugo_customers_request` WHERE id=(SELECT MAX(id) FROM `ugo_customers_request`)")
    }
}

pub async fn get_last_record_sql(connection : &mut MutexGuard<'_, PooledConn>, base_type : BaseSelector, bot : Arc<Mutex<BotInstance>>) {
    match connection.query_map(format_walgreen_sql(&base_type),
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
                    let mut unlocked_bot = bot.lock().await;
                    match rustelebot::send_message_async(&*unlocked_bot, format_new_order_message(object, base_type).as_str(), None).await {
                        Ok(_) => {}
                        Err(err) => {println!("Error when trying to write a message\n{}", err)}
                    }
                }
            }
        },
        Err(e) => {
            println!("Couldn't make a query to mysql base.\n{}", e)
        }
    }
}

