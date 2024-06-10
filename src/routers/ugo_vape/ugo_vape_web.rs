use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::routes::ugo_vape::orders_routes::write_route::write_route::write_route;

pub fn ugo_vape_web(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    return Router::new()
        .route("/data/write", post(write_route))
            .layer(Extension(Arc::clone(&arc_sql)));
}