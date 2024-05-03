use mysql::{Error, PooledConn};
use mysql::prelude::Queryable;
use tokio::sync::MutexGuard;
use crate::structs::structs::AdminAccountTemplate;

pub fn fetch_admins_data_sql(unlocked : &mut MutexGuard<PooledConn>) -> mysql::Result<Vec<AdminAccountTemplate>, Error> {
    match unlocked.query_map(format_the_query(),
                             |(id, user_login, user_password, created)| {
                                 AdminAccountTemplate {
                                     id,
                                     user_login,
                                     user_password,
                                     created
                                 }
                             }
    ) {
        Ok(result) => {
            return Ok(result)
        }
        Err(err) => { Err(err) }
    }
}

fn format_the_query() -> String {
    return String::from("SELECT id, user_login, user_password, created FROM ugo_admin_accounts")
}