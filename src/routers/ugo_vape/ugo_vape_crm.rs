use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use rustelebot::types::BotInstance;
use tokio::sync::Mutex;
use crate::routers::ugo_vape::ugo_vape_crm_routes::add_note_to_order::add_note_to_order::add_note_to_order;
use crate::routers::ugo_vape::ugo_vape_crm_routes::change_status_by_id::change_status_by_id::change_status_by_id;
use crate::routers::ugo_vape::ugo_vape_crm_routes::get_filtered_orders_by_page::get_filtered_orders_by_page::get_filtered_orders_by_page;
use crate::routers::ugo_vape::ugo_vape_crm_routes::get_orders_by_page::get_orders_by_page::get_orders_by_page;
use crate::routers::ugo_vape::ugo_vape_crm_routes::remove_note_from_order::remove_note_from_order::remove_note_from_order;
use crate::routers::ugo_vape::ugo_vape_crm_routes::remove_order_from_orders::remove_order_from_orders::remove_order_from_orders;
use crate::structs::extension_structs::SQLAndTelegramWebExtension;

// Defined routes are used for actions with orders from ugo-vape in (__admin-panel)

pub fn ugo_vape_crm(arc_sql : Arc<Mutex<PooledConn>>, telegram_bot : Arc<Mutex<BotInstance>>) -> Router {
    return Router::new()
        .route("/api/orders/get/page", post(get_orders_by_page))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/page/filtered", post(get_filtered_orders_by_page))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/change_status", post(change_status_by_id))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/add_note", post(add_note_to_order))
            .layer(Extension(SQLAndTelegramWebExtension {
                arc_sql : Arc::clone(&arc_sql),
                telegram_bot : Arc::clone(&telegram_bot)
            }))
        .route("/api/orders/remove_note", post(remove_note_from_order))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/remove_order", post(remove_order_from_orders))
            .layer(Extension(Arc::clone(&arc_sql)))
}