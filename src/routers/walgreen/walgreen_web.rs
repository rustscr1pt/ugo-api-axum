use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use rustelebot::types::BotInstance;
use tokio::sync::Mutex;
use crate::routers::walgreen::walgreen_web_routes::get_phone_and_name::get_phone_and_name::get_phone_and_name;
use crate::structs::extension_structs::SQLAndTelegramWebExtension;

// Defined routes are used for actions in the walgreenlogistics.ru website

pub fn walgreen_web(arc_sql : Arc<Mutex<PooledConn>>, telegram_bot : Arc<Mutex<BotInstance>>) -> Router {
    return Router::new()
        .route("/api/walgreen/customer/write", post(get_phone_and_name))
            .layer(Extension(SQLAndTelegramWebExtension {
                arc_sql : Arc::clone(&arc_sql),
                telegram_bot : Arc::clone(&telegram_bot)
            }))
}