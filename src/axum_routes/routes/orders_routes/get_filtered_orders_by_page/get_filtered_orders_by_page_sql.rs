use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::{BasicPartGetAll, FormattedObject};
use crate::structs::tool_functions::collect_group_notes;

fn format_the_query(page_number : u32, rows_per_page : u32, filter_type : &String, filter_query : &String) -> String {
    if filter_type == "date_time_added" {
        return format!(r#"SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM ugo_customers_request WHERE DATE(`date_time_added`) = DATE("{}") ORDER BY id DESC LIMIT {} OFFSET {}"#, filter_query, rows_per_page, rows_per_page * page_number)
    }
    return format!("SELECT id, request_status, customer_name, customer_email, customer_self_description, date_time_added FROM `ugo_customers_request` WHERE {} LIKE '%{}%' ORDER BY id DESC LIMIT {} OFFSET {}", filter_type, filter_query, rows_per_page, rows_per_page * page_number)
}
pub fn get_filtered_orders_by_page_sql(page_number : u32, rows_per_page : u32, filter_type : &String, filter_query : &String, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<FormattedObject>, Error> {
    match pool.query_map(format_the_query(page_number, rows_per_page, filter_type, filter_query),
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
        Err(err) => {
            return Err(err)
        }
    }
}