use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use mysql::PooledConn;
use tokio::sync::Mutex;
use crate::axum_routes::generic_replies::generic_replies::reply_with_serialized_struct;
use crate::axum_routes::routes::orders_routes::get_filtered_orders_by_page::get_filtered_orders_by_page_sql::get_filtered_orders_by_page_sql;
use crate::axum_routes::routes::orders_routes::get_filtered_orders_by_page::get_filtered_orders_by_page_structs::PageRequestFiltered;
use crate::axum_routes::routes::orders_routes::get_filtered_orders_by_page::total_rows_in_filtered_sql::total_rows_in_filtered_sql;
use crate::structs::tool_functions::extract_u16;

pub async fn get_filtered_orders_by_page(pool : Extension<Arc<Mutex<PooledConn>>>, Json(body) : Json<PageRequestFiltered>) -> impl IntoResponse {
    match extract_u16(body.page_number) {
        Ok(page_number) => {
            match extract_u16(body.rows_per_page) {
                Ok(rows_per_page) => {
                    let mut unlocked = pool.lock().await;
                    match get_filtered_orders_by_page_sql(page_number, rows_per_page, &body.filter_type, &body.filter_query, &mut unlocked) {
                        Ok(value) => {
                            match total_rows_in_filtered_sql(&mut unlocked, &body.filter_type, &body.filter_query) {
                                Ok(counter) => {
                                    reply_with_serialized_struct(true, counter, value)
                                }
                                Err(err) => {
                                    reply_with_serialized_struct(false, err, Vec::new())
                                }
                            }
                        }
                        Err(err) => {
                            reply_with_serialized_struct(false, err, Vec::new())
                        }
                    }
                }
                Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
            }
        }
        Err(err) => {reply_with_serialized_struct(false, err, Vec::new())}
    }
}