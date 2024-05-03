use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;

pub fn remove_admin_account_sql(id : u16, pool : &mut MutexGuard<PooledConn>) -> mysql::Result<(), Error>
{
    let iterable: [u16; 1] = [id];
    match pool.exec_batch(r#"DELETE FROM ugo_admin_accounts WHERE id = :id"#,
                          iterable.iter().map(|id| params! {
            "id" => id
        })
    )
    {
        Ok(_) => {return Ok(())}
        Err(err) => {return Err(err)}
    }
}