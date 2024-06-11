use std::sync::Arc;
use std::time::Duration;
use mysql::PooledConn;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use crate::mysql::admins_filler::fill_admins_sql::fill_admins_sql;
use crate::structs::structs::AdminsData;

// Fill the vector with admins logins & passwords from the db and refresh it every 15 minutes

pub fn admins_filler(pool : Arc<Mutex<PooledConn>>, to_fill : Arc<RwLock<Vec<AdminsData>>>) -> () {
    tokio::spawn(async move {
        let mut countdown : u16 = 900;
        loop {
            if countdown == 0 {
                let cloned_pool = Arc::clone(&pool);
                let cloned_to_fill = Arc::clone(&to_fill);
                tokio::spawn(async move {
                    fill_admins_sql(cloned_pool, cloned_to_fill).await;
                    println!("Admins data has been refreshed");
                });
                countdown = 900;
            }
            else {
                sleep(Duration::from_secs(1)).await;
                countdown -= 1;
                println!("{}", format!("======> {} secs till admins would be filled again", countdown));
            }
        }
    });
}