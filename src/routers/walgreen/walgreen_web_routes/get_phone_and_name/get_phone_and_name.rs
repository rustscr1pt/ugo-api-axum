use std::sync::{Arc};
use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::routers::walgreen::walgreen_web_routes::get_phone_and_name::get_phone_and_name_sql::get_phone_and_name_sql;
use crate::structs::extension_structs::GetOrderTelegramWebExtension;
use crate::structs::structs::WriteDataBody;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;
use crate::walgreen_bot_server::new_order_preset::get_last_record_sql::{get_last_record_sql};

pub async fn get_phone_and_name(main_actor : Extension<GetOrderTelegramWebExtension>, Json(body) : Json<WriteDataBody>) -> impl IntoResponse {
    let cloned_telegram_bot = Arc::clone(&main_actor.telegram_bot);

    let mut unlocked = main_actor.arc_sql.lock().await;
    match get_phone_and_name_sql(&mut unlocked, [body]) {
        Ok(_) => {
            get_last_record_sql(&mut unlocked, BaseSelector::Wallgreen, cloned_telegram_bot).await;
            reply_with_message(true, "Ваш запрос был отправлен! Мы свяжемся с вами как можно скорее.")
        }
        Err(err) => {
            reply_with_message(false, err)
        }
    }
}