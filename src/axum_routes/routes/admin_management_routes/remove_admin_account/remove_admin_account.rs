use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_message;
use crate::axum_routes::routes::admin_management_routes::remove_admin_account::remove_admin_account_sql::remove_admin_account_sql;
use crate::axum_routes::routes::orders_routes::remove_order_from_orders::remove_order_from_orders_structs::RemoveObjectByID;
use crate::structs::tool_functions::extract_u16;

pub async fn remove_admin_account(pool : Extension<Arc<Mutex<PooledConn>>>,Json(body) : Json<RemoveObjectByID>) -> impl IntoResponse
{
    match extract_u16(body.id) {
        Ok(result) => {
            let mut connection = pool.lock().await;
            match remove_admin_account_sql(result, &mut connection) {
                Ok(()) => {
                    reply_with_message(true, "Admin account has been deleted")
                }
                Err(err) => {
                    reply_with_message(false, err)
                }
            }
        }
        Err(err) => {
            reply_with_message(false, err)
        }
    }
}