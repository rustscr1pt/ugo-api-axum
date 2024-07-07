use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::routers::ugo_vape::ugo_vape_crm_routes::change_status_by_id::change_status_by_id_sql::change_status_by_id_sql;
use crate::structs::structs::ChangeOrderStatusBody;
use crate::structs::tool_functions::extract_u32;

pub async fn change_status_by_id(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<ChangeOrderStatusBody>) -> impl IntoResponse {
    match extract_u32(body.order_id) {
        Ok(id) => {
            let mut unlocked = pool.lock().await;
            match change_status_by_id_sql(&mut unlocked, id, body.new_status) {
                Ok(_) => {
                    reply_with_message(true, "Status of order has been changed")
                }
                Err(e) => {
                    reply_with_message(false, e)
                }
            }
        }
        Err(e) => {
            reply_with_message(false, e)
        }
    }
}