use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::{BasicPartGetAll, FormattedObject};
use crate::structs::tool_functions::collect_group_notes;

pub fn get_orders_by_page_sql(page_number : u32, rows_per_page : u32, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<FormattedObject>, Error> {
    match pool.query_map(format_the_query(page_number, rows_per_page),
                         |(id, request_status, customer_name, customer_email, customer_self_description, date_time_added)| {
                             BasicPartGetAll {
                                 id,
                                 request_status,
                                 customer_name,
                                 customer_email,
                                 customer_self_description,
                                 date_time_added
                             }
                         }
    ) {
        Ok(value) => {
            let mut cleaned : Vec<FormattedObject> = Vec::new();
            let reconstructed =
                value
                    .iter()
                    .map(|value| collect_group_notes(pool, value))
                    .collect::<Vec<Result<FormattedObject, Error>>>();
            for elements in reconstructed {
                match elements {
                    Ok(sample) => {cleaned.push(sample)}
                    Err(e) => {return Err(e)}
                }
            }
            return Ok(cleaned)
        }
        Err(err) => {return Err(err)}
    }
}

fn format_the_query(page_number : u32, rows_per_page : u32) -> String {
    return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `ugo_customers_request` ORDER BY id DESC LIMIT {} OFFSET {}", rows_per_page, rows_per_page * page_number)
}