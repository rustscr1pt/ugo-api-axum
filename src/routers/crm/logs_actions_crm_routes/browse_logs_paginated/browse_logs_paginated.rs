use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::routers::crm::logs_actions_crm_routes::browse_logs_paginated::browse_logs_paginated_sql::browse_logs_paginated_sql;
use crate::routers::crm::logs_actions_crm_routes::browse_logs_paginated::total_rows_in_logs_sql::total_rows_in_logs_sql;
use crate::structs::structs::PageRequest;
use crate::structs::tool_functions::extract_u32;

pub async fn browse_logs_paginated(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<PageRequest>) -> impl IntoResponse {
    match extract_u32(body.page_number) {
        Ok(page_number) => {
            match extract_u32(body.rows_per_page) {
                Ok(rows_per_page) => {
                    let mut unlocked = pool.lock().await;
                    match browse_logs_paginated_sql(page_number, rows_per_page, &mut unlocked) {
                        Ok(result) => {
                            match total_rows_in_logs_sql(&mut unlocked) {
                                Ok(counter) => {
                                    reply_with_serialized_struct(true, counter, result)
                                }
                                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
                            }
                        }
                        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
                    }
                }
                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
            }
        }
        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
    }
}