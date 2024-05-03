use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::axum_routes::routes::admin_management_routes::add_admin_account::add_admin_account_structs::InsertToTable;

pub fn add_admins_account_sql(pool : &mut MutexGuard<PooledConn>, login : String, password : String) -> mysql::Result<(), Error> {
    let sample : [InsertToTable; 1] = [InsertToTable { user_login: login, user_password: password }];
    match pool.exec_batch(r#"INSERT INTO ugo_admin_accounts VALUES (0, :user_login, :user_password, NOW())"#,
                          sample.iter().map(|object| params! {
            "user_login" => &object.user_login,
            "user_password" => &object.user_password
        })
    ) {
        Ok(_) => {return Ok(())}
        Err(err) => {return Err(err)}
    }
}