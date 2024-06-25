use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::{Mutex, RwLock};
use crate::routers::crm::admin_actions_crm_routes::add_admin_account::add_admin_account::add_admin_account;
use crate::routers::crm::admin_actions_crm_routes::fetch_admins_data::fetch_admins_data::fetch_admins_data;
use crate::routers::crm::admin_actions_crm_routes::remove_admin_account::remove_admin_account::remove_admin_account;
use crate::structs::structs::{AddAdminAccountExtensionBuilder, FetchAdminsDataExtension, Token};

// Defined routes are used for actions in the Admins screen (__admin-panel)

pub fn admin_actions_crm(arc_sql : Arc<Mutex<PooledConn>>, tokens_pool : Arc<RwLock<Vec<Token>>>) -> Router {
    return Router::new()
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
}