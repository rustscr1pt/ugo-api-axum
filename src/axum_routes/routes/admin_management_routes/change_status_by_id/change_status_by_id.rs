use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_message;
use crate::axum_routes::routes::admin_management_routes::change_status_by_id::change_status_by_id_sql::change_status_by_id_sql;
use crate::axum_routes::routes::admin_management_routes::change_status_by_id::change_status_by_id_structs::ChangeOrderStatusBody;

pub async fn change_status_by_id(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<ChangeOrderStatusBody>) -> impl IntoResponse {
    match body.order_id.parse::<u16>() {
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