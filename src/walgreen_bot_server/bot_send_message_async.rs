use std::fmt::Display;
use std::sync::Arc;
use rustelebot::types::BotInstance;
use tokio::sync::{Mutex};

pub async fn bot_send_message_async(bot : Arc<Mutex<BotInstance>>, formatted_text : String) -> Result<(), impl Display> {
    let unlocked_bot = bot.lock().await;
    match rustelebot::send_message_async(&unlocked_bot, formatted_text.as_str(), None).await {
        Ok(_) => {return Ok(())}
        Err(e) => {return Err(e)}
    }
}