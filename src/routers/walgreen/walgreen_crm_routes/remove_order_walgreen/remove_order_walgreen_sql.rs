use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;

pub fn remove_order_walgreen_sql(pool : &mut MutexGuard<PooledConn>, id : u32) -> mysql::Result<(), Error> {
    let iterable : [u32 ; 1] = [id];
    match pool.exec_batch(r"DELETE FROM walgreen_customers_request WHERE id = :id",
                          iterable.iter().map(|id| params! {
                              "id" => id
                          }
                          ))
    {
        Ok(_) => {
            match pool.exec_batch(r"DELETE FROM walgreen_order_notes WHERE related_id = :id",
                                  iterable.iter().map(|id| params! {
                    "id" => id
                })
            ) {
                Ok(_) => {
                    return Ok(())
                }
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Err(e) => {
            return Err(e)
        }
    }
}