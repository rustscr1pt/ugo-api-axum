use std::sync::Arc;
use axum::{Router};
use tokio::sync::{Mutex, RwLock};
use crate::axum_routes::generic_replies::generic_replies::reject_unmatched_connection;
use crate::mysql::admins_filler::async_admins_filler::admins_filler;
use crate::mysql::admins_filler::fill_admins_sql::fill_admins_sql;
use crate::mysql::establish_connection::establish_connection;
use crate::mysql::refresh_pool_connection::refresh_pool_connection;
use crate::mysql::token_worker::token_worker::token_worker;
use crate::routers::crm::admin_actions_crm::admin_actions_crm;
use crate::routers::crm::login_actions_crm::login_actions_crm;
use crate::routers::crm::logs_actions_crm::logs_actions_crm;
use crate::routers::ugo_vape::ugo_vape_crm::ugo_vape_crm;
use crate::routers::ugo_vape::ugo_vape_web::ugo_vape_web;
use crate::routers::walgreen::walgreen_crm::walgreen_crm;
use crate::routers::walgreen::walgreen_web::walgreen_web;

use crate::structs::constants::DEPLOY_PORT;
use crate::structs::cors_layer::get_cors_layer;
use crate::structs::structs::{AdminsData, Token};

mod mysql;
mod structs;
mod axum_routes;
mod tests;
mod routers;

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
        .merge(ugo_vape_web(Arc::clone(&arc_sql)))
        .merge(ugo_vape_crm(Arc::clone(&arc_sql)))

        .merge(walgreen_web(Arc::clone(&arc_sql)))
        .merge(walgreen_crm(Arc::clone(&arc_sql)))

        .merge(admin_actions_crm(Arc::clone(&arc_sql), Arc::clone(&tokens_pool)))
        .merge(login_actions_crm(Arc::clone(&arc_sql), Arc::clone(&tokens_pool), Arc::clone(&arc_admins_pool)))
        .merge(logs_actions_crm(Arc::clone(&arc_sql)))

        .fallback(reject_unmatched_connection)
        .layer(get_cors_layer()); // Set up allowed methods + allowed-origins

    let addr = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", DEPLOY_PORT)).await.unwrap();
    println!("Running on http://localhost:{}", DEPLOY_PORT);
    axum::serve(addr, app).await.unwrap();
}
