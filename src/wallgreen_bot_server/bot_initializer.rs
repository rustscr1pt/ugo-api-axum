use std::fs;
use std::sync::Arc;
use rustelebot::types::BotInstance;
use tokio::sync::Mutex;
use crate::structs::constants::BOT_TOKEN_LOCATION;

pub async fn bot_initializer() -> Arc<Mutex<BotInstance>> {
    return Arc::new(Mutex::new(rustelebot::create_instance(fs::read_to_string(BOT_TOKEN_LOCATION).unwrap().trim(), "-1002156530519")))
}