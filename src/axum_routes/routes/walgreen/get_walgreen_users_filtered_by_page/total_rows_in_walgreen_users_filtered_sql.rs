use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use crate::structs::structs::RowsGetter;

pub fn total_rows_in_walgreen_users_filtered_sql(pool : &mut PooledConn, filter_type : &String, filter_query : &String) -> mysql::Result<u32, Error> {
    match pool.query_map(get_formatted(&filter_type, &filter_query),
        |counter| {
            RowsGetter {
                count : counter
            }
        }
    ) {
        Ok(value) => {
            if value.len() > 0 {
                return Ok(value[0].count)
            }
            else {
                return Ok(0)
            }
        }
        Err(err) => {return Err(err)}
    }
}

fn get_formatted(filter_type : &String, filter_query : &String) -> String {
    format!("SELECT COUNT(*) FROM `walgreen_customers_request` WHERE {} LIKE '%{}%'", filter_type, filter_query)
}