use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::routes::ugo_vape::orders_routes::add_note_to_order::add_note_to_order::add_note_to_order;
use crate::axum_routes::routes::ugo_vape::orders_routes::change_status_by_id::change_status_by_id::change_status_by_id;
use crate::axum_routes::routes::ugo_vape::orders_routes::get_filtered_orders_by_page::get_filtered_orders_by_page::get_filtered_orders_by_page;
use crate::axum_routes::routes::ugo_vape::orders_routes::get_orders_by_page::get_orders_by_page::get_orders_by_page;
use crate::axum_routes::routes::ugo_vape::orders_routes::remove_note_from_order::remove_note_from_order::remove_note_from_order;
use crate::axum_routes::routes::ugo_vape::orders_routes::remove_order_from_orders::remove_order_from_orders::remove_order_from_orders;

pub fn ugo_vape_crm(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    let router = Router::new()
        .route("/api/orders/get/page", post(get_orders_by_page))
        .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/page/filtered", post(get_filtered_orders_by_page))
        .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/change_status", post(change_status_by_id))
        .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/add_note", post(add_note_to_order))
        .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/remove_note", post(remove_note_from_order))
        .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/orders/remove_order", post(remove_order_from_orders))
        .layer(Extension(Arc::clone(&arc_sql)));
    return router
}