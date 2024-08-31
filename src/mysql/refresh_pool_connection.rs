use std::fs;
use std::sync::Arc;
use std::time::Duration;
use mysql::{Pool, PooledConn};
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::structs::constants::FILE_LOCATION;


// refresh pool with db connection every 1 minute
pub fn refresh_pool_connection(to_refresh : Arc<Mutex<PooledConn>>) -> () {
    tokio::spawn(async move {
        let mut timer : u8 = 60;
        loop {
            if timer == 0 {
                let pool = Pool::new(fs::read_to_string(FILE_LOCATION()).unwrap().trim()).expect("Couldn't connect to a base").get_conn().unwrap();
                let mut unlocked = to_refresh.lock().await;
                *unlocked = pool;
                drop(unlocked);
                println!("Connection with MySQL pool is refreshed");
                timer = 60;
            }
            else {
                sleep(Duration::from_secs(1)).await;
                timer -= 1;
                println!("{} seconds estimated till MySQL pool is refreshed.", timer);
            }
        }
    });
}