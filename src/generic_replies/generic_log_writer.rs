use std::fmt::Display;
use mysql::{params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;


// Write logs to the mySQL db
pub fn generic_log_writer<T : Display>(log : T, pool : &mut MutexGuard<PooledConn>) -> Result<(), mysql::Error> {
    let iterable : [T ; 1] = [log];
    match pool.exec_batch(r"INSERT INTO ugo_logs VALUES (0, :contents, NOW())",
                          iterable
                              .iter()
                              .map(|value| params!
                              {
                                  "contents" => value.to_string()
                              })
    ) {
        Ok(_) => {
            return Ok(())
        }
        Err(err) => {
            return Err(err)
        }
    }
}