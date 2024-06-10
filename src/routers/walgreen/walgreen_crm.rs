use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::routers::walgreen::walgreen_crm_routes::add_note_walgreen::add_note_walgreen::add_note_walgreen;
use crate::routers::walgreen::walgreen_crm_routes::change_status_walgreen::change_status_walgreen::change_status_walgreen;
use crate::routers::walgreen::walgreen_crm_routes::get_walgreen_users_by_page::get_walgreen_users_by_page::get_walgreen_users_by_page;
use crate::routers::walgreen::walgreen_crm_routes::get_walgreen_users_filtered_by_page::get_walgreen_users_filtered_by_page::get_walgreen_users_filtered_by_page;
use crate::routers::walgreen::walgreen_crm_routes::remove_note_walgreen::remove_note_walgreen::remove_note_walgreen;
use crate::routers::walgreen::walgreen_crm_routes::remove_order_walgreen::remove_order_walgreen::remove_order_walgreen;

// Defined routes are used for actions with orders from walgreenlogistics.ru in (__admin-panel)

pub fn walgreen_crm(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    return Router::new()
        .route("/api/walgreen/walgreen_requests/get/page", post(get_walgreen_users_by_page))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/walgreen/walgreen_requests/filtered/page", post(get_walgreen_users_filtered_by_page))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/walgreen/walgreen_requests/change_status", post(change_status_walgreen))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/walgreen/walgreen_requests/add_note", post(add_note_walgreen))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/walgreen/walgreen_requests/remove_note", post(remove_note_walgreen))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/walgreen/walgreen_requests/remove_order", post(remove_order_walgreen))
            .layer(Extension(Arc::clone(&arc_sql)))
}