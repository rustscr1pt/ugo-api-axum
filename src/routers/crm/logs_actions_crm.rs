use std::sync::Arc;
use axum::{Extension, Router};
use axum::routing::post;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::routers::crm::logs_actions_crm_routes::browse_logs_paginated::browse_logs_paginated::browse_logs_paginated;

// Routes for actions in the logs screen (__admin-panel)

pub fn logs_actions_crm(arc_sql : Arc<Mutex<PooledConn>>) -> Router {
    return Router::new()
        .route("/api/logs/browse", post(browse_logs_paginated))
            .layer(Extension(Arc::clone(&arc_sql)))
}