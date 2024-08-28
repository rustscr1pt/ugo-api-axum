use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::routers::walgreen::walgreen_crm_routes::add_note_walgreen::add_note_walgreen_sql::add_note_walgreen_sql;
use crate::structs::extension_structs::SQLAndTelegramWebExtension;
use crate::structs::structs::AddNoteToOrder;
use crate::structs::tool_functions::extract_u32;
use crate::walgreen_bot_server::new_note_preset::get_last_note_sql::get_last_note_sql;
use crate::walgreen_bot_server::new_order_preset::base_selector_enum::BaseSelector;

pub async fn add_note_walgreen(main_actor : Extension<SQLAndTelegramWebExtension>, Json(body) : Json<AddNoteToOrder>) -> impl IntoResponse {
    let cloned_telegram_bot = Arc::clone(&main_actor.telegram_bot);

    match extract_u32(body.order_id) {
        Ok(id) => {
            let mut unlocked = main_actor.arc_sql.lock().await;

            match add_note_walgreen_sql(&mut unlocked, id, body.note_to_add) {
                Ok(value) => {
                    get_last_note_sql(&mut unlocked, BaseSelector::Wallgreen, cloned_telegram_bot).await;
                    reply_with_serialized_struct(true, "Your note has been added", value)
                }
                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
            }
        }
        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
    }
}