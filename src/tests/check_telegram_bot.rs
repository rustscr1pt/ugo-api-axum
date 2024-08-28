use crate::structs::constants::{BOT_CHAT_ID, BOT_TOKEN_LOCATION};

#[test]
fn check_telegram_bot() -> Result<(), String> {
    let bot = rustelebot::create_instance(BOT_TOKEN_LOCATION, BOT_CHAT_ID);
    match rustelebot::send_message(&bot, "Test", None) {
        Ok(()) => return Ok(()),
        Err(e) => return Err(e.to_string())
    }
}