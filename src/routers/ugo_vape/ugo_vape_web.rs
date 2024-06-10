use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::routers::ugo_vape::ugo_vape_web_routes::write_route::write_route::write_route;

// Defined routes are used for actions in the ugo-vape.ru website

pub fn ugo_vape_web(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    return Router::new()
        .route("/data/write", post(write_route))
            .layer(Extension(Arc::clone(&arc_sql)));
}