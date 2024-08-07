use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::{Mutex, RwLock};
use crate::routers::crm::login_actions_crm_routes::login_attempt_route::login_attempt_route::login_attempt_route;
use crate::routers::crm::login_actions_crm_routes::stealth_login_route::stealth_login::stealth_login;
use crate::structs::extension_structs::LoginAttemptExtension;
use crate::structs::structs::{AdminsData, Token};

// Defined routes are used for logging in (__admin-panel)

pub fn login_actions_crm(arc_sql : Arc<Mutex<PooledConn>>, tokens_pool : Arc<RwLock<Vec<Token>>>, arc_admins_pool : Arc<RwLock<Vec<AdminsData>>>) -> Router {
    return Router::new()
        .route("/api/login/attempt", post(login_attempt_route))
            .layer(Extension(LoginAttemptExtension {
                db_pool: Arc::clone(&arc_sql),
                tokens_pool: Arc::clone(&tokens_pool),
                admin_pool : Arc::clone(&arc_admins_pool)
            }))
        .route("/api/login/stealth", post(stealth_login))
            .layer(Extension(Arc::clone(&tokens_pool)))
}