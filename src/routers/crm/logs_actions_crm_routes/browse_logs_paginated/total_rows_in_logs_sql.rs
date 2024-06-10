use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::RowsGetter;

pub fn total_rows_in_logs_sql(pool : &mut MutexGuard<PooledConn>) -> mysql::Result<u32, Error> {
    match pool.query_map(get_formatted(),
                         |count| {
                             RowsGetter {
                                 count
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
        Err(err) => {
            Err(err)
        }
    }
}

fn get_formatted() -> String {
    return "SELECT COUNT(*) FROM `ugo_logs`".to_string()
}