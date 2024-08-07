use std::fs;
use rustelebot::types::{BotInstance};
use crate::structs::constants::BOT_TOKEN_LOCATION;

pub fn bot_initializer() -> BotInstance {
    return rustelebot::create_instance(fs::read_to_string(BOT_TOKEN_LOCATION).unwrap().trim(), "-1002156530519");
}