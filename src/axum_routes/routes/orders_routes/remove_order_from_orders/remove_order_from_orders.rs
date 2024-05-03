use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_message;
use crate::axum_routes::routes::orders_routes::remove_order_from_orders::remove_order_from_orders_sql::remove_order_from_orders_sql;
use crate::axum_routes::routes::orders_routes::remove_order_from_orders::remove_order_from_orders_structs::RemoveObjectByID;

pub async fn remove_order_from_orders(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<RemoveObjectByID>) -> impl IntoResponse {
    match body.id.parse::<u16>() {
        Ok(id) => {
            let mut unlocked = pool.lock().await;
            match remove_order_from_orders_sql(&mut unlocked, id) {
                Ok(_) => {
                    reply_with_message(true, "Order has been removed.")
                }
                Err(e) => {
                    reply_with_message(false, e)
                }
            }
        }
        Err(e) => {reply_with_message(false, e)}
    }
}