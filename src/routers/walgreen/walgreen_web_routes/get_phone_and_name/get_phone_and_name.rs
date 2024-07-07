use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use crate::generic_replies::generic_replies::reply_with_message;
use crate::routers::walgreen::walgreen_web_routes::get_phone_and_name::get_phone_and_name_sql::get_phone_and_name_sql;
use crate::structs::structs::WriteDataBody;

pub async fn get_phone_and_name(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<WriteDataBody>) -> impl IntoResponse {
    let mut unlocked = pool.lock().await;
    match get_phone_and_name_sql(&mut unlocked, [body]) {
        Ok(_) => {
            reply_with_message(true, "Ваш запрос был отправлен! Мы ответим вам как можно скорее.")
        }
        Err(err) => {
            reply_with_message(false, err)
        }
    }
}