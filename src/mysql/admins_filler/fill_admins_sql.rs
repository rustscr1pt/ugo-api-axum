use std::sync::Arc;
use mysql::PooledConn;
use mysql::prelude::Queryable;
use tokio::sync::{Mutex, RwLock};
use crate::structs::structs::AdminsData;

fn format() -> String {
    return String::from("SELECT id, user_login, user_password FROM ugo_admin_accounts")
}

// Fill the admins logins & passwords from db at the first launch

pub async fn fill_admins_sql(pool : Arc<Mutex<PooledConn>>, to_fill : Arc<RwLock<Vec<AdminsData>>>) -> () {
    let mut unlocked = pool.lock().await;
    let retrieved_admins =
        unlocked.query_map(format(), |(id, user_login, user_password)| {
            AdminsData {
                id,
                user_login,
                user_password,
            }
        }).unwrap();
    drop(unlocked);
    let mut filler = to_fill.write().await;
    println!("{:#?}", retrieved_admins);
    *filler = retrieved_admins;
}