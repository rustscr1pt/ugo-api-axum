use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::routers::walgreen::walgreen_crm_routes::remove_order_walgreen::remove_order_walgreen_sql::remove_order_walgreen_sql;
use crate::structs::structs::RemoveObjectByID;
use crate::structs::tool_functions::extract_u32;

pub async fn remove_order_walgreen(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<RemoveObjectByID>) -> impl IntoResponse {
    match extract_u32(body.id) {
        Ok(id) => {
            let mut unlocked = pool.lock().await;
            match remove_order_walgreen_sql(&mut unlocked, id) {
                Ok(_) => {reply_with_message(true, "Order has been removed")}
                Err(err) => {reply_with_message(false, err)}
            }
        }
        Err(err) => {reply_with_message(false, err)}
    }
}