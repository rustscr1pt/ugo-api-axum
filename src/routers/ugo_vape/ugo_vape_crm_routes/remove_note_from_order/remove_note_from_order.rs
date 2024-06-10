use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::routers::ugo_vape::ugo_vape_crm_routes::remove_note_from_order::remove_note_from_order_sql::remove_note_from_order_sql;
use crate::routers::ugo_vape::ugo_vape_crm_routes::remove_note_from_order::remove_note_from_order_structs::RemoveNoteBody;
use crate::structs::tool_functions::extract_u32;

pub async fn remove_note_from_order(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<RemoveNoteBody>) -> impl IntoResponse {
    match extract_u32(body.note_id) {
        Ok(note_id) => {
            match extract_u32(body.related_id) {
                Ok(related_id) => {
                    let mut unlocked = pool.lock().await;
                    match remove_note_from_order_sql(note_id, related_id, &mut unlocked) {
                        Ok(value) => {
                            reply_with_serialized_struct(true, "Your note has been deleted", value)
                        }
                        Err(err) => {
                            reply_with_serialized_struct(false, err, Vec::new())
                        }
                    }
                }
                Err(e) => {
                    reply_with_serialized_struct(false, e, Vec::new())
                }
            }
        }
        Err(e) => {
            reply_with_serialized_struct(false, e, Vec::new())
        }
    }
}