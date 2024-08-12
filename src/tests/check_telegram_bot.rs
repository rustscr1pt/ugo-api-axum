use crate::structs::constants::BOT_TOKEN_LOCATION;

#[test]
fn check_telegram_bot() -> Result<(), String> {
    match rustelebot::create_instance(BOT_TOKEN_LOCATION.trim(), ) {  }
}