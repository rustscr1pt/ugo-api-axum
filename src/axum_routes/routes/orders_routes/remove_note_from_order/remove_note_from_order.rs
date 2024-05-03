use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::axum_routes::routes::orders_routes::remove_note_from_order::remove_note_from_order_sql::remove_note_from_order_sql;
use crate::axum_routes::routes::orders_routes::remove_note_from_order::remove_note_from_order_structs::RemoveNoteBody;

pub async fn remove_note_from_order(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<RemoveNoteBody>) -> impl IntoResponse {
    match body.note_id.parse::<u16>() {
        Ok(note_id) => {
            match body.related_id.parse::<u16>() {
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