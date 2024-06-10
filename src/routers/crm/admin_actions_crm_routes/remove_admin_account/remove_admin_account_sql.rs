use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::generic_replies::generic_log_writer::generic_log_writer;

pub fn remove_admin_account_sql(id : u32, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<(), Error>
{
    let iterable: [u32; 1] = [id];
    match pool.exec_batch(r#"DELETE FROM ugo_admin_accounts WHERE id = :id"#,
                          iterable.iter().map(|id| params! {
            "id" => id
        })
    )
    {
        Ok(_) => {
            match generic_log_writer(format!("Удален аккаунт администратора под номером : {}", id), pool) {
                Ok(_) => {
                    return Ok(())
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Err(err) => {return Err(err)}
    }
}