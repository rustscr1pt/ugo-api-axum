use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::SingleLogObject;

pub fn browse_logs_paginated_sql(page_number : u32, rows_per_page : u32, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<SingleLogObject>, Error> {
    match pool.query_map(format_the_query(page_number, rows_per_page),
        |(id, contents, date_time)| {
            SingleLogObject {
                id,
                contents,
                date_time,
            }
        }
    ) {
        Ok(value) => {
            return Ok(value)
        }
        Err(err) => {return Err(err)}
    }
}

fn format_the_query(page_number : u32, rows_per_page : u32) -> String {
    return format!("SELECT id, contents, date_time FROM `ugo_logs` ORDER BY id DESC LIMIT {} OFFSET {}", rows_per_page, rows_per_page * page_number)
}