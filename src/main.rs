use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use tokio::sync::{Mutex, RwLock};
use crate::axum_routes::routes::admin_management_routes::add_note_to_order::add_note_to_order::add_note_to_order;
use crate::axum_routes::routes::admin_management_routes::change_status_by_id::change_status_by_id::change_status_by_id;
use crate::axum_routes::routes::admin_management_routes::get_filtered_orders_by_page::get_filtered_orders_by_page::get_filtered_orders_by_page;
use crate::axum_routes::routes::admin_management_routes::get_orders_by_page::get_orders_by_page::get_orders_by_page;
use crate::axum_routes::routes::admin_management_routes::remove_note_from_order::remove_note_from_order::remove_note_from_order;
use crate::axum_routes::routes::admin_management_routes::remove_order_from_orders::remove_order_from_orders::remove_order_from_orders;
use crate::axum_routes::routes::admin_management_routes::write_route::write_route::write_route;
use crate::axum_routes::routes::login_routes::login_attempt_route::login_attempt_route::login_attempt_route;
use crate::axum_routes::routes::login_routes::login_attempt_route::login_attempt_route_extension_builder::LoginAttemptExtension;
use crate::mysql::admins_filler::async_admins_filler::admins_filler;
use crate::mysql::admins_filler::fill_admins_sql::fill_admins_sql;
use crate::mysql::establish_connection::establish_connection;
use crate::mysql::refresh_pool_connection::refresh_pool_connection;
use crate::mysql::token_worker::token_worker::token_worker;
use crate::structs::structs::{AdminsData, Token};

mod mysql;
mod structs;
mod axum_routes;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection()));
    let arc_admins_pool : Arc<RwLock<Vec<AdminsData>>> = Arc::new(RwLock::new(Vec::new())); // Arc holding actual admins accounts for check
    let tokens_pool : Arc<RwLock<Vec<Token>>> = Arc::new(RwLock::new(Vec::new())); // Arc holding active tokens

    fill_admins_sql(Arc::clone(&arc_sql), Arc::clone(&arc_admins_pool)).await;

    refresh_pool_connection(Arc::clone(&arc_sql)); // spawn a refresher for MySQL connection
    admins_filler(Arc::clone(&arc_sql), Arc::clone(&arc_admins_pool)); // spawn a refresher for Admins Accounts
    token_worker(Arc::clone(&tokens_pool)); // spawn an active tokens cleaner

    let app = Router::new()
        .route("/data/write", post(write_route))
            .layer(Extension(Arc::clone(&arc_sql)))
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
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/login/attempt", post(login_attempt_route))
            .layer(Extension(LoginAttemptExtension {
                tokens_pool: Arc::clone(&tokens_pool),
                admin_pool : Arc::clone(&arc_admins_pool)
            }));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Running on http://localhost:8000");
    axum::serve(addr, app).await.unwrap();
}
