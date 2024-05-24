use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::axum_routes::routes::ugo_vape::orders_routes::get_orders_by_page::get_orders_by_page_structs::PageRequest;
use crate::structs::tool_functions::extract_u32;

pub async fn get_users_by_page(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<PageRequest>) -> impl IntoResponse {
    match extract_u32(body.page_number) {
        Ok(page_number) => {
            match extract_u32(body.rows_per_page) {
                Ok(rows_per_page) => {
                    let mut unlocked = pool.lock().await;

                }
                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
            }
        }
        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
    }
}