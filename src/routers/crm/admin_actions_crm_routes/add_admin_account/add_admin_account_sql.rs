use mysql::{Error, params, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::generic_replies::generic_log_writer::generic_log_writer;
use crate::structs::structs::InsertToTable;

pub fn add_admins_account_sql(pool : &mut MutexGuard<PooledConn>, login : String, password : String) -> mysql::Result<(), Error> {
    let sample : [InsertToTable; 1] = [InsertToTable { user_login: login.clone(), user_password: password.clone() }];
    match pool.exec_batch(r#"INSERT INTO ugo_admin_accounts VALUES (0, :user_login, :user_password, NOW())"#,
                          sample.iter().map(|object| params! {
            "user_login" => &object.user_login,
            "user_password" => &object.user_password
        })
    ) {
        Ok(_) => {
            match generic_log_writer(format!("Добавлен аккаунт администратора с данными : {} - {}", login, password), pool) {
                Ok(_) => {return Ok(())}
                Err(err) => {return Err(err)}
            }
        }
        Err(err) => {return Err(err)}
    }
}