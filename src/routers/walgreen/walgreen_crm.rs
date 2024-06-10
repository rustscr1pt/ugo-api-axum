use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::routes::walgreen::add_note_walgreen::add_note_walgreen::add_note_walgreen;
use crate::axum_routes::routes::walgreen::change_status_walgreen::change_status_walgreen::change_status_walgreen;
use crate::axum_routes::routes::walgreen::get_walgreen_users_by_page::get_walgreen_users_by_page::get_walgreen_users_by_page;
use crate::axum_routes::routes::walgreen::get_walgreen_users_filtered_by_page::get_walgreen_users_filtered_by_page::get_walgreen_users_filtered_by_page;
use crate::axum_routes::routes::walgreen::remove_note_walgreen::remove_note_walgreen::remove_note_walgreen;
use crate::axum_routes::routes::walgreen::remove_order_walgreen::remove_order_walgreen::remove_order_walgreen;

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