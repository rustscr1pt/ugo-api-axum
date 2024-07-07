use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use crate::structs::structs::Token;

// Every 30 seconds reduce remaining time for active tokens. If token has <=0 seconds left => remove it from storage

pub fn token_worker(pool : Arc<RwLock<Vec<Token>>>) -> () {
    tokio::spawn(async move {
        let mut countdown = 30;
        loop {
            if countdown == 0 {
                let unlocked = pool.read().await;
                let cloned = unlocked.clone();
                drop(unlocked);
                let filtered = cloned.into_iter().filter(|value| value.time_remaining > 0).collect::<Vec<Token>>();
                let mut to_mutate = pool.write().await;
                *to_mutate = filtered;
                println!("Cleaned tokens vector : {:#?}", to_mutate);
                countdown = 30;
            }
            else {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let mut unlocked = pool.write().await;
                unlocked.iter_mut().for_each(|object| object.time_remaining -= 30);
                countdown -= 30;
                println!("Token activity time has been reduced : {:#?}", unlocked);
            }
        }
    });
}