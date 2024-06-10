use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::routes::ugo_vape::logs_routes::browse_logs_paginated::browse_logs_paginated::browse_logs_paginated;

pub fn logs_actions_crm(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    return Router::new()
        .route("/api/logs/browse", post(browse_logs_paginated))
            .layer(Extension(Arc::clone(&arc_sql)))
}