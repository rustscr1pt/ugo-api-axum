use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Extension, Router, ServiceExt};
use axum::routing::post;
use tokio::sync::{Mutex, RwLock};
use crate::axum_routes::generic_replies::generic_replies::reject_unmatched_connection;
use crate::axum_routes::routes::admin_management_routes::add_admin_account::add_admin_account::add_admin_account;
use crate::axum_routes::routes::admin_management_routes::add_admin_account::add_admin_account_extension_builder::AddAdminAccountExtensionBuilder;
use crate::axum_routes::routes::admin_management_routes::fetch_admins_data::fetch_admins_data::fetch_admins_data;
use crate::axum_routes::routes::admin_management_routes::fetch_admins_data::fetch_admins_data_extension_builder::FetchAdminsDataExtension;
use crate::axum_routes::routes::admin_management_routes::remove_admin_account::remove_admin_account::remove_admin_account;
use crate::axum_routes::routes::login_routes::login_attempt_route::login_attempt_route::login_attempt_route;
use crate::axum_routes::routes::login_routes::login_attempt_route::login_attempt_route_extension_builder::LoginAttemptExtension;
use crate::axum_routes::routes::login_routes::stealth_login_route::stealth_login::stealth_login;
use crate::axum_routes::routes::logs_routes::browse_logs_paginated::browse_logs_paginated::browse_logs_paginated;
use crate::axum_routes::routes::orders_routes::add_note_to_order::add_note_to_order::add_note_to_order;
use crate::axum_routes::routes::orders_routes::change_status_by_id::change_status_by_id::change_status_by_id;
use crate::axum_routes::routes::orders_routes::get_filtered_orders_by_page::get_filtered_orders_by_page::get_filtered_orders_by_page;
use crate::axum_routes::routes::orders_routes::get_orders_by_page::get_orders_by_page::get_orders_by_page;
use crate::axum_routes::routes::orders_routes::remove_note_from_order::remove_note_from_order::remove_note_from_order;
use crate::axum_routes::routes::orders_routes::remove_order_from_orders::remove_order_from_orders::remove_order_from_orders;
use crate::axum_routes::routes::orders_routes::write_route::write_route::write_route;
use crate::mysql::admins_filler::async_admins_filler::admins_filler;
use crate::mysql::admins_filler::fill_admins_sql::fill_admins_sql;
use crate::mysql::establish_connection::establish_connection;
use crate::mysql::refresh_pool_connection::refresh_pool_connection;
use crate::mysql::token_worker::token_worker::token_worker;
use crate::structs::cors_layer::get_cors_layer;
use crate::structs::structs::{AdminsData, Token};

mod mysql;
mod structs;
mod axum_routes;

#[tokio::main]
async fn main() {
    let arc_sql = Arc::new(Mutex::new(establish_connection())); // create a shared instance of connection
    let arc_admins_pool : Arc<RwLock<Vec<AdminsData>>> = Arc::new(RwLock::new(Vec::new())); // Arc holding actual admins accounts for check
    let tokens_pool : Arc<RwLock<Vec<Token>>> = Arc::new(RwLock::new(Vec::new())); // Arc holding active tokens

    fill_admins_sql(Arc::clone(&arc_sql), Arc::clone(&arc_admins_pool)).await; // fill admins at the boot of the server

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
                db_pool: Arc::clone(&arc_sql),
                tokens_pool: Arc::clone(&tokens_pool),
                admin_pool : Arc::clone(&arc_admins_pool)
            }))
        .route("/api/login/stealth", post(stealth_login))
            .layer(Extension(Arc::clone(&tokens_pool)))
        .route("/api/admins/fetch", post(fetch_admins_data))
            .layer(Extension(FetchAdminsDataExtension {
                db_pool: Arc::clone(&arc_sql),
                token_pool : Arc::clone(&tokens_pool)
            }))
        .route("/api/admins/remove", post(remove_admin_account))
            .layer(Extension(Arc::clone(&arc_sql)))
        .route("/api/admins/add", post(add_admin_account))
            .layer(Extension(AddAdminAccountExtensionBuilder {
                db_pool: Arc::clone(&arc_sql),
                token_pool: Arc::clone(&tokens_pool),
            }))
        .route("/api/logs/browse", post(browse_logs_paginated))
            .layer(Extension(Arc::clone(&arc_sql)))
        .fallback(reject_unmatched_connection)
        .layer(get_cors_layer()); // Set up allowed methods + allowed-origins

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Running on http://localhost:8000");
    axum::serve(addr, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
